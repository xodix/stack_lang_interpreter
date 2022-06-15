use std::collections::HashSet;

use crate::Stack;

pub const ADD: &str = "+";
pub const SUB: &str = "-";
pub const MUL: &str = "*";
pub const DIV: &str = "/";
pub const MOD: &str = "%";
pub const POW: &str = "^";
pub const PRINT: &str = "print";
pub const PRINT_DEBUG: &str = "print_debug";
pub const IF: &str = "if";

pub fn extract_operation<'a>(src: &'a str, stack: &mut Vec<Stack<'a>>, i: &mut usize) {
    let operands = HashSet::from([ADD, SUB, MUL, DIV, MOD, POW, PRINT, PRINT_DEBUG, IF]);

    let presumable_operand_index = src.find(' ').unwrap_or(src.len());
    *i += presumable_operand_index;
    let presumable_operand = &src[..presumable_operand_index];

    if operands.contains(&presumable_operand) {
        stack.push(Stack::Operation(presumable_operand));
    } else if presumable_operand != "" {
        panic!("Unknown operation `{presumable_operand}`");
    }
}
