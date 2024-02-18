use crate::tests::test_main;

#[tokio::test]
async fn test_000001_no_path_traversal() {
    let diagnostics = test_main(file!()).await;
    assert_eq!(diagnostics.len(), 6);
}
