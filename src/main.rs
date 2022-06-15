mod ast;
mod runtime;
mod util;

pub use ast::ValueType;

#[derive(Debug, PartialEq, Clone)]
pub enum Stack<'a> {
    Value(ValueType<'a>),
    Operation(&'a str),
}

fn main() {
    let src = log_debug_time!(util::extract_src(), "Getting src from file");

    run(src.to_string());
}

fn run(src: String) {
    let mut stack = Vec::new();
    let mut value_stack: Vec<ValueType> = Vec::new();

    log_debug_time!(ast::fill_ast(&src, &mut stack), "Parsing src");
    log_debug_time!(runtime::run(stack, &mut value_stack), "Executing from ast");

    println!("{:?}", value_stack);
}
