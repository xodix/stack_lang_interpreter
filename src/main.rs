#![allow(clippy::unit_arg)]
mod ast;
mod runtime;
mod util;

use ast::*;
use std::{collections::HashMap, path::PathBuf};
use util::{cli::ExecutionMode::*, *};

const DEFAULT_STACK_SIZE: usize = 128;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Stack {
    Value(ValueType),
    Operation(OperationType),
}

fn main() {
    let execution_mode = cli::get_execution_mode();

    let _leftover_stack = match execution_mode {
        Run { path } => {
            let mut stack = Vec::with_capacity(DEFAULT_STACK_SIZE);

            let src = file::extract_text(&path);
            parse(&src, &mut stack, Some(path));

            execute(stack)
        }
        RunBinary { path } => {
            let src = file::extract_bin(&path);
            let bin: file::Binary = log_debug_time!(
                postcard::from_bytes(&src).expect("Binary file format is not valid."),
                "Building from binary."
            );

            execute(bin.stack)
        }
        Build {
            input_file,
            output_file,
        } => {
            let mut stack = Vec::with_capacity(DEFAULT_STACK_SIZE);

            let src = file::extract_text(&input_file);
            parse(&src, &mut stack, Some(input_file));

            let bin = file::Binary { stack };
            let bytes =
                postcard::to_allocvec(&bin).expect("Couldn't convert stack to binary file.");

            file::write_bin(
                bytes,
                &output_file.unwrap_or_else(|| PathBuf::from("a.out")),
            );

            Vec::new()
        }
    };

    #[cfg(debug_assertions)]
    println!("{:?}", _leftover_stack);
}

fn parse(src: &str, stack: &mut Vec<Stack>, path: Option<PathBuf>) {
    let mut user_definitions = HashMap::with_capacity(DEFAULT_STACK_SIZE);

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
        const RED: &str = "\x1b[91m";
        const UNDERLINE: &str = "\x1b[4m";
        const BOLD: &str = "\x1b[1m";
        const RESET_FORMATTING: &str = "\x1b[0m";

        println!(
            "{RED}Parsing Error at {UNDERLINE}{BOLD}{}:{line_height}:{line_width}{RESET_FORMATTING}\n{err}",
            path.unwrap_or_default().display()
        );
        std::process::exit(1);
    }
}

fn execute(stack: Vec<Stack>) -> Vec<ValueType> {
    let mut value_stack: Vec<ValueType> = Vec::with_capacity(DEFAULT_STACK_SIZE);

    if let Err(err) = log_debug_time!(runtime::run(stack, &mut value_stack), "Executing from ast") {
        const RED: &str = "\x1b[91m";
        const RESET_FORMATTING: &str = "\x1b[0m";

        println!("{RED}Runtime Error{RESET_FORMATTING}\n{err}");
        std::process::exit(1);
    }

    value_stack
}
