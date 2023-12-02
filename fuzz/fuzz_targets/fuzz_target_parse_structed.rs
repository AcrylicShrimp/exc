#![no_main]

use exc_diagnostic::DiagnosticsSender;
use exc_parse::{parse_module, token_iter, NodeIdAllocator};
use exc_span::SourceMap;
use libfuzzer_sys::fuzz_target;
use std::{path::PathBuf, sync::mpsc};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let s = format!("fn main() {{ {} }}", s);

        let mut source_map = SourceMap::new();
        let file = source_map.add_source_file(&s, "test.exc", None::<PathBuf>);
        let (sender, receiver) = mpsc::channel();
        let diagnostics = DiagnosticsSender::new(file.clone(), sender);
        let token_stream = token_iter(&file);
        let mut id_allocator = NodeIdAllocator::new();

        parse_module(token_stream, &mut id_allocator, &diagnostics);
    }
});
