use crate::{ast::operation_extracts::ADD, Stack, ValueType};

use super::operation_extracts::extract_keyword;

#[test]
fn test_extract_operation() {
    let mut stack = vec![Stack::Value(ValueType::Int(3))];
    let mut current_index = 0;

    extract_keyword("+", &mut stack, &mut current_index);

    assert_eq!(
        vec![Stack::Value(ValueType::Int(3)), Stack::Operation(ADD)],
        stack
    );

    assert_eq!(current_index, 1);
}

#[test]
#[should_panic]
fn test_extract_unknown_operation() {
    let mut stack = vec![Stack::Value(ValueType::Int(3))];
    let mut current_index = 0;

    extract_keyword("unknown_operand", &mut stack, &mut current_index);

    assert_eq!(
        vec![
            Stack::Value(ValueType::Int(3)),
            Stack::Operation("unknown_operand")
        ],
        stack
    );

    assert_eq!(current_index, 15);
}

#[test]
fn test_extract_int() {}
