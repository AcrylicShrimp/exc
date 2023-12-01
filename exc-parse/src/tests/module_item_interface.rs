use crate::{tests::parse_module_for_test, ASTModuleItemKind};

#[tokio::test]
async fn test_module_item_interface() {
    const CONTENT: &'static str = "
    interface foo { }
    interface foo {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T> {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T,> {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T, U> {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T, U,> {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T, U,> where T: path::to::interface {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T, U,> where T: path::to::interface, {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T, U,> where T: path::to::interface + path::to::interface {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T, U,> where T: path::to::interface + path::to::interface, {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T, U,> where T: path::to::interface + path::to::interface, U: path::to::interface {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T, U,> where T: path::to::interface + path::to::interface, U: path::to::interface, {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T, U,> where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    interface foo<T, U,> where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }

    pub interface foo { }
    pub interface foo {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T> {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T,> {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T, U> {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T, U,> {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T, U,> where T: path::to::interface {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T, U,> where T: path::to::interface, {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T, U,> where T: path::to::interface + path::to::interface {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T, U,> where T: path::to::interface + path::to::interface, {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T, U,> where T: path::to::interface + path::to::interface, U: path::to::interface {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T, U,> where T: path::to::interface + path::to::interface, U: path::to::interface, {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T, U,> where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
    pub interface foo<T, U,> where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, {
        fn foo();
        fn foo() -> path;
        fn foo() -> path::subpath::subsubpath;
        fn foo() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface,;
        fn foo<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface,;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface;
        fn foo<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface,;
    }
";

    let ast = parse_module_for_test(CONTENT).await;
    assert_eq!(ast.items.len(), 28);

    for item in ast.items.iter() {
        match item.kind {
            ASTModuleItemKind::InterfaceDef(_) => {}
            _ => panic!("expected interface module item"),
        }
    }
}
