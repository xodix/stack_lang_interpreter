use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{util::find_closing_bracket, Stack};

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

pub fn number(src: &str, stack: &mut Vec<Stack>, i: &mut usize) {
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

    fn extract_panic(e: Box<dyn Error>, i: usize) {
        panic!("Could not extract number on character {}, {e}", i);
    }

    if is_float {
        let res = num.parse::<f32>();

        match res {
            Ok(num) => stack.push(Stack::Value(ValueType::Float(num))),
            Err(e) => extract_panic(Box::new(e), *i),
        }
    } else {
        let res = num.parse::<i32>();

        match res {
            Ok(num) => stack.push(Stack::Value(ValueType::Int(num))),
            Err(e) => extract_panic(Box::new(e), *i),
        }
    }
}

pub fn string(src: &str, stack: &mut Vec<Stack>, i: &mut usize) {
    let word_end = src[1..]
        .find('\"')
        .unwrap_or_else(|| panic!("Could not find end of string that started at {i} character."));

    let word = {
        if word_end == 0 {
            "".to_string()
        } else {
            src[1..word_end + 1].to_string()
        }
    };

    stack.push(Stack::Value(ValueType::Text(word)));

    *i += word_end + 1;
}

pub fn scope<'a>(
    src: &'a str,
    i: &mut usize,
    user_definitions: &mut HashMap<String, Vec<Stack>>,
) -> Vec<Stack> {
    let scope_end = find_closing_bracket(&src[1..]);

    let mut scopes_stack: Vec<Stack> = Vec::new();
    crate::ast::fill(&src[1..scope_end], &mut scopes_stack, user_definitions);

    *i += scope_end + 1;

    scopes_stack
}

pub fn register_macro<'a>(
    stack: &mut Vec<Stack>,
    user_definitions: &mut HashMap<String, Vec<Stack>>,
) {
    match stack.pop().unwrap() {
        Stack::Value(ValueType::Text(name)) => match stack.pop().unwrap() {
            Stack::Value(ValueType::Scope(contents)) => {
                user_definitions.insert(name, contents);
            }
            val => {
                panic!("Expected Scope, but got {:?}", val);
            }
        },
        val => panic!("Cannot register function named with {:?}", val),
    }
}

pub fn register_constant<'a>(
    stack: &mut Vec<Stack>,
    user_definitions: &mut HashMap<String, Vec<Stack>>,
) {
    match stack.pop().unwrap() {
        Stack::Value(ValueType::Text(name)) => match stack.pop().unwrap() {
            Stack::Value(val) => {
                user_definitions.insert(name, vec![Stack::Value(val)]);
            }
            val => {
                panic!("Expected Value, but got {:?}", val);
            }
        },
        val => panic!("Cannot register a constant named with {:?}", val),
    }
}
