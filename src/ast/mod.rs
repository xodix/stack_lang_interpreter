#[cfg(test)]
mod ast_test;
pub mod extract;

mod comments;

use crate::{util::error, Stack};
pub use extract::{operation::OperationType, value::ValueType};
use std::collections::HashMap;

pub fn fill(
    src: &str,
    stack: &mut Vec<Stack>,
    line_height: &mut usize,
    line_width: &mut usize,
    user_definitions: &mut HashMap<String, Vec<Stack>>,
) -> Result<(), error::ParsingError> {
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        let old_i = i;

        match ch {
            ' ' => (),
            '\n' => {
                *line_height += 1;
                *line_width = 0;
            }
            '\r' => (),
            '\t' => (),

            '/' => {
                if i + 1 < chars.len() {
                    if chars[i + 1] == '*' {
                        comments::skip_multiline(&src[i..], &mut i);
                    } else if chars[i + 1] == '/' {
                        comments::skip_singleline(&src[i..], &mut i);
                    } else {
                        stack.push(Stack::Operation(OperationType::Div))
                    }
                } else {
                    stack.push(Stack::Operation(OperationType::Div))
                }
            }

            _ if ch.is_digit(10)
                || (ch == '-' && src.chars().nth(i + 1).unwrap_or(' ').is_digit(10)) =>
            {
                extract::value::number(&src[i..], stack, &mut i)?;
            }

            '\'' | '\"' => extract::value::string(&src[i..], stack, &mut i)?,

            '{' => stack.push(Stack::Value(ValueType::Scope(extract::value::scope(
                &src[i..],
                &mut i,
                line_width,
                line_height,
                user_definitions,
            )?))),

            _ => extract::operation::keyword(&src[i..], stack, &mut i, user_definitions)?,
        };

        *line_width += (i - old_i) + 1;
        i += 1;
    }

    Ok(())
}
