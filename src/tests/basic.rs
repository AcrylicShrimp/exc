use exc_resolve::{resolve_modules, SourceFileResolver};
use std::path::PathBuf;

#[tokio::test]
async fn test_basic() {
    let relative_root_path = PathBuf::from("src/tests");
    let absolute_root_path = std::env::current_dir()
        .unwrap()
        .join(&relative_root_path)
        .canonicalize()
        .unwrap();

    let mut source_file_resolver = SourceFileResolver::new(absolute_root_path);
    let root_module = source_file_resolver
        .resolve_file("basic.exc")
        .await
        .unwrap();

    assert_eq!(root_module.path.len(), 1);
    assert_eq!(root_module.path[0].to_str(), "basic");

    resolve_modules(&mut source_file_resolver, root_module).await;
}
