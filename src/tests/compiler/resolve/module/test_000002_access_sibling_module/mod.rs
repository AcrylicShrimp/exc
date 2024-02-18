use crate::tests::test_module;

#[tokio::test]
async fn test_000002_access_sibling_module() {
    let diagnostics = test_module(file!(), "main").await;
    assert_eq!(diagnostics.len(), 0);
}
