use exc_diagnostic::Diagnostics;
use exc_resolve::{resolve_global, SourceFileResolver};
use std::path::Path;

#[cfg(test)]
mod compiler;

pub async fn test_module(
    path: impl AsRef<Path>,
    main_file_name: impl AsRef<Path>,
) -> Vec<Diagnostics> {
    let root_path = path.as_ref().parent().unwrap().canonicalize().unwrap();
    let mut source_file_resolver = SourceFileResolver::new(root_path, true);

    {
        let root_module = source_file_resolver
            .resolve_file(main_file_name.as_ref().with_extension("exc"))
            .await
            .unwrap();

        let (_module_registry, _global_symbol_registry) =
            resolve_global(&mut source_file_resolver, root_module).await;
    }

    source_file_resolver.into_diagnostics().await
}
