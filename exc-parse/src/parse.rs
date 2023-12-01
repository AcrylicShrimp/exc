mod parser;
mod token_type;

pub use parser::*;
pub use token_type::*;

use crate::{
    before_expr, before_expr_call_item, before_expr_struct_literal_field_item,
    before_extern_block_item, before_fn_params_item, before_generic_arg_item,
    before_generic_param_item, before_generic_where_item, before_generic_where_item_condition_item,
    before_impl_block_item, before_interface_item, before_interface_item_fn_decl_params_item,
    before_module_item, before_prototype_params_item, before_stmt, before_struct_fields_item,
    before_ty_fn_pointer_param_item, before_use_path_item_group_item, ASTAliasDef, ASTExpr,
    ASTExprAs, ASTExprBinary, ASTExprBinaryOperator, ASTExprBinaryOperatorKind, ASTExprCall,
    ASTExprCallCallee, ASTExprKind, ASTExprLiteral, ASTExprMember, ASTExprParen, ASTExprPath,
    ASTExprStructLiteral, ASTExprStructLiteralField, ASTExprUnary, ASTExprUnaryOperator,
    ASTExprUnaryOperatorKind, ASTExternBlock, ASTExternBlockItem, ASTExternBlockItemKind, ASTFnDef,
    ASTFnParam, ASTFnResult, ASTGenericArg, ASTGenericParam, ASTGenericParamItem, ASTGenericWhere,
    ASTGenericWhereItem, ASTGenericWhereItemCondition, ASTGenericWhereItemConditionItem,
    ASTImplBlock, ASTImplBlockInterface, ASTImplBlockItem, ASTImplBlockItemKind, ASTInterfaceDef,
    ASTInterfaceDefItem, ASTInterfaceDefItemFnDecl, ASTInterfaceDefItemKind, ASTModule,
    ASTModuleDef, ASTModuleItem, ASTModuleItemKind, ASTPath, ASTPathSegment, ASTPrototypeDef,
    ASTStmt, ASTStmtAssignment, ASTStmtAssignmentOperator, ASTStmtAssignmentOperatorKind,
    ASTStmtBlock, ASTStmtBreak, ASTStmtContinue, ASTStmtExpr, ASTStmtIf, ASTStmtIfElse,
    ASTStmtIfElseIf, ASTStmtKind, ASTStmtLet, ASTStmtLetExpr, ASTStmtLetTy, ASTStmtLoop,
    ASTStmtReturn, ASTStmtWhile, ASTStructDef, ASTStructDefField, ASTTy, ASTTyArray,
    ASTTyFnPointer, ASTTyKind, ASTTyParen, ASTTySpan, ASTUse, ASTUsePath, ASTUsePathItem,
    ASTUsePathItemGroup, ASTUsePathItemKind, ASTUsePathItemSingle, ASTUsePathItemSingleAlias,
    ASTUsePathPrefix, ASTUsePathPrefixSegment, ASTUsePathPrefixSegmentKind, NodeIdAllocator,
    Punctuated, PunctuatedItem, Token, TokenKind, KEYWORD_ALIAS, KEYWORD_AS, KEYWORD_BREAK,
    KEYWORD_CONTINUE, KEYWORD_ELSE, KEYWORD_EXTERN, KEYWORD_FN, KEYWORD_IF, KEYWORD_IMPL,
    KEYWORD_INTERFACE, KEYWORD_LET, KEYWORD_LOOP, KEYWORD_MODULE, KEYWORD_PROTOTYPE, KEYWORD_PUB,
    KEYWORD_RETURN, KEYWORD_SELF, KEYWORD_STRUCT, KEYWORD_SUPER, KEYWORD_USE, KEYWORD_WHERE,
    KEYWORD_WHILE,
};
use exc_diagnostic::DiagnosticsSender;

pub fn parse_module(
    token_stream: impl Iterator<Item = Token>,
    id_allocator: &mut NodeIdAllocator,
    diagnostics: &DiagnosticsSender,
) -> ASTModule {
    Parser::new(token_stream, id_allocator, diagnostics).parse_module()
}

impl<'a, 'd, T> Parser<'a, 'd, T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_module(&mut self) -> ASTModule {
        let (id, pos) = self.new_node();
        let mut items = Vec::new();

        while self.is_exists() {
            match self.parse_module_item() {
                Ok(item) => {
                    items.push(item);
                }
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| before_module_item(token));
                }
            }
        }

        ASTModule {
            id,
            span: self.make_span(pos),
            items,
        }
    }

    pub fn parse_module_item(&mut self) -> Result<ASTModuleItem, ()> {
        let (id, pos) = self.new_node();

        let kind = if self.lookup_keyword(0, *KEYWORD_USE)
            || (self.lookup_keyword(0, *KEYWORD_PUB) && self.lookup_keyword(1, *KEYWORD_USE))
        {
            ASTModuleItemKind::Use(self.parse_use()?)
        } else if self.lookup_keyword(0, *KEYWORD_ALIAS)
            || (self.lookup_keyword(0, *KEYWORD_PUB) && self.lookup_keyword(1, *KEYWORD_ALIAS))
        {
            ASTModuleItemKind::AliasDef(self.parse_alias_def()?)
        } else if self.lookup_keyword(0, *KEYWORD_MODULE)
            || (self.lookup_keyword(0, *KEYWORD_PUB) && self.lookup_keyword(1, *KEYWORD_MODULE))
        {
            ASTModuleItemKind::ModuleDef(self.parse_module_def()?)
        } else if self.lookup_keyword(0, *KEYWORD_EXTERN) {
            ASTModuleItemKind::ExternBlock(self.parse_extern_block()?)
        } else if self.lookup_keyword(0, *KEYWORD_FN)
            || (self.lookup_keyword(0, *KEYWORD_PUB) && self.lookup_keyword(1, *KEYWORD_FN))
        {
            ASTModuleItemKind::FnDef(self.parse_fn_def()?)
        } else if self.lookup_keyword(0, *KEYWORD_STRUCT)
            || (self.lookup_keyword(0, *KEYWORD_PUB) && self.lookup_keyword(1, *KEYWORD_STRUCT))
        {
            ASTModuleItemKind::StructDef(self.parse_struct_def()?)
        } else if self.lookup_keyword(0, *KEYWORD_INTERFACE)
            || (self.lookup_keyword(0, *KEYWORD_PUB) && self.lookup_keyword(1, *KEYWORD_INTERFACE))
        {
            ASTModuleItemKind::InterfaceDef(self.parse_interface_def()?)
        } else {
            ASTModuleItemKind::ImplBlock(self.parse_impl_block()?)
        };

        Ok(ASTModuleItem {
            id,
            span: self.make_span(pos),
            kind,
        })
    }

    pub fn parse_use(&mut self) -> Result<ASTUse, ()> {
        let (id, pos) = self.new_node();
        let keyword_pub = self.keyword(*KEYWORD_PUB);
        let keyword_use = self.keyword_or_err(*KEYWORD_USE)?;
        let path = self.parse_use_path()?;
        let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;

        Ok(ASTUse {
            id,
            span: self.make_span(pos),
            keyword_pub,
            keyword_use,
            path,
            token_semicolon,
        })
    }

    pub fn parse_use_path(&mut self) -> Result<ASTUsePath, ()> {
        let (id, pos) = self.new_node();
        let prefix = self.parse_use_path_prefix()?;
        let item = self.parse_use_path_item()?;

        Ok(ASTUsePath {
            id,
            span: self.make_span(pos),
            prefix: if prefix.segments.is_empty() {
                None
            } else {
                Some(prefix)
            },
            item,
        })
    }

    pub fn parse_use_path_prefix(&mut self) -> Result<ASTUsePathPrefix, ()> {
        let (id, pos) = self.new_node();

        let mut segments = Vec::new();

        while self.lookup_identifier(0) && self.lookup_kind(1, TokenKind::PathSep) {
            segments.push(self.parse_use_path_prefix_segment()?);
        }

        Ok(ASTUsePathPrefix {
            id,
            span: self.make_span(pos),
            segments,
        })
    }

    pub fn parse_use_path_prefix_segment(&mut self) -> Result<ASTUsePathPrefixSegment, ()> {
        let (id, pos) = self.new_node();

        let kind = if let Some(keyword) = self.keyword(*KEYWORD_SELF) {
            ASTUsePathPrefixSegmentKind::Self_(keyword)
        } else if let Some(keyword) = self.keyword(*KEYWORD_SUPER) {
            ASTUsePathPrefixSegmentKind::Super_(keyword)
        } else {
            ASTUsePathPrefixSegmentKind::Identifier(self.identifier_or_err()?)
        };

        let token_path_sep = self.kind_or_err(TokenKind::PathSep)?;

        Ok(ASTUsePathPrefixSegment {
            id,
            span: self.make_span(pos),
            kind,
            token_path_sep,
        })
    }

    pub fn parse_use_path_item(&mut self) -> Result<ASTUsePathItem, ()> {
        let (id, pos) = self.new_node();

        let kind = if let Some(token) = self.kind(TokenKind::Mul) {
            ASTUsePathItemKind::All(token)
        } else if self.lookup_identifier(0) {
            ASTUsePathItemKind::Single(self.parse_use_path_item_single()?)
        } else {
            ASTUsePathItemKind::Group(self.parse_use_path_item_group()?)
        };

        Ok(ASTUsePathItem {
            id,
            span: self.make_span(pos),
            kind,
        })
    }

    pub fn parse_use_path_item_single(&mut self) -> Result<ASTUsePathItemSingle, ()> {
        let (id, pos) = self.new_node();
        let identifier = self.identifier_or_err()?;
        let alias = if self.lookup_keyword(0, *KEYWORD_AS) {
            Some(self.parse_use_path_item_single_alias()?)
        } else {
            None
        };

        Ok(ASTUsePathItemSingle {
            id,
            span: self.make_span(pos),
            identifier,
            alias,
        })
    }

    pub fn parse_use_path_item_single_alias(&mut self) -> Result<ASTUsePathItemSingleAlias, ()> {
        let (id, pos) = self.new_node();
        let keyword_as = self.keyword_or_err(*KEYWORD_AS)?;
        let identifier = self.identifier_or_err()?;

        Ok(ASTUsePathItemSingleAlias {
            id,
            span: self.make_span(pos),
            keyword_as,
            identifier,
        })
    }

    pub fn parse_use_path_item_group(&mut self) -> Result<ASTUsePathItemGroup, ()> {
        let (id, pos) = self.new_node();
        let token_brace_open = self.kind_or_err(TokenKind::OpenBrace)?;

        let mut items = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseBrace) {
            let item = match self.parse_use_path() {
                Ok(item) => item,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_use_path_item_group_item(token) && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };

            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    items.push(PunctuatedItem::Punctuated { item, punctuation });
                }
                None => {
                    items.push(PunctuatedItem::NotPunctuated { item });
                    break;
                }
            }
        }

        let token_brace_close = self.kind_or_err(TokenKind::CloseBrace)?;

        Ok(ASTUsePathItemGroup {
            id,
            span: self.make_span(pos),
            token_brace_open,
            items: Punctuated { items },
            token_brace_close,
        })
    }

    pub fn parse_alias_def(&mut self) -> Result<ASTAliasDef, ()> {
        let (id, pos) = self.new_node();
        let keyword_pub = self.keyword(*KEYWORD_PUB);
        let keyword_alias = self.keyword_or_err(*KEYWORD_ALIAS)?;
        let identifier = self.identifier_or_err()?;
        let token_assign = self.kind_or_err(TokenKind::Assign)?;
        let ty = self.parse_ty()?;
        let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;

        Ok(ASTAliasDef {
            id,
            span: self.make_span(pos),
            keyword_pub,
            keyword_alias,
            identifier,
            token_assign,
            ty,
            token_semicolon,
        })
    }

    pub fn parse_module_def(&mut self) -> Result<ASTModuleDef, ()> {
        let (id, pos) = self.new_node();
        let keyword_pub = self.keyword(*KEYWORD_PUB);
        let keyword_module = self.keyword_or_err(*KEYWORD_MODULE)?;
        let identifier = self.identifier_or_err()?;
        let token_brace_open = self.kind_or_err(TokenKind::OpenBrace)?;

        let mut items = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseBrace) {
            match self.parse_module_item() {
                Ok(item) => {
                    items.push(item);
                }
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| before_module_item(token));
                }
            }
        }

        let token_brace_close = self.kind_or_err(TokenKind::CloseBrace)?;

        Ok(ASTModuleDef {
            id,
            span: self.make_span(pos),
            keyword_pub,
            keyword_module,
            identifier,
            token_brace_open,
            items,
            token_brace_close,
        })
    }

    pub fn parse_extern_block(&mut self) -> Result<ASTExternBlock, ()> {
        let (id, pos) = self.new_node();
        let keyword_extern = self.keyword_or_err(*KEYWORD_EXTERN)?;
        let token_brace_open = self.kind_or_err(TokenKind::OpenBrace)?;

        let mut items = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseBrace) {
            match self.parse_extern_block_item() {
                Ok(item) => {
                    items.push(item);
                }
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_extern_block_item(token) && before_module_item(token)
                    });
                }
            }
        }

        let token_brace_close = self.kind_or_err(TokenKind::CloseBrace)?;

        Ok(ASTExternBlock {
            id,
            span: self.make_span(pos),
            keyword_extern,
            token_brace_open,
            items,
            token_brace_close,
        })
    }

    pub fn parse_extern_block_item(&mut self) -> Result<ASTExternBlockItem, ()> {
        let (id, pos) = self.new_node();

        let kind = if self.lookup_keyword(0, *KEYWORD_PROTOTYPE)
            || (self.lookup_keyword(0, *KEYWORD_PUB) && self.lookup_keyword(1, *KEYWORD_PROTOTYPE))
        {
            ASTExternBlockItemKind::PrototypeDef(self.parse_prototype_def()?)
        } else if self.lookup_keyword(0, *KEYWORD_FN)
            || (self.lookup_keyword(0, *KEYWORD_PUB) && self.lookup_keyword(1, *KEYWORD_FN))
        {
            ASTExternBlockItemKind::FnDef(self.parse_fn_def()?)
        } else if self.lookup_keyword(0, *KEYWORD_STRUCT)
            || (self.lookup_keyword(0, *KEYWORD_PUB) && self.lookup_keyword(1, *KEYWORD_STRUCT))
        {
            ASTExternBlockItemKind::StructDef(self.parse_struct_def()?)
        } else {
            ASTExternBlockItemKind::ImplBlock(self.parse_impl_block()?)
        };

        Ok(ASTExternBlockItem {
            id,
            span: self.make_span(pos),
            kind,
        })
    }

    pub fn parse_prototype_def(&mut self) -> Result<ASTPrototypeDef, ()> {
        let (id, pos) = self.new_node();
        let keyword_pub = self.keyword(*KEYWORD_PUB);
        let keyword_prototype = self.keyword_or_err(*KEYWORD_PROTOTYPE)?;
        let identifier = self.identifier_or_err()?;
        let token_paren_open = self.kind_or_err(TokenKind::OpenParen)?;

        let mut params = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseParen) {
            let param = match self.parse_fn_param() {
                Ok(param) => param,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_prototype_params_item(token)
                            && before_extern_block_item(token)
                            && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };

            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    params.push(PunctuatedItem::Punctuated {
                        item: param,
                        punctuation,
                    });
                }
                None => {
                    params.push(PunctuatedItem::NotPunctuated { item: param });
                    break;
                }
            }
        }

        let token_paren_close = self.kind_or_err(TokenKind::CloseParen)?;
        let result = if self.lookup_kind(0, TokenKind::Arrow) {
            Some(self.parse_fn_result()?)
        } else {
            None
        };
        let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;

        Ok(ASTPrototypeDef {
            id,
            span: self.make_span(pos),
            keyword_pub,
            keyword_prototype,
            identifier,
            token_paren_open,
            params: Punctuated { items: params },
            token_paren_close,
            result,
            token_semicolon,
        })
    }

    pub fn parse_fn_def(&mut self) -> Result<ASTFnDef, ()> {
        let (id, pos) = self.new_node();
        let keyword_pub = self.keyword(*KEYWORD_PUB);
        let keyword_fn = self.keyword_or_err(*KEYWORD_FN)?;
        let identifier = self.identifier_or_err()?;
        let generic_param = if self.lookup_kind(0, TokenKind::Lt) {
            Some(self.parse_generic_param()?)
        } else {
            None
        };
        let token_paren_open = self.kind_or_err(TokenKind::OpenParen)?;

        let mut params = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseParen) {
            let param = match self.parse_fn_param() {
                Ok(param) => param,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_fn_params_item(token)
                            && before_extern_block_item(token)
                            && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };

            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    params.push(PunctuatedItem::Punctuated {
                        item: param,
                        punctuation,
                    });
                }
                None => {
                    params.push(PunctuatedItem::NotPunctuated { item: param });
                    break;
                }
            }
        }

        let token_paren_close = self.kind_or_err(TokenKind::CloseParen)?;
        let result = if self.lookup_kind(0, TokenKind::Arrow) {
            Some(self.parse_fn_result()?)
        } else {
            None
        };
        let generic_where = if self.lookup_keyword(0, *KEYWORD_WHERE) {
            Some(self.parse_generic_where()?)
        } else {
            None
        };
        let stmt_block = self.parse_stmt_block()?;

        Ok(ASTFnDef {
            id,
            span: self.make_span(pos),
            keyword_pub,
            keyword_fn,
            identifier,
            generic_param,
            token_paren_open,
            params: Punctuated { items: params },
            token_paren_close,
            result,
            generic_where,
            stmt_block,
        })
    }

    pub fn parse_fn_param(&mut self) -> Result<ASTFnParam, ()> {
        let (id, pos) = self.new_node();
        let identifier = self.identifier_or_err()?;
        let token_colon = self.kind_or_err(TokenKind::Colon)?;
        let ty = self.parse_ty()?;

        Ok(ASTFnParam {
            id,
            span: self.make_span(pos),
            identifier,
            token_colon,
            ty,
        })
    }

    pub fn parse_fn_result(&mut self) -> Result<ASTFnResult, ()> {
        let (id, pos) = self.new_node();
        let token_arrow = self.kind_or_err(TokenKind::Arrow)?;
        let ty = self.parse_ty()?;

        Ok(ASTFnResult {
            id,
            span: self.make_span(pos),
            token_arrow,
            ty,
        })
    }

    pub fn parse_struct_def(&mut self) -> Result<ASTStructDef, ()> {
        let (id, pos) = self.new_node();
        let keyword_pub = self.keyword(*KEYWORD_PUB);
        let keyword_struct = self.keyword_or_err(*KEYWORD_STRUCT)?;
        let identifier = self.identifier_or_err()?;
        let generic_param = if self.lookup_kind(0, TokenKind::Lt) {
            Some(self.parse_generic_param()?)
        } else {
            None
        };
        let generic_where = if self.lookup_keyword(0, *KEYWORD_WHERE) {
            Some(self.parse_generic_where()?)
        } else {
            None
        };
        let token_brace_open = self.kind_or_err(TokenKind::OpenBrace)?;

        let mut fields = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseBrace) {
            let field = match self.parse_struct_def_field() {
                Ok(field) => field,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_struct_fields_item(token)
                            && before_extern_block_item(token)
                            && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };

            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    fields.push(PunctuatedItem::Punctuated {
                        item: field,
                        punctuation,
                    });
                }
                None => {
                    fields.push(PunctuatedItem::NotPunctuated { item: field });
                    break;
                }
            }
        }

        let token_brace_close = self.kind_or_err(TokenKind::CloseBrace)?;

        Ok(ASTStructDef {
            id,
            span: self.make_span(pos),
            keyword_pub,
            keyword_struct,
            identifier,
            generic_param,
            generic_where,
            token_brace_open,
            fields: Punctuated { items: fields },
            token_brace_close,
        })
    }

    pub fn parse_struct_def_field(&mut self) -> Result<ASTStructDefField, ()> {
        let (id, pos) = self.new_node();
        let identifier = self.identifier_or_err()?;
        let token_colon = self.kind_or_err(TokenKind::Colon)?;
        let ty = self.parse_ty()?;

        Ok(ASTStructDefField {
            id,
            span: self.make_span(pos),
            identifier,
            token_colon,
            ty,
        })
    }

    pub fn parse_interface_def(&mut self) -> Result<ASTInterfaceDef, ()> {
        let (id, pos) = self.new_node();
        let keyword_pub = self.keyword(*KEYWORD_PUB);
        let keyword_interface = self.keyword_or_err(*KEYWORD_INTERFACE)?;
        let identifier = self.identifier_or_err()?;
        let generic_param = if self.lookup_kind(0, TokenKind::Lt) {
            Some(self.parse_generic_param()?)
        } else {
            None
        };
        let generic_where = if self.lookup_keyword(0, *KEYWORD_WHERE) {
            Some(self.parse_generic_where()?)
        } else {
            None
        };
        let token_brace_open = self.kind_or_err(TokenKind::OpenBrace)?;

        let mut items = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseBrace) {
            match self.parse_interface_def_item() {
                Ok(item) => {
                    items.push(item);
                }
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_interface_item(token) && before_module_item(token)
                    });
                }
            }
        }

        let token_brace_close = self.kind_or_err(TokenKind::CloseBrace)?;

        Ok(ASTInterfaceDef {
            id,
            span: self.make_span(pos),
            keyword_pub,
            keyword_interface,
            identifier,
            generic_param,
            generic_where,
            token_brace_open,
            items,
            token_brace_close,
        })
    }

    pub fn parse_interface_def_item(&mut self) -> Result<ASTInterfaceDefItem, ()> {
        let (id, pos) = self.new_node();
        let kind = ASTInterfaceDefItemKind::FnDecl(self.parse_interface_def_item_fn_decl()?);

        Ok(ASTInterfaceDefItem {
            id,
            span: self.make_span(pos),
            kind,
        })
    }

    pub fn parse_interface_def_item_fn_decl(&mut self) -> Result<ASTInterfaceDefItemFnDecl, ()> {
        let (id, pos) = self.new_node();
        let keyword_fn = self.keyword_or_err(*KEYWORD_FN)?;
        let identifier = self.identifier_or_err()?;
        let generic_param = if self.lookup_kind(0, TokenKind::Lt) {
            Some(self.parse_generic_param()?)
        } else {
            None
        };
        let token_paren_open = self.kind_or_err(TokenKind::OpenParen)?;

        let mut params = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseParen) {
            let param = match self.parse_fn_param() {
                Ok(param) => param,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_interface_item_fn_decl_params_item(token)
                            && before_interface_item(token)
                            && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };

            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    params.push(PunctuatedItem::Punctuated {
                        item: param,
                        punctuation,
                    });
                }
                None => {
                    params.push(PunctuatedItem::NotPunctuated { item: param });
                    break;
                }
            }
        }

        let token_paren_close = self.kind_or_err(TokenKind::CloseParen)?;
        let result = if self.lookup_kind(0, TokenKind::Arrow) {
            Some(self.parse_fn_result()?)
        } else {
            None
        };
        let generic_where = if self.lookup_keyword(0, *KEYWORD_WHERE) {
            Some(self.parse_generic_where()?)
        } else {
            None
        };
        let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;

        Ok(ASTInterfaceDefItemFnDecl {
            id,
            span: self.make_span(pos),
            keyword_fn,
            identifier,
            generic_param,
            token_paren_open,
            params: Punctuated { items: params },
            token_paren_close,
            result,
            generic_where,
            token_semicolon,
        })
    }

    pub fn parse_impl_block(&mut self) -> Result<ASTImplBlock, ()> {
        let (id, pos) = self.new_node();
        let keyword_impl = self.keyword_or_err(*KEYWORD_IMPL)?;
        let generic_param = if self.lookup_kind(0, TokenKind::Lt) {
            Some(self.parse_generic_param()?)
        } else {
            None
        };
        let ty = self.parse_ty()?;
        let interface = if self.lookup_keyword(0, *KEYWORD_INTERFACE) {
            Some(self.parse_impl_block_interface()?)
        } else {
            None
        };
        let generic_where = if self.lookup_keyword(0, *KEYWORD_WHERE) {
            Some(self.parse_generic_where()?)
        } else {
            None
        };
        let token_brace_open = self.kind_or_err(TokenKind::OpenBrace)?;

        let mut items = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseBrace) {
            match self.parse_impl_block_item() {
                Ok(item) => {
                    items.push(item);
                }
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_impl_block_item(token) && before_module_item(token)
                    });
                }
            }
        }

        let token_brace_close = self.kind_or_err(TokenKind::CloseBrace)?;

        Ok(ASTImplBlock {
            id,
            span: self.make_span(pos),
            keyword_impl,
            generic_param,
            ty,
            interface,
            generic_where,
            token_brace_open,
            items,
            token_brace_close,
        })
    }

    pub fn parse_impl_block_interface(&mut self) -> Result<ASTImplBlockInterface, ()> {
        let (id, pos) = self.new_node();
        let keyword_interface = self.keyword_or_err(*KEYWORD_INTERFACE)?;
        let path = self.parse_path()?;

        Ok(ASTImplBlockInterface {
            id,
            span: self.make_span(pos),
            keyword_interface,
            path,
        })
    }

    pub fn parse_impl_block_item(&mut self) -> Result<ASTImplBlockItem, ()> {
        let (id, pos) = self.new_node();
        let kind = ASTImplBlockItemKind::FnDef(self.parse_fn_def()?);

        Ok(ASTImplBlockItem {
            id,
            span: self.make_span(pos),
            kind,
        })
    }

    pub fn parse_generic_param(&mut self) -> Result<ASTGenericParam, ()> {
        let (id, pos) = self.new_node();
        let token_angle_open = self.kind_or_err(TokenKind::Lt)?;

        let mut items = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::Gt) {
            let item = match self.parse_generic_param_item() {
                Ok(item) => item,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_generic_param_item(token)
                            && before_impl_block_item(token)
                            && before_extern_block_item(token)
                            && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };
            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    items.push(PunctuatedItem::Punctuated { item, punctuation });
                }
                None => {
                    items.push(PunctuatedItem::NotPunctuated { item });
                    break;
                }
            }
        }

        let token_angle_close = self.kind_or_err(TokenKind::Gt)?;

        Ok(ASTGenericParam {
            id,
            span: self.make_span(pos),
            token_angle_open,
            items: Punctuated { items },
            token_angle_close,
        })
    }

    pub fn parse_generic_param_item(&mut self) -> Result<ASTGenericParamItem, ()> {
        let (id, pos) = self.new_node();
        let identifier = self.identifier_or_err()?;

        Ok(ASTGenericParamItem {
            id,
            span: self.make_span(pos),
            identifier,
        })
    }

    pub fn parse_generic_where(&mut self) -> Result<ASTGenericWhere, ()> {
        let (id, pos) = self.new_node();
        let keyword_where = self.keyword_or_err(*KEYWORD_WHERE)?;

        let mut items = Vec::new();

        while self.is_exists() && self.lookup_identifier(0) {
            let item = match self.parse_generic_where_item() {
                Ok(item) => item,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_generic_where_item(token)
                            && before_impl_block_item(token)
                            && before_extern_block_item(token)
                            && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };
            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    items.push(PunctuatedItem::Punctuated { item, punctuation });
                }
                None => {
                    items.push(PunctuatedItem::NotPunctuated { item });
                    break;
                }
            }
        }

        Ok(ASTGenericWhere {
            id,
            span: self.make_span(pos),
            keyword_where,
            items: Punctuated { items },
        })
    }

    pub fn parse_generic_where_item(&mut self) -> Result<ASTGenericWhereItem, ()> {
        let (id, pos) = self.new_node();
        let identifier = self.identifier_or_err()?;
        let token_colon = self.kind_or_err(TokenKind::Colon)?;
        let condition = self.parse_generic_where_item_condition()?;

        Ok(ASTGenericWhereItem {
            id,
            span: self.make_span(pos),
            identifier,
            token_colon,
            condition,
        })
    }

    pub fn parse_generic_where_item_condition(
        &mut self,
    ) -> Result<ASTGenericWhereItemCondition, ()> {
        let (id, pos) = self.new_node();
        let path = self.parse_path()?;

        let mut extra_items = Vec::new();

        while self.is_exists() && self.lookup_kind(0, TokenKind::Add) {
            match self.parse_generic_where_item_condition_item() {
                Ok(item) => {
                    extra_items.push(item);
                }
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_generic_where_item_condition_item(token)
                            && before_generic_where_item(token)
                            && before_impl_block_item(token)
                            && before_module_item(token)
                    });
                }
            }
        }

        Ok(ASTGenericWhereItemCondition {
            id,
            span: self.make_span(pos),
            path,
            extra_items,
        })
    }

    pub fn parse_generic_where_item_condition_item(
        &mut self,
    ) -> Result<ASTGenericWhereItemConditionItem, ()> {
        let (id, pos) = self.new_node();
        let token_plus = self.kind_or_err(TokenKind::Add)?;
        let path = self.parse_path()?;

        Ok(ASTGenericWhereItemConditionItem {
            id,
            span: self.make_span(pos),
            token_plus,
            path,
        })
    }

    pub fn parse_generic_arg(&mut self) -> Result<ASTGenericArg, ()> {
        let (id, pos) = self.new_node();
        let token_angle_open = self.kind_or_err(TokenKind::Lt)?;

        let mut args = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::Gt) {
            let arg = match self.parse_ty() {
                Ok(arg) => arg,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_generic_arg_item(token)
                            && before_impl_block_item(token)
                            && before_extern_block_item(token)
                            && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };

            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    args.push(PunctuatedItem::Punctuated {
                        item: arg,
                        punctuation,
                    });
                }
                None => {
                    args.push(PunctuatedItem::NotPunctuated { item: arg });
                    break;
                }
            }
        }

        let token_angle_close = self.kind_or_err(TokenKind::Gt)?;

        Ok(ASTGenericArg {
            id,
            span: self.make_span(pos),
            token_angle_open,
            args: Punctuated { items: args },
            token_angle_close,
        })
    }

    pub fn parse_stmt_block(&mut self) -> Result<ASTStmtBlock, ()> {
        let (id, pos) = self.new_node();
        let token_brace_open = self.kind_or_err(TokenKind::OpenBrace)?;

        let mut stmts = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseBrace) {
            match self.parse_stmt() {
                Ok(stmt) => {
                    stmts.push(stmt);
                }
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_stmt(token)
                            && before_impl_block_item(token)
                            && before_module_item(token)
                    });
                }
            }
        }

        let token_brace_close = self.kind_or_err(TokenKind::CloseBrace)?;

        Ok(ASTStmtBlock {
            id,
            span: self.make_span(pos),
            token_brace_open,
            stmts,
            token_brace_close,
        })
    }

    pub fn parse_stmt_let(&mut self) -> Result<ASTStmtLet, ()> {
        let (id, pos) = self.new_node();
        let keyword_let = self.keyword_or_err(*KEYWORD_LET)?;
        let identifier = self.identifier_or_err()?;
        let ty = if self.lookup_kind(0, TokenKind::Colon) {
            Some(self.parse_stmt_let_ty()?)
        } else {
            None
        };
        let expr = if self.lookup_kind(0, TokenKind::Assign) {
            Some(self.parse_stmt_let_expr()?)
        } else {
            None
        };
        let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;

        Ok(ASTStmtLet {
            id,
            span: self.make_span(pos),
            keyword_let,
            identifier,
            ty,
            expr,
            token_semicolon,
        })
    }

    pub fn parse_stmt_let_ty(&mut self) -> Result<ASTStmtLetTy, ()> {
        let (id, pos) = self.new_node();
        let token_colon = self.kind_or_err(TokenKind::Colon)?;
        let ty = self.parse_ty()?;

        Ok(ASTStmtLetTy {
            id,
            span: self.make_span(pos),
            token_colon,
            ty,
        })
    }

    pub fn parse_stmt_let_expr(&mut self) -> Result<ASTStmtLetExpr, ()> {
        let (id, pos) = self.new_node();
        let token_assign = self.kind_or_err(TokenKind::Assign)?;
        let expr = self.parse_expr()?;

        Ok(ASTStmtLetExpr {
            id,
            span: self.make_span(pos),
            token_assign,
            expr,
        })
    }

    pub fn parse_stmt_if(&mut self) -> Result<ASTStmtIf, ()> {
        let (id, pos) = self.new_node();
        let keyword_if = self.keyword_or_err(*KEYWORD_IF)?;
        let expr = self.parse_expr()?;
        let stmt_block = self.parse_stmt_block()?;

        let mut else_ifs = Vec::new();

        while self.lookup_keyword(0, *KEYWORD_ELSE) && self.lookup_keyword(1, *KEYWORD_IF) {
            else_ifs.push(self.parse_stmt_if_else_if()?);
        }

        let else_ = if self.lookup_keyword(0, *KEYWORD_ELSE) {
            Some(self.parse_stmt_if_else()?)
        } else {
            None
        };

        Ok(ASTStmtIf {
            id,
            span: self.make_span(pos),
            keyword_if,
            expr,
            stmt_block,
            else_ifs,
            else_,
        })
    }

    pub fn parse_stmt_if_else_if(&mut self) -> Result<ASTStmtIfElseIf, ()> {
        let (id, pos) = self.new_node();
        let keyword_else = self.keyword_or_err(*KEYWORD_ELSE)?;
        let keyword_if = self.keyword_or_err(*KEYWORD_IF)?;
        let expr = self.parse_expr()?;
        let stmt_block = self.parse_stmt_block()?;

        Ok(ASTStmtIfElseIf {
            id,
            span: self.make_span(pos),
            keyword_else,
            keyword_if,
            expr,
            stmt_block,
        })
    }

    pub fn parse_stmt_if_else(&mut self) -> Result<ASTStmtIfElse, ()> {
        let (id, pos) = self.new_node();
        let keyword_else = self.keyword_or_err(*KEYWORD_ELSE)?;
        let stmt_block = self.parse_stmt_block()?;

        Ok(ASTStmtIfElse {
            id,
            span: self.make_span(pos),
            keyword_else,
            stmt_block,
        })
    }

    pub fn parse_stmt_loop(&mut self) -> Result<ASTStmtLoop, ()> {
        let (id, pos) = self.new_node();
        let keyword_loop = self.keyword_or_err(*KEYWORD_LOOP)?;
        let stmt_block = self.parse_stmt_block()?;

        Ok(ASTStmtLoop {
            id,
            span: self.make_span(pos),
            keyword_loop,
            stmt_block,
        })
    }

    pub fn parse_stmt_while(&mut self) -> Result<ASTStmtWhile, ()> {
        let (id, pos) = self.new_node();
        let keyword_while = self.keyword_or_err(*KEYWORD_WHILE)?;
        let expr = self.parse_expr()?;
        let stmt_block = self.parse_stmt_block()?;

        Ok(ASTStmtWhile {
            id,
            span: self.make_span(pos),
            keyword_while,
            expr,
            stmt_block,
        })
    }

    pub fn parse_stmt_break(&mut self) -> Result<ASTStmtBreak, ()> {
        let (id, pos) = self.new_node();
        let keyword_break = self.keyword_or_err(*KEYWORD_BREAK)?;
        let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;

        Ok(ASTStmtBreak {
            id,
            span: self.make_span(pos),
            keyword_break,
            token_semicolon,
        })
    }

    pub fn parse_stmt_continue(&mut self) -> Result<ASTStmtContinue, ()> {
        let (id, pos) = self.new_node();
        let keyword_continue = self.keyword_or_err(*KEYWORD_CONTINUE)?;
        let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;

        Ok(ASTStmtContinue {
            id,
            span: self.make_span(pos),
            keyword_continue,
            token_semicolon,
        })
    }

    pub fn parse_stmt_return(&mut self) -> Result<ASTStmtReturn, ()> {
        let (id, pos) = self.new_node();
        let keyword_return = self.keyword_or_err(*KEYWORD_RETURN)?;
        let expr = if self.lookup_kind(0, TokenKind::Semicolon) {
            None
        } else {
            Some(self.parse_expr()?)
        };
        let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;

        Ok(ASTStmtReturn {
            id,
            span: self.make_span(pos),
            keyword_return,
            expr,
            token_semicolon,
        })
    }

    pub fn parse_stmt_assignment_or_expr(&mut self) -> Result<ASTStmtKind, ()> {
        let (id, pos) = self.new_node();
        let expr = self.parse_expr()?;

        if self.lookup_assignment_op(0) {
            let operator = self.parse_stmt_assignment_operator()?;
            let operand_rhs = self.parse_expr()?;
            let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;

            Ok(ASTStmtKind::Assignment(ASTStmtAssignment {
                id,
                span: self.make_span(pos),
                operand_lhs: expr,
                operator,
                operand_rhs,
                token_semicolon,
            }))
        } else {
            let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;

            Ok(ASTStmtKind::Expr(ASTStmtExpr {
                id,
                span: self.make_span(pos),
                expr,
                token_semicolon,
            }))
        }
    }

    pub fn parse_stmt_assignment_operator(&mut self) -> Result<ASTStmtAssignmentOperator, ()> {
        let (id, pos) = self.new_node();
        let token_operator = self.assignment_op_or_err()?;
        let kind = ASTStmtAssignmentOperatorKind::from_token(&token_operator, self.diagnostics())?;

        Ok(ASTStmtAssignmentOperator {
            id,
            span: self.make_span(pos),
            token_operator,
            kind,
        })
    }

    pub fn parse_stmt(&mut self) -> Result<ASTStmt, ()> {
        let (id, pos) = self.new_node();

        let kind = if self.lookup_kind(0, TokenKind::OpenBrace) {
            ASTStmtKind::Block(self.parse_stmt_block()?)
        } else if self.lookup_keyword(0, *KEYWORD_LET) {
            ASTStmtKind::Let(self.parse_stmt_let()?)
        } else if self.lookup_keyword(0, *KEYWORD_IF) {
            ASTStmtKind::If(self.parse_stmt_if()?)
        } else if self.lookup_keyword(0, *KEYWORD_LOOP) {
            ASTStmtKind::Loop(self.parse_stmt_loop()?)
        } else if self.lookup_keyword(0, *KEYWORD_WHILE) {
            ASTStmtKind::While(self.parse_stmt_while()?)
        } else if self.lookup_keyword(0, *KEYWORD_BREAK) {
            ASTStmtKind::Break(self.parse_stmt_break()?)
        } else if self.lookup_keyword(0, *KEYWORD_CONTINUE) {
            ASTStmtKind::Continue(self.parse_stmt_continue()?)
        } else if self.lookup_keyword(0, *KEYWORD_RETURN) {
            ASTStmtKind::Return(self.parse_stmt_return()?)
        } else {
            self.parse_stmt_assignment_or_expr()?
        };

        Ok(ASTStmt {
            id,
            span: self.make_span(pos),
            kind,
        })
    }

    pub fn parse_expr(&mut self) -> Result<ASTExpr, ()> {
        let prev = self.set_unglue_tokens(false);

        let expr = self.parse_expr_binary_1_compare()?;

        self.set_unglue_tokens(prev);

        Ok(expr)
    }

    pub fn parse_expr_binary_1_compare(&mut self) -> Result<ASTExpr, ()> {
        let mut expr = self.parse_expr_binary_2_logical_or_and()?;

        while self.lookup_kind(0, TokenKind::Eq)
            || self.lookup_kind(0, TokenKind::Ne)
            || self.lookup_kind(0, TokenKind::Lt)
            || self.lookup_kind(0, TokenKind::Gt)
            || self.lookup_kind(0, TokenKind::Le)
            || self.lookup_kind(0, TokenKind::Ge)
        {
            let (id, pos) = self.new_node();
            let operator = self.parse_expr_binary_operator()?;
            let operand_rhs = self.parse_expr_binary_2_logical_or_and()?;

            expr = self.wrap_expr_binary_op(ASTExprBinary {
                id,
                span: self.make_span(pos),
                operand_lhs: Box::new(expr),
                operator,
                operand_rhs: Box::new(operand_rhs),
            })
        }

        Ok(expr)
    }

    pub fn parse_expr_binary_2_logical_or_and(&mut self) -> Result<ASTExpr, ()> {
        let mut expr = self.parse_expr_binary_3_arithmetic_add_sub()?;

        while self.lookup_kind(0, TokenKind::LogOr) || self.lookup_kind(0, TokenKind::LogAnd) {
            let (id, pos) = self.new_node();
            let operator = self.parse_expr_binary_operator()?;
            let operand_rhs = self.parse_expr_binary_3_arithmetic_add_sub()?;

            expr = self.wrap_expr_binary_op(ASTExprBinary {
                id,
                span: self.make_span(pos),
                operand_lhs: Box::new(expr),
                operator,
                operand_rhs: Box::new(operand_rhs),
            })
        }

        Ok(expr)
    }

    pub fn parse_expr_binary_3_arithmetic_add_sub(&mut self) -> Result<ASTExpr, ()> {
        let mut expr = self.parse_expr_binary_4_arithmetic_mul_div_mod()?;

        while self.lookup_kind(0, TokenKind::Add) || self.lookup_kind(0, TokenKind::Sub) {
            let (id, pos) = self.new_node();
            let operator = self.parse_expr_binary_operator()?;
            let operand_rhs = self.parse_expr_binary_4_arithmetic_mul_div_mod()?;

            expr = self.wrap_expr_binary_op(ASTExprBinary {
                id,
                span: self.make_span(pos),
                operand_lhs: Box::new(expr),
                operator,
                operand_rhs: Box::new(operand_rhs),
            })
        }

        Ok(expr)
    }

    pub fn parse_expr_binary_4_arithmetic_mul_div_mod(&mut self) -> Result<ASTExpr, ()> {
        let mut expr = self.parse_expr_binary_5_arithmetic_pow()?;

        while self.lookup_kind(0, TokenKind::Mul)
            || self.lookup_kind(0, TokenKind::Div)
            || self.lookup_kind(0, TokenKind::Mod)
        {
            let (id, pos) = self.new_node();
            let operator = self.parse_expr_binary_operator()?;
            let operand_rhs = self.parse_expr_binary_5_arithmetic_pow()?;

            expr = self.wrap_expr_binary_op(ASTExprBinary {
                id,
                span: self.make_span(pos),
                operand_lhs: Box::new(expr),
                operator,
                operand_rhs: Box::new(operand_rhs),
            })
        }

        Ok(expr)
    }

    pub fn parse_expr_binary_5_arithmetic_pow(&mut self) -> Result<ASTExpr, ()> {
        let mut expr = self.parse_expr_binary_6_bit_shift()?;

        while self.lookup_kind(0, TokenKind::Pow) {
            let (id, pos) = self.new_node();
            let operator = self.parse_expr_binary_operator()?;
            let operand_rhs = self.parse_expr_binary_6_bit_shift()?;

            expr = self.wrap_expr_binary_op(ASTExprBinary {
                id,
                span: self.make_span(pos),
                operand_lhs: Box::new(expr),
                operator,
                operand_rhs: Box::new(operand_rhs),
            })
        }

        Ok(expr)
    }

    pub fn parse_expr_binary_6_bit_shift(&mut self) -> Result<ASTExpr, ()> {
        let mut expr = self.parse_expr_binary_7_bit_or_and_xor()?;

        while self.lookup_kind(0, TokenKind::Shl) || self.lookup_kind(0, TokenKind::Shr) {
            let (id, pos) = self.new_node();
            let operator = self.parse_expr_binary_operator()?;
            let operand_rhs = self.parse_expr_binary_7_bit_or_and_xor()?;

            expr = self.wrap_expr_binary_op(ASTExprBinary {
                id,
                span: self.make_span(pos),
                operand_lhs: Box::new(expr),
                operator,
                operand_rhs: Box::new(operand_rhs),
            })
        }

        Ok(expr)
    }

    pub fn parse_expr_binary_7_bit_or_and_xor(&mut self) -> Result<ASTExpr, ()> {
        let mut expr = self.parse_expr_as()?;

        while self.lookup_kind(0, TokenKind::BitOr)
            || self.lookup_kind(0, TokenKind::BitAnd)
            || self.lookup_kind(0, TokenKind::BitXor)
        {
            let (id, pos) = self.new_node();
            let operator = self.parse_expr_binary_operator()?;
            let operand_rhs = self.parse_expr_as()?;

            expr = self.wrap_expr_binary_op(ASTExprBinary {
                id,
                span: self.make_span(pos),
                operand_lhs: Box::new(expr),
                operator,
                operand_rhs: Box::new(operand_rhs),
            })
        }

        Ok(expr)
    }

    fn wrap_expr_binary_op(&mut self, expr: ASTExprBinary) -> ASTExpr {
        let (id, _) = self.new_node();

        ASTExpr {
            id,
            span: expr.span,
            kind: ASTExprKind::Binary(expr),
        }
    }

    pub fn parse_expr_binary_operator(&mut self) -> Result<ASTExprBinaryOperator, ()> {
        let (id, pos) = self.new_node();
        let token_operator = self.binary_op_or_err()?;
        let kind = ASTExprBinaryOperatorKind::from_token(&token_operator, self.diagnostics())?;

        Ok(ASTExprBinaryOperator {
            id,
            span: self.make_span(pos),
            token_operator,
            kind,
        })
    }

    pub fn parse_expr_as(&mut self) -> Result<ASTExpr, ()> {
        let mut expr = self.parse_expr_unary()?;

        while self.lookup_keyword(0, *KEYWORD_AS) {
            let (id, pos) = self.new_node();
            let keyword_as = self.keyword_or_err(*KEYWORD_AS)?;
            let ty = self.parse_ty()?;

            expr = self.wrap_expr_as(ASTExprAs {
                id,
                span: self.make_span(pos),
                expr: Box::new(expr),
                keyword_as,
                ty,
            })
        }

        Ok(expr)
    }

    fn wrap_expr_as(&mut self, expr: ASTExprAs) -> ASTExpr {
        let (id, _) = self.new_node();

        ASTExpr {
            id,
            span: expr.span,
            kind: ASTExprKind::As(expr),
        }
    }

    pub fn parse_expr_unary(&mut self) -> Result<ASTExpr, ()> {
        let mut ops = Vec::new();

        while self.lookup_unary_op(0) {
            ops.push(self.parse_expr_unary_operator()?);
        }

        let mut expr = self.parse_expr_call_or_member()?;

        while let Some(op) = ops.pop() {
            let (id, pos) = self.new_node();
            expr = self.wrap_expr_unary_op(ASTExprUnary {
                id,
                span: self.make_span(pos),
                operator: op,
                operand_lhs: Box::new(expr),
            })
        }

        Ok(expr)
    }

    fn wrap_expr_unary_op(&mut self, expr: ASTExprUnary) -> ASTExpr {
        let (id, _) = self.new_node();

        ASTExpr {
            id,
            span: expr.span,
            kind: ASTExprKind::Unary(expr),
        }
    }

    pub fn parse_expr_unary_operator(&mut self) -> Result<ASTExprUnaryOperator, ()> {
        let (id, pos) = self.new_node();
        let token_operator = self.unary_op_or_err()?;
        let kind = ASTExprUnaryOperatorKind::from_token(&token_operator, self.diagnostics())?;

        Ok(ASTExprUnaryOperator {
            id,
            span: self.make_span(pos),
            token_operator,
            kind,
        })
    }

    pub fn parse_expr_call_or_member(&mut self) -> Result<ASTExpr, ()> {
        let mut expr = self.parse_expr_paren_or_single_item()?;

        loop {
            if self.lookup_kind(0, TokenKind::OpenParen) {
                expr = self.parse_expr_call(expr)?;
            } else if self.lookup_kind(0, TokenKind::Dot) {
                expr = self.parse_expr_member(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    pub fn parse_expr_call(&mut self, callee: ASTExpr) -> Result<ASTExpr, ()> {
        let (id, _) = self.new_node();
        let callee = self.wrap_expr_call_callee(callee)?;
        let token_paren_open = self.kind_or_err(TokenKind::OpenParen)?;

        let mut args = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseParen) {
            let arg = match self.parse_expr() {
                Ok(arg) => arg,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_expr_call_item(token)
                            && before_expr(token)
                            && before_impl_block_item(token)
                            && before_extern_block_item(token)
                            && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };

            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    args.push(PunctuatedItem::Punctuated {
                        item: arg,
                        punctuation,
                    });
                }
                None => {
                    args.push(PunctuatedItem::NotPunctuated { item: arg });
                    break;
                }
            }
        }

        let token_paren_close = self.kind_or_err(TokenKind::CloseParen)?;

        Ok(self.wrap_expr_call(ASTExprCall {
            id,
            span: self.make_span(callee.span.low),
            callee,
            token_paren_open,
            args: Punctuated { items: args },
            token_paren_close,
        }))
    }

    fn wrap_expr_call(&mut self, expr: ASTExprCall) -> ASTExpr {
        let (id, _) = self.new_node();

        ASTExpr {
            id,
            span: expr.span,
            kind: ASTExprKind::Call(expr),
        }
    }

    fn wrap_expr_call_callee(&mut self, expr: ASTExpr) -> Result<ASTExprCallCallee, ()> {
        let (id, _) = self.new_node();

        Ok(ASTExprCallCallee {
            id,
            span: expr.span,
            expr: Box::new(expr),
        })
    }

    pub fn parse_expr_member(&mut self, expr: ASTExpr) -> Result<ASTExpr, ()> {
        let (id, _) = self.new_node();
        let token_dot = self.kind_or_err(TokenKind::Dot)?;
        let member = self.identifier_or_err()?;

        Ok(self.wrap_expr_member(ASTExprMember {
            id,
            span: self.make_span(expr.span.low),
            expr: Box::new(expr),
            token_dot,
            member,
        }))
    }

    fn wrap_expr_member(&mut self, expr: ASTExprMember) -> ASTExpr {
        let (id, _) = self.new_node();

        ASTExpr {
            id,
            span: expr.span,
            kind: ASTExprKind::Member(expr),
        }
    }

    pub fn parse_expr_paren_or_single_item(&mut self) -> Result<ASTExpr, ()> {
        if self.lookup_kind(0, TokenKind::OpenParen) {
            self.parse_expr_paren()
        } else {
            self.parse_expr_single_item()
        }
    }

    pub fn parse_expr_paren(&mut self) -> Result<ASTExpr, ()> {
        let (id, pos) = self.new_node();
        let token_paren_open = self.kind_or_err(TokenKind::OpenParen)?;
        let expr = self.parse_expr()?;
        let token_paren_close = self.kind_or_err(TokenKind::CloseParen)?;

        Ok(self.wrap_expr_paren(ASTExprParen {
            id,
            span: self.make_span(pos),
            token_paren_open,
            expr: Box::new(expr),
            token_paren_close,
        }))
    }

    fn wrap_expr_paren(&mut self, expr: ASTExprParen) -> ASTExpr {
        let (id, _) = self.new_node();

        ASTExpr {
            id,
            span: expr.span,
            kind: ASTExprKind::Paren(expr),
        }
    }

    pub fn parse_expr_single_item(&mut self) -> Result<ASTExpr, ()> {
        let path = self.parse_path()?;

        if self.lookup_kind(0, TokenKind::OpenBrace) {
            self.parse_expr_struct_literal(path)
        } else {
            self.parse_expr_path(path)
        }
    }

    pub fn parse_expr_path(&mut self, path: ASTPath) -> Result<ASTExpr, ()> {
        let (id, _) = self.new_node();

        Ok(self.wrap_expr_path(ASTExprPath {
            id,
            span: self.make_span(path.span.low),
            path,
        }))
    }

    fn wrap_expr_path(&mut self, expr: ASTExprPath) -> ASTExpr {
        let (id, _) = self.new_node();

        ASTExpr {
            id,
            span: expr.span,
            kind: ASTExprKind::Path(expr),
        }
    }

    pub fn parse_expr_literal(&mut self) -> Result<ASTExpr, ()> {
        let (id, pos) = self.new_node();
        let literal = self.literal_op_or_err()?;

        Ok(self.wrap_expr_literal(ASTExprLiteral {
            id,
            span: self.make_span(pos),
            literal,
        }))
    }

    fn wrap_expr_literal(&mut self, expr: ASTExprLiteral) -> ASTExpr {
        let (id, _) = self.new_node();

        ASTExpr {
            id,
            span: expr.span,
            kind: ASTExprKind::Literal(expr),
        }
    }

    pub fn parse_expr_struct_literal(&mut self, path: ASTPath) -> Result<ASTExpr, ()> {
        let (id, _) = self.new_node();
        let token_brace_open = self.kind_or_err(TokenKind::OpenBrace)?;

        let mut fields = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseBrace) {
            let field = match self.parse_expr_struct_literal_field() {
                Ok(field) => field,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_expr_struct_literal_field_item(token)
                            && before_expr(token)
                            && before_impl_block_item(token)
                            && before_extern_block_item(token)
                            && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };

            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    fields.push(PunctuatedItem::Punctuated {
                        item: field,
                        punctuation,
                    });
                }
                None => {
                    fields.push(PunctuatedItem::NotPunctuated { item: field });
                    break;
                }
            }
        }

        let token_brace_close = self.kind_or_err(TokenKind::CloseBrace)?;

        Ok(self.wrap_expr_struct_literal(ASTExprStructLiteral {
            id,
            span: self.make_span(path.span.low),
            path,
            token_brace_open,
            fields: Punctuated { items: fields },
            token_brace_close,
        }))
    }

    fn wrap_expr_struct_literal(&mut self, expr: ASTExprStructLiteral) -> ASTExpr {
        let (id, _) = self.new_node();

        ASTExpr {
            id,
            span: expr.span,
            kind: ASTExprKind::StructLiteral(expr),
        }
    }

    pub fn parse_expr_struct_literal_field(&mut self) -> Result<ASTExprStructLiteralField, ()> {
        let (id, pos) = self.new_node();
        let identifier = self.identifier_or_err()?;
        let token_colon = self.kind_or_err(TokenKind::Colon)?;
        let expr = self.parse_expr()?;

        Ok(ASTExprStructLiteralField {
            id,
            span: self.make_span(pos),
            identifier,
            token_colon,
            expr,
        })
    }

    pub fn parse_path(&mut self) -> Result<ASTPath, ()> {
        let (id, pos) = self.new_node();

        let mut segments = vec![PunctuatedItem::NotPunctuated {
            item: self.parse_path_segment()?,
        }];

        while self.lookup_kind(0, TokenKind::PathSep) {
            let segment = segments.pop().unwrap();
            let token_path_sep = self.kind_or_err(TokenKind::PathSep)?;

            segments.push(PunctuatedItem::Punctuated {
                item: segment.into_item(),
                punctuation: token_path_sep,
            });

            let segment = self.parse_path_segment()?;

            segments.push(PunctuatedItem::NotPunctuated { item: segment });
        }

        Ok(ASTPath {
            id,
            span: self.make_span(pos),
            segments: Punctuated { items: segments },
        })
    }

    pub fn parse_path_segment(&mut self) -> Result<ASTPathSegment, ()> {
        let (id, pos) = self.new_node();
        let identifier = self.identifier_or_err()?;
        let generic = if self.lookup_kind(0, TokenKind::Lt) {
            Some(self.parse_generic_arg()?)
        } else {
            None
        };

        Ok(ASTPathSegment {
            id,
            span: self.make_span(pos),
            identifier,
            generic,
        })
    }

    pub fn parse_ty(&mut self) -> Result<ASTTy, ()> {
        let prev = self.set_unglue_tokens(true);

        let (id, pos) = self.new_node();
        let kind = if self.lookup_kind(0, TokenKind::OpenParen) {
            ASTTyKind::Paren(self.parse_ty_paren()?)
        } else if self.lookup_kind(0, TokenKind::OpenBracket) {
            self.parse_ty_span_or_array()?
        } else if self.lookup_keyword(0, *KEYWORD_FN) {
            ASTTyKind::FnPointer(self.parse_ty_fn_pointer()?)
        } else {
            ASTTyKind::Path(self.parse_path()?)
        };

        self.set_unglue_tokens(prev);

        Ok(ASTTy {
            id,
            span: self.make_span(pos),
            kind,
        })
    }

    pub fn parse_ty_paren(&mut self) -> Result<ASTTyParen, ()> {
        let (id, pos) = self.new_node();
        let token_paren_open = self.kind_or_err(TokenKind::OpenParen)?;
        let ty = self.parse_ty()?;
        let token_paren_close = self.kind_or_err(TokenKind::CloseParen)?;

        Ok(ASTTyParen {
            id,
            span: self.make_span(pos),
            token_paren_open,
            ty: Box::new(ty),
            token_paren_close,
        })
    }

    pub fn parse_ty_span_or_array(&mut self) -> Result<ASTTyKind, ()> {
        let (id, pos) = self.new_node();
        let token_bracket_open = self.kind_or_err(TokenKind::OpenBracket)?;
        let ty = self.parse_ty()?;

        if self.lookup_kind(0, TokenKind::Semicolon) {
            let token_semicolon = self.kind_or_err(TokenKind::Semicolon)?;
            let literal = self.literal_op_or_err()?;
            let token_bracket_close = self.kind_or_err(TokenKind::CloseBracket)?;

            Ok(ASTTyKind::Array(ASTTyArray {
                id,
                span: self.make_span(pos),
                token_bracket_open,
                ty: Box::new(ty),
                token_semicolon,
                literal,
                token_bracket_close,
            }))
        } else {
            let token_bracket_close = self.kind_or_err(TokenKind::CloseBracket)?;

            Ok(ASTTyKind::Span(ASTTySpan {
                id,
                span: self.make_span(pos),
                token_bracket_open,
                ty: Box::new(ty),
                token_bracket_close,
            }))
        }
    }

    pub fn parse_ty_fn_pointer(&mut self) -> Result<ASTTyFnPointer, ()> {
        let (id, pos) = self.new_node();
        let keyword_fn = self.keyword_or_err(*KEYWORD_FN)?;
        let token_paren_open = self.kind_or_err(TokenKind::OpenParen)?;

        let mut params = Vec::new();

        while self.is_exists() && !self.lookup_kind(0, TokenKind::CloseParen) {
            let param = match self.parse_ty() {
                Ok(param) => param,
                Err(_) => {
                    // eat erroneous tokens
                    self.skip_tokens(|token| {
                        before_ty_fn_pointer_param_item(token)
                            && before_expr(token)
                            && before_impl_block_item(token)
                            && before_extern_block_item(token)
                            && before_module_item(token)
                    });
                    // eat commas if any
                    self.skip_tokens(|token| token.kind == TokenKind::Comma);
                    continue;
                }
            };

            let punctuation = self.kind(TokenKind::Comma);

            match punctuation {
                Some(punctuation) => {
                    params.push(PunctuatedItem::Punctuated {
                        item: param,
                        punctuation,
                    });
                }
                None => {
                    params.push(PunctuatedItem::NotPunctuated { item: param });
                    break;
                }
            }
        }

        let token_paren_close = self.kind_or_err(TokenKind::CloseParen)?;
        let result = if self.lookup_kind(0, TokenKind::Arrow) {
            Some(self.parse_fn_result()?)
        } else {
            None
        };

        Ok(ASTTyFnPointer {
            id,
            span: self.make_span(pos),
            keyword_fn,
            token_paren_open,
            params: Punctuated { items: params },
            token_paren_close,
            result: result.map(Box::new),
        })
    }
}
