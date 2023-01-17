mod math;

use super::run;
use crate::{ast::extract::operation::*, util::error, Stack, ValueType};
use math::*;

#[inline]
fn check_argument_count(args: &[ValueType], needed: usize) -> Result<(), error::RuntimeError> {
    if args.len() < needed {
        Err(error::RuntimeError::InsufficientArguments {
            needed,
            got: args.len(),
            value_stack: args.to_vec(),
        })
    } else {
        Ok(())
    }
}

// Inlined functions below could be a part of match statement in execute_operation.
// However they are kept to give the project structure (every operation has a corresponding function).

#[inline]
pub fn add(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_common_math(stack, OperationType::Add)
}

#[inline]
pub fn mul(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_common_math(stack, OperationType::Mul)
}

#[inline]
pub fn sub(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_common_math(stack, OperationType::Sub)
}

#[inline]
pub fn div(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_common_math(stack, OperationType::Div)
}

#[inline]
pub fn pow(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_common_math(stack, OperationType::Pow)
}

#[inline]
pub fn modulo(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_common_math(stack, OperationType::Mod)
}

pub fn print(stack: &[ValueType]) -> Result<(), error::RuntimeError> {
    check_argument_count(stack, 1)?;

    let value = &stack[stack.len() - 1];
    print!("{}", value);

    Ok(())
}

pub fn println(stack: &[ValueType]) -> Result<(), error::RuntimeError> {
    check_argument_count(stack, 1)?;

    let value = &stack[stack.len() - 1];
    println!("{}", value);

    Ok(())
}

pub fn print_debug(stack: &[ValueType]) -> Result<(), error::RuntimeError> {
    check_argument_count(stack, 1)?;

    let value = &stack[stack.len() - 1];
    println!("{:?} is {} element in the stack", value, stack.len());

    Ok(())
}

pub fn print_debug_stack(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    check_argument_count(stack, 1)?;

    println!("{:#?}", stack);

    Ok(())
}

pub fn if_statement(value_stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    check_argument_count(value_stack, 2)?;

    let condition = value_stack.pop().unwrap();
    let scope = value_stack.pop().unwrap();

    if condition.truthy() {
        if let ValueType::Scope(stack) = scope {
            run(stack, value_stack)
        } else {
            Err(error::RuntimeError::MismatchedTypes {
                expected: "Scope".to_string(),
                got: scope.to_string(),
            })
        }
    } else {
        Ok(())
    }
}

pub fn for_loop(value_stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    check_argument_count(value_stack, 2)?;

    let condition = value_stack.pop().unwrap();
    let scope = value_stack.pop().unwrap();

    match condition {
        ValueType::Int(range) => {
            if let ValueType::Scope(stack) = scope {
                for _ in 0..range {
                    // The scope is copied for every iteration. NOT GOOD
                    run(stack.clone(), value_stack)?;
                }
            }
        }
        _ => {
            return Err(error::RuntimeError::MismatchedTypes {
                expected: "Int".to_string(),
                got: condition.to_string(),
            })
        }
    };

    Ok(())
}

pub fn while_loop(value_stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    check_argument_count(value_stack, 2)?;

    let condition = value_stack.pop().unwrap();
    let scope = value_stack.pop().unwrap();

    value_stack.push(condition);

    if let ValueType::Scope(stack) = scope {
        while value_stack[value_stack.len() - 1].truthy() {
            run(stack.clone(), value_stack)?;
        }
    }

    Ok(())
}

#[inline]
pub fn lt(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_comparison(stack, OperationType::Lt)
}

#[inline]
pub fn gt(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_comparison(stack, OperationType::Gt)
}

#[inline]
pub fn eq(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_comparison(stack, OperationType::Eq)
}

#[inline]
pub fn leq(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_comparison(stack, OperationType::Leq)
}

#[inline]
pub fn geq(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    execute_comparison(stack, OperationType::Geq)
}

pub fn or(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    check_argument_count(stack, 2)?;

    let condition1 = stack.pop().unwrap();

    let condition2 = stack.pop().unwrap();

    if condition1.truthy() || condition2.truthy() {
        stack.push(ValueType::Bool(true));
    } else {
        stack.push(ValueType::Bool(false));
    }

    Ok(())
}

pub fn and(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    check_argument_count(stack, 2)?;

    let condition1 = stack.pop().unwrap();

    let condition2 = stack.pop().unwrap();

    if condition1.truthy() && condition2.truthy() {
        stack.push(ValueType::Bool(true));
    } else {
        stack.push(ValueType::Bool(false));
    }

    Ok(())
}

pub fn not(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    check_argument_count(stack, 1)?;

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

    Ok(())
}

pub fn switch(stack: &mut [ValueType]) -> Result<(), error::RuntimeError> {
    check_argument_count(stack, 2)?;
    let length = stack.len();

    stack.swap(length - 1, length - 2);

    Ok(())
}

pub fn reverse(stack: &mut [ValueType]) -> Result<(), error::RuntimeError> {
    stack.reverse();

    Ok(())
}

pub fn pop(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    stack.pop();

    Ok(())
}

pub fn copy(stack: &mut Vec<ValueType>) -> Result<(), error::RuntimeError> {
    let last = stack[stack.len() - 1].clone();

    stack.push(last);
    Ok(())
}
