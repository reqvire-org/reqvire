use thiserror::Error;

/// Custom error types for the ReqFlow application
#[derive(Error, Debug)]
pub enum ReqFlowError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),

    #[allow(dead_code)]
    #[error("Relation error: {0}")]
    RelationError(String),

    #[allow(dead_code)]
    #[error("Element error: {0}")]
    ElementError(String),

    #[allow(dead_code)]
    #[error("Validation error: {0}")]
    ValidationError(String),

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
    #[error("Invalid relation format: {0}")]
    InvalidRelationFormat(String),

    #[allow(dead_code)]
    #[error("Invalid markdown structure: {0}")]
    InvalidMarkdownStructure(String),

    #[error("Path error: {0}")]
    PathError(String),
}