use crate::{
    GlobalSymbol, GlobalSymbolKind, GlobalSymbolRegistry, Module, ModuleRegistry, SymbolLevel,
    Visibility,
};
use exc_parse::{
    ASTModuleItemKind, ASTUsePath, ASTUsePathItemKind, ASTUsePathPrefixSegmentKind, Id, NodeId,
    PunctuatedItem,
};
use exc_span::Span;
use exc_symbol::Symbol;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct Redirect {
    pub module: Arc<Module>,
    pub visibility: Visibility,
    pub prefix: Vec<ASTUsePathPrefixSegmentKind>,
    pub target: RedirectTarget,
    /// whether the module this redirect points to is found at least once
    /// if this is `false` after the resolution, it means that the redirect target is invalid
    /// emit an error in this case
    pub found_target_module: bool,
    pub span: Span,
}

impl Redirect {
    pub fn new(
        module: Arc<Module>,
        visibility: Visibility,
        prefix: Vec<ASTUsePathPrefixSegmentKind>,
        target: RedirectTarget,
        span: Span,
    ) -> Self {
        Self {
            module,
            visibility,
            prefix,
            target,
            found_target_module: false,
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RedirectTarget {
    Glob,
    Single(RedirectSingleTarget),
}

impl RedirectTarget {
    pub fn all() -> Self {
        Self::Glob
    }

    pub fn single(identifier: Id, alias: Option<Id>) -> Self {
        Self::Single(RedirectSingleTarget { identifier, alias })
    }
}

#[derive(Debug, Clone)]
pub struct RedirectSingleTarget {
    pub identifier: Id,
    pub alias: Option<Id>,
}

#[derive(Debug, Clone)]
pub struct GlobRedirect {
    pub module: Arc<Module>,
    pub target_module: Arc<Module>,
    pub visibility: Visibility,
}

impl GlobRedirect {
    pub fn new(module: Arc<Module>, target_module: Arc<Module>, visibility: Visibility) -> Self {
        Self {
            module,
            target_module,
            visibility,
        }
    }
}

#[derive(Default, Debug)]
pub struct RedirectRegistry {
    pub redirects: HashMap<NodeId, Vec<Redirect>>,
}

impl RedirectRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn collect_redirects(&mut self, module: &Arc<Module>) {
        for item in module.ast.items() {
            let ast = match &item.kind {
                ASTModuleItemKind::Use(ast) => ast,
                ASTModuleItemKind::AliasDef(_) => continue,
                ASTModuleItemKind::ModuleDecl(_) => continue,
                ASTModuleItemKind::ModuleDef(_) => continue,
                ASTModuleItemKind::ExternBlock(_) => continue,
                ASTModuleItemKind::FnDef(_) => continue,
                ASTModuleItemKind::StructDef(_) => continue,
                ASTModuleItemKind::InterfaceDef(_) => continue,
                ASTModuleItemKind::ImplBlock(_) => continue,
            };

            self.collect_redirects_items(
                module,
                if ast.keyword_pub.is_some() {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
                &[],
                vec![&ast.path],
            );
        }
    }

    fn collect_redirects_items(
        &mut self,
        module: &Arc<Module>,
        visibility: Visibility,
        base_prefix: &[ASTUsePathPrefixSegmentKind],
        paths: Vec<&ASTUsePath>,
    ) {
        for path in paths {
            let mut full_prefix = base_prefix.to_vec();

            if let Some(prefix) = &path.prefix {
                for segment in &prefix.segments {
                    full_prefix.push(segment.kind.clone());
                }
            }

            match &path.item.kind {
                ASTUsePathItemKind::All(_) => {
                    let redirect = Redirect::new(
                        module.clone(),
                        visibility,
                        full_prefix,
                        RedirectTarget::all(),
                        path.span,
                    );

                    self.redirects
                        .entry(module.ast.id())
                        .or_default()
                        .push(redirect);
                }
                ASTUsePathItemKind::Single(ast) => {
                    let redirect = Redirect::new(
                        module.clone(),
                        visibility,
                        full_prefix,
                        RedirectTarget::single(
                            ast.identifier,
                            ast.alias.as_ref().map(|alias| alias.identifier),
                        ),
                        path.span,
                    );

                    self.redirects
                        .entry(module.ast.id())
                        .or_default()
                        .push(redirect);
                }
                ASTUsePathItemKind::Group(ast) => {
                    self.collect_redirects_items(
                        module,
                        visibility,
                        &full_prefix,
                        ast.items
                            .items
                            .iter()
                            .map(|item| match item {
                                PunctuatedItem::Punctuated { item, .. } => item,
                                PunctuatedItem::NotPunctuated { item } => item,
                            })
                            .collect(),
                    );
                }
            }
        }
    }

    pub fn resolve_redirects(
        mut self,
        module_registry: &ModuleRegistry,
        global_symbol_registry: &mut GlobalSymbolRegistry,
    ) {
        loop {
            let mut changed = false;

            let added = self.resolve_redirects_single_only(module_registry, global_symbol_registry);
            changed |= added;

            let added = self.resolve_redirects_glob_only(module_registry, global_symbol_registry);
            changed |= added;

            if !changed {
                break;
            }
        }

        for redirects in self.redirects.values() {
            for redirect in redirects {
                match &redirect.target {
                    RedirectTarget::Glob => {
                        if redirect.found_target_module {
                            continue;
                        }

                        // the target module of this glob redirect is not found
                        // emit compile error
                        let path = visualize_prefix(&redirect.prefix);
                        redirect
                            .module
                            .diagnostics
                            .error(redirect.span, format!("the module `{}` is not found", path));
                    }
                    RedirectTarget::Single(single_target) => {
                        // all single targets should be resolved and removed during the resolution
                        // emit compile error if it is not resolved
                        let path = visualize_prefix(&redirect.prefix);
                        redirect.module.diagnostics.error(
                            redirect.span,
                            format!(
                                "the symbol {} from module `{}` is not found",
                                single_target.identifier.symbol, path
                            ),
                        );
                    }
                }
            }
        }
    }

    fn resolve_redirects_single_only(
        &mut self,
        module_registry: &ModuleRegistry,
        global_symbol_registry: &mut GlobalSymbolRegistry,
    ) -> bool {
        let mut changed_at_least_once = false;

        loop {
            let mut changed = false;

            for redirects in self.redirects.values_mut() {
                let mut removed_indices = Vec::new();

                for (index, redirect) in redirects.iter_mut().enumerate() {
                    let target_module = match resolve_target_module(
                        module_registry,
                        global_symbol_registry,
                        &redirect.module,
                        &redirect.prefix,
                    ) {
                        Ok(target_module) => target_module,
                        Err(_) => {
                            // the target module is unable to be resolved; do not try resolve this redirect later
                            removed_indices.push(index);
                            continue;
                        }
                    };

                    let target_module = match target_module {
                        Some(target_module) => target_module,
                        None => continue,
                    };

                    // we've found the target module; mark it as found
                    redirect.found_target_module = true;

                    let single_target = match &redirect.target {
                        RedirectTarget::Glob => continue,
                        RedirectTarget::Single(single_target) => single_target,
                    };

                    let global_symbol = match global_symbol_registry
                        .get_any_symbol(target_module, single_target.identifier.symbol)
                    {
                        Some(global_symbol) => global_symbol,
                        None => {
                            // it seems that the target module does not have the symbol yet
                            // retry later, because global symbols may be added later
                            continue;
                        }
                    };

                    if check_global_symbol_visibility(&redirect.module, &global_symbol) {
                        let added = global_symbol_registry.register(GlobalSymbol::new(
                            SymbolLevel::Explicit,
                            redirect.visibility,
                            redirect.module.clone(),
                            global_symbol.kind.clone(),
                            single_target.alias.unwrap_or(single_target.identifier),
                            redirect.module.clone(),
                        ));
                        changed |= added;
                    }

                    // remove this redirect because it is resolved
                    removed_indices.push(index);
                }

                for index in removed_indices.into_iter().rev() {
                    redirects.remove(index);
                }
            }

            if !changed {
                break;
            }

            changed_at_least_once = true;
        }

        changed_at_least_once
    }

    fn resolve_redirects_glob_only(
        &mut self,
        module_registry: &ModuleRegistry,
        global_symbol_registry: &mut GlobalSymbolRegistry,
    ) -> bool {
        let mut changed_at_least_once = false;

        loop {
            let mut changed = false;

            for (module_id, redirects) in &mut self.redirects {
                let mut glob_redirects = Vec::new();
                let mut removed_indices = Vec::new();

                for (index, redirect) in redirects.iter_mut().enumerate() {
                    let target_module = match resolve_target_module(
                        module_registry,
                        global_symbol_registry,
                        &redirect.module,
                        &redirect.prefix,
                    ) {
                        Ok(target_module) => target_module,
                        Err(_) => {
                            // the target module is unable to be resolved; do not try resolve this redirect later
                            removed_indices.push(index);
                            continue;
                        }
                    };

                    let target_module = match target_module {
                        Some(target_module) => target_module,
                        None => continue,
                    };

                    // we've found the target module; mark it as found
                    redirect.found_target_module = true;

                    match &redirect.target {
                        RedirectTarget::Glob => {}
                        RedirectTarget::Single(_) => continue,
                    }

                    glob_redirects.push(GlobRedirect::new(
                        redirect.module.clone(),
                        target_module.clone(),
                        redirect.visibility,
                    ));

                    // glob redirects should not be removed,
                    // because it's unable to determine whether the redirect is fully resolved
                }

                for index in removed_indices.into_iter().rev() {
                    redirects.remove(index);
                }

                let module = module_registry.get_module_by_id(*module_id).unwrap();
                let blocklist =
                    build_glob_import_blocklist(global_symbol_registry, &module, &glob_redirects);

                for glob_redirect in glob_redirects {
                    let mut redirected = Vec::new();

                    let global_symbols =
                        global_symbol_registry.any_symbols(&glob_redirect.target_module);

                    for global_symbol in global_symbols {
                        if global_symbol.visibility == Visibility::Private
                            && !Arc::ptr_eq(&module.file, &global_symbol.module.file)
                        {
                            continue;
                        }

                        if blocklist.contains(&global_symbol.kind.identifier().symbol) {
                            continue;
                        }

                        redirected.push(global_symbol.clone());
                    }

                    for global_symbol in redirected {
                        let added = global_symbol_registry.register(GlobalSymbol::new(
                            SymbolLevel::Glob,
                            glob_redirect.visibility,
                            module.clone(),
                            global_symbol.kind.clone(),
                            global_symbol.kind.identifier(),
                            global_symbol.identifier_module.clone(),
                        ));
                        changed |= added;
                    }
                }
            }

            if !changed {
                break;
            }

            changed_at_least_once = true;
        }

        changed_at_least_once
    }
}

fn resolve_target_module<'a>(
    module_registry: &'a ModuleRegistry,
    global_symbol_registry: &'a GlobalSymbolRegistry,
    module: &'a Arc<Module>,
    prefix: &[ASTUsePathPrefixSegmentKind],
) -> Result<Option<&'a Arc<Module>>, ()> {
    let mut target_module = module;

    if !prefix.is_empty() {
        for segment in prefix {
            match segment {
                ASTUsePathPrefixSegmentKind::Self_(_) => continue,
                ASTUsePathPrefixSegmentKind::Super_(id) => {
                    if !check_module_has_super(module, target_module, *id) {
                        return Err(());
                    }

                    let mut path = target_module.path.clone();
                    path.pop();

                    // SAFETY: it is safe to call `unwrap` here
                    // because we have already checked that the module has a super module
                    target_module = module_registry.get_module(&path).unwrap();

                    if !check_module_visibility(module, target_module, id.span) {
                        return Err(());
                    }
                }
                ASTUsePathPrefixSegmentKind::Identifier(id) => {
                    let global_symbol =
                        match global_symbol_registry.get_module_symbol(target_module, id.symbol) {
                            Some(global_symbol) => global_symbol,
                            None => {
                                return Ok(None);
                            }
                        };

                    match &global_symbol.kind {
                        GlobalSymbolKind::ModuleDecl(ast) => {
                            // SAFETY: it is safe to call `unwrap` here
                            // because we have already checked that the symbol is a module
                            target_module = module_registry.get_module_by_id(ast.id).unwrap();
                        }
                        GlobalSymbolKind::ModuleDef(ast) => {
                            // SAFETY: it is safe to call `unwrap` here
                            // because we have already checked that the symbol is a module
                            target_module = module_registry.get_module_by_id(ast.id).unwrap();
                        }
                        _ => {
                            // SAFETY: it is safe to call `unwrap` here
                            // because we have already checked that the symbol is a module
                            unreachable!();
                        }
                    }

                    if !check_module_visibility(module, target_module, id.span) {
                        return Err(());
                    }
                }
            }
        }
    }

    Ok(Some(target_module))
}

fn check_module_has_super(module: &Module, target_module: &Module, id: Id) -> bool {
    if 1 < target_module.path.len() {
        return true;
    }

    let path = visualize_module_path(&target_module.path);
    module.diagnostics.error(
        id.span,
        format!(
            "{} is not allowed here; the module `{}` is already at root level",
            id.symbol, path
        ),
    );

    false
}

fn check_module_visibility(module: &Module, target_module: &Module, span: Span) -> bool {
    if target_module.visibility == Visibility::Public
        || Arc::ptr_eq(&module.file, &target_module.file)
    {
        return true;
    }

    let path = visualize_module_path(&target_module.path);
    module.diagnostics.error_sub(
        span,
        format!("the module `{}` is not visible from this module", path),
        vec![{
            module.diagnostics.sub_hint_simple(format!(
                "consider making the module public, or accessing it in same file"
            ))
        }],
    );

    false
}

fn check_global_symbol_visibility(module: &Module, global_symbol: &GlobalSymbol) -> bool {
    if global_symbol.visibility == Visibility::Public
        || Arc::ptr_eq(&module.file, &global_symbol.module.file)
    {
        return true;
    }

    let path = visualize_global_symbol_path(&global_symbol);
    module.diagnostics.error_sub(
        global_symbol.kind.identifier().span,
        format!("the symbol `{}` is not visible from this module", path),
        vec![{
            module.diagnostics.sub_hint_simple(format!(
                "consider making the symbol public, or accessing it in same file"
            ))
        }],
    );

    false
}

fn visualize_prefix(prefix: &[ASTUsePathPrefixSegmentKind]) -> String {
    prefix
        .iter()
        .map(|segment| segment.id().symbol.to_str())
        .collect::<Vec<_>>()
        .join("::")
}

fn visualize_module_path(path: &[Symbol]) -> String {
    path.iter()
        .map(|symbol| symbol.to_str())
        .collect::<Vec<_>>()
        .join("::")
}

fn visualize_global_symbol_path(global_symbol: &GlobalSymbol) -> String {
    global_symbol
        .module
        .path
        .iter()
        .chain(std::iter::once(&global_symbol.kind.identifier().symbol))
        .map(|symbol| symbol.to_str())
        .collect::<Vec<_>>()
        .join("::")
}

fn build_glob_import_blocklist(
    global_symbol_registry: &GlobalSymbolRegistry,
    module: &Module,
    glob_redirects: &[GlobRedirect],
) -> HashSet<Symbol> {
    let mut blocklist = HashSet::new();

    // 1. symbols that are already redirected
    blocklist.extend(
        global_symbol_registry
            .any_symbols(module)
            .into_iter()
            .map(|symbol| symbol.kind.identifier().symbol),
    );

    // 2. symbols that will be imported more than once by glob redirects
    let mut count_map = HashMap::<Symbol, usize>::new();

    for glob_redirect in glob_redirects {
        if let Some(global_symbols) =
            global_symbol_registry.non_module_symbols(&glob_redirect.target_module)
        {
            for global_symbol in global_symbols {
                if global_symbol.visibility == Visibility::Private
                    && !Arc::ptr_eq(&module.file, &global_symbol.module.file)
                {
                    continue;
                }

                *count_map
                    .entry(global_symbol.kind.identifier().symbol)
                    .or_default() += 1;
            }
        }
    }

    blocklist.extend(count_map.into_iter().filter_map(
        |(symbol, count)| {
            if 1 < count {
                Some(symbol)
            } else {
                None
            }
        },
    ));

    blocklist
}
