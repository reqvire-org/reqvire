//! Diagnostics helper utilities for ontology-kernel public types.

pub const MODULE: &str = "diagnostics";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

impl DiagnosticSeverity {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Hint => "hint",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagnosticCode(pub &'static str);

impl DiagnosticCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub message: String,
}

impl Diagnostic {
    #[must_use]
    pub fn new(
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            message: message.into(),
        }
    }
}

pub const CODE_SHACL_MISSING_SHAPE_NODES: DiagnosticCode =
    DiagnosticCode("o-kernel.shacl.missing_shape_nodes");
pub const CODE_SHACL_INVALID_SHAPE_REFERENCE: DiagnosticCode =
    DiagnosticCode("o-kernel.shacl.invalid_shape_reference");
pub const CODE_SHACL_INVALID_PATH: DiagnosticCode = DiagnosticCode("o-kernel.shacl.invalid_path");
pub const CODE_SHACL_INVALID_CONSTRAINT: DiagnosticCode =
    DiagnosticCode("o-kernel.shacl.invalid_constraint");

#[inline]
pub fn is_false(value: &bool) -> bool {
    !*value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_capture_generic_severity_code_message() {
        let diagnostic = Diagnostic::new(
            DiagnosticCode("ok.example"),
            DiagnosticSeverity::Error,
            "a problem was detected",
        );

        assert_eq!(diagnostic.code.as_str(), "ok.example");
        assert_eq!(diagnostic.severity.as_str(), "error");
        assert_eq!(diagnostic.message, "a problem was detected");
    }

    #[test]
    fn diagnostics_do_not_track_source_metadata() {
        let diagnostic = Diagnostic::new(
            CODE_SHACL_INVALID_PATH,
            DiagnosticSeverity::Warning,
            "path was intentionally omitted",
        );
        let debug = format!("{diagnostic:?}");
        assert!(!debug.contains("file_path"));
        assert!(!debug.contains("element_id"));
        assert!(!debug.contains("graph_layer"));
        assert!(!debug.contains("payload"));

        let second = Diagnostic::new(
            CODE_SHACL_MISSING_SHAPE_NODES,
            DiagnosticSeverity::Error,
            "shape graph is empty",
        );
        assert_eq!(second.code, CODE_SHACL_MISSING_SHAPE_NODES);
    }
}
