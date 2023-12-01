use crate::{tests::parse_module_for_test, ASTModuleItemKind};

#[tokio::test]
async fn test_module_item_extern() {
    const CONTENT: &'static str = "
    extern {
        prototype foo();
        prototype foo(bar: path);
        prototype foo(bar: path, bar: path, bar: path, bar: path) -> path;
        prototype foo(bar: path) -> path;
        prototype foo(bar: path) -> fn () -> path;
        prototype foo(bar: [path::subpath<generic_param>]) -> fn () -> path;
        prototype foo(bar: [path::subpath<generic_param>; 123]) -> [fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path; 123];

        pub prototype foo();
        pub prototype foo(bar: path);
        pub prototype foo(bar: path, bar: path, bar: path, bar: path) -> path;
        pub prototype foo(bar: path) -> path;
        pub prototype foo(bar: path) -> fn () -> path;
        pub prototype foo(bar: [path::subpath<generic_param>]) -> fn () -> path;
        pub prototype foo(bar: [path::subpath<generic_param>; 123]) -> [fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path; 123];
    }

    extern {
        fn foo() { }
        fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) { }
        fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) -> path::subpath::subsubpath<path::subpath>::subsubsubpath { }

        pub fn foo() { }
        pub fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) { }
        pub fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) -> path::subpath::subsubpath<path::subpath>::subsubsubpath { }
    }

    extern {
        struct Foo { }

        struct Foo {
            bar: path,
        }
        
        struct Foo {
            bar: path,
            bar: path::subpath,
        }
        
        struct Foo {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        struct Foo<T> {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        struct Foo<T> where T: path::to::interface {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        struct Foo<T> where T: path::to::interface + path::to::interface {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        struct Foo<T> where T: path::to::interface, {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        struct Foo<T> where T: path::to::interface + path::to::interface, {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        struct Foo<T, U,> where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        pub struct Foo { }

        pub struct Foo {
            bar: path,
        }
        
        pub struct Foo {
            bar: path,
            bar: path::subpath,
        }
        
        pub struct Foo {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        pub struct Foo<T> {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        pub struct Foo<T> where T: path::to::interface {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        pub struct Foo<T> where T: path::to::interface + path::to::interface {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        pub struct Foo<T> where T: path::to::interface, {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        pub struct Foo<T> where T: path::to::interface + path::to::interface, {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }

        pub struct Foo<T, U,> where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, {
            bar: path,
            bar: path::subpath,
            bar: path::subpath<generic_param>,
            bar: [path::subpath<generic_param>],
            bar: [path::subpath<generic_param>; 123],
            bar: fn (path, path::subpath, path::subpath<generic_param::sub_param>::subsubpath) -> path,
        }
    }

    extern {
        impl Foo { }

        impl Foo {
            fn foo() { }
        }

        impl Foo {
            fn foo() { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) -> path::subpath::subsubpath<path::subpath>::subsubsubpath { }
        }
    }

    extern {
        impl Foo interface path::to::interface { }

        impl Foo interface path::to::interface {
            fn foo() { }
        }

        impl Foo interface path::to::interface {
            fn foo() { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) -> path::subpath::subsubpath<path::subpath>::subsubsubpath { }
        }

        impl<T> Foo interface path::to::interface {
            fn foo() { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) -> path::subpath::subsubpath<path::subpath>::subsubsubpath { }
        }

        impl<T> Foo interface path::to::interface where T: path::to::interface {
            fn foo() { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) -> path::subpath::subsubpath<path::subpath>::subsubsubpath { }
        }

        impl<T> Foo interface path::to::interface where T: path::to::interface + path::to::interface {
            fn foo() { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) -> path::subpath::subsubpath<path::subpath>::subsubsubpath { }
        }

        impl<T> Foo interface path::to::interface where T: path::to::interface, {
            fn foo() { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) -> path::subpath::subsubpath<path::subpath>::subsubsubpath { }
        }

        impl<T> Foo interface path::to::interface where T: path::to::interface + path::to::interface, {
            fn foo() { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) -> path::subpath::subsubpath<path::subpath>::subsubsubpath { }
        }

        impl<T, U,> Foo interface path::to::interface where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, {
            fn foo() { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) { }
            fn foo(bar: path, bar: path::subpath, bar: path::subpath<generic_param>) -> path::subpath::subsubpath<path::subpath>::subsubsubpath { }
        }
    }
";

    let ast = parse_module_for_test(CONTENT).await;
    assert_eq!(ast.items.len(), 5);

    for item in &ast.items {
        match item.kind {
            ASTModuleItemKind::ExternBlock(_) => {}
            _ => panic!("expected extern module item"),
        }
    }
}
