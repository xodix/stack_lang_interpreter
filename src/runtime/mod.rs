#[cfg(test)]
mod runtime_test;

mod executors;

use self::executors::*;
use crate::{ast::operation_extracts::*, ast::ValueType, Stack};

pub fn run<'a>(stack: Vec<Stack<'a>>, value_stack: &mut Vec<ValueType<'a>>) {
    for elem in stack.into_iter() {
        match elem {
            Stack::Operation(operation) => execute_operation(value_stack, operation),
            Stack::Value(val) => value_stack.push(val),
        }
    }
}

fn execute_operation(stack: &mut Vec<ValueType>, operation: &str) {
    match operation {
        ADD => add(stack),
        SUB => sub(stack),
        MUL => mul(stack),
        DIV => div(stack),
        MOD => modulo(stack),
        POW => pow(stack),

        LT => lt(stack),
        GT => gt(stack),
        EQ => eq(stack),
        LEQ => leq(stack),
        GEQ => geq(stack),

        OR => or(stack),
        AND => and(stack),

        IF => if_statement(stack),

        PRINT => print(stack),
        PRINT_DEBUG => print_debug(stack),
        _ => (),
    }
}
