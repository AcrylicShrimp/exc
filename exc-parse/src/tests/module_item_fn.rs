use crate::{tests::parse_module_for_test, ASTModuleItemKind};

#[tokio::test]
async fn test_module_item_fn() {
    const CONTENT: &'static str = "
    fn foo() {}
    fn foo() -> path {}
    fn foo() -> path::subpath::subsubpath {}
    fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath {}
    fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface {}
    fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface {}

    pub fn foo() {}
    pub fn foo() -> path {}
    pub fn foo() -> path::subpath::subsubpath {}
    pub fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath {}
    pub fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface {}
    pub fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface {}

    fn foo(param0: path) {}
    fn foo(param0: path::subpath::subsubpath) {}
    fn foo(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) {}
    fn foo<T>(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface {}
    fn foo<T, U,>(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface, U: path::to::interface {}

    pub fn foo(param0: path) {}
    pub fn foo(param0: path::subpath::subsubpath) {}
    pub fn foo(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) {}
    pub fn foo<T>(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface {}
    pub fn foo<T, U,>(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface, U: path::to::interface {}

    fn foo(param0: path, param1: path,) {}
    fn foo(param0: path::subpath::subsubpath, param1: path::subpath::subsubpath,) {}
    fn foo(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param1: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) {}
    fn foo<T>(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param1: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface {}
    fn foo<T, U,>(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param1: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface, U: path::to::interface {}

    pub fn foo(param0: path, param1: path,) {}
    pub fn foo(param0: path::subpath::subsubpath, param1: path::subpath::subsubpath,) {}
    pub fn foo(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param1: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) {}
    pub fn foo<T>(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param1: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface {}
    pub fn foo<T, U,>(param0: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param1: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface, U: path::to::interface {}
";

    let ast = parse_module_for_test(CONTENT).await;
    assert_eq!(ast.items.len(), 32);

    for item in &ast.items {
        match item.kind {
            ASTModuleItemKind::FnDef(_) => {}
            _ => panic!("expected fn module item"),
        }
    }
}
