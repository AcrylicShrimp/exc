use exc_parse::ASTUsePathPrefixSegmentKind;
use exc_symbol::Symbol;

use crate::GlobalSymbol;

pub fn visualize_prefix(prefix: &[ASTUsePathPrefixSegmentKind]) -> String {
    prefix
        .iter()
        .map(|segment| segment.id().symbol.to_str())
        .collect::<Vec<_>>()
        .join("::")
}

pub fn visualize_module_path(path: &[Symbol]) -> String {
    path.iter()
        .map(|symbol| symbol.to_str())
        .collect::<Vec<_>>()
        .join("::")
}

pub fn visualize_global_symbol_path(global_symbol: &GlobalSymbol) -> String {
    global_symbol
        .module
        .path
        .iter()
        .chain(std::iter::once(&global_symbol.kind.identifier().symbol))
        .map(|symbol| symbol.to_str())
        .collect::<Vec<_>>()
        .join("::")
}
