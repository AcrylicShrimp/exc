mod fuzz;
mod module_item_alias;
mod module_item_impl;
mod module_item_extern;
mod module_item_fn;
mod module_item_interface;
mod module_item_struct;
mod module_item_use;

use crate::{parse_module, token_iter, ASTModule, NodeIdAllocator};
use exc_diagnostic::DiagnosticsSender;
use exc_span::SourceMap;
use std::{path::PathBuf, sync::mpsc};
use tokio::task::spawn_blocking;

async fn parse_module_for_test(content: impl AsRef<str>) -> ASTModule {
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
