use super::module::{Module, ModuleASTKind};
use crate::Visibility;
use exc_parse::{ASTModuleDecl, ASTModuleDef, ASTModuleItem, ASTModuleItemKind, NodeId};
use exc_symbol::Symbol;
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

#[derive(Default, Debug)]
pub struct ModuleRegistry {
    modules: HashMap<Vec<Symbol>, Arc<Module>>,
    module_id_map: HashMap<NodeId, Arc<Module>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn modules(&self) -> impl Iterator<Item = &Arc<Module>> {
        self.modules.values()
    }

    pub fn has_module(&self, path: &[Symbol]) -> bool {
        self.modules.contains_key(path)
    }

    pub fn get_module(&self, path: &[Symbol]) -> Option<&Arc<Module>> {
        self.modules.get(path)
    }

    pub fn get_module_by_id(&self, id: NodeId) -> Option<&Arc<Module>> {
        self.module_id_map.get(&id)
    }

    pub fn register(&mut self, module: Module) -> Arc<Module> {
        let path = module.path.clone();
        let module = Arc::new(module);
        self.modules.insert(path, module.clone());
        self.module_id_map.insert(module.ast.id(), module.clone());
        module
    }

    pub fn resolve_submodule(&mut self, module: &Module) {
        let submodules = collect_submodules(&module.path, &module.ast.items());

        for (path, submodule) in submodules {
            match self.modules.entry(path.clone()) {
                Entry::Occupied(entry) => {
                    let previous = entry.get();

                    module.diagnostics.error_sub(
                        exc_diagnostic::error_codes::DUPLICATED_MODULE,
                        submodule.identifier.span,
                        format!(
                            "the module {} is defined multiple times",
                            submodule.identifier.symbol
                        ),
                        vec![{
                            match &previous.ast {
                                ModuleASTKind::Module(_) => match previous.file.path() {
                                    Some(path) => previous.diagnostics.sub_hint_simple(format!(
                                        "previous definition at `{}`",
                                        path.display()
                                    )),
                                    None => previous.diagnostics.sub_hint_simple(format!(
                                        "previous definition at `{}`",
                                        previous.file.name()
                                    )),
                                },
                                ModuleASTKind::Submodule(ast) => previous.diagnostics.sub_hint(
                                    ast.identifier.span,
                                    format!("previous definition here"),
                                ),
                            }
                        }],
                    );
                }
                Entry::Vacant(entry) => {
                    let module = Arc::new(Module {
                        visibility: if submodule.keyword_pub.is_some() {
                            Visibility::Public
                        } else {
                            Visibility::Private
                        },
                        ast: ModuleASTKind::Submodule(submodule),
                        path,
                        file: module.file.clone(),
                        diagnostics: module.diagnostics.clone(),
                    });

                    entry.insert(module.clone());
                    self.module_id_map.insert(module.ast.id(), module.clone());

                    self.resolve_submodule(&module);
                }
            }
        }
    }

    pub fn register_module_decl(&mut self, ast: &ASTModuleDecl, module: Arc<Module>) {
        self.module_id_map.insert(ast.id, module);
    }
}

fn collect_submodules<'a>(
    path: &Vec<Symbol>,
    items: &'a [ASTModuleItem],
) -> Vec<(Vec<Symbol>, Arc<ASTModuleDef>)> {
    let mut submodules = Vec::new();

    for item in items {
        let submodule = match &item.kind {
            ASTModuleItemKind::Use(_) => continue,
            ASTModuleItemKind::AliasDef(_) => continue,
            ASTModuleItemKind::ModuleDecl(_) => continue,
            ASTModuleItemKind::ModuleDef(ast) => ast,
            ASTModuleItemKind::ExternBlock(_) => continue,
            ASTModuleItemKind::FnDef(_) => continue,
            ASTModuleItemKind::StructDef(_) => continue,
            ASTModuleItemKind::InterfaceDef(_) => continue,
            ASTModuleItemKind::ImplBlock(_) => continue,
        };

        let mut path = path.clone();
        path.push(submodule.identifier.symbol);

        submodules.push((path, submodule.clone()));
    }

    submodules
}
