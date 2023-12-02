use super::module::{Module, ModuleASTKind, ModuleVisibility};
use exc_parse::{ASTModuleDef, ASTModuleItem, ASTModuleItemKind};
use exc_symbol::Symbol;
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

#[derive(Default, Debug)]
pub struct ModuleRegistry {
    modules: HashMap<Vec<Symbol>, Arc<Module>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register(&mut self, module: Module) -> Arc<Module> {
        let path = module.path.clone();
        let module = Arc::new(module);
        self.modules.insert(path, module.clone());
        module
    }

    pub fn resolve_submodule(&mut self, module: &Module) {
        let submodules = collect_submodules(&module.path, &module.ast.items());

        for (path, submodule) in submodules {
            match self.modules.entry(path.clone()) {
                Entry::Occupied(entry) => {
                    let previous = entry.get();

                    module.diagnostics.error_sub(
                        submodule.identifier.span,
                        format!(
                            "the module {} is defined multiple times",
                            submodule.identifier.symbol
                        ),
                        vec![{
                            previous
                                .diagnostics
                                .sub_hint(previous.ast.span(), format!("previous definition here"))
                        }],
                    );
                }
                Entry::Vacant(entry) => {
                    entry.insert(
                        Module {
                            visibility: if submodule.keyword_pub.is_some() {
                                ModuleVisibility::Public
                            } else {
                                ModuleVisibility::Private
                            },
                            ast: ModuleASTKind::Submodule(submodule),
                            path,
                            file: module.file.clone(),
                            diagnostics: module.diagnostics.clone(),
                        }
                        .into(),
                    );
                }
            }
        }
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
