mod write_diagnostic;

use exc_diagnostic::DiagnosticsSender;
use exc_parse::{parse_module, token_iter, NodeIdAllocator};
use exc_resolve::{resolve_modules, Module, ModuleASTKind, Visibility};
use exc_span::SourceMap;
use std::{path::PathBuf, sync::mpsc};
use tokio::task::spawn_blocking;
use write_diagnostic::write_diagnostic;

#[tokio::main]
async fn main() {
    const CONTENT: &'static str = "
        module test {
            fn foo() {}
            fn foo() {}
            fn bar() {}

            module bar {
                fn foo() {}
            }
        }

        module test {}
    ";

    let mut source_map = SourceMap::new();
    let file = source_map.add_source_file(CONTENT, "test.exc", Some(PathBuf::from("test.exc")));
    let (sender, receiver) = mpsc::channel();
    let diagnostics = DiagnosticsSender::new(file.clone(), sender);
    let token_stream = token_iter(&file);
    let mut id_allocator = NodeIdAllocator::new();

    spawn_blocking(move || {
        while let Ok(diagnostic) = receiver.recv() {
            write_diagnostic(&diagnostic);
        }
    });

    let ast = parse_module(token_stream, &mut id_allocator, &diagnostics);
    let module = Module::new(
        Visibility::Private,
        ModuleASTKind::Module(ast.into()),
        "test.exc",
        file.clone(),
        diagnostics,
    )
    .unwrap();

    resolve_modules(vec![{ module }].into_iter());
}
