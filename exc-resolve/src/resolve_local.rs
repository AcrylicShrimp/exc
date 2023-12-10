mod local_symbol_registry;

pub use local_symbol_registry::*;

use crate::{GlobalSymbolRegistry, ModuleRegistry};

pub fn resolve_local(
    module_registry: &ModuleRegistry,
    global_symbol_registry: &GlobalSymbolRegistry,
) -> LocalSymbolRegistry {
    let mut local_symbol_registry = LocalSymbolRegistry::new();
    local_symbol_registry.register(module_registry, global_symbol_registry);

    local_symbol_registry
}
