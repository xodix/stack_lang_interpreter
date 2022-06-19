mod math;

use crate::{
    ast::operation_extracts::{ADD, DIV, EQ, GEQ, GT, LEQ, LT, MOD, MUL, POW, SUB},
    ValueType,
};
use math::execute_common_math;

use self::math::execute_comparison;

use super::run_from_ast;

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
    check_argument_count(value_stack, 2);

    let arg1 = value_stack.pop().unwrap();
    let arg2 = value_stack.pop().unwrap();

    if arg1.truthy() {
        if let ValueType::Scope(stack) = arg2 {
            run_from_ast(stack, value_stack);
        } else {
            panic!(
                "{:?} is not a scope. Scope is needed for the if statement",
                arg2
            );
        }
    }
}

pub fn for_loop(value_stack: &mut Vec<ValueType>) {
    check_argument_count(value_stack, 2);

    let arg1 = value_stack.pop().unwrap();
    let arg2 = value_stack.pop().unwrap();

    match arg1 {
        ValueType::Int(range) => {
            if let ValueType::Scope(stack) = arg2 {
                for _ in 0..range {
                    // The scope is copied for every iteration. NOT GOOD
                    run_from_ast(stack.clone(), value_stack);
                }
            }
        }
        _ => panic!("{:?} cannot be a range for the for loop.", arg1),
    }
}

pub fn while_loop(value_stack: &mut Vec<ValueType>) {
    check_argument_count(value_stack, 2);

    let arg1 = value_stack.pop().unwrap();
    let arg2 = value_stack.pop().unwrap();

    value_stack.push(arg1);

    if let ValueType::Scope(stack) = arg2 {
        while value_stack[value_stack.len() - 1].truthy() {
            run_from_ast(stack.clone(), value_stack);
        }
    }
}

#[inline]
pub fn lt(stack: &mut Vec<ValueType>) {
    execute_comparison(stack, LT)
}

#[inline]
pub fn gt(stack: &mut Vec<ValueType>) {
    execute_comparison(stack, GT);
}

#[inline]
pub fn eq(stack: &mut Vec<ValueType>) {
    execute_comparison(stack, EQ);
}

#[inline]
pub fn leq(stack: &mut Vec<ValueType>) {
    execute_comparison(stack, LEQ);
}

#[inline]
pub fn geq(stack: &mut Vec<ValueType>) {
    execute_comparison(stack, GEQ);
}

pub fn or(stack: &mut Vec<ValueType>) {
    check_argument_count(stack, 2);

    let arg1 = stack.pop().unwrap();

    let arg2 = stack.pop().unwrap();

    if arg1.truthy() || arg2.truthy() {
        stack.push(ValueType::Bool(true));
    } else {
        stack.push(ValueType::Bool(false));
    }
}
pub fn and(stack: &mut Vec<ValueType>) {
    check_argument_count(stack, 2);

    let arg1 = stack.pop().unwrap();

    let arg2 = stack.pop().unwrap();

    if arg1.truthy() && arg2.truthy() {
        stack.push(ValueType::Bool(true));
    } else {
        stack.push(ValueType::Bool(false));
    }
}
