mod global_symbol_registry;
mod module;
mod module_registry;
mod resolve_use;
mod visibility;

pub use global_symbol_registry::*;
pub use module::*;
pub use module_registry::*;
pub use resolve_use::*;
pub use visibility::*;

pub fn resolve_modules<'a>(modules: impl Iterator<Item = Module>) {
    let mut module_registry = ModuleRegistry::new();

    for module in modules {
        let module = module_registry.register(module);
        module_registry.resolve_submodule(&module);
    }

    let mut global_symbol_registry = GlobalSymbolRegistry::new();

    for module in module_registry.modules() {
        global_symbol_registry.register_module(module);
    }

    resolve_use(&module_registry, &mut global_symbol_registry);
}
