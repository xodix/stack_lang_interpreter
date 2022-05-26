mod executors;
use self::executors::{add, div, mul, print, sub};
use crate::{ast::operand_extracts::*, ast::ValueType, Stack};

pub fn run(stack: Vec<crate::Stack>) {
    let mut value_stack: Vec<ValueType> = Vec::new();

    for elem in stack.into_iter() {
        match elem {
            Stack::Operation(oper) => execute_operation(&mut value_stack, oper),
            Stack::Value(val) => value_stack.push(val),
        }
    }
}

fn execute_operation(stack: &mut Vec<ValueType>, operator: &str) {
    match operator {
        ADD => add(stack),
        SUB => sub(stack),
        MUL => mul(stack),
        DIV => div(stack),
        PRINT => print(stack),
        _ => (),
    }
}
