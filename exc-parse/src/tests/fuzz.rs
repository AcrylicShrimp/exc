use crate::tests::parse_module_for_test;

#[tokio::test]
async fn test_fuzz_timeout_aa5d71c522e025bc25fca44f7f7366f2f18ecfd9() {
    const CONTENT: &'static str =
        include_str!("./fuzz-artifacts/timeout-aa5d71c522e025bc25fca44f7f7366f2f18ecfd9");

    parse_module_for_test(CONTENT).await;
}

#[tokio::test]
async fn test_fuzz_timeout_bc1f229c8ca8490dfb38478ea317f3a1f0b483f2() {
    const CONTENT: &'static str =
        include_str!("./fuzz-artifacts/timeout-bc1f229c8ca8490dfb38478ea317f3a1f0b483f2");

    parse_module_for_test(CONTENT).await;
}
