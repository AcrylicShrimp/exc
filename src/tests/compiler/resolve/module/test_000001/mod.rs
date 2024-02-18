use crate::tests::test_main;

#[tokio::test]
async fn test_000001() {
    let diagnostics = test_main(file!()).await;
    assert!(diagnostics.is_empty());
}
