use std::{collections::HashMap, fmt::Display};

use crate::{
    util::{error, find_closing_bracket},
    Stack,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ValueType {
    Int(i32),
    Float(f32),
    Text(String),
    Scope(Vec<Stack>),
    Bool(bool),
}

impl ValueType {
    pub fn truthy(&self) -> bool {
        match self {
            ValueType::Int(number) => *number != 0,
            ValueType::Float(number) => *number != 0.0,
            ValueType::Text(text) => !text.is_empty(),
            ValueType::Scope(scope) => !scope.is_empty(),
            ValueType::Bool(condition) => *condition,
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(int) => write!(f, "{}", int)?,
            Self::Float(float) => write!(f, "{}", float)?,
            Self::Text(text) => write!(f, "{}", text)?,
            Self::Scope(scope) => {
                writeln!(f, "{{")?;
                for elem in scope {
                    writeln!(f, "\t{:?}", elem)?;
                }
                writeln!(f, "}}")?;
            }
            Self::Bool(condition) => write!(f, "{}", if *condition { "true" } else { "false" })?,
        };

        Ok(())
    }
}

pub fn number(src: &str, stack: &mut Vec<Stack>, i: &mut usize) -> Result<(), error::ParsingError> {
    let mut num = String::new();
    let mut index = 0;
    let is_float = src.contains('.');

    for ch in src.chars() {
        if ch != '-' && ch != '_' && !ch.is_digit(10) && ch != '.' {
            break;
        }
        index += 1;

        if ch != '_' {
            num.push(ch);
        }
    }

    *i += index - 1;

    if is_float {
        let res = num.parse::<f32>();

        match res {
            Ok(num) => Ok(stack.push(Stack::Value(ValueType::Float(num)))),
            Err(e) => Err(error::ParsingError::ExtractionError {
                what: "Float".to_string(),
                reason: e.to_string(),
            }),
        }
    } else {
        let res = num.parse::<i32>();

        match res {
            Ok(num) => Ok(stack.push(Stack::Value(ValueType::Int(num)))),
            Err(e) => Err(error::ParsingError::ExtractionError {
                what: "Int".to_string(),
                reason: e.to_string(),
            }),
        }
    }
}

pub fn string(src: &str, stack: &mut Vec<Stack>, i: &mut usize) -> Result<(), error::ParsingError> {
    let word_end = if let Some(end) = src[1..].find('\"') {
        end
    } else {
        return Err(error::ParsingError::ExtractionError {
            what: "String".to_string(),
            reason: "Could not find end of string.".to_string(),
        });
    };

    let word = {
        if word_end == 0 {
            "".to_string()
        } else {
            src[1..word_end + 1].to_string()
        }
    };

    stack.push(Stack::Value(ValueType::Text(word)));

    *i += word_end + 1;

    Ok(())
}

pub fn scope(
    src: &str,
    i: &mut usize,
    line_width: &mut usize,
    line_height: &mut usize,
    user_definitions: &mut HashMap<String, Vec<Stack>>,
) -> Result<Vec<Stack>, error::ParsingError> {
    let scope_end = find_closing_bracket(&src[1..]);

    let mut scopes_stack: Vec<Stack> = Vec::new();
    crate::ast::fill(
        &src[1..scope_end],
        &mut scopes_stack,
        line_height,
        line_width,
        user_definitions,
    )?;

    *i += scope_end + 1;

    Ok(scopes_stack)
}

pub fn register_macro(
    stack: &mut Vec<Stack>,
    user_definitions: &mut HashMap<String, Vec<Stack>>,
) -> Result<(), error::ParsingError> {
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
) -> Result<(), error::ParsingError> {
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
