use crate::tests::test_module;

#[tokio::test]
async fn test_000003_no_invalid_module_access() {
    let diagnostics = test_module(file!(), "invalid-01-glob").await;
    assert_eq!(diagnostics.len(), 1);
}
