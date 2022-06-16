use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

use crate::{Stack, ValueType};

pub const ADD: &str = "+";
pub const SUB: &str = "-";
pub const MUL: &str = "*";
pub const DIV: &str = "/";
pub const MOD: &str = "%";
pub const POW: &str = "^";

pub const LT: &str = "<";
pub const GT: &str = ">";
pub const EQ: &str = "==";
pub const LEQ: &str = "<=";
pub const GEQ: &str = ">=";

pub const OR: &str = "||";
pub const AND: &str = "&&";

pub const IF: &str = "if";

pub const PRINT: &str = "print";
pub const PRINT_DEBUG: &str = "print_debug";

lazy_static! {
    static ref OPERANDS: HashSet<&'static str> = HashSet::from([
        ADD,
        SUB,
        MUL,
        DIV,
        MOD,
        POW,
        PRINT,
        PRINT_DEBUG,
        IF,
        LT,
        GT,
        EQ,
        LEQ,
        GEQ,
        OR,
        AND
    ]);
    static ref KEYWORDS: HashMap<&'static str, ValueType<'static>> = HashMap::from([
        ("true", ValueType::Bool(true)),
        ("false", ValueType::Bool(false))
    ]);
}

pub fn extract_keyword<'a>(src: &'a str, stack: &mut Vec<Stack<'a>>, i: &mut usize) {
    let presumable_operand_index = src.find(' ').unwrap_or(src.len());
    *i += presumable_operand_index;
    let presumable_keyword = &src[..presumable_operand_index];

    if OPERANDS.contains(&presumable_keyword) {
        stack.push(Stack::Operation(presumable_keyword));
    } else if let Some(value) = KEYWORDS.get(&presumable_keyword) {
        stack.push(Stack::Value(value.clone()));
    } else if presumable_keyword != "" {
        panic!("Unknown operation `{presumable_keyword}`");
    }
}
