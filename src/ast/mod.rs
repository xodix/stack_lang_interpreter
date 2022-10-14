#[cfg(test)]
mod ast_test;

mod comments;
pub mod operation_extracts;
mod value_extracts;

use std::rc::Rc;

use self::comments::{ignore_multiline, ignore_single_line};
use self::value_extracts::extract_scope;
use self::value_extracts::extract_string;
use crate::{Stack, UserDefinitions};
use value_extracts::extract_num;
pub use value_extracts::ValueType;

use self::operation_extracts::{extract_keyword, DIV};

pub fn fill_ast<'a>(
    src: &'a str,
    stack: &mut Vec<Stack<'a>>,
    user_definitions: UserDefinitions<'a>,
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
                    ignore_multiline(&src[i..], &mut i);
                } else if chars[i + 1] == '/' {
                    ignore_single_line(&src[i..], &mut i);
                } else {
                    stack.push(Stack::Operation(DIV))
                }
            }

            _ if ch.is_digit(10)
                || (ch == '-' && src.chars().nth(i + 1).unwrap_or(' ').is_digit(10)) =>
            {
                extract_num(&src[i..], stack, &mut i);
            }

            '\'' | '\"' => extract_string(&src[i..], stack, &mut i),

            '{' => stack.push(Stack::Value(ValueType::Scope(extract_scope(
                &src[i..],
                &mut i,
                Rc::clone(&user_definitions),
            )))),

            _ => extract_keyword(&src[i..], stack, &mut i, user_definitions.clone()),
        };

        i += 1;
    }
}
