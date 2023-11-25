use crate::{Id, NodeId, NodeIdAllocator, Token, TokenKind, TokenLiteral, TokenType};
use exc_diagnostic::DiagnosticsSender;
use exc_span::{Pos, Span};
use exc_symbol::Symbol;
use std::collections::VecDeque;

pub struct Parser<'a, 'd, T>
where
    T: Iterator<Item = Token>,
{
    unglue_tokens: bool,
    token_stream: T,
    token_buffer: VecDeque<Token>,
    expected: Vec<TokenType>,
    last_span: Span,
    id_allocator: &'a mut NodeIdAllocator,
    diagnostics: &'d DiagnosticsSender,
}

impl<'a, 'd, T> Parser<'a, 'd, T>
where
    T: Iterator<Item = Token>,
{
    pub fn new(
        token_stream: T,
        id_allocator: &'a mut NodeIdAllocator,
        diagnostics: &'d DiagnosticsSender,
    ) -> Self {
        Self {
            unglue_tokens: false,
            token_stream,
            token_buffer: VecDeque::new(),
            expected: Vec::new(),
            last_span: diagnostics.file().span_begin(),
            id_allocator,
            diagnostics,
        }
    }

    pub fn diagnostics(&self) -> &'d DiagnosticsSender {
        self.diagnostics
    }

    // TODO: make this recoverable
    pub fn set_unglue_tokens(&mut self, unglue_tokens: bool) -> bool {
        let prev = self.unglue_tokens;
        self.unglue_tokens = unglue_tokens;
        prev
    }

    pub fn is_exists(&mut self) -> bool {
        self.fetch_tokens(1);
        !self.token_buffer.is_empty()
    }

    pub fn new_node(&mut self) -> (NodeId, Pos) {
        (self.id_allocator.allocate(), self.current_pos())
    }

    pub fn lookup_identifier(&mut self, offset: usize) -> bool {
        self.fetch_tokens(offset + 1);
        self.expected.push(TokenType::Identifier);

        let token = if let Some(token) = self.token_buffer.get(offset) {
            token
        } else {
            return false;
        };

        match token.kind {
            TokenKind::Id { .. } => true,
            _ => false,
        }
    }

    pub fn identifier(&mut self) -> Option<Id> {
        self.fetch_tokens(1);
        self.expected.push(TokenType::Identifier);

        let token = if let Some(token) = self.token_buffer.get(0).cloned() {
            token
        } else {
            return None;
        };

        match token.kind {
            TokenKind::Id { symbol } => {
                self.token_buffer.pop_front();
                self.expected.clear();
                self.last_span = token.span;

                Some(Id {
                    span: token.span,
                    symbol,
                })
            }
            _ => None,
        }
    }

    pub fn identifier_or_err(&mut self) -> Result<Id, ()> {
        let item = self.identifier();
        self.make_item_or_err(item)
    }

    pub fn lookup_keyword(&mut self, offset: usize, keyword: Symbol) -> bool {
        self.fetch_tokens(offset + 1);
        self.expected.push(TokenType::Keyword(keyword));

        let token = if let Some(token) = self.token_buffer.get(offset) {
            token
        } else {
            return false;
        };

        match token.kind {
            TokenKind::Id { symbol } if symbol == keyword => true,
            _ => false,
        }
    }

    pub fn keyword(&mut self, keyword: Symbol) -> Option<Id> {
        self.fetch_tokens(1);
        self.expected.push(TokenType::Keyword(keyword));

        let token = if let Some(token) = self.token_buffer.get(0).cloned() {
            token
        } else {
            return None;
        };

        match token.kind {
            TokenKind::Id { symbol } if symbol == keyword => {
                self.token_buffer.pop_front();
                self.expected.clear();
                self.last_span = token.span;

                Some(Id {
                    span: token.span,
                    symbol,
                })
            }
            _ => None,
        }
    }

    pub fn keyword_or_err(&mut self, keyword: Symbol) -> Result<Id, ()> {
        let item = self.keyword(keyword);
        self.make_item_or_err(item)
    }

    pub fn lookup_kind(&mut self, offset: usize, kind: TokenKind) -> bool {
        self.fetch_tokens(offset + 1);
        self.expected.push(TokenType::Token(kind));

        let token = if let Some(token) = self.token_buffer.get(offset) {
            token
        } else {
            return false;
        };

        token.kind == kind
    }

    pub fn kind(&mut self, kind: TokenKind) -> Option<Token> {
        self.fetch_tokens(1);
        self.expected.push(TokenType::Token(kind));

        let token = if let Some(token) = self.token_buffer.get(0).cloned() {
            token
        } else {
            return None;
        };

        match token.kind == kind {
            true => {
                self.token_buffer.pop_front();
                self.expected.clear();
                self.last_span = token.span;

                Some(token)
            }
            false => None,
        }
    }

    pub fn kind_or_err(&mut self, kind: TokenKind) -> Result<Token, ()> {
        let item = self.kind(kind);
        self.make_item_or_err(item)
    }

    pub fn lookup_assignment_op(&mut self, offset: usize) -> bool {
        self.fetch_tokens(offset + 1);
        self.expected.push(TokenType::AssignmentOp);

        let token = if let Some(token) = self.token_buffer.get(offset) {
            token
        } else {
            return false;
        };

        match token.kind {
            TokenKind::Assign
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
            | TokenKind::AssignBitXor => true,
            _ => false,
        }
    }

    pub fn assignment_op(&mut self) -> Option<Token> {
        self.fetch_tokens(1);
        self.expected.push(TokenType::AssignmentOp);

        let token = if let Some(token) = self.token_buffer.get(0).cloned() {
            token
        } else {
            return None;
        };

        match token.kind {
            TokenKind::Assign
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
            | TokenKind::AssignBitXor => {
                self.token_buffer.pop_front();
                self.expected.clear();
                self.last_span = token.span;

                Some(token)
            }
            _ => None,
        }
    }

    pub fn assignment_op_or_err(&mut self) -> Result<Token, ()> {
        let item = self.assignment_op();
        self.make_item_or_err(item)
    }

    pub fn lookup_unary_op(&mut self, offset: usize) -> bool {
        self.fetch_tokens(offset + 1);
        self.expected.push(TokenType::UnaryOp);

        let token = if let Some(token) = self.token_buffer.get(offset) {
            token
        } else {
            return false;
        };

        match token.kind {
            TokenKind::Add
            | TokenKind::Sub
            | TokenKind::BitNot
            | TokenKind::LogNot
            | TokenKind::BitAnd
            | TokenKind::Mul => true,
            _ => false,
        }
    }

    pub fn unary_op(&mut self) -> Option<Token> {
        self.fetch_tokens(1);
        self.expected.push(TokenType::UnaryOp);

        let token = if let Some(token) = self.token_buffer.get(0).cloned() {
            token
        } else {
            return None;
        };

        match token.kind {
            TokenKind::Add
            | TokenKind::Sub
            | TokenKind::BitNot
            | TokenKind::LogNot
            | TokenKind::BitAnd
            | TokenKind::Mul => {
                self.token_buffer.pop_front();
                self.expected.clear();
                self.last_span = token.span;

                Some(token)
            }
            _ => None,
        }
    }

    pub fn unary_op_or_err(&mut self) -> Result<Token, ()> {
        let item = self.unary_op();
        self.make_item_or_err(item)
    }

    pub fn lookup_binary_op(&mut self, offset: usize) -> bool {
        self.fetch_tokens(offset + 1);
        self.expected.push(TokenType::BinaryOp);

        let token = if let Some(token) = self.token_buffer.get(offset) {
            token
        } else {
            return false;
        };

        match token.kind {
            TokenKind::Eq
            | TokenKind::Ne
            | TokenKind::Lt
            | TokenKind::Gt
            | TokenKind::Le
            | TokenKind::Ge
            | TokenKind::Add
            | TokenKind::Sub
            | TokenKind::Mul
            | TokenKind::Div
            | TokenKind::Mod
            | TokenKind::Pow
            | TokenKind::Shl
            | TokenKind::Shr
            | TokenKind::BitOr
            | TokenKind::BitAnd
            | TokenKind::BitXor
            | TokenKind::LogOr
            | TokenKind::LogAnd => true,
            _ => false,
        }
    }

    pub fn binary_op(&mut self) -> Option<Token> {
        self.fetch_tokens(1);
        self.expected.push(TokenType::BinaryOp);

        let token = if let Some(token) = self.token_buffer.get(0).cloned() {
            token
        } else {
            return None;
        };

        match token.kind {
            TokenKind::Eq
            | TokenKind::Ne
            | TokenKind::Lt
            | TokenKind::Gt
            | TokenKind::Le
            | TokenKind::Ge
            | TokenKind::Add
            | TokenKind::Sub
            | TokenKind::Mul
            | TokenKind::Div
            | TokenKind::Mod
            | TokenKind::Pow
            | TokenKind::Shl
            | TokenKind::Shr
            | TokenKind::BitOr
            | TokenKind::BitAnd
            | TokenKind::BitXor
            | TokenKind::LogOr
            | TokenKind::LogAnd => {
                self.token_buffer.pop_front();
                self.expected.clear();
                self.last_span = token.span;

                Some(token)
            }
            _ => None,
        }
    }

    pub fn binary_op_or_err(&mut self) -> Result<Token, ()> {
        let item = self.binary_op();
        self.make_item_or_err(item)
    }

    pub fn literal(&mut self) -> Option<TokenLiteral> {
        self.fetch_tokens(1);
        self.expected.push(TokenType::Literal);

        let token = if let Some(token) = self.token_buffer.get(0).cloned() {
            token
        } else {
            return None;
        };

        match token.kind {
            TokenKind::Literal(literal) => {
                self.token_buffer.pop_front();
                self.expected.clear();
                self.last_span = token.span;

                Some(literal)
            }
            _ => None,
        }
    }

    pub fn literal_op_or_err(&mut self) -> Result<TokenLiteral, ()> {
        let item = self.literal();
        self.make_item_or_err(item)
    }

    pub fn skip_tokens(&mut self, mut f: impl FnMut(&Token) -> bool) {
        self.expected.clear();

        while let Some(token) = self.next() {
            if !f(&token) {
                break;
            }

            self.token_buffer.pop_front();
            self.last_span = token.span;
        }
    }

    fn fill_buffer(&mut self) -> bool {
        let token = if let Some(token) = self.token_stream.next() {
            match token.kind {
                TokenKind::Whitespace | TokenKind::Comment => {
                    self.last_span = token.span;
                    return true;
                }
                _ => token,
            }
        } else {
            return false;
        };

        match self.unglue_tokens {
            true => {
                token.unglue(&mut self.token_buffer);
            }
            false => {
                self.token_buffer.push_back(token);
            }
        }

        true
    }

    fn fetch_tokens(&mut self, min_len: usize) {
        while self.token_buffer.len() < min_len {
            if !self.fill_buffer() {
                break;
            }
        }
    }

    fn current_pos(&mut self) -> Pos {
        self.fetch_tokens(1);

        match self.token_buffer.front() {
            Some(front) => front.span.low,
            None => self.diagnostics.file().span().high,
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.fetch_tokens(1);
        self.token_buffer.front().cloned()
    }

    fn make_expected_but_found_err(&mut self, found: TokenType) -> String {
        self.expected.sort_unstable();
        self.expected.dedup();

        match self.expected.split_last() {
            Some((last, rest)) if !rest.is_empty() => {
                let found = match found {
                    found @ TokenType::Eof => {
                        format!("{} reached", found)
                    }
                    found @ _ => {
                        format!("found {}", found)
                    }
                };
                let rest = rest
                    .into_iter()
                    .map(|expected| expected.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} or {} expected, but {}", rest, *last, found)
            }
            Some((last, _)) => {
                let found = match found {
                    found @ TokenType::Eof => {
                        format!("{} reached", found)
                    }
                    found @ _ => {
                        format!("found {}", found)
                    }
                };
                format!("{} expected, but {}", *last, found)
            }
            None => match found {
                found @ TokenType::Eof => {
                    format!("unexpected {} reached", found)
                }
                found @ _ => {
                    format!("unexpected {} found", found)
                }
            },
        }
    }

    fn make_item_or_err<U>(&mut self, item: Option<U>) -> Result<U, ()> {
        item.ok_or_else(|| {
            match self.next() {
                Some(token) => {
                    self.diagnostics.error(
                        token.span,
                        self.make_expected_but_found_err(TokenType::Token(token.kind)),
                    );
                }
                None => {
                    self.diagnostics.error(
                        self.diagnostics.file().span_end(),
                        self.make_expected_but_found_err(TokenType::Eof),
                    );
                }
            }
            self.expected.clear();
        })
    }

    pub fn make_span(&self, pos: Pos) -> Span {
        Span::new(pos, self.last_span.high)
    }
}
