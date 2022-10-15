use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

use super::value::{register_constant, register_macro};
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
pub const NOT: &str = "!";

pub const IF: &str = "if";
pub const FOR: &str = "for";
pub const WHILE: &str = "while";

pub const PRINT: &str = "print";
pub const PRINTLN: &str = "println";
pub const PRINT_DEBUG: &str = "print_debug";
pub const PRINT_DEBUG_STACK: &str = "print_debug_stack";

pub const SWITCH: &str = "switch";
pub const REVERSE: &str = "reverse";
pub const POP: &str = "pop";
pub const COPY: &str = "copy";

pub const MACRO: &str = "macro";
pub const CONST: &str = "const";

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
        PRINT_DEBUG_STACK,
        IF,
        LT,
        GT,
        EQ,
        LEQ,
        GEQ,
        OR,
        AND,
        FOR,
        WHILE,
        SWITCH,
        REVERSE,
        POP,
        NOT,
        COPY,
        PRINTLN,
        MACRO,
        CONST
    ]);
    static ref KEYWORDS: HashMap<&'static str, ValueType<'static>> = HashMap::from([
        ("true", ValueType::Bool(true)),
        ("false", ValueType::Bool(false))
    ]);
}

pub fn keyword<'a>(
    src: &'a str,
    stack: &mut Vec<Stack<'a>>,
    i: &mut usize,
    user_definitions: &mut crate::UserDefinitions<'a>,
) {
    let presumable_operand_index = src
        .find(|c| c == ' ' || c == '\n' || c == '\r')
        .unwrap_or(src.len());
    *i += presumable_operand_index;
    let presumable_keyword = &src[..presumable_operand_index];

    if OPERANDS.contains(&presumable_keyword) {
        if presumable_keyword == MACRO {
            register_macro(stack, user_definitions);
        } else if presumable_keyword == CONST {
            register_constant(stack, user_definitions);
        } else {
            stack.push(Stack::Operation(presumable_keyword));
        }
    } else if let Some(function) = user_definitions.get(&presumable_keyword.to_string()) {
        stack.extend_from_slice(function);
    } else if let Some(value) = KEYWORDS.get(&presumable_keyword) {
        stack.push(Stack::Value(value.clone()));
    } else if !presumable_keyword.is_empty() {
        panic!("Unknown keyword: `{presumable_keyword}`");
    }
}
