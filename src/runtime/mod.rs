#[cfg(test)]
mod runtime_test;

mod executors;

use self::executors::*;
use crate::{
    ast::{extract::operation::*, ValueType},
    Stack,
};

pub fn run<'a>(stack: Vec<Stack<'a>>, value_stack: &mut Vec<ValueType<'a>>) {
    for element in stack.into_iter() {
        match element {
            Stack::Operation(operation) => execute_operation(value_stack, operation),
            Stack::Value(value) => value_stack.push(value),
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
        PRINT_DEBUG_STACK => print_debug_stack(stack),

        SWITCH => switch(stack),
        REVERSE => reverse(stack),
        POP => pop(stack),
        COPY => copy(stack),
        _ => panic!("Invalid operation `{}`.", operation),
    }
}
