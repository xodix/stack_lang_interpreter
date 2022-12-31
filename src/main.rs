mod ast;
mod runtime;
mod util;

use ast::extract::operation::OperationType;
pub use ast::ValueType;
use std::{collections::HashMap, path::PathBuf, str::FromStr};

use crate::util::{cli, extract_src_bin, extract_src_text, write_file_bin};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct BinaryFile {
    stack: Vec<Stack>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Stack {
    Value(ValueType),
    Operation(OperationType),
}

pub enum ExecutionMode {
    Run(PathBuf),
    RunBinary(PathBuf),
    Build(PathBuf, Option<PathBuf>),
}

fn main() {
    let execution_mode = cli::get_execution_mode();

    let leftover_stack = match execution_mode {
        ExecutionMode::Run(path) => {
            let mut stack = Vec::new();

            let src = extract_src_text(path);
            parse(&src, &mut stack);

            run(stack)
        }
        ExecutionMode::RunBinary(path) => {
            let src = extract_src_bin(path);
            let bin: BinaryFile =
                postcard::from_bytes(&src).expect("Binary file format is not valid.");

            run(bin.stack)
        }
        ExecutionMode::Build(input_file, output_file) => {
            let mut stack = Vec::new();

            let src = extract_src_text(input_file);
            parse(&src, &mut stack);

            let bin = BinaryFile { stack };
            let bytes =
                postcard::to_allocvec(&bin).expect("Couldn't convert stack to binary file.");

            write_file_bin(
                bytes,
                output_file.unwrap_or(PathBuf::from_str("a.out").expect("Could not build path")),
            );

            Vec::new()
        }
    };

    #[cfg(debug_assertions)]
    println!("{:?}", leftover_stack);
}

fn parse<'a>(src: &'a str, stack: &mut Vec<Stack>) {
    let mut user_definitions = HashMap::new();

    log_debug_time!(ast::fill(src, stack, &mut user_definitions), "Parsing src");
}

fn run(stack: Vec<Stack>) -> Vec<ValueType> {
    let mut value_stack: Vec<ValueType> = Vec::new();

    log_debug_time!(runtime::run(stack, &mut value_stack), "Executing from ast");

    value_stack
}
