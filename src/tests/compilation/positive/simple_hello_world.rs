use crate::tests::test_module;

#[tokio::test]
async fn simple_hello_world() {
    let diagnostics = test_module(file!(), "sources", "simple_hello_world").await;

    assert_eq!(diagnostics.len(), 0);
}
