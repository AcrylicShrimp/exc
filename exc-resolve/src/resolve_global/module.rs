use crate::Visibility;
use exc_diagnostic::DiagnosticsSender;
use exc_parse::{ASTModule, ASTModuleDef, ASTModuleItem, NodeId};
use exc_span::{SourceFile, Span};
use exc_symbol::Symbol;
use std::sync::Arc;

#[derive(Debug)]
pub enum ModuleASTKind {
    Module(Arc<ASTModule>),
    Submodule(Arc<ASTModuleDef>),
}

impl ModuleASTKind {
    pub fn id(&self) -> NodeId {
        match self {
            Self::Module(ast) => ast.id,
            Self::Submodule(ast) => ast.id,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Module(ast) => ast.span,
            Self::Submodule(ast) => ast.span,
        }
    }

    pub fn items(&self) -> &[ASTModuleItem] {
        match self {
            Self::Module(ast) => &ast.items,
            Self::Submodule(ast) => &ast.items,
        }
    }
}

#[derive(Debug)]
pub struct Module {
    pub visibility: Visibility,
    pub ast: ModuleASTKind,
    pub path: Vec<Symbol>,
    pub file: Arc<SourceFile>,
    pub diagnostics: DiagnosticsSender,
}

impl Module {
    pub fn new(
        visibility: Visibility,
        ast: ModuleASTKind,
        path: Vec<Symbol>,
        file: Arc<SourceFile>,
        diagnostics: DiagnosticsSender,
    ) -> Self {
        Self {
            visibility,
            ast,
            path,
            file,
            diagnostics,
        }
    }
}
