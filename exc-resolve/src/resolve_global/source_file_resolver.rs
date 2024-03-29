use crate::{DiagnosticsReceiver, Module, ModuleASTKind, Visibility};
use exc_diagnostic::{Diagnostics, DiagnosticsSender};
use exc_parse::{parse_module, token_iter, NodeIdAllocator};
use exc_span::SourceMap;
use exc_symbol::Symbol;
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SourceFileResolveError {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("path `{0}` is absolute; relative paths are expected")]
    PathIsAbsolute(PathBuf),
    #[error("path `{0}` has no file name")]
    PathHasNoFileName(PathBuf),
    #[error("path `{0}` contains invalid UTF-8")]
    InvalidUTF8(PathBuf),
}

#[derive(Debug)]
pub struct SourceFileResolver {
    root_path: PathBuf,
    source_map: SourceMap,
    node_id_alloc: NodeIdAllocator,
    diagnostics_receiver: DiagnosticsReceiver,
}

impl SourceFileResolver {
    pub fn new(root_path: impl Into<PathBuf>, print_diagnostics: bool) -> Self {
        let root_path = root_path.into();

        if root_path.is_relative() {
            panic!("root path must be absolute");
        }

        Self {
            root_path,
            source_map: SourceMap::new(),
            node_id_alloc: NodeIdAllocator::new(),
            diagnostics_receiver: DiagnosticsReceiver::new(print_diagnostics),
        }
    }

    pub async fn into_diagnostics(self) -> Vec<Diagnostics> {
        self.diagnostics_receiver.into_diagnostics().await
    }

    pub async fn resolve_file(
        &mut self,
        relative_path: impl AsRef<Path>,
    ) -> Result<Module, SourceFileResolveError> {
        let relative_path = relative_path.as_ref();

        if relative_path.is_absolute() {
            return Err(SourceFileResolveError::PathIsAbsolute(
                relative_path.to_owned(),
            ));
        }

        let file_name = relative_path
            .file_name()
            .ok_or_else(|| SourceFileResolveError::PathHasNoFileName(relative_path.to_owned()))?
            .to_str()
            .ok_or_else(|| SourceFileResolveError::InvalidUTF8(relative_path.to_owned()))?;
        let absolute_path = self.root_path.join(relative_path);
        let content = std::io::read_to_string(File::open(absolute_path)?)?;
        let file =
            self.source_map
                .add_source_file(&content, file_name, Some(relative_path.to_owned()));
        let diagnostics = DiagnosticsSender::new(file.clone(), self.diagnostics_receiver.sender());
        let token_stream = token_iter(&file);

        let mut path = Vec::new();

        for component in &relative_path.with_extension("") {
            let segment = component
                .to_str()
                .ok_or_else(|| SourceFileResolveError::InvalidUTF8(relative_path.to_owned()))?;
            let segment = Symbol::from_str(segment);
            path.push(segment);
        }

        let ast = parse_module(token_stream, &mut self.node_id_alloc, &diagnostics);
        let module = Module::new(
            Visibility::Public,
            ModuleASTKind::Module(ast.into()),
            path,
            file.clone(),
            diagnostics,
        );

        Ok(module)
    }
}
