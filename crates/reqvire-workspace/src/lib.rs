//! Shared repository composition, Git plumbing and workspace orchestration.
pub mod baseline;
pub mod branches;
pub mod git;
pub mod lease;
pub mod legacy;
pub mod locks;
pub mod manifest;
pub mod process;
pub mod publication;
pub mod scope;
pub mod service;
pub use manifest::{WorkspaceComposition, WorkspaceProject};
pub use service::{Operation, Report, Workspace};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ProcessError(String),
    #[error("{0}")]
    GitCommandError(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("{message}")]
    Command {
        message: String,
        outcome_unknown: bool,
    },
    #[error("{source}")]
    Preparation {
        source: Box<Self>,
        outcomes: Vec<service::Outcome>,
    },
}
impl Error {
    pub fn outcomes(&self) -> &[service::Outcome] {
        match self {
            Self::Preparation { outcomes, .. } => outcomes,
            _ => &[],
        }
    }
    pub fn outcome_unknown(&self) -> bool {
        match self {
            Self::Command {
                outcome_unknown, ..
            } => *outcome_unknown,
            Self::Preparation { source, .. } => source.outcome_unknown(),
            _ => false,
        }
    }
}
impl From<quick_xml::Error> for Error {
    fn from(e: quick_xml::Error) -> Self {
        Self::ProcessError(e.to_string())
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::ProcessError(e.to_string())
    }
}
