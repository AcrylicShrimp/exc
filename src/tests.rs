use exc_diagnostic::Diagnostics;
use exc_resolve::{resolve_global, resolve_local, SourceFileResolver};
use std::path::Path;

#[cfg(test)]
mod compilation;

pub async fn test_module(
    file: impl AsRef<Path>,
    sub_path: impl AsRef<Path>,
    main_file_name: impl AsRef<Path>,
) -> Vec<Diagnostics> {
    let root_path = file
        .as_ref()
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap()
        .join(sub_path);
    let mut source_file_resolver = SourceFileResolver::new(root_path, true);

    {
        let root_module = source_file_resolver
            .resolve_file(main_file_name.as_ref().with_extension("exc"))
            .await
            .unwrap();

        let (module_registry, global_symbol_registry) =
            resolve_global(&mut source_file_resolver, root_module).await;

        let _local_symbol_registry = resolve_local(&module_registry, &global_symbol_registry);
    }

    source_file_resolver.into_diagnostics().await
}
