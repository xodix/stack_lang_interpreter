use std::{collections::HashMap, fmt::Display};

use crate::{util::*, Stack};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ValueType {
    Int(i64),
    Float(f64),
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

pub fn number(src: &str, stack: &mut Vec<Stack>, i: &mut usize) -> error::parsing::Result<()> {
    let mut num = String::with_capacity(40);
    let mut index = 0;
    let mut is_float = false;

    for ch in src.chars() {
        if ch != '-' && ch != '_' && !ch.is_ascii_digit() && ch != '.' {
            break;
        }
        index += 1;

        if ch != '_' {
            num.push(ch);
        }
        if ch == '.' {
            is_float = true;
        }
    }

    *i += index - 1;

    if is_float {
        let res = num.parse::<f64>();

        match res {
            Ok(num) => Ok(stack.push(Stack::Value(ValueType::Float(num)))),
            Err(e) => Err(error::ParsingError::ExtractionError {
                what: "Float".to_string(),
                reason: e.to_string(),
            }),
        }
    } else {
        let res = num.parse::<i64>();

        match res {
            Ok(num) => Ok(stack.push(Stack::Value(ValueType::Int(num)))),
            Err(e) => Err(error::ParsingError::ExtractionError {
                what: "Int".to_string(),
                reason: e.to_string(),
            }),
        }
    }
}

pub fn string(
    src: &str,
    stack: &mut Vec<Stack>,
    i: &mut usize,
    line_height: &mut usize,
    line_width: &mut usize,
) -> error::parsing::Result<()> {
    let word_end = if let Some(end) = src[1..].find(|ch| ch == '\"' || ch == '\'') {
        end
    } else {
        return Err(error::ParsingError::ExtractionError {
            what: "String".to_string(),
            reason: "Could not find end of string.".to_string(),
        });
    };

    let word = if word_end == 0 {
        "".to_string()
    } else {
        src[1..word_end + 1].to_string()
    };

    let lines = word.matches('\n').count();
    if lines != 0 {
        *line_height += lines;
        *line_width = 0;
    }

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
) -> error::parsing::Result<Vec<Stack>> {
    let scope_end = parsing::find_closing_bracket(&src[1..]);

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
