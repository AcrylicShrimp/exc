use crate::{parse_module, token_iter, ASTModule, ASTModuleItemKind, NodeIdAllocator};
use exc_diagnostic::DiagnosticsSender;
use exc_span::SourceMap;
use std::{path::PathBuf, sync::mpsc};
use tokio::task::spawn_blocking;

async fn test(content: impl AsRef<str>) -> ASTModule {
    let mut source_map = SourceMap::new();
    let file = source_map.add_source_file(content.as_ref(), "test.exc", None::<PathBuf>);
    let (sender, receiver) = mpsc::channel();
    let diagnostics = DiagnosticsSender::new(file.clone(), sender);
    let token_stream = token_iter(&file);
    let mut id_allocator = NodeIdAllocator::new();

    spawn_blocking(move || {
        while let Ok(diagnostic) = receiver.recv() {
            eprintln!("{:?}", diagnostic);
        }
    });

    parse_module(token_stream, &mut id_allocator, &diagnostics)
}

#[tokio::test]
async fn use_empty() {
    let ast = test("").await;
    assert_eq!(ast.items.len(), 0);

    let ast = test("                ").await;
    assert_eq!(ast.items.len(), 0);

    let ast = test("\n\n\n\n\n\n\n\n").await;
    assert_eq!(ast.items.len(), 0);

    let ast = test("\t\t\t\t\t\t\t\t").await;
    assert_eq!(ast.items.len(), 0);

    let ast = test("#comment\n#comment\n#comment").await;
    assert_eq!(ast.items.len(), 0);
}

#[tokio::test]
async fn use_simple() {
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
";

    let ast = test(CONTENT).await;
    assert_eq!(ast.items.len(), 16);

    match ast.items[0].kind {
        ASTModuleItemKind::Use(_) => {}
        _ => panic!("expected use statement"),
    }
}
