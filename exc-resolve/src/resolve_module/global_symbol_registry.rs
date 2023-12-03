use crate::{Module, Visibility};
use exc_parse::{
    ASTAliasDef, ASTExternBlock, ASTExternBlockItemKind, ASTFnDef, ASTInterfaceDef,
    ASTModuleItemKind, ASTPrototypeDef, ASTStructDef, Id, NodeId,
};
use exc_symbol::Symbol;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    hash::{Hash, Hasher},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct GlobalSymbol {
    pub visibility: Visibility,
    pub module: Arc<Module>,
    pub kind: GlobalSymbolKind,
}

impl GlobalSymbol {
    pub fn new(visibility: Visibility, module: Arc<Module>, kind: GlobalSymbolKind) -> Self {
        Self {
            visibility,
            module,
            kind,
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
    Alias(Arc<ASTAliasDef>),
    Prototype(Arc<ASTPrototypeDef>),
    Fn(Arc<ASTFnDef>),
    Struct(Arc<ASTStructDef>),
    Interface(Arc<ASTInterfaceDef>),
}

impl GlobalSymbolKind {
    pub fn id(&self) -> NodeId {
        match self {
            GlobalSymbolKind::Alias(ast) => ast.id,
            GlobalSymbolKind::Prototype(ast) => ast.id,
            GlobalSymbolKind::Fn(ast) => ast.id,
            GlobalSymbolKind::Struct(ast) => ast.id,
            GlobalSymbolKind::Interface(ast) => ast.id,
        }
    }

    pub fn identifier(&self) -> Id {
        match self {
            GlobalSymbolKind::Alias(ast) => ast.identifier,
            GlobalSymbolKind::Prototype(ast) => ast.identifier,
            GlobalSymbolKind::Fn(ast) => ast.identifier,
            GlobalSymbolKind::Struct(ast) => ast.identifier,
            GlobalSymbolKind::Interface(ast) => ast.identifier,
        }
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
    symbols: HashMap<Vec<Symbol>, GlobalSymbol>,
    module_symbols: HashMap<NodeId, HashSet<GlobalSymbol>>,
}

impl GlobalSymbolRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn module_symbols(
        &self,
        module: &Arc<Module>,
    ) -> Option<impl Iterator<Item = &GlobalSymbol>> {
        self.module_symbols
            .get(&module.ast.id())
            .map(|symbols| symbols.iter())
    }

    pub fn get_symbol(&self, path: &[Symbol]) -> Option<&GlobalSymbol> {
        self.symbols.get(path)
    }

    pub fn register_module(&mut self, module: &Arc<Module>) {
        for item in module.ast.items() {
            let (is_pub, identifier, kind) = match &item.kind {
                ASTModuleItemKind::Use(_) => {
                    continue;
                }
                ASTModuleItemKind::AliasDef(ast) => (
                    ast.keyword_pub.is_some(),
                    ast.identifier,
                    ast.clone().into(),
                ),
                ASTModuleItemKind::ModuleDef(_) => continue,
                ASTModuleItemKind::ExternBlock(ast) => {
                    self.register_extern_block(module, ast);
                    continue;
                }
                ASTModuleItemKind::FnDef(ast) => (
                    ast.keyword_pub.is_some(),
                    ast.identifier,
                    ast.clone().into(),
                ),
                ASTModuleItemKind::StructDef(ast) => (
                    ast.keyword_pub.is_some(),
                    ast.identifier,
                    ast.clone().into(),
                ),
                ASTModuleItemKind::InterfaceDef(ast) => (
                    ast.keyword_pub.is_some(),
                    ast.identifier,
                    ast.clone().into(),
                ),
                ASTModuleItemKind::ImplBlock(_) => continue,
            };

            let mut path = module.path.clone();
            path.push(identifier.symbol.clone());

            self.register(
                path,
                GlobalSymbol::new(
                    if is_pub {
                        Visibility::Public
                    } else {
                        Visibility::Private
                    },
                    module.clone(),
                    kind,
                ),
            );
        }
    }

    fn register_extern_block(&mut self, module: &Arc<Module>, ast: &ASTExternBlock) {
        for item in &ast.items {
            let (is_pub, identifier, kind) = match &item.kind {
                ASTExternBlockItemKind::PrototypeDef(ast) => (
                    ast.keyword_pub.is_some(),
                    ast.identifier,
                    ast.clone().into(),
                ),
                ASTExternBlockItemKind::FnDef(ast) => (
                    ast.keyword_pub.is_some(),
                    ast.identifier,
                    ast.clone().into(),
                ),
                ASTExternBlockItemKind::StructDef(ast) => (
                    ast.keyword_pub.is_some(),
                    ast.identifier,
                    ast.clone().into(),
                ),
                ASTExternBlockItemKind::ImplBlock(_) => continue,
            };

            let mut path = module.path.clone();
            path.push(identifier.symbol.clone());

            self.register(
                path,
                GlobalSymbol::new(
                    if is_pub {
                        Visibility::Public
                    } else {
                        Visibility::Private
                    },
                    module.clone(),
                    kind,
                ),
            );
        }
    }

    pub fn register(&mut self, path: Vec<Symbol>, symbol: GlobalSymbol) {
        match self.symbols.entry(path) {
            Entry::Occupied(entry) => {
                let previous = entry.get();
                let identifier = symbol.kind.identifier();

                symbol.module.diagnostics.error_sub(
                    symbol.kind.identifier().span,
                    format!("the symbol {} is defined multiple times", identifier.symbol),
                    vec![{
                        previous.module.diagnostics.sub_hint(
                            previous.kind.identifier().span,
                            format!("previous definition here"),
                        )
                    }],
                );
            }
            Entry::Vacant(entry) => {
                self.module_symbols
                    .entry(symbol.module.ast.id())
                    .or_default()
                    .insert(symbol.clone());
                entry.insert(symbol);
            }
        }
    }
}
