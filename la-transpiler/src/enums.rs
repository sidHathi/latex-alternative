
use strum_macros::EnumIter; // 0.17.1
use std::fmt;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    PLAINTEXT,
    OPENPAREN,
    CLOSEPAREN,
    PARAM,
    PARAMSEP,
    STYLE,
    ENUMITEM,
    ENUMLABEL,
    EQ,
    EQEND,
    EQBLOCK,
    EQBLOCKEND,
    LXSYMBOL,
    LXFUNC,
    OPER, // not yet implemented
    SUBSCRIPT, // not yet implemented
    SUPERSCRIPT, // not yet implemented
    QUOTESTART, // not yet implemented
    QUOTEEND, // not yet implemented
    BLOCK,
    BACKTAB,
    IGNORE,
    ERROR
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::PLAINTEXT => write!(f, "PLAINTEXT"),
            TokenType::OPENPAREN => write!(f, "OPENPAREN"),
            TokenType::CLOSEPAREN => write!(f, "CLOSEPAREN"),
            TokenType::PARAM => write!(f, "PARAM"),
            TokenType::PARAMSEP => write!(f, "PARAMSEP"),
            TokenType::   STYLE => write!(f, "STYLE"),
            TokenType:: ENUMITEM => write!(f, "ENUMITEM"),
            TokenType::ENUMLABEL => write!(f, "ENUMLABEL"),
            TokenType:: EQ => write!(f, "EQ"),
            TokenType:: EQEND => write!(f, "EQEND"),
            TokenType:: EQBLOCK => write!(f, "EQBLOCK"),
            TokenType::  EQBLOCKEND => write!(f, "EQBLOCKEND"),
            TokenType::LXSYMBOL => write!(f, "LXSYMBOL"),
            TokenType:: LXFUNC => write!(f, "LXFUNC"),
            TokenType:: OPER => write!(f, "OPER"),
            TokenType:: SUBSCRIPT => write!(f, "SUBSCRIPT"),
            TokenType::SUPERSCRIPT => write!(f, "SUPERSCRIPT"),
            TokenType:: QUOTESTART => write!(f, "QUOTESTART"),
            TokenType::QUOTEEND => write!(f, "QUOTEEND"),
            TokenType::BLOCK => write!(f, "BLOCK"),
            TokenType::BACKTAB => write!(f, "BACKTAB"),
            TokenType::IGNORE => write!(f, "IGNORE"),
            TokenType::ERROR => write!(f, "ERROR"),
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum TranscriptionMode {
    DEFAULT,
    EQBLOCK,
    EQ,
    PARAMREAD,
    SPECIALTEXT,
    ANNOREAD,
}

impl fmt::Display for TranscriptionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranscriptionMode::DEFAULT => write!(f, "default"),
            TranscriptionMode::EQBLOCK => write!(f, "eqblock"),
            TranscriptionMode::EQ => write!(f, "eq"),
            TranscriptionMode::PARAMREAD => write!(f, "paramread"),
            TranscriptionMode::SPECIALTEXT => write!(f, "specialtext"),
            TranscriptionMode::ANNOREAD => write!(f, "annoread"),
        }
    }
}
