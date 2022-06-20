#[cfg(test)]
mod runtime_test;

mod executors;

use self::executors::*;
use crate::{ast::operation_extracts::*, ast::ValueType, Stack};

pub fn run_from_ast<'a>(stack: Vec<Stack<'a>>, value_stack: &mut Vec<ValueType<'a>>) {
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
        NOT => not(stack),

        IF => if_statement(stack),
        FOR => for_loop(stack),
        WHILE => while_loop(stack),

        PRINT => print(stack),
        PRINTLN => println(stack),
        PRINT_DEBUG => print_debug(stack),

        SWITCH => switch(stack),
        REVERSE => reverse(stack),
        POP => pop(stack),
        COPY => copy(stack),
        _ => (),
    }
}
