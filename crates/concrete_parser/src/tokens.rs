use logos::Logos;
use std::{convert::Infallible, fmt}; // to implement the Display trait

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other,
}

impl From<std::num::ParseIntError> for LexingError {
    fn from(_: std::num::ParseIntError) -> Self {
        LexingError::NumberParseError
    }
}

impl From<Infallible> for LexingError {
    fn from(_: Infallible) -> Self {
        LexingError::Other
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError, skip r"[ \t\n\f]+", skip r"//.*\n?", skip r"/\*(?:[^*]|\*[^/])*\*/")]
pub enum Token {
    #[token("let")]
    KeywordLet,
    #[token("const")]
    KeywordConst,
    #[token("fn")]
    KeywordFn,
    #[token("return")]
    KeywordReturn,
    #[token("struct")]
    KeywordStruct,
    #[token("if")]
    KeywordIf,
    #[token("else")]
    KeywordElse,
    #[token("while")]
    KeywordWhile,
    #[token("for")]
    KeywordFor,
    #[token("match")]
    KeywordMatch,
    #[token("mod")]
    KeywordMod,
    #[token("pub")]
    KeywordPub,

    // Modern way of allowing identifiers, read: https://unicode.org/reports/tr31/
    #[regex(r"_?\p{XID_Start}\p{XID_Continue}*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Literals
    #[regex(r"\d+", |lex| lex.slice().to_string())]
    Integer(String),
    #[regex(r#""(?:[^"]|\\")*""#, |lex| lex.slice().to_string())]
    String(String),
    #[regex(r"(true|false)", |lex| lex.slice().parse::<bool>().unwrap())]
    Boolean(bool),

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBracket,
    #[token("}")]
    RightBracket,
    #[token("[")]
    LeftSquareBracket,
    #[token("]")]
    RightSquareBracket,
    #[token("=")]
    Assign,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token(",")]
    Coma,
    #[token("<")]
    LessThanSign,
    #[token(">")]
    MoreThanSign,

    #[token("+")]
    OperatorAdd,
    #[token("-")]
    OperatorSub,
    #[token("*")]
    OperatorMul,
    #[token("/")]
    OperatorDiv,
    #[token("%")]
    OperatorRem,
    #[token("&&")]
    OperatorAnd,
    #[token("||")]
    OperatorOr,
    #[token("==")]
    OperatorEq,
    #[token("!=")]
    OperatorNe,
    #[token("!")]
    OperatorNot,
}
