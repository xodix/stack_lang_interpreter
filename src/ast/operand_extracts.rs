use crate::Stack;

pub const ADD: &str = "+";
pub const SUB: &str = "-";
pub const MUL: &str = "*";
pub const DIV: &str = "/";
pub const MOD: &str = "%";
pub const PRINT: &str = "print";

pub fn extract_operand<'a>(str: &'a str, stack: &mut Vec<Stack<'a>>, i: &mut usize) {
    const OPERANDS: [&str; 6] = [ADD, SUB, MUL, DIV, MOD, PRINT];

    let presumable_operand_index = str.find(' ').unwrap_or(str.len());
    *i += presumable_operand_index;
    let presumable_operand = &str[..presumable_operand_index];

    if OPERANDS.contains(&presumable_operand) {
        stack.push(Stack::Operation(presumable_operand));
    } else {
        if presumable_operand != "" {
            panic!("Unknown operand `{presumable_operand}`");
        }
    }
}
