mod module;
mod module_registry;

pub use module::*;
pub use module_registry::*;

use self::module_registry::ModuleRegistry;

pub fn resolve_modules<'a>(modules: impl Iterator<Item = Module>) {
    let mut registry = ModuleRegistry::new();

    for module in modules {
        let module = registry.register(module);
        registry.resolve_submodule(&module);
    }
}
