mod ast;
mod runtime;
mod util;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub use ast::ValueType;

#[derive(Debug, PartialEq, Clone)]
pub enum Stack<'a> {
    Value(ValueType<'a>),
    Operation(&'a str),
}

pub type UserDefinitions<'a> = Rc<RefCell<HashMap<String, Vec<Stack<'a>>>>>;

fn main() {
    let src = log_debug_time!(util::extract_src(), "Getting src from file");

    run(src);
}

fn run(src: String) {
    let mut stack = Vec::new();
    let mut value_stack: Vec<ValueType> = Vec::new();

    // User defined actions are inlined during parsing. Thus they are freed here.
    let user_definitions = Rc::new(RefCell::new(HashMap::new()));

    log_debug_time!(
        ast::fill_ast(&src, &mut stack, user_definitions),
        "Parsing src"
    );
    log_debug_time!(
        runtime::run_from_ast(stack, &mut value_stack),
        "Executing from ast"
    );

    #[cfg(debug_assertions)]
    println!("{:?}", value_stack);
}
