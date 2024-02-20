use exc_span::{SourceFile, Span};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

pub mod error_codes;
pub mod warning_codes;

#[derive(Debug, Clone)]
pub struct DiagnosticsSender {
    file: Arc<SourceFile>,
    sender: UnboundedSender<Diagnostics>,
}

impl DiagnosticsSender {
    pub fn new(file: Arc<SourceFile>, sender: UnboundedSender<Diagnostics>) -> Self {
        Self { file, sender }
    }

    pub fn file(&self) -> &Arc<SourceFile> {
        &self.file
    }

    pub fn hint(&self, span: Span, message: String) {
        self.sender
            .send(Diagnostics {
                code: 0,
                level: DiagnosticsLevel::Hint,
                message,
                origin: Some(DiagnosticsOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_diagnostics: vec![],
            })
            .unwrap();
    }

    pub fn hint_sub(&self, span: Span, message: String, sub_diagnostics: Vec<SubDiagnostics>) {
        self.sender
            .send(Diagnostics {
                code: 0,
                level: DiagnosticsLevel::Hint,
                message,
                origin: Some(DiagnosticsOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_diagnostics,
            })
            .unwrap();
    }

    pub fn hint_simple(&self, message: String) {
        self.sender
            .send(Diagnostics {
                code: 0,
                level: DiagnosticsLevel::Hint,
                message,
                origin: None,
                sub_diagnostics: vec![],
            })
            .unwrap()
    }

    pub fn warning(&self, code: u32, span: Span, message: String) {
        self.sender
            .send(Diagnostics {
                code,
                level: DiagnosticsLevel::Warning,
                message,
                origin: Some(DiagnosticsOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_diagnostics: vec![],
            })
            .unwrap()
    }

    pub fn warning_sub(
        &self,
        code: u32,
        span: Span,
        message: String,
        sub_diagnostics: Vec<SubDiagnostics>,
    ) {
        self.sender
            .send(Diagnostics {
                code,
                level: DiagnosticsLevel::Warning,
                message,
                origin: Some(DiagnosticsOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_diagnostics,
            })
            .unwrap()
    }

    pub fn warning_simple(&self, code: u32, message: String) {
        self.sender
            .send(Diagnostics {
                code,
                level: DiagnosticsLevel::Warning,
                message,
                origin: None,
                sub_diagnostics: vec![],
            })
            .unwrap()
    }

    pub fn error(&self, code: u32, span: Span, message: String) {
        self.sender
            .send(Diagnostics {
                code,
                level: DiagnosticsLevel::Error,
                message,
                origin: Some(DiagnosticsOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_diagnostics: vec![],
            })
            .unwrap()
    }

    pub fn error_sub(
        &self,
        code: u32,
        span: Span,
        message: String,
        sub_diagnostics: Vec<SubDiagnostics>,
    ) {
        self.sender
            .send(Diagnostics {
                code,
                level: DiagnosticsLevel::Error,
                message,
                origin: Some(DiagnosticsOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_diagnostics,
            })
            .unwrap()
    }

    pub fn error_simple(&self, code: u32, message: String) {
        self.sender
            .send(Diagnostics {
                code,
                level: DiagnosticsLevel::Error,
                message,
                origin: None,
                sub_diagnostics: vec![],
            })
            .unwrap()
    }

    pub fn sub_hint(&self, span: Span, message: String) -> SubDiagnostics {
        SubDiagnostics {
            level: DiagnosticsLevel::Hint,
            message,
            origin: Some(DiagnosticsOrigin {
                file: self.file.clone(),
                span,
            }),
        }
    }

    pub fn sub_hint_simple(&self, message: String) -> SubDiagnostics {
        SubDiagnostics {
            level: DiagnosticsLevel::Hint,
            message,
            origin: None,
        }
    }

    pub fn sub_warning(&self, span: Span, message: String) -> SubDiagnostics {
        SubDiagnostics {
            level: DiagnosticsLevel::Warning,
            message,
            origin: Some(DiagnosticsOrigin {
                file: self.file.clone(),
                span,
            }),
        }
    }

    pub fn sub_warning_simple(&self, message: String) -> SubDiagnostics {
        SubDiagnostics {
            level: DiagnosticsLevel::Warning,
            message,
            origin: None,
        }
    }

    pub fn sub_error(&self, span: Span, message: String) -> SubDiagnostics {
        SubDiagnostics {
            level: DiagnosticsLevel::Error,
            message,
            origin: Some(DiagnosticsOrigin {
                file: self.file.clone(),
                span,
            }),
        }
    }

    pub fn sub_error_simple(&self, message: String) -> SubDiagnostics {
        SubDiagnostics {
            level: DiagnosticsLevel::Error,
            message,
            origin: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostics {
    /// code of the diagnostic, 0 for helper diagnostics
    /// in other words, warning and error diagnostics should have a non-zero code
    pub code: u32,
    pub level: DiagnosticsLevel,
    pub message: String,
    pub origin: Option<DiagnosticsOrigin>,
    pub sub_diagnostics: Vec<SubDiagnostics>,
}

#[derive(Debug, Clone)]
pub struct SubDiagnostics {
    pub level: DiagnosticsLevel,
    pub message: String,
    pub origin: Option<DiagnosticsOrigin>,
}

#[derive(Debug, Clone)]
pub struct DiagnosticsOrigin {
    pub file: Arc<SourceFile>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticsLevel {
    Hint,
    Warning,
    Error,
}
