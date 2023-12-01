use crate::{tests::parse_module_for_test, ASTModuleItemKind};

#[tokio::test]
async fn test_module_item_impl() {
    const CONTENT: &'static str = "
    impl path { }
    impl path::subpath { }
    impl path::subpath::subsubpath { }
    impl path::subpath::subsubpath<generic_param> { }
    impl path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
    impl path::subpath::subsubpath<generic_param::sub_param,>::subsubsubpath { }
    impl path::subpath::subsubpath<generic_param::sub_param,generic_param::sub_param>::subsubsubpath { }
    impl path::subpath::subsubpath<generic_param::sub_param,generic_param::sub_param,>::subsubsubpath { }
    
    impl path interface path { }
    impl path::subpath interface path::subpath { }
    impl path::subpath::subsubpath interface path::subpath::subsubpath { }
    impl path::subpath::subsubpath<generic_param> interface path::subpath::subsubpath<generic_param> { }
    impl path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath interface path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
    impl path::subpath::subsubpath<generic_param::sub_param,>::subsubsubpath interface path::subpath::subsubpath<generic_param::sub_param,>::subsubsubpath { }
    impl path::subpath::subsubpath<generic_param::sub_param,generic_param::sub_param>::subsubsubpath interface path::subpath::subsubpath<generic_param::sub_param,generic_param::sub_param>::subsubsubpath { }
    impl path::subpath::subsubpath<generic_param::sub_param,generic_param::sub_param,>::subsubsubpath interface path::subpath::subsubpath<generic_param::sub_param,generic_param::sub_param,>::subsubsubpath { }

    impl path {
        fn method() { }
        fn method() -> path { }
        fn method() -> path::subpath::subsubpath { }
        fn method() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, { }

        fn method(param: path) { }
        fn method(param: path::subpath::subsubpath) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path,) { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath,) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path, param: path,) -> path { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath, param: path::subpath::subsubpath,) -> path::subpath::subsubpath { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
    }
    impl path::subpath {
        fn method() { }
        fn method() -> path { }
        fn method() -> path::subpath::subsubpath { }
        fn method() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, { }

        fn method(param: path) { }
        fn method(param: path::subpath::subsubpath) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path,) { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath,) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path, param: path,) -> path { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath, param: path::subpath::subsubpath,) -> path::subpath::subsubpath { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
    }
    impl path::subpath::subsubpath {
        fn method() { }
        fn method() -> path { }
        fn method() -> path::subpath::subsubpath { }
        fn method() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, { }

        fn method(param: path) { }
        fn method(param: path::subpath::subsubpath) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path,) { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath,) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path, param: path,) -> path { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath, param: path::subpath::subsubpath,) -> path::subpath::subsubpath { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
    }
    impl path::subpath::subsubpath<generic_param> {
        fn method() { }
        fn method() -> path { }
        fn method() -> path::subpath::subsubpath { }
        fn method() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, { }

        fn method(param: path) { }
        fn method(param: path::subpath::subsubpath) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path,) { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath,) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path, param: path,) -> path { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath, param: path::subpath::subsubpath,) -> path::subpath::subsubpath { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
    }
    impl path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath {
        fn method() { }
        fn method() -> path { }
        fn method() -> path::subpath::subsubpath { }
        fn method() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, { }

        fn method(param: path) { }
        fn method(param: path::subpath::subsubpath) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path,) { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath,) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path, param: path,) -> path { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath, param: path::subpath::subsubpath,) -> path::subpath::subsubpath { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
    }
    impl path::subpath::subsubpath<generic_param::sub_param,>::subsubsubpath {
        fn method() { }
        fn method() -> path { }
        fn method() -> path::subpath::subsubpath { }
        fn method() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, { }

        fn method(param: path) { }
        fn method(param: path::subpath::subsubpath) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path,) { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath,) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path, param: path,) -> path { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath, param: path::subpath::subsubpath,) -> path::subpath::subsubpath { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
    }
    impl path::subpath::subsubpath<generic_param::sub_param,generic_param::sub_param>::subsubsubpath {
        fn method() { }
        fn method() -> path { }
        fn method() -> path::subpath::subsubpath { }
        fn method() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, { }

        fn method(param: path) { }
        fn method(param: path::subpath::subsubpath) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path,) { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath,) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path, param: path,) -> path { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath, param: path::subpath::subsubpath,) -> path::subpath::subsubpath { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
    }
    impl path::subpath::subsubpath<generic_param::sub_param,generic_param::sub_param,>::subsubsubpath {
        fn method() { }
        fn method() -> path { }
        fn method() -> path::subpath::subsubpath { }
        fn method() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, { }
        fn method<T>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface, { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface { }
        fn method<T, U,>() -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface + path::to::interface, U: path::to::interface + path::to::interface, { }

        fn method(param: path) { }
        fn method(param: path::subpath::subsubpath) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path,) { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath,) { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) where T: path::to::interface, U: path::to::interface { }

        fn method(param: path, param: path, param: path,) -> path { }
        fn method(param: path::subpath::subsubpath, param: path::subpath::subsubpath, param: path::subpath::subsubpath,) -> path::subpath::subsubpath { }
        fn method(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath { }
        fn method<T>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface { }
        fn method<T, U,>(param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath, param: path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath,) -> path::subpath::subsubpath<generic_param::sub_param>::subsubsubpath where T: path::to::interface, U: path::to::interface { }
    }
";

    let ast = parse_module_for_test(CONTENT).await;
    assert_eq!(ast.items.len(), 24);

    for item in ast.items.iter() {
        match item.kind {
            ASTModuleItemKind::ImplBlock(_) => {}
            _ => panic!("expected impl module item"),
        }
    }
}
