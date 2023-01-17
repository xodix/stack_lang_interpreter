#![allow(clippy::unit_arg)]
mod ast;
mod runtime;
mod util;

use ast::extract::operation::OperationType;
pub use ast::ValueType;
use std::{collections::HashMap, path::PathBuf};
use util::*;

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
    Run {
        path: PathBuf,
    },
    RunBinary {
        path: PathBuf,
    },
    Build {
        input_file: PathBuf,
        output_file: Option<PathBuf>,
    },
}

fn main() {
    let execution_mode = cli::get_execution_mode();

    let leftover_stack = match execution_mode {
        ExecutionMode::Run { path } => {
            let mut stack = Vec::new();

            let src = extract_src_text(path.clone());
            parse(&src, &mut stack, Some(path));

            run(stack)
        }
        ExecutionMode::RunBinary { path } => {
            let src = extract_src_bin(path);
            let bin: BinaryFile =
                postcard::from_bytes(&src).expect("Binary file format is not valid.");

            run(bin.stack)
        }
        ExecutionMode::Build {
            input_file,
            output_file,
        } => {
            let mut stack = Vec::new();

            let src = extract_src_text(input_file.clone());
            parse(&src, &mut stack, Some(input_file));

            let bin = BinaryFile { stack };
            let bytes =
                postcard::to_allocvec(&bin).expect("Couldn't convert stack to binary file.");

            write_file_bin(bytes, output_file.unwrap_or_else(|| PathBuf::from("a.out")));

            Vec::new()
        }
    };

    #[cfg(debug_assertions)]
    println!("{:?}", leftover_stack);
}

fn parse(src: &str, stack: &mut Vec<Stack>, path: Option<PathBuf>) {
    let mut user_definitions = HashMap::new();

    let mut line_width = 1;
    let mut line_height = 1;
    if let Err(err) = log_debug_time!(
        ast::fill(
            src,
            stack,
            &mut line_height,
            &mut line_width,
            &mut user_definitions
        ),
        "Parsing src"
    ) {
        println!(
            "\x1b[91mParsing Error at \x1b[4m\x1b[1m{}:{line_height}:{line_width}\x1b[0m\n{err}",
            path.unwrap_or_default().display()
        );
        std::process::exit(0);
    }
}

fn run(stack: Vec<Stack>) -> Vec<ValueType> {
    let mut value_stack: Vec<ValueType> = Vec::new();

    if let Err(err) = log_debug_time!(runtime::run(stack, &mut value_stack), "Executing from ast") {
        println!("\x1b[91mRuntime Error\x1b[0m\n{err}");
        std::process::exit(0);
    }

    value_stack
}
