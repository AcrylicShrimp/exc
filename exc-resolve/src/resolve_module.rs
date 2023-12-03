mod global_symbol_registry;
mod module;
mod module_registry;
mod resolve_use;
mod source_file_resolver;
mod visibility;

pub use global_symbol_registry::*;
pub use module::*;
pub use module_registry::*;
pub use resolve_use::*;
pub use source_file_resolver::*;
pub use visibility::*;

use async_recursion::async_recursion;
use exc_parse::{ASTModuleItemKind, ASTUsePath, ASTUsePathItemKind, PunctuatedItem};
use exc_symbol::Symbol;
use std::path::PathBuf;

pub async fn resolve_modules(file_resolver: &mut SourceFileResolver, root_module: Module) {
    let mut module_registry = ModuleRegistry::new();

    let root_module = module_registry.register(root_module);
    module_registry.resolve_submodule(&root_module);

    resolve_external_modules(file_resolver, &mut module_registry, &root_module).await;

    let mut global_symbol_registry = GlobalSymbolRegistry::new();

    for module in module_registry.modules() {
        global_symbol_registry.register_module(module);
    }

    resolve_use(&module_registry, &mut global_symbol_registry);
}

#[async_recursion]
async fn resolve_external_modules(
    file_resolver: &mut SourceFileResolver,
    module_registry: &mut ModuleRegistry,
    module: &Module,
) {
    for item in module.ast.items() {
        let ast = match &item.kind {
            ASTModuleItemKind::Use(ast) => ast,
            ASTModuleItemKind::AliasDef(_) => continue,
            ASTModuleItemKind::ModuleDef(_) => continue,
            ASTModuleItemKind::ExternBlock(_) => continue,
            ASTModuleItemKind::FnDef(_) => continue,
            ASTModuleItemKind::StructDef(_) => continue,
            ASTModuleItemKind::InterfaceDef(_) => continue,
            ASTModuleItemKind::ImplBlock(_) => continue,
        };

        resolve_external_modules_from_use_items(
            file_resolver,
            module_registry,
            module,
            &module.path,
            &vec![&ast.path],
        )
        .await;
    }
}

#[async_recursion]
async fn resolve_external_modules_from_use_items(
    file_resolver: &mut SourceFileResolver,
    module_registry: &mut ModuleRegistry,
    module: &Module,
    base_path: &[Symbol],
    paths: &[&ASTUsePath],
) {
    for &path in paths {
        let target_path = match &path.prefix {
            Some(prefix) => {
                if let Ok((path, _)) = resolve_full_prefix(module, base_path, prefix) {
                    path
                } else {
                    continue;
                }
            }
            None => base_path.to_vec(),
        };

        for index in 1..=target_path.len() {
            let path = &target_path[..index];

            if module_registry.has_module(path) {
                continue;
            }

            // module is not in the registry, so we need to resolve it
            let fs_path = make_fs_path(path);
            let module = match file_resolver.resolve_file(&fs_path).await {
                Ok(module) => module,
                Err(err) => {
                    module.diagnostics.error_simple(format!(
                        "failed to resolve module at `{}` due to: {}",
                        fs_path.display(),
                        err,
                    ));
                    continue;
                }
            };
            let module = module_registry.register(module);
            module_registry.resolve_submodule(&module);

            resolve_external_modules(file_resolver, module_registry, &module).await;
        }

        match &path.item.kind {
            ASTUsePathItemKind::All(_) => {}
            ASTUsePathItemKind::Single(_) => {}
            ASTUsePathItemKind::Group(ast) => {
                resolve_external_modules_from_use_items(
                    file_resolver,
                    module_registry,
                    module,
                    &target_path,
                    &ast.items
                        .items
                        .iter()
                        .map(|item| match item {
                            PunctuatedItem::Punctuated { item, .. } => item,
                            PunctuatedItem::NotPunctuated { item } => item,
                        })
                        .collect::<Vec<_>>(),
                )
                .await;
            }
        }
    }
}

fn make_fs_path(path: &[Symbol]) -> PathBuf {
    let mut fs_path = PathBuf::new();

    for symbol in path {
        fs_path.push(symbol.to_str());
    }

    fs_path.with_extension("exc")
}
