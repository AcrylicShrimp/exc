mod node_id;
mod node_id_allocator;
mod punctuated;

pub use node_id::*;
pub use node_id_allocator::*;
pub use punctuated::*;

use crate::{Token, TokenKind, TokenLiteral};
use exc_diagnostic::DiagnosticsSender;
use exc_span::Span;
use exc_symbol::Symbol;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Id {
    pub span: Span,
    pub symbol: Symbol,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTModule {
    pub id: NodeId,
    pub span: Span,
    pub items: Vec<ASTModuleItem>,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTModuleItem {
    pub id: NodeId,
    pub span: Span,
    pub kind: ASTModuleItemKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTModuleItemKind {
    Use(Arc<ASTUse>),
    AliasDef(Arc<ASTAliasDef>),
    ModuleDecl(Arc<ASTModuleDecl>),
    ModuleDef(Arc<ASTModuleDef>),
    ExternBlock(ASTExternBlock),
    FnDef(Arc<ASTFnDef>),
    StructDef(Arc<ASTStructDef>),
    InterfaceDef(Arc<ASTInterfaceDef>),
    ImplBlock(ASTImplBlock),
}

#[derive(Debug, Clone, Hash)]
pub struct ASTUse {
    pub id: NodeId,
    pub span: Span,
    pub keyword_pub: Option<Id>, // pub
    pub keyword_use: Id,         // use
    pub path: ASTUsePath,        // self::super::identifier::*
    pub token_semicolon: Token,  // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTUsePath {
    pub id: NodeId,
    pub span: Span,
    pub prefix: Option<ASTUsePathPrefix>, // self::super::identifier::
    pub item: ASTUsePathItem,             // * | identifier | { ... }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTUsePathPrefix {
    pub id: NodeId,
    pub span: Span,
    pub segments: Vec<ASTUsePathPrefixSegment>, // self::super::identifier::
}

#[derive(Debug, Clone, Hash)]
pub struct ASTUsePathPrefixSegment {
    pub id: NodeId,
    pub span: Span,
    pub kind: ASTUsePathPrefixSegmentKind, // self | super | identifier
    pub token_path_sep: Token,             // ::
}

#[derive(Debug, Clone, Hash)]
pub enum ASTUsePathPrefixSegmentKind {
    /// TODO: consider spec out `self`, as it is not needed
    Self_(Id), // self
    Super_(Id),     // super
    Identifier(Id), // identifier
}

impl ASTUsePathPrefixSegmentKind {
    pub fn id(&self) -> Id {
        match self {
            Self::Self_(id) => *id,
            Self::Super_(id) => *id,
            Self::Identifier(id) => *id,
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTUsePathItem {
    pub id: NodeId,
    pub span: Span,
    pub kind: ASTUsePathItemKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTUsePathItemKind {
    All(Token),                   // *
    Single(ASTUsePathItemSingle), // identifier | identifier as identifier
    Group(ASTUsePathItemGroup),   // { ... }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTUsePathItemSingle {
    pub id: NodeId,
    pub span: Span,
    pub identifier: Id,                           // identifier
    pub alias: Option<ASTUsePathItemSingleAlias>, // as identifier
}

#[derive(Debug, Clone, Hash)]
pub struct ASTUsePathItemSingleAlias {
    pub id: NodeId,
    pub span: Span,
    pub keyword_as: Id, // as
    pub identifier: Id, // identifier
}

#[derive(Debug, Clone, Hash)]
pub struct ASTUsePathItemGroup {
    pub id: NodeId,
    pub span: Span,
    pub token_brace_open: Token, // {
    pub items: Punctuated<ASTUsePath, { PUNCUATION_KIND_COMMA }>,
    pub token_brace_close: Token, // }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTAliasDef {
    pub id: NodeId,
    pub span: Span,
    pub keyword_pub: Option<Id>, // pub
    pub keyword_alias: Id,       // alias
    pub identifier: Id,          // identifier
    pub token_assign: Token,     // =
    pub ty: ASTTy,               // ty
    pub token_semicolon: Token,  // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTModuleDecl {
    pub id: NodeId,
    pub span: Span,
    pub keyword_pub: Option<Id>, // pub
    pub keyword_module: Id,      // module
    pub identifier: Id,          // identifier
    pub token_semicolon: Token,  // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTModuleDef {
    pub id: NodeId,
    pub span: Span,
    pub keyword_pub: Option<Id>, // pub
    pub keyword_module: Id,      // module
    pub identifier: Id,          // identifier
    pub token_brace_open: Token, // {
    pub items: Vec<ASTModuleItem>,
    pub token_brace_close: Token, // }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExternBlock {
    pub id: NodeId,
    pub span: Span,
    pub keyword_extern: Id,      // extern
    pub token_brace_open: Token, // {
    pub items: Vec<ASTExternBlockItem>,
    pub token_brace_close: Token, // }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExternBlockItem {
    pub id: NodeId,
    pub span: Span,
    pub kind: ASTExternBlockItemKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTExternBlockItemKind {
    PrototypeDef(Arc<ASTPrototypeDef>),
    FnDef(Arc<ASTFnDef>),
    StructDef(Arc<ASTStructDef>),
    ImplBlock(ASTImplBlock),
}

#[derive(Debug, Clone, Hash)]
pub struct ASTPrototypeDef {
    pub id: NodeId,
    pub span: Span,
    pub keyword_pub: Option<Id>,                                   // pub
    pub keyword_prototype: Id,                                     // prototype
    pub identifier: Id,                                            // identifier
    pub token_paren_open: Token,                                   // (
    pub params: Punctuated<ASTFnParam, { PUNCUATION_KIND_COMMA }>, // ...
    pub token_paren_close: Token,                                  // )
    pub result: Option<ASTFnResult>,                               // -> ty
    pub token_semicolon: Token,                                    // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTFnDef {
    pub id: NodeId,
    pub span: Span,
    pub keyword_pub: Option<Id>,                // pub
    pub keyword_fn: Id,                         // fn
    pub identifier: Id,                         // identifier
    pub generic_param: Option<ASTGenericParam>, // <...>
    pub token_paren_open: Token,                // (
    pub params: Punctuated<ASTFnParam, { PUNCUATION_KIND_COMMA }>,
    pub token_paren_close: Token,               // )
    pub result: Option<ASTFnResult>,            // -> ty
    pub generic_where: Option<ASTGenericWhere>, // where ...
    pub stmt_block: ASTStmtBlock,               // { ... }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTFnParam {
    pub id: NodeId,
    pub span: Span,
    pub identifier: Id,     // identifier
    pub token_colon: Token, // :
    pub ty: ASTTy,          // ty
}

#[derive(Debug, Clone, Hash)]
pub struct ASTFnResult {
    pub id: NodeId,
    pub span: Span,
    pub token_arrow: Token, // ->
    pub ty: ASTTy,          // ty
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStructDef {
    pub id: NodeId,
    pub span: Span,
    pub keyword_pub: Option<Id>,                // pub
    pub keyword_struct: Id,                     // struct
    pub identifier: Id,                         // identifier
    pub generic_param: Option<ASTGenericParam>, // <...>
    pub generic_where: Option<ASTGenericWhere>, // where ...
    pub token_brace_open: Token,                // {
    pub fields: Punctuated<ASTStructDefField, { PUNCUATION_KIND_COMMA }>,
    pub token_brace_close: Token, // }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStructDefField {
    pub id: NodeId,
    pub span: Span,
    pub identifier: Id,     // identifier
    pub token_colon: Token, // :
    pub ty: ASTTy,          // ty
}

#[derive(Debug, Clone, Hash)]
pub struct ASTInterfaceDef {
    pub id: NodeId,
    pub span: Span,
    pub keyword_pub: Option<Id>,                // pub
    pub keyword_interface: Id,                  // interface
    pub identifier: Id,                         // identifier
    pub generic_param: Option<ASTGenericParam>, // <...>
    pub generic_where: Option<ASTGenericWhere>, // where ...
    pub token_brace_open: Token,                // {
    pub items: Vec<ASTInterfaceDefItem>,        // ...
    pub token_brace_close: Token,               // }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTInterfaceDefItem {
    pub id: NodeId,
    pub span: Span,
    pub kind: ASTInterfaceDefItemKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTInterfaceDefItemKind {
    FnDecl(ASTInterfaceDefItemFnDecl),
}

#[derive(Debug, Clone, Hash)]
pub struct ASTInterfaceDefItemFnDecl {
    pub id: NodeId,
    pub span: Span,
    pub keyword_fn: Id,                                            // fn
    pub identifier: Id,                                            // identifier
    pub generic_param: Option<ASTGenericParam>,                    // <...>
    pub token_paren_open: Token,                                   // (
    pub params: Punctuated<ASTFnParam, { PUNCUATION_KIND_COMMA }>, // identifier: ty, ...
    pub token_paren_close: Token,                                  // )
    pub result: Option<ASTFnResult>,                               // -> ty
    pub generic_where: Option<ASTGenericWhere>,                    // where ...
    pub token_semicolon: Token,                                    // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTImplBlock {
    pub id: NodeId,
    pub span: Span,
    pub keyword_impl: Id,                         // impl
    pub generic_param: Option<ASTGenericParam>,   // <...>
    pub ty: ASTTy,                                // ty
    pub interface: Option<ASTImplBlockInterface>, // interface path::to::interface
    pub generic_where: Option<ASTGenericWhere>,   // where ...
    pub token_brace_open: Token,                  // {
    pub items: Vec<ASTImplBlockItem>,             // ...
    pub token_brace_close: Token,                 // }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTImplBlockInterface {
    pub id: NodeId,
    pub span: Span,
    pub keyword_interface: Id, // interface
    pub path: ASTPath,         // path::to::interface
}

#[derive(Debug, Clone, Hash)]
pub struct ASTImplBlockItem {
    pub id: NodeId,
    pub span: Span,
    pub kind: ASTImplBlockItemKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTImplBlockItemKind {
    FnDef(ASTFnDef),
}

#[derive(Debug, Clone, Hash)]
pub struct ASTGenericParam {
    pub id: NodeId,
    pub span: Span,
    pub token_angle_open: Token, // <
    pub items: Punctuated<ASTGenericParamItem, { PUNCUATION_KIND_COMMA }>,
    pub token_angle_close: Token, // >
}

#[derive(Debug, Clone, Hash)]
pub struct ASTGenericParamItem {
    pub id: NodeId,
    pub span: Span,
    pub identifier: Id, // identifier
}

#[derive(Debug, Clone, Hash)]
pub struct ASTGenericWhere {
    pub id: NodeId,
    pub span: Span,
    pub keyword_where: Id, // where
    pub items: Punctuated<ASTGenericWhereItem, { PUNCUATION_KIND_COMMA }>,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTGenericWhereItem {
    pub id: NodeId,
    pub span: Span,
    pub identifier: Id,                          // identifier
    pub token_colon: Token,                      // :
    pub condition: ASTGenericWhereItemCondition, // path::to::interface + path::to::interface
}

#[derive(Debug, Clone, Hash)]
pub struct ASTGenericWhereItemCondition {
    pub id: NodeId,
    pub span: Span,
    pub path: ASTPath,                                      // path::to::interface
    pub extra_items: Vec<ASTGenericWhereItemConditionItem>, // + path::to::interface
}

#[derive(Debug, Clone, Hash)]
pub struct ASTGenericWhereItemConditionItem {
    pub id: NodeId,
    pub span: Span,
    pub token_plus: Token, // +
    pub path: ASTPath,     // path::to::interface
}

#[derive(Debug, Clone, Hash)]
pub struct ASTGenericArg {
    pub id: NodeId,
    pub span: Span,
    pub token_angle_open: Token, // <
    pub args: Punctuated<ASTTy, { PUNCUATION_KIND_COMMA }>,
    pub token_angle_close: Token, // >
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtBlock {
    pub id: NodeId,
    pub span: Span,
    pub token_brace_open: Token, // {
    pub stmts: Vec<ASTStmt>,
    pub token_brace_close: Token, // }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtLet {
    pub id: NodeId,
    pub span: Span,
    pub keyword_let: Id,              // let
    pub identifier: Id,               // identifier
    pub ty: Option<ASTStmtLetTy>,     // : ty
    pub expr: Option<ASTStmtLetExpr>, // = expression
    pub token_semicolon: Token,       // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtLetTy {
    pub id: NodeId,
    pub span: Span,
    pub token_colon: Token, // :
    pub ty: ASTTy,          // ty
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtLetExpr {
    pub id: NodeId,
    pub span: Span,
    pub token_assign: Token, // =
    pub expr: ASTExpr,       // expression
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtIf {
    pub id: NodeId,
    pub span: Span,
    pub keyword_if: Id,           // if
    pub expr: ASTExpr,            // expression
    pub stmt_block: ASTStmtBlock, // { ... }
    pub else_ifs: Vec<ASTStmtIfElseIf>,
    pub else_: Option<ASTStmtIfElse>,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtIfElseIf {
    pub id: NodeId,
    pub span: Span,
    pub keyword_else: Id,         // else
    pub keyword_if: Id,           // if
    pub expr: ASTExpr,            // expression
    pub stmt_block: ASTStmtBlock, // { ... }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtIfElse {
    pub id: NodeId,
    pub span: Span,
    pub keyword_else: Id,         // else
    pub stmt_block: ASTStmtBlock, // { ... }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtLoop {
    pub id: NodeId,
    pub span: Span,
    pub keyword_loop: Id,         // loop
    pub stmt_block: ASTStmtBlock, // { ... }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtWhile {
    pub id: NodeId,
    pub span: Span,
    pub keyword_while: Id,        // while
    pub expr: ASTExpr,            // expression
    pub stmt_block: ASTStmtBlock, // { ... }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtBreak {
    pub id: NodeId,
    pub span: Span,
    pub keyword_break: Id,      // break
    pub token_semicolon: Token, // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtContinue {
    pub id: NodeId,
    pub span: Span,
    pub keyword_continue: Id,   // continue
    pub token_semicolon: Token, // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtReturn {
    pub id: NodeId,
    pub span: Span,
    pub keyword_return: Id, // return
    pub expr: Option<ASTExpr>,
    pub token_semicolon: Token, // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtAssignment {
    pub id: NodeId,
    pub span: Span,
    pub operand_lhs: ASTExpr,
    pub operator: ASTStmtAssignmentOperator,
    pub operand_rhs: ASTExpr,
    pub token_semicolon: Token, // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtAssignmentOperator {
    pub id: NodeId,
    pub span: Span,
    pub token_operator: Token,
    pub kind: ASTStmtAssignmentOperatorKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTStmtAssignmentOperatorKind {
    Assignment, // =
    Add,        // +=
    Sub,        // -=
    Mul,        // *=
    Div,        // /=
    Mod,        // %=
    Pow,        // **=
    Shl,        // <<=
    Shr,        // >>=
    BitOr,      // |=
    BitAnd,     // &=
    BitXor,     // ^=
}

impl ASTStmtAssignmentOperatorKind {
    pub fn from_token(token: &Token, diagnostics: &DiagnosticsSender) -> Result<Self, ()> {
        match token.kind {
            TokenKind::Assign => Ok(Self::Assignment),
            TokenKind::AssignAdd => Ok(Self::Add),
            TokenKind::AssignSub => Ok(Self::Sub),
            TokenKind::AssignMul => Ok(Self::Mul),
            TokenKind::AssignDiv => Ok(Self::Div),
            TokenKind::AssignMod => Ok(Self::Mod),
            TokenKind::AssignPow => Ok(Self::Pow),
            TokenKind::AssignShl => Ok(Self::Shl),
            TokenKind::AssignShr => Ok(Self::Shr),
            TokenKind::AssignBitOr => Ok(Self::BitOr),
            TokenKind::AssignBitAnd => Ok(Self::BitAnd),
            TokenKind::AssignBitXor => Ok(Self::BitXor),
            _ => {
                diagnostics.error(
                    token.span,
                    format!(
                        "{} is not a valid assignment operator",
                        token.kind.into_symbol()
                    ),
                );
                Err(())
            }
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmtExpr {
    pub id: NodeId,
    pub span: Span,
    pub expr: ASTExpr,          // expression
    pub token_semicolon: Token, // ;
}

#[derive(Debug, Clone, Hash)]
pub struct ASTStmt {
    pub id: NodeId,
    pub span: Span,
    pub kind: ASTStmtKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTStmtKind {
    Block(ASTStmtBlock),
    Let(ASTStmtLet),
    If(ASTStmtIf),
    Loop(ASTStmtLoop),
    While(ASTStmtWhile),
    Break(ASTStmtBreak),
    Continue(ASTStmtContinue),
    Return(ASTStmtReturn),
    Assignment(ASTStmtAssignment),
    Expr(ASTStmtExpr),
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExpr {
    pub id: NodeId,
    pub span: Span,
    pub kind: ASTExprKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTExprKind {
    Binary(ASTExprBinary),               // Precedence 5 : postfix
    As(ASTExprAs),                       // Precedence 4 : postfix
    Unary(ASTExprUnary),                 // Precedence 3 : prefix
    Call(ASTExprCall),                   // Precedence 2 : postfix
    Member(ASTExprMember),               // Precedence 2 : postfix
    Paren(ASTExprParen),                 // Precedence 1 : prefix
    Path(ASTExprPath),                   // Precedence 1 : single item
    Literal(ASTExprLiteral),             // Precedence 1 : single item
    StructLiteral(ASTExprStructLiteral), // Precedence 1 : single item
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprBinary {
    pub id: NodeId,
    pub span: Span,
    pub operand_lhs: Box<ASTExpr>,
    pub operator: ASTExprBinaryOperator,
    pub operand_rhs: Box<ASTExpr>,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprBinaryOperator {
    pub id: NodeId,
    pub span: Span,
    pub token_operator: Token,
    pub kind: ASTExprBinaryOperatorKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTExprBinaryOperatorKind {
    Eq,     // Precedence 7 : compare
    Ne,     // Precedence 7 : compare
    Lt,     // Precedence 7 : compare
    Gt,     // Precedence 7 : compare
    Le,     // Precedence 7 : compare
    Ge,     // Precedence 7 : compare
    LogOr,  // Precedence 6 : logical_or_and
    LogAnd, // Precedence 6 : logical_or_and
    Add,    // Precedence 5 : arithmetic_add_sub
    Sub,    // Precedence 5 : arithmetic_add_sub
    Mul,    // Precedence 4 : arithmetic_mul_div_mod
    Div,    // Precedence 4 : arithmetic_mul_div_mod
    Mod,    // Precedence 4 : arithmetic_mul_div_mod
    Pow,    // Precedence 3 : arithmetic_pow
    Shl,    // Precedence 2 : bit_shift
    Shr,    // Precedence 2 : bit_shift
    BitOr,  // Precedence 1 : bit_or_and_xor
    BitAnd, // Precedence 1 : bit_or_and_xor
    BitXor, // Precedence 1 : bit_or_and_xor
}

impl ASTExprBinaryOperatorKind {
    pub fn from_token(token: &Token, diagnostics: &DiagnosticsSender) -> Result<Self, ()> {
        match token.kind {
            TokenKind::Eq => Ok(Self::Eq),
            TokenKind::Ne => Ok(Self::Ne),
            TokenKind::Lt => Ok(Self::Lt),
            TokenKind::Gt => Ok(Self::Gt),
            TokenKind::Le => Ok(Self::Le),
            TokenKind::Ge => Ok(Self::Ge),
            TokenKind::Add => Ok(Self::Add),
            TokenKind::Sub => Ok(Self::Sub),
            TokenKind::Mul => Ok(Self::Mul),
            TokenKind::Div => Ok(Self::Div),
            TokenKind::Mod => Ok(Self::Mod),
            TokenKind::Pow => Ok(Self::Pow),
            TokenKind::Shl => Ok(Self::Shl),
            TokenKind::Shr => Ok(Self::Shr),
            TokenKind::BitOr => Ok(Self::BitOr),
            TokenKind::BitAnd => Ok(Self::BitAnd),
            TokenKind::BitXor => Ok(Self::BitXor),
            TokenKind::LogOr => Ok(Self::LogOr),
            TokenKind::LogAnd => Ok(Self::LogAnd),
            _ => {
                diagnostics.error(
                    token.span,
                    format!(
                        "{} is not a valid binary operator",
                        token.kind.into_symbol()
                    ),
                );
                Err(())
            }
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprAs {
    pub id: NodeId,
    pub span: Span,
    pub expr: Box<ASTExpr>,
    pub keyword_as: Id, // as
    pub ty: ASTTy,      // ty
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprUnary {
    pub id: NodeId,
    pub span: Span,
    pub operator: ASTExprUnaryOperator,
    pub operand_lhs: Box<ASTExpr>,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprUnaryOperator {
    pub id: NodeId,
    pub span: Span,
    pub token_operator: Token,
    pub kind: ASTExprUnaryOperatorKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTExprUnaryOperatorKind {
    Plus,
    Minus,
    BitNot,
    LogNot,
    AddressOf,
    Dereference,
}

impl ASTExprUnaryOperatorKind {
    pub fn from_token(token: &Token, diagnostics: &DiagnosticsSender) -> Result<Self, ()> {
        match token.kind {
            TokenKind::Add => Ok(Self::Plus),
            TokenKind::Sub => Ok(Self::Minus),
            TokenKind::BitNot => Ok(Self::BitNot),
            TokenKind::LogNot => Ok(Self::LogNot),
            TokenKind::BitAnd => Ok(Self::AddressOf),
            TokenKind::Mul => Ok(Self::Dereference),
            _ => {
                diagnostics.error(
                    token.span,
                    format!("{} is not a valid unary operator", token.kind.into_symbol()),
                );
                Err(())
            }
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprCall {
    pub id: NodeId,
    pub span: Span,
    pub callee: ASTExprCallCallee,
    pub token_paren_open: Token, // (
    pub args: Punctuated<ASTExpr, { PUNCUATION_KIND_COMMA }>,
    pub token_paren_close: Token, // )
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprCallCallee {
    pub id: NodeId,
    pub span: Span,
    pub expr: Box<ASTExpr>,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprMember {
    pub id: NodeId,
    pub span: Span,
    pub expr: Box<ASTExpr>,
    pub token_dot: Token, // .
    pub member: Id,       // identifier
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprParen {
    pub id: NodeId,
    pub span: Span,
    pub token_paren_open: Token, // (
    pub expr: Box<ASTExpr>,
    pub token_paren_close: Token, // )
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprPath {
    pub id: NodeId,
    pub span: Span,
    pub path: ASTPath,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprLiteral {
    pub id: NodeId,
    pub span: Span,
    pub literal: TokenLiteral,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprStructLiteral {
    pub id: NodeId,
    pub span: Span,
    pub path: ASTPath,
    pub token_brace_open: Token, // {
    pub fields: Punctuated<ASTExprStructLiteralField, { PUNCUATION_KIND_COMMA }>,
    pub token_brace_close: Token, // }
}

#[derive(Debug, Clone, Hash)]
pub struct ASTExprStructLiteralField {
    pub id: NodeId,
    pub span: Span,
    pub identifier: Id,     // identifier
    pub token_colon: Token, // :
    pub expr: ASTExpr,      // expression
}

#[derive(Debug, Clone, Hash)]
pub struct ASTPath {
    pub id: NodeId,
    pub span: Span,
    pub segments: Punctuated<ASTPathSegment, { PUNCUATION_KIND_PATH_SEP }>,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTPathSegment {
    pub id: NodeId,
    pub span: Span,
    pub identifier: Id, // identifier
    pub generic: Option<ASTGenericArg>,
}

#[derive(Debug, Clone, Hash)]
pub struct ASTTy {
    pub id: NodeId,
    pub span: Span,
    pub kind: ASTTyKind,
}

#[derive(Debug, Clone, Hash)]
pub enum ASTTyKind {
    Paren(ASTTyParen),
    Span(ASTTySpan),
    Array(ASTTyArray),
    FnPointer(ASTTyFnPointer),
    Path(ASTPath),
}

#[derive(Debug, Clone, Hash)]
pub struct ASTTyParen {
    pub id: NodeId,
    pub span: Span,
    pub token_paren_open: Token,  // (
    pub ty: Box<ASTTy>,           // ty
    pub token_paren_close: Token, // )
}

#[derive(Debug, Clone, Hash)]
pub struct ASTTySpan {
    pub id: NodeId,
    pub span: Span,
    pub token_bracket_open: Token,  // [
    pub ty: Box<ASTTy>,             // ty
    pub token_bracket_close: Token, // ]
}

#[derive(Debug, Clone, Hash)]
pub struct ASTTyArray {
    pub id: NodeId,
    pub span: Span,
    pub token_bracket_open: Token,  // [
    pub ty: Box<ASTTy>,             // ty
    pub token_semicolon: Token,     // ;
    pub literal: TokenLiteral,      // literal
    pub token_bracket_close: Token, // ]
}

#[derive(Debug, Clone, Hash)]
pub struct ASTTyFnPointer {
    pub id: NodeId,
    pub span: Span,
    pub keyword_fn: Id,                                       // fn
    pub token_paren_open: Token,                              // (
    pub params: Punctuated<ASTTy, { PUNCUATION_KIND_COMMA }>, // ty, ty, ..
    pub token_paren_close: Token,                             // )
    pub result: Option<Box<ASTFnResult>>,                     // -> ty
}
