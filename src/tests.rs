use exc_diagnostic::Diagnostics;
use exc_resolve::{resolve_global, SourceFileResolver};
use std::path::Path;

#[cfg(test)]
mod compiler;

pub async fn test_main(path: impl AsRef<Path>) -> Vec<Diagnostics> {
    let root_path = path.as_ref().parent().unwrap().canonicalize().unwrap();
    let mut source_file_resolver = SourceFileResolver::new(root_path, false);

    {
        let root_module = source_file_resolver.resolve_file("main.exc").await.unwrap();

        assert_eq!(root_module.path.len(), 1);
        assert_eq!(root_module.path[0].to_str(), "main");

        let (_module_registry, _global_symbol_registry) =
            resolve_global(&mut source_file_resolver, root_module).await;
    }

    source_file_resolver.into_diagnostics().await
}
