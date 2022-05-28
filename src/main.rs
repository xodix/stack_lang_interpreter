mod ast;
mod runtime;
mod util;

pub use ast::ValueType;
use std::time::Instant;

#[derive(Debug, PartialEq)]
pub enum Stack<'a> {
    Value(ValueType),
    Operation(&'a str),
}

fn main() {
    let src = log_debug_time!(util::extract_src(), "Getting src from file");

    run(src.to_string());
}

fn run(src: String) {
    let mut stack = Vec::new();

    log_debug_time!(ast::fill_ast(&src, &mut stack), "Parsing src");
    log_debug_time!(runtime::run(stack), "Executing from ast");
}
