use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        extract_num, extract_string,
        operation_extracts::{ADD, MUL},
        value_extracts::{extract_scope, register_user_definition},
    },
    Stack, ValueType,
};

use super::operation_extracts::extract_keyword;

#[test]
fn test_extract_operation() {
    let mut stack = vec![Stack::Value(ValueType::Int(3))];
    let user_definitions = Rc::new(RefCell::new(HashMap::new()));
    let mut current_index = 0;

    extract_keyword("+", &mut stack, &mut current_index, user_definitions);

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
    let user_definitions = Rc::new(RefCell::new(HashMap::new()));
    let mut current_index = 0;

    extract_keyword(
        "unknown_operand",
        &mut stack,
        &mut current_index,
        user_definitions,
    );

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
fn test_extract_int() {
    let mut stack = vec![Stack::Value(ValueType::Int(4))];
    let mut current_index = 0;

    extract_num("5_6_7_8_9", &mut stack, &mut current_index);

    assert_eq!(
        vec![
            Stack::Value(ValueType::Int(4)),
            Stack::Value(ValueType::Int(5_6_7_8_9)),
        ],
        stack
    );

    assert_eq!(current_index, 9);
}

#[test]
fn test_extract_float() {
    let mut stack = vec![Stack::Value(ValueType::Int(4))];
    let mut current_index = 0;

    extract_num("5_6._7_8_9", &mut stack, &mut current_index);

    assert_eq!(
        vec![
            Stack::Value(ValueType::Int(4)),
            Stack::Value(ValueType::Float(56.789)),
        ],
        stack
    );

    assert_eq!(current_index, 10);
}

#[test]
fn test_extract_string() {
    let mut stack = vec![Stack::Value(ValueType::Int(4))];
    let mut current_index = 0;

    extract_string(r#""Hello""#, &mut stack, &mut current_index);

    assert_eq!(
        vec![
            Stack::Value(ValueType::Int(4)),
            Stack::Value(ValueType::Text("Hello".to_string())),
        ],
        stack
    );

    assert_eq!(current_index, 7);
}

#[test]
fn test_extract_scope() {
    let mut stack = vec![Stack::Value(ValueType::Int(4))];
    let user_definitions = Rc::new(RefCell::new(HashMap::new()));
    let mut current_index = 0;

    stack.push(Stack::Value(ValueType::Scope(extract_scope(
        r#"{*}"#,
        &mut current_index,
        user_definitions,
    ))));

    assert_eq!(
        vec![
            Stack::Value(ValueType::Int(4)),
            Stack::Value(ValueType::Scope(vec![Stack::Operation(MUL)])),
        ],
        stack
    );

    assert_eq!(current_index, 3);
}

#[test]
fn test_extract_function() {
    let mut stack = vec![
        Stack::Value(ValueType::Int(4)),
        Stack::Value(ValueType::Scope(vec![
            Stack::Value(ValueType::Int(2)),
            Stack::Operation(MUL),
        ])),
        Stack::Value(ValueType::Text("double".to_string())),
    ];

    let user_definitions = Rc::new(RefCell::new(HashMap::new()));

    register_user_definition(&mut stack, user_definitions.clone());

    assert!(user_definitions
        .borrow()
        .contains_key(&"double".to_string()));

    extract_keyword("double", &mut stack, &mut 0, user_definitions.clone());

    assert_eq!(
        vec![
            Stack::Value(ValueType::Int(4)),
            Stack::Value(ValueType::Int(2)),
            Stack::Operation(MUL),
        ],
        stack
    );
}
