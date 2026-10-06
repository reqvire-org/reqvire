use serde::Serialize;
use thiserror::Error;

/// Lightweight source context for an error or diagnostic.
///
/// Carries optional file/line/column/element metadata so JSON/MCP consumers
/// can surface structured location information alongside the human-readable
/// error message.
#[derive(Debug, Clone, Serialize)]
pub struct ErrorContext {
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub element_id: Option<String>,
}

impl ErrorContext {
    pub const fn new() -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            element_id: None,
        }
    }

    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub const fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub const fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub fn with_element_id(mut self, element_id: impl Into<String>) -> Self {
        self.element_id = Some(element_id.into());
        self
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

/// A structured validation diagnostic for model validation.
///
/// `code` is a stable machine-readable diagnostic code, `message` is the
/// human-readable description, and `context` optionally carries source
/// location or element metadata.
#[derive(Debug, Clone, Serialize)]
pub struct ValidationDiagnostic {
    pub code: &'static str,
    pub message: String,
    pub context: Option<ErrorContext>,
}

impl ValidationDiagnostic {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            context: None,
        }
    }

    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = Some(context);
        self
    }
}

impl std::fmt::Display for ValidationDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.context {
            Some(ctx) => {
                let mut parts: Vec<String> = Vec::new();
                if let Some(file) = &ctx.file {
                    parts.push(format!("file={}", file));
                }
                if let Some(line) = ctx.line {
                    parts.push(format!("line={}", line));
                }
                if let Some(column) = ctx.column {
                    parts.push(format!("column={}", column));
                }
                if let Some(element_id) = &ctx.element_id {
                    parts.push(format!("element={}", element_id));
                }
                if parts.is_empty() {
                    write!(f, "[{}] {}", self.code, self.message)
                } else {
                    write!(f, "[{}] {} ({})", self.code, self.message, parts.join(", "))
                }
            }
            None => write!(f, "[{}] {}", self.code, self.message),
        }
    }
}

/// Custom error types for the Reqvire application
#[derive(Error, Debug)]
pub enum ReqvireError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Git Command error: {0}")]
    GitCommandError(String),

    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),

    #[error("Relation error: {0}")]
    RelationError(String),

    #[error("Element error: {0}")]
    ElementError(String),

    #[error("Circular dependency error: {0}")]
    CircularDependencyError(String),

    #[error("Duplicate element: {0}")]
    DuplicateElement(String),

    #[error("Duplicate subsection: {0}")]
    DuplicateSubsection(String),

    #[error("Missing element: {0}")]
    MissingElement(String),

    #[error("Ambiguous element selection: {0}")]
    AmbiguousElementSelection(String),

    #[error("Conflicting element selectors: {0}")]
    ConflictingElementSelectors(String),

    #[error("Missing relation target: {0}")]
    MissingRelationTarget(String),

    #[error("Missing parent relation: {0}")]
    MissingParentRelation(String),

    #[error("Invalid relation format: {0}")]
    InvalidRelationFormat(String),

    #[error("Invalid metadata format: {0}")]
    InvalidMetadataFormat(String),

    #[error("Invalid markdown structure: {0}")]
    InvalidMarkdownStructure(String),

    #[error("Duplicate relation: {0}")]
    DuplicateRelation(String),

    #[error("Unsupported relation type: {0}")]
    UnsupportedRelationType(String),

    #[error("Incompatible element types for relation: {0}")]
    IncompatibleElementTypes(String),

    #[error("Mixed hierarchical relations in chain: {0}")]
    MixedHierarchicalRelations(String),

    #[error("Path error: {0}")]
    PathError(String),

    #[error("Invalid regex: {0}")]
    InvalidRegex(#[from] regex::Error),

    #[error("Invalid glob pattern: {0}")]
    InvalidGlob(#[from] globset::Error),

    #[error("{0}")]
    ProcessError(String),

    #[error("Element move error: {0}")]
    ElementMoveError(String),

    #[error("Location not found: {0}")]
    LocationNotFound(String),

    #[error("Location already exists: {0}")]
    LocationAlreadyExists(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Validation failed with {} errors", .0.len())]
    ValidationError(Vec<Self>),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("JSON serialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Invalid contract_bindings format: {0}")]
    InvalidContractBindingFormat(String),

    #[error("Missing contract_bindings target: {0}")]
    MissingContractBindingTarget(String),

    #[error("Invalid contract_bindings target: {0}")]
    InvalidContractBindingTarget(String),

    #[error("Invalid contract_bindings scope: {0}")]
    InvalidContractBindingScope(String),

    #[error("Duplicate contract_bindings: {0}")]
    DuplicateContractBinding(String),

    #[error("Invalid contract_references: {0}")]
    InvalidContractReference(String),

    #[error("Cross-section duplicate: {0}")]
    CrossSectionDuplicate(String),

    #[error("Element not found: {0}")]
    ElementNotFound(String),

    #[error("Merge type mismatch: {0}")]
    MergeTypeMismatch(String),

    #[error("Merge cross-section duplicate: {0}")]
    MergeCrossSectionDuplicate(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Validation failure carrying structured diagnostics.
    ///
    /// Kept display-compatible with the legacy `ValidationError` shape so e2e
    /// fixtures that match on human-readable output remain stable. The
    /// diagnostics vector is the structured payload surfaced to JSON/MCP
    /// callers; `related_errors` mirrors the legacy flat error list.
    #[error("Validation failed with {} errors", .diagnostics.len())]
    ValidationDiagnostics {
        diagnostics: Vec<ValidationDiagnostic>,
        related_errors: Vec<Self>,
    },
}

// Concurrent cache waiters receive the same error category and diagnostics.
// OS and JSON errors own non-Clone internals; reconstruct their display payload.
impl Clone for ReqvireError {
    fn clone(&self) -> Self {
        match self {
            Self::IoError(error) => Self::IoError(
                error
                    .raw_os_error()
                    .map(std::io::Error::from_raw_os_error)
                    .unwrap_or_else(|| std::io::Error::new(error.kind(), error.to_string())),
            ),
            Self::GitCommandError(error) => Self::GitCommandError(error.clone()),
            Self::InvalidIdentifier(error) => Self::InvalidIdentifier(error.clone()),
            Self::RelationError(error) => Self::RelationError(error.clone()),
            Self::ElementError(error) => Self::ElementError(error.clone()),
            Self::CircularDependencyError(error) => Self::CircularDependencyError(error.clone()),
            Self::DuplicateElement(error) => Self::DuplicateElement(error.clone()),
            Self::DuplicateSubsection(error) => Self::DuplicateSubsection(error.clone()),
            Self::MissingElement(error) => Self::MissingElement(error.clone()),
            Self::AmbiguousElementSelection(error) => {
                Self::AmbiguousElementSelection(error.clone())
            }
            Self::ConflictingElementSelectors(error) => {
                Self::ConflictingElementSelectors(error.clone())
            }
            Self::MissingRelationTarget(error) => Self::MissingRelationTarget(error.clone()),
            Self::MissingParentRelation(error) => Self::MissingParentRelation(error.clone()),
            Self::InvalidRelationFormat(error) => Self::InvalidRelationFormat(error.clone()),
            Self::InvalidMetadataFormat(error) => Self::InvalidMetadataFormat(error.clone()),
            Self::InvalidMarkdownStructure(error) => Self::InvalidMarkdownStructure(error.clone()),
            Self::DuplicateRelation(error) => Self::DuplicateRelation(error.clone()),
            Self::UnsupportedRelationType(error) => Self::UnsupportedRelationType(error.clone()),
            Self::IncompatibleElementTypes(error) => Self::IncompatibleElementTypes(error.clone()),
            Self::MixedHierarchicalRelations(error) => {
                Self::MixedHierarchicalRelations(error.clone())
            }
            Self::PathError(error) => Self::PathError(error.clone()),
            Self::InvalidRegex(error) => Self::InvalidRegex(error.clone()),
            Self::InvalidGlob(error) => Self::InvalidGlob(error.clone()),
            Self::ProcessError(error) => Self::ProcessError(error.clone()),
            Self::ElementMoveError(error) => Self::ElementMoveError(error.clone()),
            Self::LocationNotFound(error) => Self::LocationNotFound(error.clone()),
            Self::LocationAlreadyExists(error) => Self::LocationAlreadyExists(error.clone()),
            Self::InvalidPath(error) => Self::InvalidPath(error.clone()),
            Self::ValidationError(error) => Self::ValidationError(error.clone()),
            Self::SerializationError(error) => Self::SerializationError(error.clone()),
            Self::SerdeJsonError(error) => Self::SerdeJsonError(
                <serde_json::Error as serde::de::Error>::custom(error.to_string()),
            ),
            Self::InvalidContractBindingFormat(error) => {
                Self::InvalidContractBindingFormat(error.clone())
            }
            Self::MissingContractBindingTarget(error) => {
                Self::MissingContractBindingTarget(error.clone())
            }
            Self::InvalidContractBindingTarget(error) => {
                Self::InvalidContractBindingTarget(error.clone())
            }
            Self::InvalidContractBindingScope(error) => {
                Self::InvalidContractBindingScope(error.clone())
            }
            Self::DuplicateContractBinding(error) => Self::DuplicateContractBinding(error.clone()),
            Self::InvalidContractReference(error) => Self::InvalidContractReference(error.clone()),
            Self::CrossSectionDuplicate(error) => Self::CrossSectionDuplicate(error.clone()),
            Self::ElementNotFound(error) => Self::ElementNotFound(error.clone()),
            Self::MergeTypeMismatch(error) => Self::MergeTypeMismatch(error.clone()),
            Self::MergeCrossSectionDuplicate(error) => {
                Self::MergeCrossSectionDuplicate(error.clone())
            }
            Self::InvalidOperation(error) => Self::InvalidOperation(error.clone()),
            Self::ValidationDiagnostics {
                diagnostics,
                related_errors,
            } => Self::ValidationDiagnostics {
                diagnostics: diagnostics.clone(),
                related_errors: related_errors.clone(),
            },
        }
    }
}

impl ReqvireError {
    /// Returns a stable, machine-readable diagnostic code for this error variant.
    pub const fn diagnostic_code(&self) -> &'static str {
        match self {
            Self::IoError(_) => "io_error",
            Self::GitCommandError(_) => "git_command_error",
            Self::InvalidIdentifier(_) => "invalid_identifier",
            Self::RelationError(_) => "relation_error",
            Self::ElementError(_) => "element_error",
            Self::CircularDependencyError(_) => "circular_dependency",
            Self::DuplicateElement(_) => "duplicate_element",
            Self::DuplicateSubsection(_) => "duplicate_subsection",
            Self::MissingElement(_) => "missing_element",
            Self::AmbiguousElementSelection(_) => "ambiguous_element_selection",
            Self::ConflictingElementSelectors(_) => "conflicting_element_selectors",
            Self::MissingRelationTarget(_) => "missing_relation_target",
            Self::MissingParentRelation(_) => "missing_parent_relation",
            Self::InvalidRelationFormat(_) => "invalid_relation_format",
            Self::InvalidMetadataFormat(_) => "invalid_metadata_format",
            Self::InvalidMarkdownStructure(_) => "invalid_markdown_structure",
            Self::DuplicateRelation(_) => "duplicate_relation",
            Self::UnsupportedRelationType(_) => "unsupported_relation_type",
            Self::IncompatibleElementTypes(_) => "incompatible_element_types",
            Self::MixedHierarchicalRelations(_) => "mixed_hierarchical_relations",
            Self::PathError(_) => "path_error",
            Self::InvalidRegex(_) => "invalid_regex",
            Self::InvalidGlob(_) => "invalid_glob",
            Self::ProcessError(_) => "process_error",
            Self::ElementMoveError(_) => "element_move_error",
            Self::LocationNotFound(_) => "location_not_found",
            Self::LocationAlreadyExists(_) => "location_already_exists",
            Self::InvalidPath(_) => "invalid_path",
            Self::ValidationError(_) => "validation_failed",
            Self::SerializationError(_) => "serialization_error",
            Self::SerdeJsonError(_) => "serialization_error",
            Self::InvalidContractBindingFormat(_) => "invalid_contract_binding_format",
            Self::MissingContractBindingTarget(_) => "missing_contract_binding_target",
            Self::InvalidContractBindingTarget(_) => "invalid_contract_binding_target",
            Self::InvalidContractBindingScope(_) => "invalid_contract_binding_scope",
            Self::DuplicateContractBinding(_) => "duplicate_contract_binding",
            Self::InvalidContractReference(_) => "invalid_contract_reference",
            Self::CrossSectionDuplicate(_) => "cross_section_duplicate",
            Self::ElementNotFound(_) => "element_not_found",
            Self::MergeTypeMismatch(_) => "merge_type_mismatch",
            Self::MergeCrossSectionDuplicate(_) => "merge_cross_section_duplicate",
            Self::InvalidOperation(_) => "invalid_operation",
            Self::ValidationDiagnostics { .. } => "validation_failed",
        }
    }

    /// Builds a [`ValidationDiagnostic`] from this error with a stable code and
    /// the human-readable message from `Display`.
    pub fn to_diagnostic(&self) -> ValidationDiagnostic {
        ValidationDiagnostic {
            code: self.diagnostic_code(),
            message: self.to_string(),
            context: None,
        }
    }

    /// Constructs a `ValidationDiagnostics` error from a flat list of validation
    /// errors, deriving structured diagnostics while preserving the legacy
    /// `related_errors` list for backward-compatible text rendering.
    pub fn validation_diagnostics(errors: Vec<Self>) -> Self {
        let diagnostics = Self::errors_to_diagnostics(&errors);
        Self::ValidationDiagnostics {
            diagnostics,
            related_errors: errors,
        }
    }

    /// Flattens a list of validation errors into structured diagnostics.
    /// Nested `ValidationError` / `ValidationDiagnostics` wrappers are expanded
    /// so each leaf error becomes one diagnostic.
    pub fn errors_to_diagnostics(errors: &[Self]) -> Vec<ValidationDiagnostic> {
        let mut out = Vec::new();
        for err in errors {
            match err {
                Self::ValidationError(inner) => {
                    out.extend(Self::errors_to_diagnostics(inner));
                }
                Self::ValidationDiagnostics {
                    diagnostics,
                    related_errors,
                } => {
                    out.extend(diagnostics.iter().cloned());
                    out.extend(Self::errors_to_diagnostics(related_errors));
                }
                other => out.push(other.to_diagnostic()),
            }
        }
        out
    }

    /// Returns the structured diagnostics for this error if it carries them.
    pub fn diagnostics(&self) -> Vec<ValidationDiagnostic> {
        match self {
            Self::ValidationError(errors) => Self::errors_to_diagnostics(errors),
            Self::ValidationDiagnostics { diagnostics, .. } => diagnostics.clone(),
            _ => Vec::new(),
        }
    }
}
