mod math;

use super::run;
use crate::{ast::extract::operation::*, Stack, ValueType};
use math::*;

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

    let value = &stack[stack.len() - 1];
    print!("{}", value);
}

pub fn println(stack: &mut Vec<ValueType>) {
    check_argument_count(stack, 1);

    let value = &stack[stack.len() - 1];
    println!("{}", value);
}

pub fn print_debug(stack: &mut Vec<ValueType>) {
    check_argument_count(stack, 1);

    let value = &stack[stack.len() - 1];
    println!("{:?} is {} element in the stack", value, stack.len())
}

pub fn print_debug_stack(stack: &mut Vec<ValueType>) {
    check_argument_count(stack, 1);

    println!("{:#?}", stack);
}

pub fn if_statement(value_stack: &mut Vec<ValueType>) {
    check_argument_count(value_stack, 2);

    let condition = value_stack.pop().unwrap();
    let scope = value_stack.pop().unwrap();

    if condition.truthy() {
        if let ValueType::Scope(stack) = scope {
            run(stack, value_stack);
        } else {
            panic!(
                "{:?} is not a scope. Scope is needed for the if statement",
                scope
            );
        }
    }
}

pub fn for_loop(value_stack: &mut Vec<ValueType>) {
    check_argument_count(value_stack, 2);

    let condition = value_stack.pop().unwrap();
    let scope = value_stack.pop().unwrap();

    match condition {
        ValueType::Int(range) => {
            if let ValueType::Scope(stack) = scope {
                for _ in 0..range {
                    // The scope is copied for every iteration. NOT GOOD
                    run(stack.clone(), value_stack);
                }
            }
        }
        _ => panic!("{:?} cannot be a range for the for loop.", condition),
    }
}

pub fn while_loop(value_stack: &mut Vec<ValueType>) {
    check_argument_count(value_stack, 2);

    let condition = value_stack.pop().unwrap();
    let scope = value_stack.pop().unwrap();

    value_stack.push(condition);

    if let ValueType::Scope(stack) = scope {
        while value_stack[value_stack.len() - 1].truthy() {
            run(stack.clone(), value_stack);
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

    let condition1 = stack.pop().unwrap();

    let condition2 = stack.pop().unwrap();

    if condition1.truthy() || condition2.truthy() {
        stack.push(ValueType::Bool(true));
    } else {
        stack.push(ValueType::Bool(false));
    }
}

pub fn and(stack: &mut Vec<ValueType>) {
    check_argument_count(stack, 2);

    let condition1 = stack.pop().unwrap();

    let condition2 = stack.pop().unwrap();

    if condition1.truthy() && condition2.truthy() {
        stack.push(ValueType::Bool(true));
    } else {
        stack.push(ValueType::Bool(false));
    }
}

pub fn not(stack: &mut Vec<ValueType>) {
    check_argument_count(stack, 1);

    let condition = stack.pop().unwrap();

    stack.push(if condition.truthy() {
        match condition {
            ValueType::Int(_) => ValueType::Int(0),
            ValueType::Float(_) => ValueType::Float(0.0),
            ValueType::Text(_) => ValueType::Text("".to_string()),
            ValueType::Scope(_) => ValueType::Scope(vec![]),
            ValueType::Bool(_) => ValueType::Bool(false),
        }
    } else {
        match condition {
            ValueType::Int(_) => ValueType::Int(1),
            ValueType::Float(_) => ValueType::Float(1.0),
            ValueType::Text(_) => ValueType::Text("true".to_string()),
            ValueType::Scope(_) => ValueType::Scope(vec![Stack::Value(ValueType::Bool(true))]),
            ValueType::Bool(_) => ValueType::Bool(true),
        }
    });
}

pub fn switch(stack: &mut Vec<ValueType>) {
    check_argument_count(stack, 2);
    let length = stack.len();

    stack.swap(length - 1, length - 2);
}

pub fn reverse(stack: &mut [ValueType]) {
    stack.reverse();
}

pub fn pop(stack: &mut Vec<ValueType>) {
    stack.pop();
}

pub fn copy(stack: &mut Vec<ValueType>) {
    let last = stack[stack.len() - 1].clone();

    stack.push(last);
}
