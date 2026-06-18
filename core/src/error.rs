use thiserror::Error;

/// Custom error types for the Reqvire application
#[derive(Error, Debug)]
pub enum ReqvireError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Git Command error: {0}")]
    GitCommandError(String),

    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),

    #[allow(dead_code)]
    #[error("Relation error: {0}")]
    RelationError(String),

    #[allow(dead_code)]
    #[error("Element error: {0}")]
    ElementError(String),

    #[allow(dead_code)]
    #[error("Circular dependency error: {0}")]
    CircularDependencyError(String),

    #[allow(dead_code)]
    #[error("Duplicate element: {0}")]
    DuplicateElement(String),

    #[allow(dead_code)]
    #[error("Duplicate subsection: {0}")]
    DuplicateSubsection(String),

    #[allow(dead_code)]
    #[error("Missing element: {0}")]
    MissingElement(String),

    #[allow(dead_code)]
    #[error("Missing relation target: {0}")]
    MissingRelationTarget(String),

    #[allow(dead_code)]
    #[error("Missing parent relation: {0}")]
    MissingParentRelation(String),

    #[allow(dead_code)]
    #[error("Invalid relation format: {0}")]
    InvalidRelationFormat(String),

    #[allow(dead_code)]
    #[error("Invalid metadata format: {0}")]
    InvalidMetadataFormat(String),

    #[allow(dead_code)]
    #[error("Invalid markdown structure: {0}")]
    InvalidMarkdownStructure(String),

    #[allow(dead_code)]
    #[error("Duplicate relation: {0}")]
    DuplicateRelation(String),

    #[allow(dead_code)]
    #[error("Unsupported relation type: {0}")]
    UnsupportedRelationType(String),

    #[allow(dead_code)]
    #[error("Incompatible element types for relation: {0}")]
    IncompatibleElementTypes(String),

    #[allow(dead_code)]
    #[error("Mixed hierarchical relations in chain: {0}")]
    MixedHierarchicalRelations(String),

    #[error("Path error: {0}")]
    PathError(String),

    #[error("Invalid regex: {0}")]
    InvalidRegex(String),

    #[error("Invalid glob pattern: {0}")]
    InvalidGlob(String),

    #[error("Initialization error: {0}")]
    InitializationError(String),

    #[error("Linting error: {0}")]
    LintError(String),

    #[error("{0}")]
    ProcessError(String),

    #[allow(dead_code)]
    #[error("Element move error: {0}")]
    ElementMoveError(String),

    #[allow(dead_code)]
    #[error("Location not found: {0}")]
    LocationNotFound(String),

    #[allow(dead_code)]
    #[error("Location already exists: {0}")]
    LocationAlreadyExists(String),

    #[allow(dead_code)]
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[allow(dead_code)]
    #[error("Validation failed with {} errors", .0.len())]
    ValidationError(Vec<ReqvireError>),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[allow(dead_code)]
    #[error("Invalid reused_contract_context format: {0}")]
    InvalidReusedContractContextFormat(String),

    #[allow(dead_code)]
    #[error("Missing reused_contract_context target: {0}")]
    MissingReusedContractContextTarget(String),

    #[allow(dead_code)]
    #[error("Missing reused_contract_context file: {0}")]
    MissingReusedContractContextFile(String),

    #[allow(dead_code)]
    #[error("Invalid reused_contract_context target: {0}")]
    InvalidReusedContractContextTarget(String),

    #[allow(dead_code)]
    #[error("Invalid reused_contract_context scope: {0}")]
    InvalidReusedContractContextScope(String),

    #[allow(dead_code)]
    #[error("Duplicate reused_contract_context: {0}")]
    DuplicateReusedContractContext(String),

    #[allow(dead_code)]
    #[error("Cross-section duplicate: {0}")]
    CrossSectionDuplicate(String),

    #[allow(dead_code)]
    #[error("Element not found: {0}")]
    ElementNotFound(String),

    #[allow(dead_code)]
    #[error("Merge type mismatch: {0}")]
    MergeTypeMismatch(String),

    #[allow(dead_code)]
    #[error("Merge cross-section duplicate: {0}")]
    MergeCrossSectionDuplicate(String),

    #[allow(dead_code)]
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}
