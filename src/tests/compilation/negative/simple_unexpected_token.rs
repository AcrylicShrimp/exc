use crate::tests::test_module;
use exc_diagnostic::error_codes;

#[tokio::test]
async fn simple_unexpected_token() {
    let diagnostics = test_module(file!(), "sources", "simple_unexpected_token").await;

    for diagnostic in &diagnostics {
        assert_eq!(diagnostic.code, error_codes::UNEXPECTED_TOKEN);
    }
}
