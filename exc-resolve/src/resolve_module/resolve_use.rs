use crate::{GlobalSymbolRegistry, Module, ModuleRegistry, Visibility};
use exc_parse::{
    ASTModuleItemKind, ASTUsePath, ASTUsePathItemKind, ASTUsePathPrefix,
    ASTUsePathPrefixSegmentKind, Id, PunctuatedItem,
};
use exc_span::Span;
use exc_symbol::Symbol;
use std::sync::Arc;

pub fn resolve_use(
    module_registry: &ModuleRegistry,
    global_symbol_registry: &mut GlobalSymbolRegistry,
) {
    for module in module_registry.modules() {
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

            resolve_use_items(
                module_registry,
                global_symbol_registry,
                module,
                &module.path,
                &vec![&ast.path],
            );
        }
    }
}

fn resolve_use_items<'a>(
    module_registry: &ModuleRegistry,
    global_symbol_registry: &mut GlobalSymbolRegistry,
    module: &Module,
    base_path: &[Symbol],
    paths: &[&ASTUsePath],
) {
    for path in paths {
        let (target_path, span) = match &path.prefix {
            Some(prefix) => {
                if let Ok((path, span)) = resolve_full_prefix(module, base_path, prefix) {
                    (path, Some(span))
                } else {
                    continue;
                }
            }
            None => (base_path.to_vec(), None),
        };

        if let Some(span) = span {
            if !check_module_reachability(module_registry, module, &target_path, span) {
                continue;
            }
        }

        match &path.item.kind {
            ASTUsePathItemKind::All(_) => {
                // SAFETY: it is safe to call `unwrap` here because the module is always exist
                // this is guaranteed by the `check_module_reachability` function
                let target_module = module_registry.get_module(&target_path).unwrap();

                let identifiers =
                    if let Some(items) = global_symbol_registry.module_symbols(&target_module) {
                        items
                            .filter(|item| {
                                item.visibility == Visibility::Public
                                    || Arc::ptr_eq(&item.module.file, &module.file)
                            })
                            .map(|item| item.kind.identifier())
                            .collect::<Vec<_>>()
                    } else {
                        continue;
                    };

                for identifier in identifiers {
                    register_item(
                        module_registry,
                        global_symbol_registry,
                        module,
                        &target_path,
                        identifier,
                        None,
                    );
                }
            }
            ASTUsePathItemKind::Single(ast) => {
                register_item(
                    module_registry,
                    global_symbol_registry,
                    module,
                    &target_path,
                    ast.identifier,
                    ast.alias.as_ref().map(|alias| alias.identifier.symbol),
                );
            }
            ASTUsePathItemKind::Group(ast) => {
                resolve_use_items(
                    module_registry,
                    global_symbol_registry,
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
                );
            }
        }
    }
}

fn resolve_full_prefix(
    module: &Module,
    base_path: &[Symbol],
    prefix: &ASTUsePathPrefix,
) -> Result<(Vec<Symbol>, Span), ()> {
    let mut path = base_path.to_vec();

    for segment in &prefix.segments {
        match &segment.kind {
            ASTUsePathPrefixSegmentKind::Self_(_) => {}
            ASTUsePathPrefixSegmentKind::Super_(id) => {
                if path.is_empty() {
                    module.diagnostics.error(
                        id.span,
                        format!("cannot use {} outside of module scope", id.symbol),
                    );
                    return Err(());
                }

                path.pop();
            }
            ASTUsePathPrefixSegmentKind::Identifier(id) => {
                path.push(id.symbol);
            }
        }
    }

    // SAFETY: it is safe to call `unwrap` here because the prefix is not empty
    // this is guaranteed by the parser
    let span = prefix
        .segments
        .first()
        .unwrap()
        .span
        .to(prefix.segments.last().unwrap().span);

    Ok((path, span))
}

fn check_module_reachability(
    module_registry: &ModuleRegistry,
    module: &Module,
    target_path: &[Symbol],
    path_span: Span,
) -> bool {
    for index in 1..=target_path.len() {
        let path = &target_path[..index];
        let target_module = match module_registry.get_module(path) {
            Some(module) => module,
            None => {
                // ignore if the module is not exist
                // this happens when the module is in the directory
                continue;
            }
        };

        if target_module.visibility == Visibility::Private
            && !Arc::ptr_eq(&target_module.file, &module.file)
        {
            module.diagnostics.error_sub(
                path_span,
                format!(
                    "the module `::{}` is not visible from this module",
                    path.iter()
                        .map(|segment| segment.to_str())
                        .collect::<Vec<_>>()
                        .join("::")
                ),
                vec![{
                    module.diagnostics.sub_hint_simple(format!(
                        "consider making it public, or access it in the same file"
                    ))
                }],
            );
            return false;
        }
    }

    let target_module = match module_registry.get_module(target_path) {
        Some(module) => module,
        None => {
            module.diagnostics.error(
                path_span,
                format!(
                    "the module `::{}` is not exist",
                    target_path
                        .iter()
                        .map(|segment| segment.to_str())
                        .collect::<Vec<_>>()
                        .join("::")
                ),
            );
            return false;
        }
    };

    if target_module.visibility == Visibility::Private
        && !Arc::ptr_eq(&target_module.file, &module.file)
    {
        module.diagnostics.error_sub(
            path_span,
            format!(
                "the module `::{}` is not visible from this module",
                target_path
                    .iter()
                    .map(|segment| segment.to_str())
                    .collect::<Vec<_>>()
                    .join("::")
            ),
            vec![{
                module.diagnostics.sub_hint_simple(format!(
                    "consider making it public, or access it in the same file"
                ))
            }],
        );
        return false;
    }

    true
}

fn register_item(
    module_registry: &ModuleRegistry,
    global_symbol_registry: &mut GlobalSymbolRegistry,
    module: &Module,
    target_path: &[Symbol],
    identifier: Id,
    alias: Option<Symbol>,
) {
    let target_module = if let Some(target_module) = module_registry.get_module(&target_path) {
        target_module
    } else {
        return;
    };

    let mut path = target_path.to_vec();
    path.push(identifier.symbol);

    let target = match global_symbol_registry.get_symbol(&path) {
        Some(target) => target,
        None => {
            module.diagnostics.error(
                identifier.span,
                format!(
                    "the symbol {} is not exist in the module `::{}`",
                    identifier.symbol,
                    target_path
                        .iter()
                        .map(|segment| segment.to_str())
                        .collect::<Vec<_>>()
                        .join("::")
                ),
            );
            return;
        }
    };

    if target.visibility == Visibility::Private && !Arc::ptr_eq(&target.module.file, &module.file) {
        module.diagnostics.error_sub(
            identifier.span,
            format!(
                "the symbol {} is not visible from this module",
                identifier.symbol
            ),
            vec![
                {
                    target_module.diagnostics.sub_hint(
                        target.kind.identifier().span,
                        format!(
                            "the symbol {} is defined here",
                            target.kind.identifier().symbol
                        ),
                    )
                },
                {
                target_module.diagnostics.sub_hint_simple(format!(
                    "consider making it public, or access it in the same file"
                ))
                },
            ],
        );
        return;
    }

    let mut path = module.path.clone();

    match alias {
        Some(alias) => {
            path.push(alias);
        }
        None => {
            path.push(identifier.symbol);
        }
    }

    global_symbol_registry.register(path, target.clone());
}
