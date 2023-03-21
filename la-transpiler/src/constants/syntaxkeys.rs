use crate::enums::TokenType;

pub static logickeys: [&str; 2] = [
    "block",
    "*"
];

pub static stylekeys: [&str; 9] = [
    "rm",
    "tt",
    "sf",
    "bf",
    "md",
    "it",
    "sl",
    "up",
    "sc"
];

pub static functional_tokentypes: [TokenType; 6] = [
    TokenType::BLOCK,
    TokenType::EQBLOCK,
    TokenType::EQ,
    TokenType::STYLE,
    TokenType::LXFUNC,
    TokenType::ENUMITEM,
];