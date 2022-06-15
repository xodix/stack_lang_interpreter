mod math;

use crate::{
    ast::operation_extracts::{ADD, DIV, MOD, MUL, POW, SUB},
    ValueType,
};
use math::execute_common_math;

use super::run;

#[inline]
fn check_argument_count(stack: &Vec<ValueType>, count: usize) {
    if stack.len() < count {
        panic!(
            "Expected {} arguments, got {};\nStack values: {:?}",
            count,
            stack.len(),
            stack
        );
    }
}

// Inlined functions below could be a part of match statement in execute_operation.
// However they are kept to give the project structure (every operation has a corresponding function).

#[inline]
pub fn add(stack: &mut Vec<ValueType>) {
    execute_common_math(stack, ADD);
}

#[inline]
pub fn mul(stack: &mut Vec<ValueType>) {
    execute_common_math(stack, MUL);
}

#[inline]
pub fn sub(stack: &mut Vec<ValueType>) {
    execute_common_math(stack, SUB);
}

#[inline]
pub fn div(stack: &mut Vec<ValueType>) {
    execute_common_math(stack, DIV);
}

#[inline]
pub fn pow(stack: &mut Vec<ValueType>) {
    execute_common_math(stack, POW);
}

#[inline]
pub fn modulo(stack: &mut Vec<ValueType>) {
    execute_common_math(stack, MOD);
}

pub fn print(stack: &mut Vec<ValueType>) {
    check_argument_count(stack, 1);

    let arg1 = &stack[stack.len() - 1];
    println!("{}", arg1);
}

pub fn print_debug(stack: &mut Vec<ValueType>) {
    check_argument_count(stack, 1);

    let arg1 = &stack[stack.len() - 1];
    println!("{:?} is {} element in the stack", arg1, stack.len())
}

pub fn if_statement(value_stack: &mut Vec<ValueType>) {
    let arg1 = value_stack
        .pop()
        .expect("Not enough arguments to execute if operation");
    let arg2 = value_stack
        .pop()
        .expect("Not enough arguments to execute if operation.");

    if arg1.truthy() {
        if let ValueType::Scope(stack) = arg2 {
            run(stack, value_stack);
        }
    }
}
