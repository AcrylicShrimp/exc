use crate::TokenLiteralKind;
use exc_symbol::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenLiteral {
    pub kind: TokenLiteralKind,
    pub content: Symbol,
    pub suffix: Option<Symbol>,
}

impl TokenLiteral {
    pub fn new(kind: TokenLiteralKind, content: Symbol, suffix: Option<Symbol>) -> Self {
        Self {
            kind,
            content,
            suffix,
        }
    }
}
