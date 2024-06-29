#[cfg(test)]
mod runtime_test;

mod executors;

use self::executors::*;
use crate::{
    ast::{
        extract::operation::OperationType::{self, *},
        ValueType,
    },
    util::error,
    Stack,
};

pub fn run(stack: Vec<Stack>, value_stack: &mut Vec<ValueType>) -> error::runtime::Result<()> {
    for element in stack.into_iter() {
        match element {
            Stack::Operation(operation) => execute_operation(value_stack, operation)?,
            // TODO
            Stack::Value(value) => value_stack.push(value),
        }
    }

    Ok(())
}

fn execute_operation(
    stack: &mut Vec<ValueType>,
    operation: OperationType,
) -> error::runtime::Result<()> {
    match operation {
        Add => add(stack),
        Sub => sub(stack),
        Mul => mul(stack),
        Div => div(stack),
        Mod => modulo(stack),
        Pow => pow(stack),

        Lt => lt(stack),
        Gt => gt(stack),
        Eq => eq(stack),
        Leq => leq(stack),
        Geq => geq(stack),

        Or => or(stack),
        And => and(stack),
        Not => not(stack),

        If => if_statement(stack),
        For => for_loop(stack),
        While => while_loop(stack),

        Print => print(stack),
        Println => println(stack),
        PrintDebug => print_debug(stack),
        PrintDebugStack => print_debug_stack(stack),

        Switch => switch(stack),
        Reverse => reverse(stack),
        Pop => pop(stack),
        Copy => copy(stack),
        _ => Err(error::RuntimeError::InvalidOperation { operation }),
    }
}
