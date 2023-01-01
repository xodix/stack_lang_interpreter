use super::*;
use crate::ast::extract::operation::*;

#[test]
fn test_execute_add() {
    let mut stack = vec![ValueType::Int(2), ValueType::Int(2)];
    execute_operation(&mut stack, OperationType::Add);

    assert_eq!(ValueType::Int(4), stack[0]);
}

#[test]
fn test_execute_sub() {
    let mut stack = vec![ValueType::Float(2.5), ValueType::Float(2.)];
    execute_operation(&mut stack, OperationType::Sub);

    assert_eq!(ValueType::Float(-0.5), stack[0]);
}

#[test]
fn test_execute_mul() {
    let mut stack = vec![ValueType::Float(1.5), ValueType::Float(2.0)];
    execute_operation(&mut stack, OperationType::Mul);

    assert_eq!(ValueType::Float(3.0), stack[0]);
}

#[test]
fn test_execute_div() {
    let mut stack = vec![ValueType::Int(2), ValueType::Int(5)];
    execute_operation(&mut stack, OperationType::Div);

    assert_eq!(ValueType::Int(2), stack[0]);
}

#[test]
#[should_panic = "attempt to divide by zero"]
fn test_execute_div_by_zero() {
    let mut stack = vec![ValueType::Int(0), ValueType::Int(5)];
    execute_operation(&mut stack, OperationType::Div);

    assert_eq!(ValueType::Int(2), stack[0]);
}

#[test]
#[should_panic = "Expected Int, but got Float"]
fn test_execute_mul_mixed_types() {
    let mut stack = vec![ValueType::Float(5.0), ValueType::Int(0)];
    execute_operation(&mut stack, OperationType::Div);

    assert_eq!(ValueType::Int(2), stack[0]);
}

#[test]
fn test_execute_pow() {
    let mut stack = vec![ValueType::Int(3), ValueType::Int(2)];
    execute_operation(&mut stack, OperationType::Pow);

    assert_eq!(ValueType::Int(8), stack[0]);
}

#[test]
fn test_execute_mod() {
    let mut stack = vec![ValueType::Int(2), ValueType::Int(3)];
    execute_operation(&mut stack, OperationType::Mod);

    assert_eq!(ValueType::Int(1), stack[0]);
}

#[test]
fn test_execute_print() {
    let mut stack = vec![ValueType::Text("Hell'o, World!".to_string())];
    execute_operation(&mut stack, OperationType::Print);
}

#[test]
fn test_execute_print_debug() {
    let mut stack = vec![ValueType::Text("Hell'o, World!".to_string())];
    execute_operation(&mut stack, OperationType::PrintDebug);
}

#[test]
fn test_execute_if_statement_true() {
    let mut stack = vec![
        ValueType::Int(1),
        ValueType::Int(2),
        ValueType::Scope(vec![Stack::Operation(OperationType::Mul)]),
        ValueType::Bool(true),
    ];

    execute_operation(&mut stack, OperationType::If);

    assert_eq!(stack, vec![ValueType::Int(2)]);
}

#[test]
fn test_execute_if_statement_false() {
    let mut stack = vec![
        ValueType::Int(1),
        ValueType::Int(2),
        ValueType::Scope(vec![Stack::Operation(OperationType::Mul)]),
        ValueType::Bool(false),
    ];

    execute_operation(&mut stack, OperationType::If);

    assert_eq!(stack, vec![ValueType::Int(1), ValueType::Int(2)]);
}

#[test]
fn test_execute_lt() {
    let mut stack = vec![ValueType::Int(2), ValueType::Int(1)];

    execute_operation(&mut stack, OperationType::Lt);

    // 1 is less than 2
    assert_eq!(stack, vec![ValueType::Bool(true)]);
}

#[test]
fn test_execute_gt() {
    let mut stack = vec![ValueType::Int(1), ValueType::Int(2)];

    execute_operation(&mut stack, OperationType::Gt);

    // 2 is grater than 1
    assert_eq!(stack, vec![ValueType::Bool(true)]);
}

#[test]
fn test_execute_eq() {
    let mut stack = vec![ValueType::Int(5), ValueType::Int(5)];

    execute_operation(&mut stack, OperationType::Eq);

    // 5 is equal to 5
    assert_eq!(stack, vec![ValueType::Bool(true)]);
}

#[test]
fn test_execute_leq() {
    let mut stack = vec![ValueType::Int(5), ValueType::Int(5)];

    execute_operation(&mut stack, OperationType::Leq);

    // 5 is equal or less than 5
    assert_eq!(stack, vec![ValueType::Bool(true)]);
}

#[test]
fn test_execute_geq() {
    let mut stack = vec![ValueType::Int(5), ValueType::Int(5)];

    execute_operation(&mut stack, OperationType::Geq);

    // 5 is equal or grater than 5
    assert_eq!(stack, vec![ValueType::Bool(true)]);
}

#[test]
fn test_execute_or() {
    let mut stack = vec![ValueType::Bool(false), ValueType::Bool(true)];

    execute_operation(&mut stack, OperationType::Or);

    // one of: [false, true] is true
    assert_eq!(stack, vec![ValueType::Bool(true)]);
}

#[test]
fn test_execute_and() {
    let mut stack = vec![ValueType::Bool(true), ValueType::Bool(true)];

    execute_operation(&mut stack, OperationType::Or);

    // all of: [true, true] are true
    assert_eq!(stack, vec![ValueType::Bool(true)]);
}

#[test]
fn test_execute_not() {
    let mut stack = vec![ValueType::Bool(true)];

    execute_operation(&mut stack, OperationType::Not);

    assert_eq!(stack, vec![ValueType::Bool(false)]);
}

#[test]
fn test_execute_for() {
    let mut stack = vec![
        ValueType::Int(2),
        ValueType::Scope(vec![
            Stack::Value(ValueType::Int(1)),
            Stack::Operation(OperationType::Add),
            Stack::Operation(OperationType::Print),
        ]),
        ValueType::Int(3),
    ];

    execute_operation(&mut stack, OperationType::For);

    // 2 + 3 = 5
    assert_eq!(vec![ValueType::Int(5)], stack);
}

#[test]
fn test_execute_while() {
    let mut stack = vec![
        ValueType::Scope(vec![
            Stack::Value(ValueType::Int(-1)),
            Stack::Operation(OperationType::Add),
            Stack::Operation(OperationType::Print),
        ]),
        ValueType::Int(3),
    ];

    execute_operation(&mut stack, OperationType::While);

    // top value needs to be falsy to stop execution
    assert_eq!(vec![ValueType::Int(0)], stack);
}

#[test]
fn test_execute_switch() {
    let mut stack = vec![ValueType::Int(1), ValueType::Int(2), ValueType::Int(3)];

    execute_operation(&mut stack, OperationType::Switch);

    // top value needs to be falsy to stop execution
    assert_eq!(
        vec![ValueType::Int(1), ValueType::Int(3), ValueType::Int(2)],
        stack
    );
}

#[test]
fn test_execute_reverse() {
    let mut stack = vec![ValueType::Int(1), ValueType::Int(2), ValueType::Int(3)];

    execute_operation(&mut stack, OperationType::Reverse);

    // top value needs to be falsy to stop execution
    assert_eq!(
        vec![ValueType::Int(3), ValueType::Int(2), ValueType::Int(1)],
        stack
    );
}

#[test]
fn test_execute_pop() {
    let mut stack = vec![ValueType::Int(1), ValueType::Int(2), ValueType::Int(3)];

    execute_operation(&mut stack, OperationType::Pop);

    // top value needs to be falsy to stop execution
    assert_eq!(vec![ValueType::Int(1), ValueType::Int(2)], stack);
}
