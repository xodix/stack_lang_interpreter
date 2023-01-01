#[cfg(test)]
mod ast_test;
pub mod extract;

mod comments;

use crate::Stack;
pub use extract::{operation::OperationType, value::ValueType};
use std::collections::HashMap;

pub fn fill<'a>(
    src: &'a str,
    stack: &mut Vec<Stack>,
    user_definitions: &mut HashMap<String, Vec<Stack>>,
) {
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        match ch {
            ' ' => (),
            '\n' => (),
            '\r' => (),
            '\t' => (),

            '/' => {
                if chars[i + 1] == '*' {
                    comments::skip_multiline(&src[i..], &mut i);
                } else if chars[i + 1] == '/' {
                    comments::skip_singleline(&src[i..], &mut i);
                } else {
                    stack.push(Stack::Operation(OperationType::Div))
                }
            }

            _ if ch.is_digit(10)
                || (ch == '-' && src.chars().nth(i + 1).unwrap_or(' ').is_digit(10)) =>
            {
                extract::value::number(&src[i..], stack, &mut i);
            }

            '\'' | '\"' => extract::value::string(&src[i..], stack, &mut i),

            '{' => stack.push(Stack::Value(ValueType::Scope(extract::value::scope(
                &src[i..],
                &mut i,
                user_definitions,
            )))),

            _ => extract::operation::keyword(&src[i..], stack, &mut i, user_definitions),
        };

        i += 1;
    }
}
