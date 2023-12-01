use crate::{tests::parse_module_for_test, ASTModuleItemKind};

#[tokio::test]
async fn test_module_item_alias() {
    const CONTENT: &'static str = "
    alias identifier = path;
    alias identifier = path::subpath;
    alias identifier = path::subpath::subsubpath<generic_param>;
    alias identifier = path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;

    pub alias identifier = path;
    pub alias identifier = path::subpath;
    pub alias identifier = path::subpath::subsubpath<generic_param>;
    pub alias identifier = path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
";

    let ast = parse_module_for_test(CONTENT).await;
    assert_eq!(ast.items.len(), 8);

    match ast.items[0].kind {
        ASTModuleItemKind::AliasDef(_) => {}
        _ => panic!("expected alias module item"),
    }
}
