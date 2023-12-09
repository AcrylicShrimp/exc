mod global_symbol_registry;
mod module;
mod module_registry;
mod redirect_registry;
mod resolve_module_decl;
mod source_file_resolver;
mod visibility;

pub use global_symbol_registry::*;
pub use module::*;
pub use module_registry::*;
pub use redirect_registry::*;
pub use resolve_module_decl::*;
pub use source_file_resolver::*;
pub use visibility::*;

pub async fn resolve_global(
    file_resolver: &mut SourceFileResolver,
    root_module: Module,
) -> (ModuleRegistry, GlobalSymbolRegistry) {
    let mut module_registry = ModuleRegistry::new();

    let root_module = module_registry.register(root_module);
    module_registry.resolve_submodule(&root_module);

    resolve_module_decl(file_resolver, &mut module_registry, &root_module).await;

    let mut global_symbol_registry = GlobalSymbolRegistry::new();

    for module in module_registry.modules() {
        global_symbol_registry.register_module(module);
    }

    let mut redirect_registry = RedirectRegistry::new();

    for module in module_registry.modules() {
        redirect_registry.collect_redirects(module);
    }

    redirect_registry.resolve_redirects(&module_registry, &mut global_symbol_registry);

    (module_registry, global_symbol_registry)
}
