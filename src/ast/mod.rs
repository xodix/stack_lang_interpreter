#[cfg(test)]
mod ast_test;

pub mod operation_extracts;
pub mod value_extracts;

use self::value_extracts::extract_scope;
pub use self::value_extracts::extract_string;
use crate::Stack;
pub use value_extracts::extract_num;
pub use value_extracts::ValueType;

use self::operation_extracts::extract_keyword;

pub fn fill_ast<'a>(src: &'a str, stack: &mut Vec<Stack<'a>>) {
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        match ch {
            ' ' => (),
            '\n' => (),

            _ if ch.is_digit(10)
                || (ch == '-' && src.chars().nth(i + 1).unwrap_or(' ').is_digit(10)) =>
            {
                extract_num(&src[i..], stack, &mut i);
            }

            '\'' | '\"' => extract_string(&src[i..], stack, &mut i),

            '{' => stack.push(Stack::Value(ValueType::Scope(extract_scope(
                &src[i..],
                &mut i,
            )))),

            _ => extract_keyword(&src[i..], stack, &mut i),
        };

        i += 1;
    }
}
