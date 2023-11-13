use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::{util::error, Stack, ValueType};

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
        // arithmetic
        ("+", OperationType::Add),
        ("-", OperationType::Sub),
        ("*", OperationType::Mul),
        ("/", OperationType::Div),
        ("%", OperationType::Mod),
        ("^", OperationType::Pow),
        // control flow
        ("if", OperationType::If),
        ("for", OperationType::For),
        ("while", OperationType::While),
        // conditions
        ("<", OperationType::Lt),
        (">", OperationType::Gt),
        ("==", OperationType::Eq),
        ("<=", OperationType::Leq),
        (">=", OperationType::Geq),
        ("||", OperationType::Or),
        ("&&", OperationType::And),
        ("!", OperationType::Not),
        // console
        ("print", OperationType::Print),
        ("println", OperationType::Println),
        ("print_debug", OperationType::PrintDebug),
        ("print_debug_stack", OperationType::PrintDebugStack),
        // stack
        ("switch", OperationType::Switch),
        ("reverse", OperationType::Reverse),
        ("pop", OperationType::Pop),
        ("copy", OperationType::Copy),
        // register
        ("macro", OperationType::Macro),
        ("const", OperationType::Const),
    ]);
    static ref KEYWORDS: HashMap<&'static str, ValueType> = HashMap::from([
        ("true", ValueType::Bool(true)),
        ("false", ValueType::Bool(false))
    ]);
}

pub fn keyword(
    src: &str,
    stack: &mut Vec<Stack>,
    i: &mut usize,
    user_definitions: &mut crate::HashMap<String, Vec<Stack>>,
) -> error::parsing::Result<()> {
    let presumable_keyword_index = src
        .find(|c| c == ' ' || c == '\r' || c == '\n')
        .unwrap_or(src.len());
    *i += presumable_keyword_index - 1;
    let presumable_keyword = &src[..presumable_keyword_index];

    if let Some(operation_type) = OPERANDS.get(&presumable_keyword) {
        if *operation_type == OperationType::Macro {
            register_macro(stack, user_definitions)?;
        } else if *operation_type == OperationType::Const {
            register_constant(stack, user_definitions)?;
        } else {
            stack.push(Stack::Operation(operation_type.clone()));
        }
    } else if let Some(function) = user_definitions.get(&presumable_keyword.to_string()) {
        stack.extend_from_slice(function);
    } else if let Some(value) = KEYWORDS.get(&presumable_keyword) {
        stack.push(Stack::Value(value.clone()));
    } else if !presumable_keyword.is_empty() {
        return Err(error::ParsingError::KeywordError {
            reason: format!("Invalid keyword: `{presumable_keyword}`."),
        });
    }

    Ok(())
}

pub fn register_macro(
    stack: &mut Vec<Stack>,
    user_definitions: &mut HashMap<String, Vec<Stack>>,
) -> error::parsing::Result<()> {
    if stack.len() < 2 {
        return Err(error::ParsingError::RegistrationError {
            what: "Macro".to_string(),
            reason: "Not enough arguments.".to_string(),
        });
    }

    match stack.pop().unwrap() {
        Stack::Value(ValueType::Text(name)) => match stack.pop().unwrap() {
            Stack::Value(ValueType::Scope(contents)) => {
                user_definitions.insert(name, contents);
                Ok(())
            }
            val => Err(error::ParsingError::MismatchedTypes {
                expected: "Scope".to_string(),
                got: format!("{:?}", val),
            }),
        },
        val => Err(error::ParsingError::ExtractionError {
            what: "Function".to_string(),
            reason: format!("Cannot register function named with {:?}", val),
        }),
    }
}

pub fn register_constant(
    stack: &mut Vec<Stack>,
    user_definitions: &mut HashMap<String, Vec<Stack>>,
) -> error::parsing::Result<()> {
    if stack.len() < 2 {
        return Err(error::ParsingError::RegistrationError {
            what: "Constant".to_string(),
            reason: "Not enough arguments.".to_string(),
        });
    }

    return match stack.pop().unwrap() {
        Stack::Value(ValueType::Text(name)) => match stack.pop().unwrap() {
            Stack::Value(val) => {
                user_definitions.insert(name, vec![Stack::Value(val)]);
                Ok(())
            }
            val => Err(error::ParsingError::MismatchedTypes {
                expected: "Value".to_string(),
                got: format!("Expected Value, but got {:?}", val),
            }),
        },
        val => Err(error::ParsingError::RegistrationError {
            what: "Constant".to_string(),
            reason: format!("Cannot register a constant named with {:?}", val),
        }),
    };
}
