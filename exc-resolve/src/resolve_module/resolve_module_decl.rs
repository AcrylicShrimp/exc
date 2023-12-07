use crate::{Module, ModuleRegistry, SourceFileResolver};
use async_recursion::async_recursion;
use exc_parse::{ASTModuleDecl, ASTModuleItemKind};
use exc_symbol::Symbol;
use std::path::PathBuf;

#[async_recursion]
pub async fn resolve_module_decl(
    file_resolver: &mut SourceFileResolver,
    module_registry: &mut ModuleRegistry,
    module: &Module,
) {
    for item in module.ast.items() {
        let ast = match &item.kind {
            ASTModuleItemKind::Use(_) => continue,
            ASTModuleItemKind::AliasDef(_) => continue,
            ASTModuleItemKind::ModuleDecl(ast) => ast,
            ASTModuleItemKind::ModuleDef(_) => continue,
            ASTModuleItemKind::ExternBlock(_) => continue,
            ASTModuleItemKind::FnDef(_) => continue,
            ASTModuleItemKind::StructDef(_) => continue,
            ASTModuleItemKind::InterfaceDef(_) => continue,
            ASTModuleItemKind::ImplBlock(_) => continue,
        };

        resolve_single_module_decl(file_resolver, module_registry, module, &ast).await;
    }
}

pub async fn resolve_single_module_decl(
    file_resolver: &mut SourceFileResolver,
    module_registry: &mut ModuleRegistry,
    module: &Module,
    ast: &ASTModuleDecl,
) {
    let mut path = module.path.clone();
    debug_assert!(!path.is_empty());
    path.pop();
    path.push(ast.identifier.symbol);

    let fs_path = make_fs_path(&path);
    let new_module = match file_resolver.resolve_file(&fs_path).await {
        Ok(module) => module,
        Err(_) => {
            // TODO: display the path in absolute form
            module.diagnostics.error(
                ast.span,
                format!(
                    "the module {} is not reachable; failed to read file at path `{}`",
                    ast.identifier.symbol,
                    fs_path.display()
                ),
            );
            return;
        }
    };

    let new_module = module_registry.register(new_module);
    module_registry.resolve_submodule(&new_module);
    module_registry.register_module_decl(ast, new_module.clone());

    resolve_module_decl(file_resolver, module_registry, &new_module).await;
}

fn make_fs_path(path: &[Symbol]) -> PathBuf {
    let mut fs_path = PathBuf::new();

    for symbol in path {
        fs_path.push(symbol.to_str());
    }

    fs_path.with_extension("exc")
}
