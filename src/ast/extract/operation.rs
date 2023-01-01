use lazy_static::lazy_static;
use std::collections::HashMap;

use super::value::{register_constant, register_macro};
use crate::{Stack, ValueType};

pub const ADD: &str = "+";
pub const SUB: &str = "-";
pub const MUL: &str = "*";
pub const DIV: &str = "/";
pub const MOD: &str = "%";
pub const POW: &str = "^";

pub const LT: &str = "<";
pub const GT: &str = ">";
pub const EQ: &str = "==";
pub const LEQ: &str = "<=";
pub const GEQ: &str = ">=";

pub const OR: &str = "||";
pub const AND: &str = "&&";
pub const NOT: &str = "!";

pub const IF: &str = "if";
pub const FOR: &str = "for";
pub const WHILE: &str = "while";

pub const PRINT: &str = "print";
pub const PRINTLN: &str = "println";
pub const PRINT_DEBUG: &str = "print_debug";
pub const PRINT_DEBUG_STACK: &str = "print_debug_stack";

pub const SWITCH: &str = "switch";
pub const REVERSE: &str = "reverse";
pub const POP: &str = "pop";
pub const COPY: &str = "copy";

pub const MACRO: &str = "macro";
pub const CONST: &str = "const";

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Print,
    PrintDebug,
    PrintDebugStack,
    If,
    Lt,
    Gt,
    Eq,
    Leq,
    Geq,
    Or,
    And,
    For,
    While,
    Switch,
    Reverse,
    Pop,
    Not,
    Copy,
    Println,
    Macro,
    Const,
}

lazy_static! {
    static ref OPERANDS: HashMap<&'static str, OperationType> = HashMap::from([
        (ADD, OperationType::Add),
        (SUB, OperationType::Sub),
        (MUL, OperationType::Mul),
        (DIV, OperationType::Div),
        (MOD, OperationType::Mod),
        (POW, OperationType::Pow),
        (PRINT, OperationType::Print),
        (PRINT_DEBUG, OperationType::PrintDebug),
        (PRINT_DEBUG_STACK, OperationType::PrintDebugStack),
        (IF, OperationType::If),
        (LT, OperationType::Lt),
        (GT, OperationType::Gt),
        (EQ, OperationType::Eq),
        (LEQ, OperationType::Leq),
        (GEQ, OperationType::Geq),
        (OR, OperationType::Or),
        (AND, OperationType::And),
        (FOR, OperationType::For),
        (WHILE, OperationType::While),
        (SWITCH, OperationType::Switch),
        (REVERSE, OperationType::Reverse),
        (POP, OperationType::Pop),
        (NOT, OperationType::Not),
        (COPY, OperationType::Copy),
        (PRINTLN, OperationType::Println),
        (MACRO, OperationType::Macro),
        (CONST, OperationType::Const),
    ]);
    static ref KEYWORDS: HashMap<&'static str, ValueType> = HashMap::from([
        ("true", ValueType::Bool(true)),
        ("false", ValueType::Bool(false))
    ]);
}

pub fn keyword<'a>(
    src: &'a str,
    stack: &mut Vec<Stack>,
    i: &mut usize,
    user_definitions: &mut crate::HashMap<String, Vec<Stack>>,
) {
    let presumable_operand_index = src
        .find(|c| c == ' ' || c == '\n' || c == '\r')
        .unwrap_or(src.len());
    *i += presumable_operand_index - 1;
    let presumable_keyword = &src[..presumable_operand_index];

    if let Some(operation_type) = OPERANDS.get(&presumable_keyword) {
        if *operation_type == OperationType::Macro {
            register_macro(stack, user_definitions);
        } else if *operation_type == OperationType::Const {
            register_constant(stack, user_definitions);
        } else {
            stack.push(Stack::Operation(operation_type.clone()));
        }
    } else if let Some(function) = user_definitions.get(&presumable_keyword.to_string()) {
        stack.extend_from_slice(function);
    } else if let Some(value) = KEYWORDS.get(&presumable_keyword) {
        stack.push(Stack::Value(value.clone()));
    } else if !presumable_keyword.is_empty() {
        panic!("Unknown keyword: `{presumable_keyword}`");
    }
}
