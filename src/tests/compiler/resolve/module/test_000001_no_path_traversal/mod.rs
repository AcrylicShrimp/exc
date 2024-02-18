use crate::tests::test_module;

#[tokio::test]
async fn test_000001_no_path_traversal() {
    let diagnostics = test_module(file!(), "main").await;
    assert_eq!(diagnostics.len(), 6);
}
