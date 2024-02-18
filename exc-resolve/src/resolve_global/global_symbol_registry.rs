use crate::{Module, Visibility};
use exc_parse::{
    ASTAliasDef, ASTExternBlock, ASTExternBlockItemKind, ASTFnDef, ASTInterfaceDef, ASTModuleDecl,
    ASTModuleDef, ASTModuleItemKind, ASTPrototypeDef, ASTStructDef, Id, NodeId,
};
use exc_symbol::Symbol;
use std::{
    collections::{hash_map::Entry, HashMap},
    hash::{Hash, Hasher},
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SymbolLevel {
    /// The symbol is glob-imported. It has lower priority than explicit symbols.
    /// It can be shadowed by explicit symbols.
    Glob,
    /// The symbol is explicitly imported. It has higher priority than glob-redirected symbols.
    /// It can shadow glob-imported symbols.
    Explicit,
}

#[derive(Debug, Clone)]
pub struct GlobalSymbol {
    pub level: SymbolLevel,
    pub visibility: Visibility,
    pub module: Arc<Module>,
    pub kind: GlobalSymbolKind,
    pub identifier: Id,
}

impl GlobalSymbol {
    pub fn new(
        level: SymbolLevel,
        visibility: Visibility,
        module: Arc<Module>,
        kind: GlobalSymbolKind,
        identifier: Id,
    ) -> Self {
        Self {
            level,
            visibility,
            module,
            kind,
            identifier,
        }
    }
}

impl PartialEq for GlobalSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.kind.id() == other.kind.id()
    }
}

impl Eq for GlobalSymbol {}

impl Hash for GlobalSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.id().hash(state);
    }
}

#[derive(Debug, Clone)]
pub enum GlobalSymbolKind {
    ModuleDecl(Arc<ASTModuleDecl>),
    ModuleDef(Arc<ASTModuleDef>),
    Alias(Arc<ASTAliasDef>),
    Prototype(Arc<ASTPrototypeDef>),
    Fn(Arc<ASTFnDef>),
    Struct(Arc<ASTStructDef>),
    Interface(Arc<ASTInterfaceDef>),
}

impl GlobalSymbolKind {
    pub fn is_module(&self) -> bool {
        matches!(self, Self::ModuleDecl(_) | Self::ModuleDef(_))
    }

    pub fn id(&self) -> NodeId {
        match self {
            GlobalSymbolKind::ModuleDecl(ast) => ast.id,
            GlobalSymbolKind::ModuleDef(ast) => ast.id,
            GlobalSymbolKind::Alias(ast) => ast.id,
            GlobalSymbolKind::Prototype(ast) => ast.id,
            GlobalSymbolKind::Fn(ast) => ast.id,
            GlobalSymbolKind::Struct(ast) => ast.id,
            GlobalSymbolKind::Interface(ast) => ast.id,
        }
    }

    pub fn identifier(&self) -> Id {
        match self {
            GlobalSymbolKind::ModuleDecl(ast) => ast.identifier,
            GlobalSymbolKind::ModuleDef(ast) => ast.identifier,
            GlobalSymbolKind::Alias(ast) => ast.identifier,
            GlobalSymbolKind::Prototype(ast) => ast.identifier,
            GlobalSymbolKind::Fn(ast) => ast.identifier,
            GlobalSymbolKind::Struct(ast) => ast.identifier,
            GlobalSymbolKind::Interface(ast) => ast.identifier,
        }
    }
}

impl From<Arc<ASTModuleDecl>> for GlobalSymbolKind {
    fn from(ast: Arc<ASTModuleDecl>) -> Self {
        Self::ModuleDecl(ast)
    }
}

impl From<Arc<ASTModuleDef>> for GlobalSymbolKind {
    fn from(ast: Arc<ASTModuleDef>) -> Self {
        Self::ModuleDef(ast)
    }
}

impl From<Arc<ASTAliasDef>> for GlobalSymbolKind {
    fn from(ast: Arc<ASTAliasDef>) -> Self {
        Self::Alias(ast)
    }
}

impl From<Arc<ASTPrototypeDef>> for GlobalSymbolKind {
    fn from(ast: Arc<ASTPrototypeDef>) -> Self {
        Self::Prototype(ast)
    }
}

impl From<Arc<ASTFnDef>> for GlobalSymbolKind {
    fn from(ast: Arc<ASTFnDef>) -> Self {
        Self::Fn(ast)
    }
}

impl From<Arc<ASTStructDef>> for GlobalSymbolKind {
    fn from(ast: Arc<ASTStructDef>) -> Self {
        Self::Struct(ast)
    }
}

impl From<Arc<ASTInterfaceDef>> for GlobalSymbolKind {
    fn from(ast: Arc<ASTInterfaceDef>) -> Self {
        Self::Interface(ast)
    }
}

#[derive(Default, Debug)]
pub struct GlobalSymbolRegistry {
    module_symbols: HashMap<NodeId, HashMap<Symbol, GlobalSymbol>>,
    non_module_symbols: HashMap<NodeId, HashMap<Symbol, GlobalSymbol>>,
}

impl GlobalSymbolRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn any_symbols(&self, module: &Module) -> Vec<&GlobalSymbol> {
        self.module_symbols(module)
            .into_iter()
            .flatten()
            .chain(self.non_module_symbols(module).into_iter().flatten())
            .collect()
    }

    pub fn module_symbols(&self, module: &Module) -> Option<impl Iterator<Item = &GlobalSymbol>> {
        self.module_symbols
            .get(&module.ast.id())
            .map(|symbols| symbols.values())
    }

    pub fn non_module_symbols(
        &self,
        module: &Module,
    ) -> Option<impl Iterator<Item = &GlobalSymbol>> {
        self.non_module_symbols
            .get(&module.ast.id())
            .map(|symbols| symbols.values())
    }

    pub fn get_any_symbol(&self, module: &Module, symbol: Symbol) -> Option<&GlobalSymbol> {
        self.get_module_symbol(module, symbol)
            .or_else(|| self.get_non_module_symbol(module, symbol))
    }

    pub fn get_module_symbol(&self, module: &Module, symbol: Symbol) -> Option<&GlobalSymbol> {
        self.module_symbols
            .get(&module.ast.id())
            .and_then(|map| map.get(&symbol))
    }

    pub fn get_non_module_symbol(&self, module: &Module, symbol: Symbol) -> Option<&GlobalSymbol> {
        self.non_module_symbols
            .get(&module.ast.id())
            .and_then(|map| map.get(&symbol))
    }

    pub fn register_module(&mut self, module: &Arc<Module>) {
        for item in module.ast.items() {
            let (is_pub, kind): (_, GlobalSymbolKind) = match &item.kind {
                ASTModuleItemKind::Use(_) => continue,
                ASTModuleItemKind::AliasDef(ast) => (ast.keyword_pub.is_some(), ast.clone().into()),
                ASTModuleItemKind::ModuleDecl(ast) => {
                    (ast.keyword_pub.is_some(), ast.clone().into())
                }
                ASTModuleItemKind::ModuleDef(ast) => {
                    (ast.keyword_pub.is_some(), ast.clone().into())
                }
                ASTModuleItemKind::ExternBlock(ast) => {
                    self.register_extern_block(module, ast);
                    continue;
                }
                ASTModuleItemKind::FnDef(ast) => (ast.keyword_pub.is_some(), ast.clone().into()),
                ASTModuleItemKind::StructDef(ast) => {
                    (ast.keyword_pub.is_some(), ast.clone().into())
                }
                ASTModuleItemKind::InterfaceDef(ast) => {
                    (ast.keyword_pub.is_some(), ast.clone().into())
                }
                ASTModuleItemKind::ImplBlock(_) => continue,
            };

            let identifier = kind.identifier();
            self.register(GlobalSymbol::new(
                SymbolLevel::Explicit,
                if is_pub {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
                module.clone(),
                kind,
                identifier,
            ));
        }
    }

    fn register_extern_block(&mut self, module: &Arc<Module>, ast: &ASTExternBlock) {
        for item in &ast.items {
            let (is_pub, kind): (_, GlobalSymbolKind) = match &item.kind {
                ASTExternBlockItemKind::PrototypeDef(ast) => {
                    (ast.keyword_pub.is_some(), ast.clone().into())
                }
                ASTExternBlockItemKind::FnDef(ast) => {
                    (ast.keyword_pub.is_some(), ast.clone().into())
                }
                ASTExternBlockItemKind::StructDef(ast) => {
                    (ast.keyword_pub.is_some(), ast.clone().into())
                }
                ASTExternBlockItemKind::ImplBlock(_) => continue,
            };

            let identifier = kind.identifier();
            self.register(GlobalSymbol::new(
                SymbolLevel::Explicit,
                if is_pub {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
                module.clone(),
                kind,
                identifier,
            ));
        }
    }

    pub fn register(&mut self, symbol: GlobalSymbol) -> bool {
        self.register_with_rename(symbol.kind.identifier(), symbol)
    }

    pub fn register_with_rename(&mut self, identifier: Id, symbol: GlobalSymbol) -> bool {
        register_into(
            if symbol.kind.is_module() {
                &mut self.module_symbols
            } else {
                &mut self.non_module_symbols
            },
            identifier,
            symbol,
        )
    }
}

fn register_into(
    map: &mut HashMap<NodeId, HashMap<Symbol, GlobalSymbol>>,
    identifier: Id,
    symbol: GlobalSymbol,
) -> bool {
    match map
        .entry(symbol.module.ast.id())
        .or_default()
        .entry(identifier.symbol)
    {
        Entry::Occupied(mut entry) => {
            let previous = entry.get();

            if previous.level < symbol.level {
                // replace the previous symbol if the new symbol has higher priority
                entry.insert(symbol);
                return true;
            }

            if previous.level > symbol.level {
                // ignore the new symbol if the previous symbol has higher priority
                return false;
            }

            emit_conflict_error(identifier, &symbol, previous);
            false
        }
        Entry::Vacant(entry) => {
            entry.insert(symbol);
            true
        }
    }
}

fn emit_conflict_error(identifier: Id, symbol: &GlobalSymbol, previous: &GlobalSymbol) {
    symbol.module.diagnostics.error_sub(
        identifier.span,
        format!("the symbol {} is defined multiple times", identifier.symbol),
        vec![{
            previous.module.diagnostics.sub_hint(
                previous.identifier.span,
                format!("previous definition here"),
            )
        }],
    );
}
