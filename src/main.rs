mod ast;
mod runtime;
mod util;

use std::time::Instant;

pub use ast::ValueType;

#[derive(Debug, PartialEq)]
pub enum Stack<'a> {
    Value(ValueType),
    Operation(&'a str),
}

fn main() {
    let now = Instant::now();
    let src = util::extract_src();
    println!("Getting src from file took: {:?}", now.elapsed());

    run(src);
}

fn run(src: String) {
    let mut stack = Vec::new();

    let now = Instant::now();
    ast::fill_ast(&src, &mut stack);
    println!("Filling the ast took {:?}", now.elapsed());

    let now = Instant::now();
    runtime::run(stack);
    println!("Executing from ast took: {:?}", now.elapsed());
}
