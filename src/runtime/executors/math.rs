use num::{traits::Pow, Num};

use crate::{
    ast::extract::operation::*,
    ValueType::{self, *},
};

use std::fmt::Debug;

use super::check_argument_count;

#[inline]
fn mismatched_args<T: Debug>(expected: &str, got: T) {
    panic!("Expected {}, but got {:?}", expected, got);
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
pub fn execute_common_math(stack: &mut Vec<ValueType>, operation: &str) {
    check_argument_count(stack, 2);

    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();

    match arg1 {
        Int(num1) => {
            if let ValueType::Int(num2) = arg2 {
                //  Due to lack of traits POW operation needs to be checked here.
                if operation == POW {
                    stack.push(Int(num1.pow(num2 as u32)));
                } else {
                    stack.push(Int(calculate_operation(operation, num1, num2)));
                }
            } else {
                mismatched_args("Int", arg2);
            }
        }
        Float(num1) => {
            if let ValueType::Float(num2) = arg2 {
                //  Due to lack of traits POW operation needs to be checked here.
                if operation == POW {
                    stack.push(Float(num1.pow(num2)));
                } else {
                    stack.push(Float(calculate_operation(operation, num1, num2)));
                }
            } else {
                mismatched_args("Float", arg2);
            }
        }
        _ => panic!("Expected a numeric type, got {:?}", arg1),
    }
}

fn calculate_operation<T>(operation: &str, num1: T, num2: T) -> T
where
    T: Num,
{
    return match operation {
        ADD => num1 + num2,
        SUB => num1 - num2,
        MUL => num1 * num2,
        DIV => num1 / num2,
        MOD => num1 % num2,
        _ => panic!("{}: not a valid operation!", operation),
    };
}

pub fn execute_comparison(stack: &mut Vec<ValueType>, operation: &str) {
    check_argument_count(stack, 2);

    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();

    match arg1 {
        Int(num1) => {
            if let ValueType::Int(num2) = arg2 {
                stack.push(Bool(compare_operation(operation, num1, num2)));
            } else {
                mismatched_args("Int", arg2);
            }
        }
        Float(num1) => {
            if let ValueType::Float(num2) = arg2 {
                stack.push(Bool(compare_operation(operation, num1, num2)));
            } else {
                mismatched_args("Float", arg2);
            }
        }
        _ => panic!("Expected a numeric type, got {:?}", arg1),
    }
}

fn compare_operation<T: std::cmp::PartialOrd>(operation: &str, num1: T, num2: T) -> bool {
    match operation {
        LT => num1 < num2,
        GT => num1 > num2,
        EQ => num1 == num2,
        LEQ => num1 <= num2,
        GEQ => num1 >= num2,
        _ => panic!("{}: not a valid operation!", operation),
    }
}
