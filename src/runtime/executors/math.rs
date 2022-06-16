use num::{traits::Pow, Num};

use crate::{
    ast::operation_extracts::{ADD, DIV, EQ, GEQ, GT, LEQ, LT, MOD, MUL, POW, SUB},
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
        Int(n1) => {
            if let ValueType::Int(n2) = arg2 {
                //  Due to lack of traits POW operation needs to be checked here.
                if operation == POW {
                    stack.push(Int(n1.pow(n2 as u32)));
                } else {
                    stack.push(Int(calculate_operation(operation, n1, n2)));
                }
            } else {
                mismatched_args("Int", arg2);
            }
        }
        Float(n1) => {
            if let ValueType::Float(n2) = arg2 {
                //  Due to lack of traits POW operation needs to be checked here.
                if operation == POW {
                    stack.push(Float(n1.pow(n2)));
                } else {
                    stack.push(Float(calculate_operation(operation, n1, n2)));
                }
            } else {
                mismatched_args("Float", arg2);
            }
        }
        _ => panic!("Expected a numeric type, got {:?}", arg1),
    }
}

fn calculate_operation<'a, T>(operation: &str, n1: T, n2: T) -> T
where
    T: Num,
{
    return match operation {
        ADD => n1 + n2,
        SUB => n1 - n2,
        MUL => n1 * n2,
        DIV => n1 / n2,
        MOD => n1 % n2,
        _ => panic!("{}: not a valid operation!", operation),
    };
}

pub fn execute_comparison(stack: &mut Vec<ValueType>, operation: &str) {
    check_argument_count(stack, 2);

    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();

    match arg1 {
        Int(n1) => {
            if let ValueType::Int(n2) = arg2 {
                stack.push(Bool(compare_operation(operation, n1, n2)));
            } else {
                mismatched_args("Int", arg2);
            }
        }
        Float(n1) => {
            if let ValueType::Float(n2) = arg2 {
                stack.push(Bool(compare_operation(operation, n1, n2)));
            } else {
                mismatched_args("Float", arg2);
            }
        }
        _ => panic!("Expected a numeric type, got {:?}", arg1),
    }
}

fn compare_operation<T: std::cmp::PartialOrd>(operation: &str, n1: T, n2: T) -> bool {
    match operation {
        LT => n1 < n2,
        GT => n1 > n2,
        EQ => n1 == n2,
        LEQ => n1 <= n2,
        GEQ => n1 >= n2,
        _ => panic!("{}: not a valid operation!", operation),
    }
}
