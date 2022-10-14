mod ast;
mod runtime;
mod util;

pub use ast::ValueType;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type UserDefinitions<'a> = Rc<RefCell<HashMap<String, Vec<Stack<'a>>>>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Stack<'a> {
    Value(ValueType<'a>),
    Operation(&'a str),
}

fn main() {
    let src = log_debug_time!(util::extract_src(), "Getting src from file");

    let mut stack = Vec::new();

    parse(&src, &mut stack);
    let leftover_stack = run(stack);

    #[cfg(debug_assertions)]
    println!("{:?}", leftover_stack);
}

fn parse<'a>(src: &'a str, stack: &mut Vec<Stack<'a>>) {
    let user_definitions = Rc::new(RefCell::new(HashMap::new()));

    log_debug_time!(ast::fill(src, stack, user_definitions), "Parsing src");
}

fn run(stack: Vec<Stack>) -> Vec<ValueType> {
    let mut value_stack: Vec<ValueType> = Vec::new();

    log_debug_time!(runtime::run(stack, &mut value_stack), "Executing from ast");

    value_stack
}
