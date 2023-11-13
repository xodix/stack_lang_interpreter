use num::{traits::Pow, Num};

use crate::{
    ast::extract::operation::*,
    util::error,
    ValueType::{self, *},
};

use std::fmt::Debug;

use super::check_argument_count;

#[inline]
fn mismatched_args<T: Debug>(expected: &str, got: T) -> error::runtime::Result<()> {
    Err(error::RuntimeError::MismatchedTypes {
        expected: expected.to_string(),
        got: format!("{:?}", got),
    })
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
pub fn execute_common_math(
    stack: &mut Vec<ValueType>,
    operation: OperationType,
) -> error::runtime::Result<()> {
    check_argument_count(stack, 2)?;

    let arg2 = stack.pop().unwrap();
    let arg1 = stack.pop().unwrap();

    match arg1 {
        Int(num1) => {
            if let ValueType::Int(num2) = arg2 {
                //  Due to lack of traits POW operation needs to be checked here.
                if operation == OperationType::Pow {
                    Ok(stack.push(Int(num1.pow(num2 as u32))))
                } else {
                    Ok(stack.push(Int(calculate_operation(operation, num1, num2)?)))
                }
            } else {
                mismatched_args("Int", arg2)
            }
        }
        Float(num1) => {
            if let ValueType::Float(num2) = arg2 {
                //  Due to lack of traits POW operation needs to be checked here.
                if operation == OperationType::Pow {
                    Ok(stack.push(Float(num1.pow(num2))))
                } else {
                    Ok(stack.push(Float(calculate_operation(operation, num1, num2)?)))
                }
            } else {
                mismatched_args("Float", arg2)
            }
        }
        _ => Err(error::RuntimeError::MismatchedTypes {
            expected: "a numeric type".to_string(),
            got: format!("{arg1:?}"),
        }),
    }
}

fn calculate_operation<T>(operation: OperationType, num1: T, num2: T) -> error::runtime::Result<T>
where
    T: Num,
{
    match operation {
        OperationType::Add => Ok(num1 + num2),
        OperationType::Sub => Ok(num1 - num2),
        OperationType::Mul => Ok(num1 * num2),
        OperationType::Div => Ok(num1 / num2),
        OperationType::Mod => Ok(num1 % num2),
        _ => Err(error::RuntimeError::InvalidOperation { operation }),
    }
}

pub fn execute_comparison(
    stack: &mut Vec<ValueType>,
    operation: OperationType,
) -> error::runtime::Result<()> {
    check_argument_count(stack, 2)?;

    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();

    match arg1 {
        Int(num1) => {
            if let ValueType::Int(num2) = arg2 {
                Ok(stack.push(Bool(compare_operation(operation, num1, num2)?)))
            } else {
                mismatched_args("Int", arg2)
            }
        }
        Float(num1) => {
            if let ValueType::Float(num2) = arg2 {
                Ok(stack.push(Bool(compare_operation(operation, num1, num2)?)))
            } else {
                mismatched_args("Float", arg2)
            }
        }
        _ => Err(error::RuntimeError::MismatchedTypes {
            expected: "a numeric type".to_string(),
            got: format!("{arg1:?}"),
        }),
    }
}

fn compare_operation<T: std::cmp::PartialOrd>(
    operation: OperationType,
    num1: T,
    num2: T,
) -> error::runtime::Result<bool> {
    match operation {
        OperationType::Lt => Ok(num1 < num2),
        OperationType::Gt => Ok(num1 > num2),
        OperationType::Eq => Ok(num1 == num2),
        OperationType::Leq => Ok(num1 <= num2),
        OperationType::Geq => Ok(num1 >= num2),
        _ => Err(error::RuntimeError::InvalidOperation { operation }),
    }
}
