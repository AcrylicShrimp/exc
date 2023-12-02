use crate::Visibility;
use exc_diagnostic::DiagnosticsSender;
use exc_parse::{ASTModule, ASTModuleDef, ASTModuleItem};
use exc_span::{SourceFile, Span};
use exc_symbol::Symbol;
use std::{path::Path, sync::Arc};

#[derive(Debug)]
pub enum ModuleASTKind {
    Module(Arc<ASTModule>),
    Submodule(Arc<ASTModuleDef>),
}

impl ModuleASTKind {
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
        base_path: impl AsRef<Path>,
        file: Arc<SourceFile>,
        diagnostics: DiagnosticsSender,
    ) -> Option<Self> {
        let base_path = base_path.as_ref();
        let relative_path = file.path()?.strip_prefix(base_path).ok()?;

        let mut path = Vec::new();

        for component in relative_path.components() {
            let component = component.as_os_str().to_str()?;
            let component = Symbol::from_str(component);
            path.push(component);
        }

        Some(Self {
            visibility,
            ast,
            path,
            file,
            diagnostics,
        })
    }
}
