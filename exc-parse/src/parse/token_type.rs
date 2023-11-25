use crate::TokenKind;
use exc_symbol::Symbol;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenType {
    Token(TokenKind),
    Keyword(Symbol),
    AssignmentOp,
    UnaryOp,
    BinaryOp,
    Identifier,
    Path,
    Typename,
    Literal,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Token(kind) => write!(f, "{}", kind.into_symbol()),
            TokenType::Keyword(keyword) => write!(f, "{keyword}"),
            TokenType::AssignmentOp => write!(f, "an assignment operator"),
            TokenType::UnaryOp => write!(f, "an unary operator"),
            TokenType::BinaryOp => write!(f, "a binary operator"),
            TokenType::Identifier => write!(f, "identifier"),
            TokenType::Path => write!(f, "path"),
            TokenType::Typename => write!(f, "typename"),
            TokenType::Literal => write!(f, "a literal"),
            TokenType::Eof => write!(f, "eof"),
        }
    }
}
