#[cfg(test)]
mod ast_test;
mod comments;
pub mod extract;

use crate::{util::*, Stack};
pub use extract::{operation::OperationType, value::ValueType};
use std::collections::HashMap;

pub fn fill(
    src: &str,
    stack: &mut Vec<Stack>,
    line_height: &mut usize,
    line_width: &mut usize,
    user_definitions: &mut HashMap<String, Vec<Stack>>,
) -> error::parsing::Result<()> {
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
                        comments::skip_multiline(&src[i..], &mut i, line_height, line_width);
                    } else if chars[i + 1] == '/' {
                        comments::skip_singleline(&src[i..], &mut i);
                    } else {
                        stack.push(Stack::Operation(OperationType::Div))
                    }
                } else {
                    stack.push(Stack::Operation(OperationType::Div))
                }
            }

            _ if parsing::looks_like_digit(&src[i..]) => {
                extract::value::number(&src[i..], stack, &mut i)?;
            }

            '\'' | '\"' => {
                extract::value::string(&src[i..], stack, &mut i, line_height, line_width)?
            }

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
