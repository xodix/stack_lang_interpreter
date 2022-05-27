use crate::Stack;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Int(i32),
    Float(f32),
    Text(String),
}

pub fn extract_num(src: &str, stack: &mut Vec<Stack>, i: &mut usize) {
    let mut num = String::new();
    let mut index = 0;
    let is_float = src.contains(".");

    for ch in src.chars() {
        if ch != '-' && ch != '_' && !ch.is_digit(10) && ch != '.' {
            break;
        }
        index += 1;

        if ch != '_' {
            num.push(ch);
        }
    }

    *i += index;

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

pub fn extract_string(src: &str, stack: &mut Vec<Stack>, i: &mut usize) {
    let word_end = src[1..].find('\"').expect(&format!(
        "Could not find end of string that started at {i} character."
    ));

    let word = src[1..word_end].to_string();

    stack.push(Stack::Value(ValueType::Text(word)));

    *i += word_end;
}
