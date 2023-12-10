use crate::{GlobalSymbol, GlobalSymbolKind, GlobalSymbolRegistry, Module, ModuleRegistry};
use exc_parse::{
    ASTExpr, ASTExprKind, ASTFnDef, ASTStmtBlock, ASTStmtKind, Id, NodeId, PunctuatedItem,
};
use exc_symbol::Symbol;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct LocalSymbol {
    pub module: Arc<Module>,
    pub function: NodeId,
    pub kind: LocalSymbolKind,
    pub identifier: Id,
}

impl LocalSymbol {
    pub fn new(
        module: Arc<Module>,
        function: NodeId,
        kind: LocalSymbolKind,
        identifier: Id,
    ) -> Self {
        Self {
            module,
            function,
            kind,
            identifier,
        }
    }
}

impl PartialEq for LocalSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.kind.id() == other.kind.id()
    }
}

impl Eq for LocalSymbol {}

impl Hash for LocalSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.id().hash(state);
    }
}

#[derive(Debug, Clone)]
pub enum LocalSymbolKind {
    Global(GlobalSymbol),
    Parameter(LocalSymbolParameter),
    Variable(LocalSymbolVariable),
}

impl LocalSymbolKind {
    pub fn id(&self) -> NodeId {
        match self {
            LocalSymbolKind::Global(global) => global.kind.id(),
            LocalSymbolKind::Parameter(param) => param.id,
            LocalSymbolKind::Variable(var) => var.id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LocalSymbolParameter {
    pub id: NodeId,
    pub index: usize,
}

impl LocalSymbolParameter {
    pub fn new(id: NodeId, index: usize) -> Self {
        Self { id, index }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariableIndex(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeIndex(pub usize);

#[derive(Debug, Clone)]
pub struct LocalSymbolVariable {
    pub id: NodeId,
    pub index: VariableIndex,
    pub scope: ScopeIndex,
}

impl LocalSymbolVariable {
    pub fn new(id: NodeId, index: VariableIndex, scope: ScopeIndex) -> Self {
        Self { id, index, scope }
    }
}

#[derive(Debug, Clone)]
pub struct ScopeTable {
    pub function: NodeId,
    pub scopes: Vec<Scope>,
    pub variables: Vec<LocalSymbolVariable>,
}

impl ScopeTable {
    pub fn new(function: NodeId) -> Self {
        Self {
            function,
            variables: Vec::new(),
            scopes: Vec::new(),
        }
    }

    pub fn new_scope(&mut self, parent: Option<ScopeIndex>) -> ScopeIndex {
        let index = ScopeIndex(self.scopes.len());
        self.scopes.push(Scope::new(index, parent));
        index
    }

    pub fn new_variable(&mut self, id: NodeId, symbol: Symbol, scope_index: ScopeIndex) {
        let index = VariableIndex(self.variables.len());
        self.variables
            .push(LocalSymbolVariable::new(id, index, scope_index));
        self.scopes[scope_index.0].add_variable(symbol, index);
    }

    pub fn lookup_variable(
        &self,
        scope_index: ScopeIndex,
        symbol: Symbol,
    ) -> Option<VariableIndex> {
        let mut scope = Some(scope_index);

        while let Some(scope_index) = scope {
            if let Some(variables) = self.scopes[scope_index.0].variables.get(&symbol) {
                if let Some(index) = variables.last() {
                    return Some(*index);
                }
            }

            scope = self.scopes[scope_index.0].parent;
        }

        None
    }
}

impl Index<ScopeIndex> for ScopeTable {
    type Output = Scope;

    fn index(&self, index: ScopeIndex) -> &Self::Output {
        &self.scopes[index.0]
    }
}

impl IndexMut<ScopeIndex> for ScopeTable {
    fn index_mut(&mut self, index: ScopeIndex) -> &mut Self::Output {
        &mut self.scopes[index.0]
    }
}

impl Index<VariableIndex> for ScopeTable {
    type Output = LocalSymbolVariable;

    fn index(&self, index: VariableIndex) -> &Self::Output {
        &self.variables[index.0]
    }
}

impl IndexMut<VariableIndex> for ScopeTable {
    fn index_mut(&mut self, index: VariableIndex) -> &mut Self::Output {
        &mut self.variables[index.0]
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub index: ScopeIndex,
    pub parent: Option<ScopeIndex>,
    pub variables: HashMap<Symbol, Vec<VariableIndex>>,
}

impl Scope {
    pub fn new(index: ScopeIndex, parent: Option<ScopeIndex>) -> Self {
        Self {
            index,
            parent,
            variables: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, symbol: Symbol, index: VariableIndex) {
        self.variables.entry(symbol).or_default().push(index);
    }
}

#[derive(Default, Debug)]
pub struct LocalSymbolRegistry {
    symbols: HashMap<NodeId, HashMap<NodeId, LocalSymbol>>,
}

impl LocalSymbolRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register(
        &mut self,
        module_registry: &ModuleRegistry,
        global_symbol_registry: &GlobalSymbolRegistry,
    ) {
        for module in module_registry.modules() {
            let symbols = if let Some(symbols) = global_symbol_registry.non_module_symbols(module) {
                symbols
            } else {
                continue;
            };

            for symbol in symbols {
                let ast = match &symbol.kind {
                    GlobalSymbolKind::ModuleDecl(_) => continue,
                    GlobalSymbolKind::ModuleDef(_) => continue,
                    GlobalSymbolKind::Alias(_) => continue,
                    GlobalSymbolKind::Prototype(_) => continue,
                    GlobalSymbolKind::Fn(ast) => ast,
                    GlobalSymbolKind::Struct(_) => continue,
                    GlobalSymbolKind::Interface(_) => continue,
                };

                self.register_function(module_registry, global_symbol_registry, module, ast);
            }
        }
    }

    fn register_function(
        &mut self,
        module_registry: &ModuleRegistry,
        global_symbol_registry: &GlobalSymbolRegistry,
        module: &Arc<Module>,
        ast: &ASTFnDef,
    ) {
        let mut scope_table = ScopeTable::new(ast.id);

        self.register_function_stmt_block(
            module_registry,
            global_symbol_registry,
            module,
            &ast.stmt_block,
            None,
            &mut scope_table,
        );

        // TODO: store scope table somewhere
    }

    fn register_function_stmt_block(
        &mut self,
        module_registry: &ModuleRegistry,
        global_symbol_registry: &GlobalSymbolRegistry,
        module: &Arc<Module>,
        ast: &ASTStmtBlock,
        parent: Option<ScopeIndex>,
        scope_table: &mut ScopeTable,
    ) {
        let scope_index = scope_table.new_scope(parent);

        for stmt in &ast.stmts {
            match &stmt.kind {
                ASTStmtKind::Block(ast) => {
                    self.register_function_stmt_block(
                        module_registry,
                        global_symbol_registry,
                        module,
                        ast,
                        Some(scope_index),
                        scope_table,
                    );
                }
                ASTStmtKind::Let(ast) => {
                    scope_table.new_variable(ast.id, ast.identifier.symbol, scope_index);

                    if let Some(ast) = &ast.expr {
                        self.register_function_expr(
                            module_registry,
                            global_symbol_registry,
                            module,
                            &ast.expr,
                            scope_index,
                            scope_table,
                        );
                    }
                }
                ASTStmtKind::If(ast) => {
                    self.register_function_expr(
                        module_registry,
                        global_symbol_registry,
                        module,
                        &ast.expr,
                        scope_index,
                        scope_table,
                    );

                    self.register_function_stmt_block(
                        module_registry,
                        global_symbol_registry,
                        module,
                        &ast.stmt_block,
                        parent,
                        scope_table,
                    );

                    for ast in &ast.else_ifs {
                        self.register_function_expr(
                            module_registry,
                            global_symbol_registry,
                            module,
                            &ast.expr,
                            scope_index,
                            scope_table,
                        );

                        self.register_function_stmt_block(
                            module_registry,
                            global_symbol_registry,
                            module,
                            &ast.stmt_block,
                            parent,
                            scope_table,
                        );
                    }

                    if let Some(ast) = &ast.else_ {
                        self.register_function_stmt_block(
                            module_registry,
                            global_symbol_registry,
                            module,
                            &ast.stmt_block,
                            parent,
                            scope_table,
                        );
                    }
                }
                ASTStmtKind::Loop(ast) => {
                    self.register_function_stmt_block(
                        module_registry,
                        global_symbol_registry,
                        module,
                        &ast.stmt_block,
                        parent,
                        scope_table,
                    );
                }
                ASTStmtKind::While(ast) => {
                    self.register_function_expr(
                        module_registry,
                        global_symbol_registry,
                        module,
                        &ast.expr,
                        scope_index,
                        scope_table,
                    );

                    self.register_function_stmt_block(
                        module_registry,
                        global_symbol_registry,
                        module,
                        &ast.stmt_block,
                        parent,
                        scope_table,
                    );
                }
                ASTStmtKind::Break(_) => continue,
                ASTStmtKind::Continue(_) => continue,
                ASTStmtKind::Return(ast) => {
                    if let Some(ast) = &ast.expr {
                        self.register_function_expr(
                            module_registry,
                            global_symbol_registry,
                            module,
                            ast,
                            scope_index,
                            scope_table,
                        );
                    }
                }
                ASTStmtKind::Assignment(ast) => {
                    self.register_function_expr(
                        module_registry,
                        global_symbol_registry,
                        module,
                        &ast.operand_lhs,
                        scope_index,
                        scope_table,
                    );

                    self.register_function_expr(
                        module_registry,
                        global_symbol_registry,
                        module,
                        &ast.operand_rhs,
                        scope_index,
                        scope_table,
                    );
                }
                ASTStmtKind::Expr(ast) => {
                    self.register_function_expr(
                        module_registry,
                        global_symbol_registry,
                        module,
                        &ast.expr,
                        scope_index,
                        scope_table,
                    );
                }
            }
        }
    }

    fn register_function_expr(
        &mut self,
        module_registry: &ModuleRegistry,
        global_symbol_registry: &GlobalSymbolRegistry,
        module: &Arc<Module>,
        ast: &ASTExpr,
        scope_index: ScopeIndex,
        scope_table: &mut ScopeTable,
    ) {
        match &ast.kind {
            ASTExprKind::Binary(ast) => {
                self.register_function_expr(
                    module_registry,
                    global_symbol_registry,
                    module,
                    &ast.operand_lhs,
                    scope_index,
                    scope_table,
                );

                self.register_function_expr(
                    module_registry,
                    global_symbol_registry,
                    module,
                    &ast.operand_rhs,
                    scope_index,
                    scope_table,
                );
            }
            ASTExprKind::As(ast) => {
                self.register_function_expr(
                    module_registry,
                    global_symbol_registry,
                    module,
                    &ast.expr,
                    scope_index,
                    scope_table,
                );
            }
            ASTExprKind::Unary(ast) => {
                self.register_function_expr(
                    module_registry,
                    global_symbol_registry,
                    module,
                    &ast.operand_lhs,
                    scope_index,
                    scope_table,
                );
            }
            ASTExprKind::Call(ast) => {
                self.register_function_expr(
                    module_registry,
                    global_symbol_registry,
                    module,
                    &ast.callee.expr,
                    scope_index,
                    scope_table,
                );

                for ast in &ast.args.items {
                    match &ast {
                        PunctuatedItem::Punctuated { item, .. } => {
                            self.register_function_expr(
                                module_registry,
                                global_symbol_registry,
                                module,
                                item,
                                scope_index,
                                scope_table,
                            );
                        }
                        PunctuatedItem::NotPunctuated { item } => {
                            self.register_function_expr(
                                module_registry,
                                global_symbol_registry,
                                module,
                                item,
                                scope_index,
                                scope_table,
                            );
                        }
                    }
                }
            }
            ASTExprKind::Member(ast) => {
                // NOTE: we don't need to check the member identifier
                // because it is known to be a member of given type of expression
                self.register_function_expr(
                    module_registry,
                    global_symbol_registry,
                    module,
                    &ast.expr,
                    scope_index,
                    scope_table,
                );
            }
            ASTExprKind::Paren(ast) => {
                self.register_function_expr(
                    module_registry,
                    global_symbol_registry,
                    module,
                    &ast.expr,
                    scope_index,
                    scope_table,
                );
            }
            ASTExprKind::Path(ast) => {
                if ast.path.segments.items.len() == 1 {
                    let segment = match ast.path.segments.items.first().unwrap() {
                        PunctuatedItem::Punctuated { item, .. } => item,
                        PunctuatedItem::NotPunctuated { item } => item,
                    };

                    if segment.generic.is_none() {
                        // it has no prefix and no generic, so it can be a local symbol
                        if let Some(variable_index) =
                            scope_table.lookup_variable(scope_index, segment.identifier.symbol)
                        {
                            let local_symbol = LocalSymbol::new(
                                module.clone(),
                                scope_table.function,
                                LocalSymbolKind::Variable(scope_table[variable_index].clone()),
                                segment.identifier,
                            );

                            self.symbols
                                .entry(scope_table.function)
                                .or_default()
                                .insert(ast.id, local_symbol);

                            return;
                        }
                    }
                }

                let (last, prefix) = ast.path.segments.items.split_last().unwrap();
                let mut target_module = module;

                for segment in prefix {
                    let segment = match segment {
                        PunctuatedItem::Punctuated { item, .. } => item,
                        PunctuatedItem::NotPunctuated { item } => item,
                    };

                    if let Some(generic) = &segment.generic {
                        // TODO: emit compilation error; prefix cannot have generic arguments
                    }

                    let module = match global_symbol_registry
                        .get_module_symbol(&target_module, segment.identifier.symbol)
                    {
                        Some(symbol) => match &symbol.kind {
                            GlobalSymbolKind::ModuleDecl(ast) => {
                                module_registry.get_module_by_id(ast.id).unwrap()
                            }
                            GlobalSymbolKind::ModuleDef(ast) => {
                                module_registry.get_module_by_id(ast.id).unwrap()
                            }
                            GlobalSymbolKind::Alias(_) => unreachable!(),
                            GlobalSymbolKind::Prototype(_) => unreachable!(),
                            GlobalSymbolKind::Fn(_) => unreachable!(),
                            GlobalSymbolKind::Struct(_) => unreachable!(),
                            GlobalSymbolKind::Interface(_) => unreachable!(),
                        },
                        None => {
                            // TODO: emit compilation error; cannot find prefix or it is not a module
                            return;
                        }
                    };

                    target_module = module;
                }

                let last = match last {
                    PunctuatedItem::Punctuated { item, .. } => item,
                    PunctuatedItem::NotPunctuated { item } => item,
                };

                match global_symbol_registry
                    .get_non_module_symbol(target_module, last.identifier.symbol)
                {
                    Some(symbol) => {
                        let local_symbol = LocalSymbol::new(
                            module.clone(),
                            scope_table.function,
                            LocalSymbolKind::Global(symbol.clone()),
                            last.identifier,
                        );

                        self.symbols
                            .entry(scope_table.function)
                            .or_default()
                            .insert(ast.id, local_symbol);
                    }
                    None => {
                        // TODO: emit compilation error; cannot find symbol
                    }
                }
            }
            ASTExprKind::Literal(_) => {}
            ASTExprKind::StructLiteral(ast) => {
                for ast in &ast.fields.items {
                    match &ast {
                        PunctuatedItem::Punctuated { item, .. } => {
                            self.register_function_expr(
                                module_registry,
                                global_symbol_registry,
                                module,
                                &item.expr,
                                scope_index,
                                scope_table,
                            );
                        }
                        PunctuatedItem::NotPunctuated { item } => {
                            self.register_function_expr(
                                module_registry,
                                global_symbol_registry,
                                module,
                                &item.expr,
                                scope_index,
                                scope_table,
                            );
                        }
                    }
                }
            }
        }
    }
}
