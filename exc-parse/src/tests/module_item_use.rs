use crate::{tests::parse_module_for_test, ASTModuleItemKind};

#[tokio::test]
async fn test_module_item_use() {
    const CONTENT: &'static str = "
    use path;
    use path::subpath;
    use path::subpath::subsubpath;
    use path::subpath::subsubpath as alias;
    use path::{subpath, subpath2};
    use path::{subpath, subpath2,};
    use path::{subpath, subpath2 as alias};
    use path::{subpath, subpath2 as alias,};
    use path::*;
    use path::{subpath, subpath2, *};
    use path::{subpath, subpath2, *,};
    use path::{subpath, subpath2 as alias, *};
    use path::{subpath, subpath2 as alias, *,};
    use path::{subpath, subpath2 as alias, *, subpath3::{subsubpath, subsubpath2 as alias2, *}};
    use path::{subpath, subpath2 as alias, *, subpath3::{subsubpath, subsubpath2 as alias2, *},};
    use path::{};

    pub use path;
    pub use path::subpath;
    pub use path::subpath::subsubpath;
    pub use path::subpath::subsubpath as alias;
    pub use path::{subpath, subpath2};
    pub use path::{subpath, subpath2,};
    pub use path::{subpath, subpath2 as alias};
    pub use path::{subpath, subpath2 as alias,};
    pub use path::*;
    pub use path::{subpath, subpath2, *};
    pub use path::{subpath, subpath2, *,};
    pub use path::{subpath, subpath2 as alias, *};
    pub use path::{subpath, subpath2 as alias, *,};
    pub use path::{subpath, subpath2 as alias, *, subpath3::{subsubpath, subsubpath2 as alias2, *}};
    pub use path::{subpath, subpath2 as alias, *, subpath3::{subsubpath, subsubpath2 as alias2, *},};
    pub use path::{};
";

    let ast = parse_module_for_test(CONTENT).await;
    assert_eq!(ast.items.len(), 32);

    for item in ast.items.iter() {
        match item.kind {
            ASTModuleItemKind::Use(_) => {}
            _ => panic!("expected use module item"),
        }
    }
}
