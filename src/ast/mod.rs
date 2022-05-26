pub mod operand_extracts;
pub mod value_extracts;

pub use self::value_extracts::extract_string;
use crate::Stack;
pub use value_extracts::extract_num;
pub use value_extracts::ValueType;

use self::operand_extracts::extract_operand;

pub fn fill_ast<'a>(src: &'a String, stack: &mut Vec<Stack<'a>>) {
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        match ch {
            _ if ch.is_digit(10)
                || (ch == '-' && src.chars().nth(i + 1).unwrap_or(' ').is_digit(10)) =>
            {
                extract_num(&src[i..], stack, &mut i);
            }

            '\'' | '\"' => extract_string(&src[i..], stack, &mut i),

            ' ' => (),

            _ => extract_operand(&src[i..], stack, &mut i),
        };

        i += 1;
    }
}
