use crate::{
    ast::operand_extracts::{ADD, DIV, MUL, SUB},
    ValueType::{self, *},
};
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

#[inline]
fn check_argument_count(count: usize, stack: &Vec<ValueType>) {
    if stack.len() < count {
        panic!("Expected {} arguments, got {}", count, stack.len());
    }
}

#[inline]
fn mismatched_args<T: Debug>(expected: &str, given: T) {
    panic!("Expected {}, but got {:?}", expected, given);
}

/**
A size fits all solution for normalization of number types and common math operations.

Example:
```
    use crate::ValueType::*;

    let mut vec = vec![Int(1),Int(2)];
    execute_common_math(&mut vec, "+");

    assert_eq!(vec, vec![Int(3)]);
```

Checks for:
- Sufficient argument amount.
- Valid types.
 */
fn execute_common_math(stack: &mut Vec<ValueType>, oper: &str) {
    check_argument_count(2, stack);

    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();

    match arg1 {
        Int(n1) => {
            if let ValueType::Int(n2) = arg2 {
                stack.push(Int(calculate_oper(oper, n1, n2)));
            } else {
                mismatched_args("Int", arg2);
            }
        }
        Float(n1) => {
            if let ValueType::Float(n2) = arg2 {
                stack.push(Float(calculate_oper(oper, n1, n2)));
            } else {
                mismatched_args("Float", arg2);
            }
        }
        _ => panic!("Expected a numeric type, got {:?}", arg1),
    }
}

fn calculate_oper<T>(oper: &str, n1: T, n2: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    return match oper {
        ADD => n1 + n2,
        SUB => n1 - n2,
        MUL => n1 * n2,
        DIV => n1 / n2,
        _ => panic!("{}: not a valid operand!", oper),
    };
}

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

pub fn print(stack: &mut Vec<ValueType>) {
    check_argument_count(1, stack);

    let arg1 = stack.pop().unwrap();
    println!("{:?}", arg1);
}
