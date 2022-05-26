pub use ast::ValueType;

mod ast;
mod runtime;

#[derive(Debug, PartialEq)]
pub enum Stack<'a> {
    Value(ValueType),
    Operation(&'a str),
}

fn main() {
    #[cfg(debug_assertions)]
    run("-125 2525 + print".to_string());

    #[cfg(not(debug_assertions))]
    {
        extract_path();
        extract_src();
        run();
    }
}

fn run(src: String) {
    let mut stack = Vec::new();

    ast::fill_ast(&src, &mut stack);
    runtime::run(stack);
}
