
use strum_macros::EnumIter; // 0.17.1
use std::fmt;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    PLAINTEXT,
    BLOCKLABEL,
    OPENPAREN,
    CLOSEPAREN,
    PARAM,
    PARAMSEP,
    STYLE,
    STYLEDTEXT,
    ENUMITEM,
    ENUMLABEL,
    EQSTART,
    EQEND,
    EQBLOCKSTART,
    EQBLOCKEND,
    LXSYMBOL,
    LXFUNC,
    OPER,
    SUBSCRIPT,
    SUPERSCRIPT,
    QUOTESTART,
    QUOTEEND,
    BLOCKSTART,
    BACKTAB,
    IGNORE,
    ERROR
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::PLAINTEXT => write!(f, "PLAINTEXT"),
            TokenType::BLOCKLABEL => write!(f, "BLOCKLABEL"),
            TokenType::OPENPAREN => write!(f, "OPENPAREN"),
            TokenType::CLOSEPAREN => write!(f, "CLOSEPAREN"),
            TokenType::PARAM => write!(f, "PARAM"),
            TokenType::PARAMSEP => write!(f, "PARAMSEP"),
            TokenType::   STYLE => write!(f, "STYLE"),
            TokenType::  STYLEDTEXT => write!(f, "STYLEDTEXT"),
            TokenType:: ENUMITEM => write!(f, "ENUMITEM"),
            TokenType::ENUMLABEL => write!(f, "ENUMLABEL"),
            TokenType:: EQSTART => write!(f, "EQSTART"),
            TokenType:: EQEND => write!(f, "EQEND"),
            TokenType:: EQBLOCKSTART => write!(f, "EQBLOCKSTART"),
            TokenType::  EQBLOCKEND => write!(f, "EQBLOCKEND"),
            TokenType::LXSYMBOL => write!(f, "LXSYMBOL"),
            TokenType:: LXFUNC => write!(f, "LXFUNC"),
            TokenType:: OPER => write!(f, "OPER"),
            TokenType:: SUBSCRIPT => write!(f, "SUBSCRIPT"),
            TokenType::SUPERSCRIPT => write!(f, "SUPERSCRIPT"),
            TokenType:: QUOTESTART => write!(f, "QUOTESTART"),
            TokenType::QUOTEEND => write!(f, "QUOTEEND"),
            TokenType::BLOCKSTART => write!(f, "BLOCKSTART"),
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
