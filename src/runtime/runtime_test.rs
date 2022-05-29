use super::*;

#[test]
fn test_execute_add() {
    let mut stack = vec![ValueType::Int(2), ValueType::Int(2)];
    execute_operation(&mut stack, ADD);

    assert_eq!(ValueType::Int(4), stack[0]);
}

#[test]
fn test_execute_sub() {
    let mut stack = vec![ValueType::Float(2.5), ValueType::Float(2.)];
    execute_operation(&mut stack, SUB);

    assert_eq!(ValueType::Float(-0.5), stack[0]);
}

#[test]
fn test_execute_mul() {
    let mut stack = vec![ValueType::Float(1.5), ValueType::Float(2.0)];
    execute_operation(&mut stack, MUL);

    assert_eq!(ValueType::Float(3.0), stack[0]);
}

#[test]
fn test_execute_div() {
    let mut stack = vec![ValueType::Int(2), ValueType::Int(5)];
    execute_operation(&mut stack, DIV);

    assert_eq!(ValueType::Int(2), stack[0]);
}

#[test]
#[should_panic = "attempt to divide by zero"]
fn test_execute_div_by_zero() {
    let mut stack = vec![ValueType::Int(0), ValueType::Int(5)];
    execute_operation(&mut stack, DIV);

    assert_eq!(ValueType::Int(2), stack[0]);
}

#[test]
#[should_panic = "Expected Int, but got Float"]
fn test_execute_mul_mixed_types() {
    let mut stack = vec![ValueType::Float(5.0), ValueType::Int(0)];
    execute_operation(&mut stack, DIV);

    assert_eq!(ValueType::Int(2), stack[0]);
}

#[test]
fn test_execute_print() {
    let mut stack = vec![ValueType::Text("Hell'o, World!".to_string())];
    execute_operation(&mut stack, PRINT);
}

#[test]
fn test_execute_print_debug() {
    let mut stack = vec![ValueType::Text("Hell'o, World!".to_string())];
    execute_operation(&mut stack, PRINT_DEBUG);
}
