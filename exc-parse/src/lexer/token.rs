use crate::TokenKind;
use exc_span::Span;
use std::collections::VecDeque;

#[derive(Debug, Clone, Hash)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(span: Span, kind: TokenKind) -> Self {
        Self { span, kind }
    }

    pub fn glue(&self, next: &Self) -> Option<Self> {
        let kind = match self.kind {
            TokenKind::Colon => match next.kind {
                TokenKind::Colon => TokenKind::PathSep,
                _ => return None,
            },
            TokenKind::Dot => match next.kind {
                TokenKind::Dot => TokenKind::Rng,
                _ => return None,
            },
            TokenKind::Rng => match next.kind {
                TokenKind::Assign => TokenKind::RngInclusive,
                _ => return None,
            },
            TokenKind::Assign => match next.kind {
                TokenKind::Assign => TokenKind::Eq,
                _ => return None,
            },
            TokenKind::Lt => match next.kind {
                TokenKind::Assign => TokenKind::Le,
                TokenKind::Lt => TokenKind::Shl,
                _ => return None,
            },
            TokenKind::Gt => match next.kind {
                TokenKind::Assign => TokenKind::Ge,
                TokenKind::Gt => TokenKind::Shr,
                _ => return None,
            },
            TokenKind::Add => match &next.kind {
                TokenKind::Assign => TokenKind::AssignAdd,
                _ => return None,
            },
            TokenKind::Sub => match &next.kind {
                TokenKind::Assign => TokenKind::AssignSub,
                TokenKind::Gt => TokenKind::Arrow,
                _ => return None,
            },
            TokenKind::Mul => match next.kind {
                TokenKind::Mul => TokenKind::Pow,
                TokenKind::Assign => TokenKind::AssignMul,
                _ => return None,
            },
            TokenKind::Div => match next.kind {
                TokenKind::Assign => TokenKind::AssignDiv,
                _ => return None,
            },
            TokenKind::Mod => match next.kind {
                TokenKind::Assign => TokenKind::AssignMod,
                _ => return None,
            },
            TokenKind::Pow => match next.kind {
                TokenKind::Assign => TokenKind::AssignPow,
                _ => return None,
            },
            TokenKind::Shl => match next.kind {
                TokenKind::Assign => TokenKind::AssignShl,
                _ => return None,
            },
            TokenKind::Shr => match next.kind {
                TokenKind::Assign => TokenKind::AssignShr,
                _ => return None,
            },
            TokenKind::BitOr => match next.kind {
                TokenKind::Assign => TokenKind::AssignBitOr,
                TokenKind::BitOr => TokenKind::LogOr,
                _ => return None,
            },
            TokenKind::BitAnd => match next.kind {
                TokenKind::Assign => TokenKind::AssignBitAnd,
                TokenKind::BitAnd => TokenKind::LogAnd,
                _ => return None,
            },
            TokenKind::BitXor => match next.kind {
                TokenKind::Assign => TokenKind::AssignBitXor,
                _ => return None,
            },
            TokenKind::LogNot => match next.kind {
                TokenKind::Assign => TokenKind::Ne,
                _ => return None,
            },
            TokenKind::Unknown { .. }
            | TokenKind::Comment
            | TokenKind::OpenParen
            | TokenKind::CloseParen
            | TokenKind::OpenBrace
            | TokenKind::CloseBrace
            | TokenKind::OpenBracket
            | TokenKind::CloseBracket
            | TokenKind::Comma
            | TokenKind::Semicolon
            | TokenKind::Arrow
            | TokenKind::AssignAdd
            | TokenKind::AssignSub
            | TokenKind::AssignMul
            | TokenKind::AssignDiv
            | TokenKind::AssignMod
            | TokenKind::AssignPow
            | TokenKind::AssignShl
            | TokenKind::AssignShr
            | TokenKind::AssignBitOr
            | TokenKind::AssignBitAnd
            | TokenKind::AssignBitXor
            | TokenKind::RngInclusive
            | TokenKind::Eq
            | TokenKind::Ne
            | TokenKind::Le
            | TokenKind::Ge
            | TokenKind::BitNot
            | TokenKind::LogOr
            | TokenKind::LogAnd
            | TokenKind::PathSep
            | TokenKind::Literal { .. }
            | TokenKind::Id { .. } => return None,
        };

        Some(Token::new(self.span.to(next.span), kind))
    }

    pub fn unglue(self, unglued_tokens: &mut VecDeque<Self>) {
        match self.kind {
            TokenKind::Arrow => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Sub,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Gt,
                });
            }
            TokenKind::AssignAdd => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Add,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignSub => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Sub,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignMul => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Mul,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignDiv => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Div,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignMod => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Mod,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignPow => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Mul,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Mul,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 2, self.span.low + 3),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignShl => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Lt,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Lt,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 2, self.span.low + 3),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignShr => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Gt,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Gt,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 2, self.span.low + 3),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignBitOr => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::BitOr,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignBitAnd => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::BitAnd,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignBitXor => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::BitXor,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Rng => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Dot,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Dot,
                });
            }
            TokenKind::RngInclusive => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Dot,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Dot,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 2, self.span.low + 3),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Eq => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Assign,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Ne => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::LogNot,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Le => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Lt,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Ge => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Gt,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Pow => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Mul,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Mul,
                });
            }
            TokenKind::Shl => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Lt,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Lt,
                });
            }
            TokenKind::Shr => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Gt,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Gt,
                });
            }
            TokenKind::LogOr => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::BitOr,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::BitOr,
                });
            }
            TokenKind::LogAnd => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::BitAnd,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::BitAnd,
                });
            }
            TokenKind::PathSep => {
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low, self.span.low + 1),
                    kind: TokenKind::Colon,
                });
                unglued_tokens.push_back(Self {
                    span: Span::new(self.span.low + 1, self.span.low + 2),
                    kind: TokenKind::Colon,
                });
            }
            TokenKind::Unknown { .. }
            | TokenKind::Comment
            | TokenKind::OpenParen
            | TokenKind::CloseParen
            | TokenKind::OpenBrace
            | TokenKind::CloseBrace
            | TokenKind::OpenBracket
            | TokenKind::CloseBracket
            | TokenKind::Dot
            | TokenKind::Comma
            | TokenKind::Colon
            | TokenKind::Semicolon
            | TokenKind::Assign
            | TokenKind::Lt
            | TokenKind::Gt
            | TokenKind::Add
            | TokenKind::Sub
            | TokenKind::Mul
            | TokenKind::Div
            | TokenKind::Mod
            | TokenKind::BitOr
            | TokenKind::BitAnd
            | TokenKind::BitXor
            | TokenKind::BitNot
            | TokenKind::LogNot
            | TokenKind::Id { .. }
            | TokenKind::Literal(_) => unglued_tokens.push_back(self),
        }
    }
}
