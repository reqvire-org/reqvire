use crate::serve;
use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use globset::GlobSet;
use log::info;
use reqvire::change_impact;
use reqvire::crud;
use reqvire::diagrams;
use reqvire::diff::{render_crud_json, render_crud_result};
use reqvire::element::Element;
use reqvire::error::ReqvireError;
use reqvire::export;
use reqvire::format::{format_files, render_diff, render_diff_json};
use reqvire::git_commands;
use reqvire::graph_registry::Page;
use reqvire::lint;
use reqvire::report_collect;
use reqvire::report_coverage;
use reqvire::report_model;
use reqvire::report_resources;
use reqvire::report_submodels;
use reqvire::verification_trace;
use reqvire::GraphRegistry;
use reqvire::ModelManager;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Reqvire requirements & traceability management tool",
    long_about = None,
    name = "reqvire"
)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Export model to browsable HTML documentation with complete traceability
    Export {
        /// Output directory for HTML files (defaults to temporary directory if not specified)
        #[clap(long, help_heading = "EXPORT OPTIONS")]
        output: Option<String>,
    },

    /// Serve model as browsable HTML documentation via HTTP server
    #[clap(
        override_help = "Serve model as browsable HTML documentation via HTTP server\n\nSERVE OPTIONS:\n      --host <HOST>          Bind address (default: localhost)\n      --port <PORT>          Server port (default: 8080)"
    )]
    Serve {
        /// Bind address
        #[clap(long, default_value = "localhost", help_heading = "SERVE OPTIONS")]
        host: String,

        /// Server port
        #[clap(long, default_value = "8080", help_heading = "SERVE OPTIONS")]
        port: u16,
    },

    /// Format and normalize requirements files. By default, shows preview without applying changes
    #[clap(
        override_help = "Format and normalize requirements files. By default, shows preview without applying changes\n\nFORMAT OPTIONS:\n      --fix                   Apply formatting changes to files\n      --json                  Output results in JSON format\n      --output <FILE>         Save JSON output to file (requires --json)\n      --with-full-relations   Include all relations (user-created and auto-generated)"
    )]
    Format {
        /// Apply formatting changes to files
        #[clap(long, help_heading = "FORMAT OPTIONS")]
        fix: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "FORMAT OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "FORMAT OPTIONS")]
        output: Option<String>,

        /// Include all relations (user-created and auto-generated inverse relations)
        #[clap(long, help_heading = "FORMAT OPTIONS")]
        with_full_relations: bool,
    },

    /// Validate model
    #[clap(
        override_help = "Validate model\n\nVALIDATION OPTIONS:\n      --json              Output results in JSON format\n      --output <FILE>     Save JSON output to file (requires --json)"
    )]
    Validate {
        /// Output results in JSON format
        #[clap(long, help_heading = "VALIDATION OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "VALIDATION OPTIONS")]
        output: Option<String>,
    },

    /// Search and filter model elements with comprehensive filtering options
    #[clap(
        override_help = "Search and filter model elements with comprehensive filtering options\n\nSEARCH OPTIONS:\n      --json                            Output results in JSON format\n      --output <FILE>                   Save JSON output to file (requires --json)\n      --short                           Output abbreviated format (one-line per element)\n      --filter-file <GLOB>              Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`\n      --filter-name <REGEX>             Only include elements whose name matches this regular expression\n      --filter-type <TYPE>              Only include elements of the given type. Valid types: user-requirement, requirement, test-verification, analysis-verification, inspection-verification, demonstration-verification, constraint, behavior, specification. For custom types use: other-TYPENAME\n      --filter-content <REGEX>          Only include elements whose content matches this regular expression\n      --filter-page-content <REGEX>     Only include elements whose parent file page content matches this regular expression\n      --have-relations <LIST>           Only include elements that have ALL specified relations (comma-separated)\n      --not-have-relations <LIST>       Only include elements that do NOT have ALL specified relations (comma-separated)"
    )]
    Search {
        /// Output results in JSON format
        #[clap(long, help_heading = "SEARCH OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "SEARCH OPTIONS")]
        output: Option<String>,

        /// Output abbreviated format (one-line per element in text, omit fields in JSON)
        #[clap(long, help_heading = "SEARCH OPTIONS")]
        short: bool,

        /// Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`
        #[clap(long, value_name = "GLOB", help_heading = "SEARCH OPTIONS")]
        filter_file: Option<String>,

        /// Only include elements whose name matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "SEARCH OPTIONS")]
        filter_name: Option<String>,

        /// Only include elements of the given type(s). Supports comma-separated list. Valid: user-requirement, requirement, test-verification, analysis-verification, inspection-verification, demonstration-verification, constraint, behavior, specification. Custom: other-TYPENAME
        #[clap(long, value_name = "TYPE[,TYPE...]", help_heading = "SEARCH OPTIONS")]
        filter_type: Option<String>,

        /// Only include elements whose content matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "SEARCH OPTIONS")]
        filter_content: Option<String>,

        /// Only include elements whose parent file page content matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "SEARCH OPTIONS")]
        filter_page_content: Option<String>,

        /// Only include elements that have ALL specified relations (comma-separated, e.g., "verifiedBy,satisfiedBy")
        #[clap(long, value_name = "LIST", help_heading = "SEARCH OPTIONS")]
        have_relations: Option<String>,

        /// Only include elements that do NOT have ALL specified relations (comma-separated, e.g., "verifiedBy")
        #[clap(long, value_name = "LIST", help_heading = "SEARCH OPTIONS")]
        not_have_relations: Option<String>,

        /// Only include elements that have attachments
        #[clap(long, help_heading = "SEARCH OPTIONS")]
        has_attachments: bool,

        /// Only include elements with attachments matching this glob pattern (e.g., "*.pdf", "docs/**/*")
        #[clap(long, value_name = "GLOB", help_heading = "SEARCH OPTIONS")]
        filter_attachment: Option<String>,
    },

    /// Analyze change impact and provide report
    #[clap(
        override_help = "Analyze change impact and provide report\n\nCHANGE IMPACT OPTIONS:\n      --git-commit <GIT_COMMIT>  Git commit hash to use when comparing models [default: HEAD]\n      --json                     Output results in JSON format\n      --output <FILE>            Save JSON output to file (requires --json)"
    )]
    ChangeImpact {
        /// Git commit hash to use when comparing models
        #[clap(long, default_value = "HEAD", help_heading = "CHANGE IMPACT OPTIONS")]
        git_commit: String,

        /// Output results in JSON format
        #[clap(long, help_heading = "CHANGE IMPACT OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "CHANGE IMPACT OPTIONS")]
        output: Option<String>,
    },

    /// Generate verification traces showing upward paths from verifications to root requirements
    #[clap(
        override_help = "Generate verification traces showing upward paths from verifications to root requirements\n\nTRACES OPTIONS:\n      --json                      Output results in JSON format\n      --output <FILE>             Save JSON output to file (requires --json)\n      --from-folder <PATH>        Generate links relative to this folder path\n      --links-with-blobs          Use GitHub blob URLs in diagram links instead of relative paths\n      --filter-id <ID>            Only include verification with this specific identifier\n      --filter-name <REGEX>       Only include verifications whose name matches this regular expression\n      --filter-type <TYPE>        Only include verifications of the given type. Valid types: test-verification, analysis-verification, inspection-verification, demonstration-verification"
    )]
    Traces {
        /// Output results in JSON format
        #[clap(long, help_heading = "TRACES OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "TRACES OPTIONS")]
        output: Option<String>,

        /// Relative path to folder where output will be saved (for generating relative links in Mermaid diagrams)
        #[clap(long, value_name = "PATH", help_heading = "TRACES OPTIONS")]
        from_folder: Option<String>,

        /// Use GitHub blob URLs in diagram links instead of relative paths
        #[clap(long, help_heading = "TRACES OPTIONS")]
        links_with_blobs: bool,

        /// Only include verification with this specific identifier
        #[clap(long, value_name = "ID", help_heading = "TRACES OPTIONS")]
        filter_id: Option<String>,

        /// Only include verifications whose name matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "TRACES OPTIONS")]
        filter_name: Option<String>,

        /// Only include verifications of the given type. Valid: test-verification, analysis-verification, inspection-verification, demonstration-verification
        #[clap(long, value_name = "TYPE", help_heading = "TRACES OPTIONS")]
        filter_type: Option<String>,
    },

    /// Generate verification and implementation coverage report
    #[clap(
        override_help = "Generate verification and implementation coverage report\n\nCOVERAGE OPTIONS:\n      --json                      Output results in JSON format\n      --output <FILE>             Save JSON output to file (requires --json)"
    )]
    Coverage {
        /// Output results in JSON format
        #[clap(long, help_heading = "COVERAGE OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "COVERAGE OPTIONS")]
        output: Option<String>,
    },

    /// Generate model-centric structure with nested relations
    ///
    /// By default, shows root requirements (no hierarchical parent).
    /// Use --from <NAME> to start from specific element.
    /// Use --reverse for leaf-to-root traversal.
    ///
    /// Output formats:
    /// - JSON: Nested structure with element details in relations
    /// - Markdown: Mermaid diagrams with all nested relationships
    #[clap(
        override_help = "Generate model-centric structure with nested relations\n\nBy default, shows root requirements (no hierarchical parent).\nUse --from <NAME> to start from specific element.\nUse --reverse for leaf-to-root traversal.\n\nOutput formats:\n  - JSON: Nested structure with element details in relations\n  - Markdown: Mermaid diagrams with all nested relationships\n\nMODEL OPTIONS:\n      --from <NAME>               Start from specific element by name\n      --reverse                   Traverse from leaves to roots (follow backward relations)\n      --filter-type <TYPE>        Filter starting elements by type (comma-separated). Valid types: user-requirement, requirement, test-verification, analysis-verification, inspection-verification, demonstration-verification, constraint, behavior, specification. For custom types use: other-TYPENAME\n      --json                      Output results in JSON format (nested structure)\n      --output <FILE>             Save JSON output to file (requires --json)"
    )]
    Model {
        /// Start from specific element by name
        #[clap(long, value_name = "NAME", help_heading = "MODEL OPTIONS")]
        from: Option<String>,

        /// Traverse from leaves to roots (follow backward relations)
        #[clap(long, help_heading = "MODEL OPTIONS")]
        reverse: bool,

        /// Filter starting elements by type (comma-separated). Valid: user-requirement, requirement, test-verification, analysis-verification, inspection-verification, demonstration-verification, constraint, behavior, specification. Custom: other-TYPENAME
        #[clap(long, value_name = "TYPE", help_heading = "MODEL OPTIONS")]
        filter_type: Option<String>,

        /// Output results in JSON format (nested structure)
        #[clap(long, help_heading = "MODEL OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "MODEL OPTIONS")]
        output: Option<String>,
    },

    /// Analyze model quality and detect issues in requirements relations
    #[clap(
        override_help = "Analyze model quality and detect issues in requirements relations\n\nLINT OPTIONS:\n      --fixable                   Show only auto-fixable issues\n      --auditable                 Show only issues requiring manual review\n      --fix                       Apply automatic fixes for auto-fixable issues\n      --json                      Output results in JSON format\n      --output <FILE>             Save JSON output to file (requires --json)"
    )]
    Lint {
        /// Show only auto-fixable issues
        #[clap(long, help_heading = "LINT OPTIONS", conflicts_with = "auditable")]
        fixable: bool,

        /// Show only issues requiring manual review
        #[clap(long, help_heading = "LINT OPTIONS", conflicts_with = "fixable")]
        auditable: bool,

        /// Apply automatic fixes for auto-fixable issues
        #[clap(long, help_heading = "LINT OPTIONS")]
        fix: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "LINT OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "LINT OPTIONS")]
        output: Option<String>,
    },

    /// Add new element to model from Markdown definition
    #[clap(
        override_help = "Add new element to model from Markdown definition\n\nADD OPTIONS:\n       <FILE>                    Target file path (relative to git repository root)\n      --content <MARKDOWN>       Element markdown content (alternative to stdin)\n      --override                 Replace existing element with same name\n      --dry-run                  Preview changes without applying\n      --json                     Output results in JSON format\n      --output <FILE>            Save JSON output to file (requires --json)\n\nUSAGE:\n    reqvire add <file>                          # reads from stdin\n    reqvire add <file> --content \"### Name...\"   # reads from argument"
    )]
    Add {
        /// Target file path (relative to git repository root)
        file: String,

        /// Element markdown content (alternative to stdin)
        #[clap(long, value_name = "MARKDOWN", help_heading = "ADD OPTIONS")]
        content: Option<String>,

        /// Replace existing element with same name
        #[clap(long = "override", help_heading = "ADD OPTIONS")]
        override_existing: bool,

        /// Preview changes without applying
        #[clap(long, help_heading = "ADD OPTIONS")]
        dry_run: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "ADD OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "ADD OPTIONS")]
        output: Option<String>,
    },

    /// Remove element from model
    #[clap(
        override_help = "Remove element from model\n\nRM OPTIONS:\n       <ELEMENT_NAME>           Element name\n      --dry-run                 Preview changes without applying\n      --json                    Output results in JSON format\n      --output <FILE>           Save JSON output to file (requires --json)\n\nUSAGE:\n    reqvire rm <element-name>"
    )]
    Rm {
        /// Element name
        element_name: String,

        /// Preview changes without applying
        #[clap(long, help_heading = "RM OPTIONS")]
        dry_run: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "RM OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "RM OPTIONS")]
        output: Option<String>,
    },

    /// Move element to different location
    #[clap(
        override_help = "Move element to different location\n\nMV OPTIONS:\n       <ELEMENT_NAME>           Element name\n       <FILE>                   Target file path (relative to git repository root)\n      --dry-run                 Preview changes without applying\n      --json                    Output results in JSON format\n      --output <FILE>           Save JSON output to file (requires --json)\n\nUSAGE:\n    reqvire mv <element-name> <file>"
    )]
    Mv {
        /// Element name
        element_name: String,

        /// Target file path (relative to git repository root)
        file: String,

        /// Preview changes without applying
        #[clap(long, help_heading = "MV OPTIONS")]
        dry_run: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "MV OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "MV OPTIONS")]
        output: Option<String>,
    },

    /// Rename element
    #[clap(
        override_help = "Rename element\n\nRENAME OPTIONS:\n       <ELEMENT_NAME>           Current element name\n       <NEW_NAME>               New element name\n      --dry-run                 Preview changes without applying\n      --json                    Output results in JSON format\n      --output <FILE>           Save JSON output to file (requires --json)\n\nUSAGE:\n    reqvire rename <element-name> <new-name>"
    )]
    Rename {
        /// Current element name
        element_name: String,

        /// New element name
        new_name: String,

        /// Preview changes without applying
        #[clap(long, help_heading = "RENAME OPTIONS")]
        dry_run: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "RENAME OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "RENAME OPTIONS")]
        output: Option<String>,
    },

    /// Merge multiple elements into target element
    #[clap(
        override_help = "Merge multiple elements into target element\n\nMERGE OPTIONS:\n       <TARGET>                 Target element name (receives merged content)\n       <SOURCES>...             One or more source element names to merge\n      --dry-run                 Preview changes without applying\n      --json                    Output results in JSON format\n      --output <FILE>           Save JSON output to file (requires --json)\n\nMERGE BEHAVIOR:\n    - Source main content is appended to target's Details section\n    - Source Details sections become 'Merged Details (source name)' subsections\n    - Relations and attachments are merged with deduplication\n    - Source elements are deleted after successful merge\n    - Relations pointing to sources are redirected to target\n\nTYPE COMPATIBILITY:\n    - Requirements can merge into requirements (of any subtype)\n    - Verifications can merge into verifications (of any subtype)\n    - Refinements can merge into refinements (of any subtype)\n    - Other types can only merge into other types\n\nUSAGE:\n    reqvire merge \"Target Req\" \"Source Req 1\" \"Source Req 2\"\n    reqvire merge \"Combined Requirement\" \"Feature A\" \"Feature B\" --dry-run"
    )]
    Merge {
        /// Target element name (receives merged content)
        target: String,

        /// One or more source element names to merge into target
        #[clap(required = true, num_args = 1..)]
        sources: Vec<String>,

        /// Preview changes without applying
        #[clap(long, help_heading = "MERGE OPTIONS")]
        dry_run: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "MERGE OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "MERGE OPTIONS")]
        output: Option<String>,
    },

    /// Move entire specification file with all its elements
    #[clap(
        name = "mv-file",
        override_help = "Move entire specification file with all its elements\n\nMV-FILE OPTIONS:\n       <SOURCE_FILE>            Source file path (relative to current working directory)\n       <TARGET_FILE>            Target file path (relative to current working directory)\n      --squash                  Move all elements to target file's first section (if target exists)\n      --dry-run                 Preview changes without applying\n      --json                    Output results in JSON format\n      --output <FILE>           Save JSON output to file (requires --json)\n\nUSAGE:\n    reqvire mv-file <source-file> <target-file>\n    reqvire mv-file <source-file> <target-file> --squash"
    )]
    MvFile {
        /// Source file path (relative to current working directory)
        source_file: String,

        /// Target file path (relative to current working directory)
        target_file: String,

        /// Move all elements to target file's first section (if target exists)
        #[clap(long, help_heading = "MV-FILE OPTIONS")]
        squash: bool,

        /// Preview changes without applying
        #[clap(long, help_heading = "MV-FILE OPTIONS")]
        dry_run: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "MV-FILE OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "MV-FILE OPTIONS")]
        output: Option<String>,
    },

    /// Add relation or attachment between elements
    #[clap(
        name = "link",
        override_help = "Add relation or attachment between elements\n\nLINK OPTIONS:\n       <SOURCE>                 Source element name\n       <RELATION_TYPE or attaching>  Relation type OR 'attaching' keyword for attachments\n       <TARGET>                 Target: element name, internal path, or external URL\n      --dry-run                 Preview changes without applying\n\nRELATION TYPES:\n    derivedFrom  - Source is derived from target (parent traceability)\n    derive       - Source derives target (child traceability)\n    satisfiedBy  - Source requirement is satisfied by target implementation\n    satisfy      - Source implementation satisfies target requirement\n    verifiedBy   - Source requirement is verified by target verification\n    verify       - Source verification verifies target requirement\n    trace        - Generic traceability link\n\nATTACHING:\n    Use 'attaching' keyword to attach file or Refinement element to source\n\nTARGET TYPES:\n    For relations: element name, internal file path, or external URL (http/https)\n    For attaching: internal file path or Refinement element name\n\nUSAGE:\n    reqvire link \"Feature Requirement\" derivedFrom \"System Requirement\"\n    reqvire link \"Test Verification\" verify \"Feature Requirement\"\n    reqvire link \"Requirement\" satisfiedBy src/impl.rs\n    reqvire link \"Requirement\" trace https://example.com/spec.html\n    reqvire link \"System Requirement\" attaching docs/SLO.pdf\n    reqvire link \"System Requirement\" attaching \"My Constraint Element\""
    )]
    Link {
        /// Source element name
        source: String,

        /// Relation type OR 'attaching'.
        /// Relations: derivedFrom, derive, satisfiedBy, satisfy, verifiedBy, verify, trace.
        /// Use 'attaching' to attach files or refinement elements (constraint, behavior, specification)
        relation_type: String,

        /// Target: element name, internal path, or external URL (for relations); file path or element name (for attaching)
        target: String,

        /// Preview changes without applying
        #[clap(long, help_heading = "LINK OPTIONS")]
        dry_run: bool,
    },

    /// Remove relation or attachment between elements (auto-detects type)
    #[clap(
        name = "unlink",
        override_help = "Remove relation or attachment between elements (auto-detects type)\n\nUNLINK OPTIONS:\n       <SOURCE>                 Source element name\n       <TARGET>                 Target element name OR file path\n      --dry-run                 Preview changes without applying\n\nAUTO-DETECTION:\n    Searches relations first, then attachments.\n    Only one relation per source-target pair is allowed.\n\nUSAGE:\n    reqvire unlink \"Feature Requirement\" \"System Requirement\"\n    reqvire unlink \"System Requirement\" docs/SLO.pdf\n    reqvire unlink \"System Requirement\" \"My Constraint Element\""
    )]
    Unlink {
        /// Source element name
        source: String,

        /// Target element name OR file path
        target: String,

        /// Preview changes without applying
        #[clap(long, help_heading = "UNLINK OPTIONS")]
        dry_run: bool,
    },

    /// Replace an existing relation target with a new target in one operation
    #[clap(
        name = "relink",
        override_help = "Replace an existing relation target with a new target in one operation\n\nRELINK OPTIONS:\n       <SOURCE>                 Source element name\n       <RELATION_TYPE>          Relation type to preserve\n       <FROM_TARGET>            Existing target to replace\n       <TO_TARGET>              New target\n      --dry-run                 Preview changes without applying\n\nUSAGE:\n    reqvire relink \"Child Requirement\" derivedFrom \"Old Parent\" \"New Parent\""
    )]
    Relink {
        /// Source element name
        source: String,

        /// Relation type to preserve
        relation_type: String,

        /// Existing target to replace
        from_target: String,

        /// New target
        to_target: String,

        /// Preview changes without applying
        #[clap(long, help_heading = "RELINK OPTIONS")]
        dry_run: bool,
    },

    /// Move/rename asset file and update all references (Attachments and Relations)
    #[clap(
        name = "mv-asset",
        override_help = "Move/rename asset file and update all references\n\nMV-ASSET OPTIONS:\n       <OLD_PATH>               Current file path\n       <NEW_PATH>               New file path\n      --dry-run                 Preview changes without applying\n\nUSAGE:\n    reqvire mv-asset docs/old.pdf docs/new.pdf"
    )]
    MvAsset {
        /// Current file path
        old_path: String,

        /// New file path
        new_path: String,

        /// Preview changes without applying
        #[clap(long, help_heading = "MV-ASSET OPTIONS")]
        dry_run: bool,
    },

    /// Remove asset file and remove all references (Attachments and Relations)
    #[clap(
        name = "rm-asset",
        override_help = "Remove asset file and remove all references\n\nRM-ASSET OPTIONS:\n       <FILE_PATH>              Path to file to remove\n      --dry-run                 Preview changes without applying\n\nUSAGE:\n    reqvire rm-asset docs/obsolete.pdf"
    )]
    RmAsset {
        /// Path to file to remove
        file_path: String,

        /// Preview changes without applying
        #[clap(long, help_heading = "RM-ASSET OPTIONS")]
        dry_run: bool,
    },

    /// Generate containment view showing folder/file/element hierarchy
    #[clap(
        override_help = "Generate containment view showing folder/file/element hierarchy\n\nCONTAINMENT OPTIONS:\n      --json              Output results in JSON format\n      --output <FILE>     Save JSON output to file (requires --json)\n      --short             Show only root elements (without hierarchical parents)"
    )]
    Containment {
        /// Output results in JSON format
        #[clap(long, help_heading = "CONTAINMENT OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "CONTAINMENT OPTIONS")]
        output: Option<String>,

        /// Show only root elements (without hierarchical parents in same file)
        #[clap(long, help_heading = "CONTAINMENT OPTIONS")]
        short: bool,
    },

    /// Generate resources report showing files referenced by the model
    #[clap(
        override_help = "Generate resources report showing files referenced by the model\n\nRESOURCES OPTIONS:\n      --json              Output results in JSON format\n      --output <FILE>     Save JSON output to file (requires --json)"
    )]
    Resources {
        /// Output results in JSON format
        #[clap(long, help_heading = "RESOURCES OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "RESOURCES OPTIONS")]
        output: Option<String>,
    },

    /// Analyze independent requirement submodels and cross-submodel couplings
    #[clap(
        override_help = "Analyze independent requirement submodels and cross-submodel couplings\n\nSUBMODELS OPTIONS:\n      --from <NAME>      Scope report to a specific requirement subtree by name\n      --json              Output results in JSON output format\n      --output <FILE>     Save JSON output to file (requires --json)"
    )]
    Submodels {
        /// Scope report to a specific requirement subtree by name
        #[clap(long, value_name = "NAME", help_heading = "SUBMODELS OPTIONS")]
        from: Option<String>,

        /// Output results in JSON format
        #[clap(long, help_heading = "SUBMODELS OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "SUBMODELS OPTIONS")]
        output: Option<String>,
    },

    /// Collect content from requirement chain
    #[clap(
        override_help = "Collect content from requirement chain\n\nCOLLECT OPTIONS:\n      <ELEMENT_NAME>        Name of the requirement element to collect from\n      --direction <DIR>     Traversal direction: UPSTREAM (default) or DOWNSTREAM\n      --json                Output results in JSON format\n      --output <FILE>       Save JSON output to file (requires --json)"
    )]
    Collect {
        /// Name of the requirement element to collect from
        element_name: String,

        /// Traversal direction: UPSTREAM (ancestors) or DOWNSTREAM (descendants)
        #[clap(
            long,
            value_name = "DIRECTION",
            default_value = "UPSTREAM",
            help_heading = "COLLECT OPTIONS"
        )]
        direction: String,

        /// Output results in JSON format
        #[clap(long, help_heading = "COLLECT OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "COLLECT OPTIONS")]
        output: Option<String>,
    },

    /// Interactive shell for GraphRegistry operations (undocumented)
    #[clap(hide = true)]
    Shell,

    /// Single output stream for all pages, sections, and requirements (undocumented)
    #[clap(hide = true)]
    Sout,
}

impl Args {
    pub fn parse_args() -> Self {
        // Check if help was requested before parsing
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 && (args[1] == "--help" || args[1] == "-h" || args[1] == "help") {
            let cmd = Args::command();
            print_custom_help(&cmd);
            std::process::exit(0);
        }
        Args::parse()
    }

    pub fn print_help() {
        let cmd = Args::command();
        print_custom_help(&cmd);
    }
}

fn print_custom_help(cmd: &clap::Command) {
    // Print basic info
    if let Some(about) = cmd.get_about() {
        println!("{}", about);
    }
    println!();

    println!(
        "Usage: {} [OPTIONS] <COMMAND> [COMMAND OPTIONS]",
        cmd.get_name()
    );
    println!();

    // Print commands
    println!("Commands:");
    for subcommand in cmd.get_subcommands() {
        // Skip hidden commands
        if subcommand.is_hide_set() {
            continue;
        }

        let name = subcommand.get_name();
        let about = subcommand
            .get_about()
            .map(|s| s.to_string())
            .unwrap_or_default();

        // Check if this command has subcommands (like verifications)
        if subcommand.has_subcommands() {
            println!("  {:<17} {}", name, about);
            // List nested subcommands indented
            for nested in subcommand.get_subcommands() {
                let nested_name = format!("{} {}", name, nested.get_name());
                let nested_about = nested
                    .get_about()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                println!("    {:<15} {}", nested_name, nested_about);
            }
        } else {
            println!("  {:<17} {}", name, about);
        }
    }
    println!("  help               Print this message or the help of the given subcommand(s)");
    println!();

    // Print global options
    println!("Options:");
    for arg in cmd.get_arguments() {
        if arg.is_global_set() {
            let long = arg
                .get_long()
                .map(|l| format!("--{}", l))
                .unwrap_or_default();
            let short = arg
                .get_short()
                .map(|s| format!("-{}, ", s))
                .unwrap_or_default();
            let value_name = if arg.get_action().takes_values() {
                let value = arg
                    .get_value_names()
                    .and_then(|v| v.first())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "VALUE".to_string());
                format!(" <{}>", value)
            } else {
                String::new()
            };
            let help = arg.get_help().map(|s| s.to_string()).unwrap_or_default();
            let option_part = format!("{}{}{}", short, long, value_name);
            println!("  {:<25} {}", option_part, help);
        }
    }
    println!("  -h, --help               Print help");
    println!("  -V, --version            Print version");
    println!();

    // Print command-specific options organized by command
    for subcommand in cmd.get_subcommands() {
        // Skip hidden commands
        if subcommand.is_hide_set() {
            continue;
        }

        // Check if this command has nested subcommands (like verifications)
        if subcommand.has_subcommands() {
            // Print options for each nested subcommand
            for nested in subcommand.get_subcommands() {
                let mut has_options = false;
                let mut options = Vec::new();

                for arg in nested.get_arguments() {
                    if !arg.is_global_set() {
                        has_options = true;
                        let long = arg
                            .get_long()
                            .map(|l| format!("--{}", l))
                            .unwrap_or_default();
                        let value_name = if arg.get_action().takes_values() {
                            let value = arg
                                .get_value_names()
                                .and_then(|v| v.first())
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "VALUE".to_string());
                            format!(" <{}>", value)
                        } else {
                            String::new()
                        };
                        let help = arg.get_help().map(|s| s.to_string()).unwrap_or_default();
                        let option_part = format!("{}{}", long, value_name);
                        options.push(format!("      {:<25} {}", option_part, help));
                    }
                }

                if has_options {
                    let parent_name = subcommand.get_name().to_uppercase();
                    let nested_name = nested.get_name().to_uppercase().replace("-", " ");
                    println!("{} {} OPTIONS:", parent_name, nested_name);
                    for option in options {
                        println!("{}", option);
                    }
                    println!();
                }
            }
        } else {
            // Regular command with options
            let mut has_options = false;
            let mut options = Vec::new();

            for arg in subcommand.get_arguments() {
                if !arg.is_global_set() {
                    has_options = true;
                    let long = arg
                        .get_long()
                        .map(|l| format!("--{}", l))
                        .unwrap_or_default();
                    let value_name = if arg.get_action().takes_values() {
                        let value = arg
                            .get_value_names()
                            .and_then(|v| v.first())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "VALUE".to_string());
                        format!(" <{}>", value)
                    } else {
                        String::new()
                    };
                    let help = arg.get_help().map(|s| s.to_string()).unwrap_or_default();
                    let option_part = format!("{}{}", long, value_name);
                    options.push(format!("      {:<25} {}", option_part, help));
                }
            }

            if has_options {
                let command_name = subcommand.get_name().to_uppercase().replace("-", " ");
                println!("{} OPTIONS:", command_name);
                for option in options {
                    println!("{}", option);
                }
                println!();
            }
        }
    }
}

/// Structure for JSON output of validation results
#[derive(Serialize)]
struct ValidationResult {
    errors: Vec<String>,
}

/// Helper function to print validation results
fn print_validation_results(errors: &[ReqvireError], json_output: bool) {
    if json_output {
        let mut error_strings: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        error_strings.sort(); // Sort for deterministic output
        let json_result = ValidationResult {
            errors: error_strings,
        };
        println!("{}", serde_json::to_string_pretty(&json_result).unwrap());
    } else {
        println!("\n❌ {} validation failed with error(s):", errors.len());
        println!();
        for (i, error) in errors.iter().enumerate() {
            println!("  {}. {}", i + 1, error);
            println!();
        }
        println!();
    }
}

fn wants_json(args: &Args) -> bool {
    match &args.command {
        Some(Commands::Format { json, .. }) => *json,
        Some(Commands::Validate { json, .. }) => *json,
        Some(Commands::ChangeImpact { json, .. }) => *json,
        Some(Commands::Search { json, .. }) => *json,
        Some(Commands::Traces { json, .. }) => *json,
        Some(Commands::Coverage { json, .. }) => *json,
        Some(Commands::Model { json, .. }) => *json,
        Some(Commands::Lint { json, .. }) => *json,
        Some(Commands::Submodels { json, .. }) => *json,
        _ => false,
    }
}

/// Write JSON content to file or stdout
fn handle_json_output(json_content: &str, output: &Option<String>) -> Result<(), ReqvireError> {
    if let Some(path) = output {
        std::fs::write(path, json_content).map_err(|e| {
            ReqvireError::ProcessError(format!("Failed to write output file '{}': {}", path, e))
        })?;
        println!("✅ Output saved to {}", path);
    } else {
        println!("{}", json_content);
    }
    Ok(())
}

struct TempDirGuard {
    path: PathBuf,
}

impl TempDirGuard {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

pub async fn handle_command(
    args: Args,
    excluded_filename_patterns: &GlobSet,
) -> Result<i32, ReqvireError> {
    // If no command provided, show help
    if args.command.is_none() {
        Args::print_help();
        return Ok(0);
    }

    // Early validation: --output requires --json
    // Check before model parsing so we can fail fast with a clear error
    if let Some(ref cmd) = args.command {
        let (has_output, has_json) = match cmd {
            Commands::Format { output, json, .. } => (output.is_some(), *json),
            Commands::Validate { output, json, .. } => (output.is_some(), *json),
            Commands::Search { output, json, .. } => (output.is_some(), *json),
            Commands::ChangeImpact { output, json, .. } => (output.is_some(), *json),
            Commands::Traces { output, json, .. } => (output.is_some(), *json),
            Commands::Coverage { output, json, .. } => (output.is_some(), *json),
            Commands::Model { output, json, .. } => (output.is_some(), *json),
            Commands::Lint { output, json, .. } => (output.is_some(), *json),
            Commands::Add { output, json, .. } => (output.is_some(), *json),
            Commands::Rm { output, json, .. } => (output.is_some(), *json),
            Commands::Mv { output, json, .. } => (output.is_some(), *json),
            Commands::Rename { output, json, .. } => (output.is_some(), *json),
            Commands::Merge { output, json, .. } => (output.is_some(), *json),
            Commands::MvFile { output, json, .. } => (output.is_some(), *json),
            Commands::Containment { output, json, .. } => (output.is_some(), *json),
            Commands::Resources { output, json, .. } => (output.is_some(), *json),
            Commands::Submodels { output, json, .. } => (output.is_some(), *json),
            Commands::Collect { output, json, .. } => (output.is_some(), *json),
            _ => (false, false),
        };
        if has_output && !has_json {
            eprintln!("error: --output requires --json flag");
            return Ok(1);
        }
    }

    // Get current working directory once at the start
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    let mut model_manager = ModelManager::new();
    let is_lint_command = matches!(args.command, Some(Commands::Lint { .. }));
    let parse_result = if is_lint_command {
        model_manager.parse_and_validate_with_mode(None, excluded_filename_patterns, true)
    } else {
        model_manager.parse_and_validate(None, excluded_filename_patterns)
    };

    let json_output = wants_json(&args);

    // Handle validation failures for all commands (including validate)
    match &parse_result {
        Err(ReqvireError::ValidationError(errors)) => {
            print_validation_results(errors, json_output);
            return Ok(1);
        }
        Err(e) => {
            if json_output {
                let json_result = ValidationResult {
                    errors: vec![e.to_string()],
                };
                println!("{}", serde_json::to_string_pretty(&json_result).unwrap());
            } else {
                eprintln!("❌ Parsing failed: {}", e);
            }
            return Ok(1);
        }
        Ok(_) => {
            // No validation errors, proceed with command
        }
    }

    match args.command {
        Some(Commands::Validate { json, output }) => {
            // For validate command, if we get here it means no validation errors
            if json {
                let json_result = ValidationResult { errors: vec![] };
                let json_str = serde_json::to_string_pretty(&json_result).unwrap();
                handle_json_output(&json_str, &output)?;
            } else {
                println!("✅ No validation issues found");
            }
            Ok(0)
        }
        Some(Commands::Search {
            json,
            output,
            short,
            filter_file,
            filter_name,
            filter_type,
            filter_content,
            filter_page_content,
            have_relations,
            not_have_relations,
            has_attachments,
            filter_attachment,
        }) => {
            // Build search filters
            let filters = reqvire::search::SearchFilters::new(
                filter_file.as_deref(),
                filter_name.as_deref(),
                filter_type.as_deref(),
                filter_content.as_deref(),
                filter_page_content.as_deref(),
                have_relations.as_deref(),
                not_have_relations.as_deref(),
                has_attachments,
                filter_attachment.as_deref(),
            )?;

            // Generate search report
            let report_output = reqvire::search::generate_search_report(
                &model_manager.graph_registry,
                &filters,
                json,
                short,
            )?;

            if json {
                handle_json_output(&report_output, &output)?;
            } else {
                println!("{}", report_output);
            }
            Ok(0)
        }
        Some(Commands::ChangeImpact {
            json,
            git_commit,
            output,
        }) => {
            let base_url = git_commands::get_repository_base_url().map_err(|_| {
                ReqvireError::ProcessError(
                    "❌ Failed to determine repository base url.".to_string(),
                )
            })?;

            let current_commit = git_commands::get_commit_hash().map_err(|_| {
                ReqvireError::ProcessError(
                    "❌ Failed to retrieve the current commit hash.".to_string(),
                )
            })?;

            let mut reference_model_manager = ModelManager::new();
            match reference_model_manager.parse_and_validate_with_mode(
                Some(&git_commit),
                excluded_filename_patterns,
                false,
            ) {
                Ok(_) => {}
                Err(ReqvireError::ValidationError(errors)) => {
                    log::warn!(
                        "Reference model at commit {} has {} validation issue(s); continuing in lenient mode.",
                        git_commit,
                        errors.len()
                    );
                    reference_model_manager.parse_and_validate_with_mode(
                        Some(&git_commit),
                        excluded_filename_patterns,
                        true,
                    )?;
                }
                Err(e) => {
                    return Err(ReqvireError::ProcessError(format!(
                        "❌ Failed to parse reference model at commit {}: {}",
                        git_commit, e
                    )));
                }
            }

            let report = change_impact::compute_change_impact(
                &model_manager.graph_registry,
                &reference_model_manager.graph_registry,
            )
            .map_err(|e| {
                ReqvireError::ProcessError(format!(
                    "❌ Failed to generate change impact report: {:?}",
                    e
                ))
            })?;

            if json {
                let json_str = report.to_json_string(&base_url, &current_commit, &git_commit);
                handle_json_output(&json_str, &output)?;
            } else {
                println!(
                    "{}",
                    report.to_text(&base_url, &current_commit, &git_commit)
                );
            }

            Ok(0)
        }
        Some(Commands::Format {
            fix,
            json,
            output,
            with_full_relations,
        }) => {
            // Default is dry-run mode (preview only), --fix flag applies changes
            let dry_run = !fix;
            let format_result =
                format_files(&model_manager.graph_registry, dry_run, with_full_relations)?;

            if json {
                let json_str = render_diff_json(&format_result);
                handle_json_output(&json_str, &output)?;
            } else {
                render_diff(&format_result);
            }
            Ok(0)
        }
        Some(Commands::Traces {
            json,
            output,
            from_folder,
            links_with_blobs,
            filter_id,
            filter_name,
            filter_type,
        }) => {
            // Generate verification traces report (upward paths from verifications to requirements)
            let generator = verification_trace::VerificationTraceGenerator::new(
                &model_manager.graph_registry,
                links_with_blobs,
                from_folder.clone(),
            );

            let mut report = generator.generate();

            // Apply filters
            if filter_id.is_some() || filter_name.is_some() || filter_type.is_some() {
                report = verification_trace::apply_filters(
                    report,
                    filter_id.as_deref(),
                    filter_name.as_deref(),
                    filter_type.as_deref(),
                )?;
            }

            // Output the report
            if json {
                let json_str = serde_json::to_string_pretty(&report).map_err(|e| {
                    ReqvireError::ProcessError(format!("Failed to serialize report: {}", e))
                })?;
                handle_json_output(&json_str, &output)?;
            } else {
                let markdown_output = generator.generate_markdown(&report);
                println!("{}", markdown_output);
            }

            Ok(0)
        }
        Some(Commands::Coverage { json, output }) => {
            let coverage_report =
                report_coverage::generate_coverage_report(&model_manager.graph_registry);
            if json {
                handle_json_output(&coverage_report.to_json_string(), &output)?;
            } else {
                coverage_report.print(false);
            }
            Ok(0)
        }
        Some(Commands::Model {
            from,
            reverse,
            filter_type,
            json,
            output,
        }) => {
            // Parse filter types if provided
            let type_filter: Option<Vec<&str>> = filter_type
                .as_ref()
                .map(|s| s.split(',').map(|t| t.trim()).collect());

            // Generate model-centric report with optional filtering
            let report_output = report_model::generate_model_report(
                &model_manager.graph_registry,
                from.as_deref(),
                reverse,
                type_filter,
                json,
                "LR", // Left-to-right diagrams for markdown output
            )?;
            if json {
                handle_json_output(&report_output, &output)?;
            } else {
                println!("{}", report_output);
            }
            Ok(0)
        }
        Some(Commands::Lint {
            fixable,
            auditable,
            fix,
            json,
            output,
        }) => {
            // Run lint analysis
            let lint_report = lint::analyze_model(&model_manager.graph_registry);

            if fix {
                // Apply automatic fixes
                match lint_report.apply_fixes(&mut model_manager.graph_registry) {
                    Ok(relations_removed) => {
                        if relations_removed > 0 {
                            // Rewrite all files with updated relations (use default relations, not full)
                            let format_result =
                                format_files(&model_manager.graph_registry, false, false)?;

                            if !json {
                                println!(
                                    "✅ Fixed {} redundant verify relation(s)\n",
                                    relations_removed
                                );
                                println!(
                                    "Formatted {} file(s) with removed relations.\n",
                                    format_result.files_changed
                                );
                            }

                            // Show remaining issues that need manual review
                            if !lint_report.needs_manual_review.is_empty() {
                                if json {
                                    handle_json_output(
                                        &lint_report.to_json_string(false, true),
                                        &output,
                                    )?;
                                } else {
                                    lint_report.print(false, false, true);
                                }
                            }
                        } else {
                            if !json {
                                println!("No auto-fixable issues found.\n");
                            }
                            if json {
                                handle_json_output(
                                    &lint_report.to_json_string(fixable, auditable),
                                    &output,
                                )?;
                            } else {
                                lint_report.print(false, fixable, auditable);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to apply fixes: {}", e);
                        return Ok(1);
                    }
                }
            } else {
                // Just print the report based on flags
                if json {
                    handle_json_output(&lint_report.to_json_string(fixable, auditable), &output)?;
                } else {
                    lint_report.print(false, fixable, auditable);
                }
            }

            Ok(0)
        }
        Some(Commands::Export { output }) => {
            let git_root = git_commands::get_git_root_dir()?;

            if let Some(output_dir) = output {
                // Export to specified directory
                info!("Exporting model to HTML folder: {}", &output_dir);
                // Convert to absolute path before export (cwd changes during export)
                let output_path = if PathBuf::from(&output_dir).is_absolute() {
                    PathBuf::from(&output_dir)
                } else {
                    current_dir.join(&output_dir)
                };
                export::export_model_with_artifacts(
                    &model_manager.graph_registry,
                    &output_path,
                    excluded_filename_patterns,
                    false, // always generate links without blobs for Export
                    &current_dir,
                    &git_root,
                )?;
                info!(
                    "✅ Export completed successfully to: {}",
                    output_path.display()
                );
            } else {
                // Export to temporary directory
                let temp_dir = export::generate_artifacts_in_temp(
                    &model_manager.graph_registry,
                    excluded_filename_patterns,
                    false, // always generate links without blobs for Export
                    &current_dir,
                    &git_root,
                )?;
                println!(
                    "✅ Export completed successfully to: {}",
                    temp_dir.display()
                );
            }
            Ok(0)
        }
        Some(Commands::Serve { host, port }) => {
            // Enable quiet mode for serve command (suppress verbose export output)
            reqvire::utils::enable_quiet_mode();

            let git_root = git_commands::get_git_root_dir()?;
            let temp_dir = export::generate_artifacts_in_temp(
                &model_manager.graph_registry,
                excluded_filename_patterns,
                false, // always generate links without blobs for Serve
                &current_dir,
                &git_root,
            )?;
            let _temp_dir_guard = TempDirGuard::new(temp_dir.clone());

            // Start HTTP server (runs until Ctrl-C)
            info!("Starting HTTP server at http://{}:{}/", host, port);
            serve::serve_directory(&temp_dir, &host, port).await?;

            Ok(0)
        }
        Some(Commands::Add {
            file,
            content,
            override_existing,
            dry_run,
            json,
            output,
        }) => {
            // Use --content if provided, otherwise read from stdin
            let element_markdown = if let Some(content_str) = content {
                content_str
            } else {
                use std::io::Read;
                let mut stdin_content = String::new();
                std::io::stdin().read_to_string(&mut stdin_content)?;
                stdin_content
            };

            if element_markdown.trim().is_empty() {
                return Err(ReqvireError::ProcessError(
                    "Element markdown is empty. Provide content via --content or pipe to stdin."
                        .to_string(),
                ));
            }

            // Call CRUD operation
            let git_root = git_commands::get_git_root_dir()?;
            let result = crud::add_element(
                &mut model_manager,
                &element_markdown,
                &file,
                excluded_filename_patterns,
                &current_dir,
                &git_root,
                dry_run,
                override_existing,
            )?;

            // Output result
            if json {
                handle_json_output(&render_crud_json(&result), &output)?;
            } else {
                render_crud_result(&result);
            }

            Ok(0)
        }
        Some(Commands::Rm {
            element_name,
            dry_run,
            json,
            output,
        }) => {
            // Resolve element name to identifier
            let element_id = model_manager
                .graph_registry
                .find_element_by_name(&element_name)?;

            // Call CRUD operation
            let git_root = git_commands::get_git_root_dir()?;
            let result = crud::remove_element(&mut model_manager, &element_id, &git_root, dry_run)?;

            // Output result
            if json {
                handle_json_output(&render_crud_json(&result), &output)?;
            } else {
                render_crud_result(&result);
            }

            Ok(0)
        }
        Some(Commands::Mv {
            element_name,
            file,
            dry_run,
            json,
            output,
        }) => {
            // Resolve element name to identifier
            let element_id = model_manager
                .graph_registry
                .find_element_by_name(&element_name)?;

            // Call CRUD operation
            let git_root = git_commands::get_git_root_dir()?;
            let result = crud::move_element(
                &mut model_manager,
                &element_id,
                &file,
                excluded_filename_patterns,
                &current_dir,
                &git_root,
                dry_run,
            )?;

            // Output result
            if json {
                handle_json_output(&render_crud_json(&result), &output)?;
            } else {
                render_crud_result(&result);
            }

            Ok(0)
        }
        Some(Commands::Rename {
            element_name,
            new_name,
            dry_run,
            json,
            output,
        }) => {
            // Resolve element name to identifier
            let element_id = model_manager
                .graph_registry
                .find_element_by_name(&element_name)?;

            // Call CRUD operation
            let git_root = git_commands::get_git_root_dir()?;
            let result = crud::rename_element(
                &mut model_manager,
                &element_id,
                &new_name,
                &git_root,
                dry_run,
            )?;

            // Output result
            if json {
                handle_json_output(&render_crud_json(&result), &output)?;
            } else {
                render_crud_result(&result);
            }

            Ok(0)
        }
        Some(Commands::Merge {
            target,
            sources,
            dry_run,
            json,
            output,
        }) => {
            // Call CRUD operation
            let git_root = git_commands::get_git_root_dir()?;
            let result =
                crud::merge_elements(&mut model_manager, &target, &sources, &git_root, dry_run)?;

            // Output result
            if json {
                handle_json_output(&render_crud_json(&result), &output)?;
            } else {
                render_crud_result(&result);
            }

            Ok(0)
        }
        Some(Commands::MvFile {
            source_file,
            target_file,
            squash,
            dry_run,
            json,
            output,
        }) => {
            // Call CRUD operation
            let git_root = git_commands::get_git_root_dir()?;
            let result = crud::move_file(
                &mut model_manager,
                &source_file,
                &target_file,
                &current_dir,
                &git_root,
                dry_run,
                squash,
            )?;

            // Output result
            if json {
                handle_json_output(&render_crud_json(&result), &output)?;
            } else {
                render_crud_result(&result);
            }

            Ok(0)
        }
        Some(Commands::Link {
            source,
            relation_type,
            target,
            dry_run,
        }) => {
            let git_root = git_commands::get_git_root_dir()?;

            // Check if relation_type is 'attaching' - special keyword for attachments
            if relation_type == "attaching" {
                // External URLs are not allowed for attachments
                if reqvire::utils::is_external_url(&target) {
                    return Err(ReqvireError::ProcessError(format!(
                        "External URLs cannot be attached. Use a relation type (e.g., 'trace') instead:\n  reqvire link \"{}\" trace \"{}\"",
                        source, target
                    )));
                }

                // Auto-detect: check if target is a file or element name
                let cwd = std::env::current_dir().unwrap_or_default();
                let file_exists_cwd = cwd.join(&target).exists();
                let file_exists_git_root = git_root.join(&target).exists();

                if file_exists_cwd || file_exists_git_root {
                    // It's a file path - use file attachment logic
                    let result = reqvire::crud::attach(
                        &mut model_manager,
                        &source,
                        &target,
                        &git_root,
                        dry_run,
                    )?;
                    render_crud_result(&result);
                } else {
                    // Not a file - try to resolve as element name
                    let result = reqvire::crud::attach_element(
                        &mut model_manager,
                        &source,
                        &target,
                        &git_root,
                        dry_run,
                    )?;
                    render_crud_result(&result);
                }
            } else {
                // Regular relation link
                let result = reqvire::crud::link(
                    &mut model_manager,
                    &source,
                    &relation_type,
                    &target,
                    &git_root,
                    dry_run,
                )?;
                render_crud_result(&result);
            }
            Ok(0)
        }
        Some(Commands::Unlink {
            source,
            target,
            dry_run,
        }) => {
            let git_root = git_commands::get_git_root_dir()?;
            let result =
                reqvire::crud::unlink(&mut model_manager, &source, &target, &git_root, dry_run)?;
            render_crud_result(&result);
            Ok(0)
        }
        Some(Commands::Relink {
            source,
            relation_type,
            from_target,
            to_target,
            dry_run,
        }) => {
            let git_root = git_commands::get_git_root_dir()?;
            let result = reqvire::crud::relink(
                &mut model_manager,
                &source,
                &relation_type,
                &from_target,
                &to_target,
                &git_root,
                dry_run,
            )?;
            render_crud_result(&result);
            Ok(0)
        }
        Some(Commands::MvAsset {
            old_path,
            new_path,
            dry_run,
        }) => {
            let git_root = git_commands::get_git_root_dir()?;
            let result = reqvire::crud::mv_asset(
                &mut model_manager,
                &old_path,
                &new_path,
                &git_root,
                dry_run,
            )?;

            render_crud_result(&result);
            Ok(0)
        }
        Some(Commands::RmAsset { file_path, dry_run }) => {
            let git_root = git_commands::get_git_root_dir()?;
            let result =
                reqvire::crud::rm_asset(&mut model_manager, &file_path, &git_root, dry_run)?;

            render_crud_result(&result);
            Ok(0)
        }
        Some(Commands::Containment {
            json,
            output,
            short,
        }) => {
            if json {
                // Build containment hierarchy
                let hierarchy = reqvire::containment::ContainmentHierarchy::build(
                    &model_manager.graph_registry,
                    short,
                )?;
                // Serialize to JSON
                let json_str = serde_json::to_string_pretty(&hierarchy).map_err(|e| {
                    ReqvireError::ElementError(format!("JSON serialization error: {}", e))
                })?;
                handle_json_output(&json_str, &output)?;
            } else {
                let diagram_output =
                    diagrams::generate_containment_diagram(&model_manager.graph_registry, short)?;
                println!("{}", diagram_output);
            }
            Ok(0)
        }
        Some(Commands::Resources { json, output }) => {
            if json {
                handle_json_output(
                    &report_resources::generate_resources_report(&model_manager.graph_registry)
                        .to_json_string(),
                    &output,
                )?;
            } else {
                let report =
                    report_resources::generate_resources_report(&model_manager.graph_registry);
                report.print(false);
            }
            Ok(0)
        }
        Some(Commands::Submodels { from, json, output }) => {
            let report =
                report_submodels::generate_submodels_report(&model_manager.graph_registry, from.as_deref())?;
            if json {
                handle_json_output(&report.to_json_string(), &output)?;
            } else {
                println!("{}", report.format_text());
            }
            Ok(0)
        }
        Some(Commands::Collect {
            element_name,
            direction,
            json,
            output,
        }) => {
            let collect_direction = match direction.to_uppercase().as_str() {
                "UPSTREAM" => report_collect::CollectDirection::Upstream,
                "DOWNSTREAM" => report_collect::CollectDirection::Downstream,
                _ => {
                    eprintln!(
                        "error: invalid direction '{}'. Valid values: UPSTREAM, DOWNSTREAM",
                        direction
                    );
                    return Ok(1);
                }
            };
            let git_root = git_commands::get_git_root_dir()?;
            let report_output = report_collect::generate_collect_report(
                &model_manager.graph_registry,
                &element_name,
                &git_root,
                json,
                collect_direction,
            )?;
            if json {
                handle_json_output(&report_output, &output)?;
            } else {
                println!("{}", report_output);
            }
            Ok(0)
        }
        Some(Commands::Shell) => {
            run_shell(&mut model_manager)?;
            Ok(0)
        }
        Some(Commands::Sout) => {
            run_sout(&model_manager.graph_registry)?;
            Ok(0)
        }
        None => {
            // This case is handled at the beginning of handle_command
            unreachable!("Command is None but should have been handled earlier");
        }
    }
}

fn run_sout(graph_registry: &GraphRegistry) -> Result<(), ReqvireError> {
    use std::collections::BTreeMap;

    // Collect all file paths from pages and elements
    let mut file_map: BTreeMap<String, (Option<&Page>, Vec<&Element>)> = BTreeMap::new();

    // Collect pages
    for (file_path, page) in &graph_registry.pages {
        file_map.entry(file_path.clone()).or_default().0 = Some(page);
    }

    // Collect elements grouped by file
    for element_node in graph_registry.nodes.values() {
        let element = &element_node.element;
        file_map
            .entry(element.file_path.clone())
            .or_default()
            .1
            .push(element);
    }

    // Output content for each file in sorted order
    for (file_path, (page, mut elements)) in file_map {
        println!("File: {}", file_path);
        println!();

        // Output page content if exists
        if let Some(page) = page {
            if !page.frontmatter_content.trim().is_empty() {
                println!("{}", page.frontmatter_content);
                println!();
            }
        }

        // Sort elements by file_order_index for consistent ordering
        elements.sort_by_key(|e| e.file_order_index);

        // Output elements
        for element in elements {
            println!("### {}", element.name);
            println!();
            if !element.content.trim().is_empty() {
                println!("{}", element.content);
                println!();
            }

            // Output metadata if exists
            if !element.metadata.is_empty() {
                println!("#### Metadata");
                for (key, value) in &element.metadata {
                    println!("  * {}: {}", key, value);
                }
                println!();
            }

            // Output relations if exists
            if !element.relations.is_empty() {
                println!("#### Relations");
                for relation in &element.relations {
                    println!(
                        "  * {}: [{}]({})",
                        relation.relation_type.name,
                        relation.target.text,
                        relation.target.link.as_str()
                    );
                }
                println!();
            }

            println!("---");
            println!();
        }

        // Add separator between files
        println!();
        println!();
    }

    Ok(())
}

fn run_shell(model_manager: &mut ModelManager) -> Result<(), ReqvireError> {
    use std::io::{self, Write};

    println!("Reqvire Interactive Shell");
    println!("Type 'help' for available commands, 'exit' to quit");
    println!();

    // Use the existing graph registry from the model manager
    let graph_registry = &mut model_manager.graph_registry;

    loop {
        print!("reqvire> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }

                match input {
                    "exit" | "quit" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" => {
                        print_shell_help();
                    }
                    _ => {
                        if let Err(e) = process_shell_command(graph_registry, input) {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }

    Ok(())
}

fn print_shell_help() {
    println!("Available commands:");
    println!("  help                                        - Show this help message");
    println!("  exit, quit                                  - Exit the shell");
    println!(
        "  list-elements [filter]                     - List all elements or filter by pattern"
    );
    println!(
        "  show-element <element_id>                  - Show detailed information about an element"
    );
    println!("  move-element <element_id> <file> <section> - Move element to existing location");
    println!("  create-section <file> <section>            - Create new section in existing file");
    println!("  create-file <file> <section>               - Create new file with section");
    println!(
        "  list-locations                             - Show all available file/section locations"
    );
    println!("  get-move-impact <element_id>               - Show elements affected by moving an element");
    println!(
        "  impact-tree <element_id>                   - Show change impact tree for an element"
    );
    println!("  flush <output_dir>                         - Flush all changes to directory");
    println!("  flush-files <file1,file2,...> <output_dir> - Flush specific files to directory");
    println!("  stats                                       - Show registry statistics");
    println!();
    println!("  Dynamic Graph Management:");
    println!("  add-element <id> <name> <file> [section]   - Add new element to graph");
    println!("  remove-element <element_id>                - Remove element from graph");
    println!("  add-relation <source> <target> <type>      - Add relation between elements");
    println!("  remove-relation <source> <target> <type>   - Remove relation between elements");
    println!("  graph-stats                                 - Show graph statistics");
    println!();
}

fn process_shell_command(
    graph_registry: &mut GraphRegistry,
    command: &str,
) -> Result<(), ReqvireError> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    match parts[0] {
        "list-elements" => {
            let filter = parts.get(1).unwrap_or(&"");
            let elements = graph_registry.get_all_elements();
            let filtered: Vec<_> = if filter.is_empty() {
                elements
            } else {
                elements
                    .into_iter()
                    .filter(|elem| elem.identifier.contains(filter) || elem.name.contains(filter))
                    .collect()
            };

            println!("Found {} elements:", filtered.len());
            for element in filtered {
                println!(
                    "  {} ({:?}): {}",
                    element.identifier, element.element_type, element.name
                );
            }
        }
        "show-element" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError(
                    "Usage: show-element <element_id>".to_string(),
                ));
            }
            let element_id = parts[1];
            if let Some(element) = graph_registry.get_element(element_id) {
                println!("Element ID: {}", element.identifier);
                println!("Name: {}", element.name);
                println!("Type: {:?}", element.element_type);
                println!("File: {}", element.file_path);
                println!("Content: {}", element.content);
                if !element.relations.is_empty() {
                    println!("Relations:");
                    for relation in &element.relations {
                        println!(
                            "  {} -> {}",
                            relation.relation_type.name, relation.target.text
                        );
                    }
                }
            } else {
                println!("Element '{}' not found", element_id);
            }
        }
        "move-element" => {
            if parts.len() < 3 {
                return Err(ReqvireError::ProcessError(
                    "Usage: move-element <element_id> <file>".to_string(),
                ));
            }
            let element_id = parts[1];
            let file_path = parts[2];

            graph_registry.move_element_to_location(element_id, file_path)?;
            println!("Element '{}' moved to {}", element_id, file_path);
        }
        "create-file" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError(
                    "Usage: create-file <file>".to_string(),
                ));
            }
            let file_path = parts[1];

            graph_registry.create_virtual_file(file_path)?;
            println!("Virtual file '{}' created", file_path);
        }
        "list-locations" => {
            let locations = graph_registry.get_available_locations();
            println!("Available file locations:");
            for file in locations {
                println!("  {}", file);
            }
        }
        "get-move-impact" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError(
                    "Usage: get-move-impact <element_id>".to_string(),
                ));
            }
            let element_id = parts[1];
            let impact = graph_registry.get_move_impact(element_id);
            if impact.is_empty() {
                println!("No elements would be affected by moving '{}'", element_id);
            } else {
                println!(
                    "Elements affected by moving '{}': {}",
                    element_id,
                    impact.join(", ")
                );
            }
        }
        "impact-tree" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError(
                    "Usage: impact-tree <element_id>".to_string(),
                ));
            }
            let element_id = parts[1];

            // Check if element exists
            if graph_registry.get_element(element_id).is_none() {
                println!("Element '{}' not found", element_id);
                return Ok(());
            }

            println!("Change Impact Tree for element '{}':", element_id);
            let impact_tree = graph_registry.get_impact_tree(element_id);
            print_impact_tree(&impact_tree, 0);
        }
        "flush" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError(
                    "Usage: flush <output_dir>".to_string(),
                ));
            }
            let output_dir = Path::new(parts[1]);

            let (md_count, file_count) = graph_registry.flush_to_directory(output_dir, false)?;
            println!(
                "Flushed {} markdown files and {} other files to '{}'",
                md_count,
                file_count,
                output_dir.display()
            );
        }
        "flush-files" => {
            if parts.len() < 3 {
                return Err(ReqvireError::ProcessError(
                    "Usage: flush-files <file1,file2,...> <output_dir>".to_string(),
                ));
            }
            let file_list = parts[1];
            let output_dir = Path::new(parts[2]);

            let file_paths: Vec<String> =
                file_list.split(',').map(|s| s.trim().to_string()).collect();
            let (md_count, file_count) =
                graph_registry.flush_files_to_directory(&file_paths, output_dir, false)?;
            println!(
                "Flushed {} markdown files and {} other files to '{}'",
                md_count,
                file_count,
                output_dir.display()
            );
        }
        "stats" => {
            let elements = graph_registry.get_all_elements();
            let mut type_counts = HashMap::new();
            for element in &elements {
                let type_str = format!("{:?}", element.element_type);
                *type_counts.entry(type_str).or_insert(0) += 1;
            }

            println!("Registry Statistics:");
            println!("  Total elements: {}", elements.len());
            println!("  By type:");
            for (element_type, count) in &type_counts {
                println!("    {}: {}", element_type, count);
            }

            let total_relations: usize = elements.iter().map(|e| e.relations.len()).sum();
            println!("  Total relations: {}", total_relations);
        }
        "add-element" => {
            if parts.len() < 4 {
                return Err(ReqvireError::ProcessError(
                    "Usage: add-element <element_id> <element_name> <file_path> [section]"
                        .to_string(),
                ));
            }
            let element_id = parts[1];
            let element_name = parts[2];
            let file_path = parts[3];
            // Note: section parameter removed - sections are no longer tracked in the model

            let element = reqvire::element::Element::new(
                element_name,
                element_id,
                file_path,
                1, // REPL-added elements default to line 1
                None,
            );

            match graph_registry.add_element(element) {
                Ok(()) => println!("Successfully added element '{}'", element_id),
                Err(e) => println!("Failed to add element '{}': {}", element_id, e),
            }
        }
        "remove-element" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError(
                    "Usage: remove-element <element_id>".to_string(),
                ));
            }
            let element_id = parts[1];

            match graph_registry.remove_element(element_id) {
                Ok(()) => println!("Successfully removed element '{}'", element_id),
                Err(e) => println!("Failed to remove element '{}': {}", element_id, e),
            }
        }
        "add-relation" => {
            if parts.len() < 4 {
                return Err(ReqvireError::ProcessError(
                    "Usage: add-relation <source_id> <target_id> <relation_type>".to_string(),
                ));
            }
            let source_id = parts[1];
            let target_id = parts[2];
            let relation_type = parts[3];

            match graph_registry.add_relation(source_id, target_id, relation_type) {
                Ok(()) => println!(
                    "Successfully added relation '{}' from '{}' to '{}'",
                    relation_type, source_id, target_id
                ),
                Err(e) => println!("Failed to add relation: {}", e),
            }
        }
        "remove-relation" => {
            if parts.len() < 4 {
                return Err(ReqvireError::ProcessError(
                    "Usage: remove-relation <source_id> <target_id> <relation_type>".to_string(),
                ));
            }
            let source_id = parts[1];
            let target_id = parts[2];
            let relation_type = parts[3];

            match graph_registry.remove_relation(source_id, target_id, relation_type) {
                Ok(()) => println!(
                    "Successfully removed relation '{}' from '{}' to '{}'",
                    relation_type, source_id, target_id
                ),
                Err(e) => println!("Failed to remove relation: {}", e),
            }
        }
        "list-relations" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError(
                    "Usage: list-relations <element_id>".to_string(),
                ));
            }
            let element_id = parts[1];

            match graph_registry.list_relations(element_id) {
                Ok(relations) => {
                    if relations.is_empty() {
                        println!("Element '{}' has no relations", element_id);
                    } else {
                        println!("Relations for element '{}':", element_id);
                        for (relation_type, target_id) in relations {
                            println!("  {} -> {}", relation_type, target_id);
                        }
                    }
                }
                Err(e) => println!("Failed to list relations: {}", e),
            }
        }
        "graph-stats" => {
            let (element_count, relation_count) = graph_registry.get_graph_stats();
            println!("Graph Statistics:");
            println!("  Elements: {}", element_count);
            println!("  Relations: {}", relation_count);
            println!(
                "  Average relations per element: {:.2}",
                if element_count > 0 {
                    relation_count as f64 / element_count as f64
                } else {
                    0.0
                }
            );
        }
        _ => {
            println!(
                "Unknown command: '{}'. Type 'help' for available commands.",
                parts[0]
            );
        }
    }

    Ok(())
}

fn print_impact_tree(node: &reqvire::graph_registry::ElementNode, depth: usize) {
    let indent = "  ".repeat(depth);
    let element = &node.element;

    // Print current element with impact propagation info
    if depth == 0 {
        println!("{}📍 {} ({})", indent, element.identifier, element.name);
    } else {
        println!("{}└─ {} ({})", indent, element.identifier, element.name);
    }

    // Print element details
    println!("{}   Type: {:?}", indent, element.element_type);
    println!("{}   Location: {}", indent, element.file_path);

    // Print relations that caused this impact
    if !node.relations.is_empty() {
        println!("{}   Impacts through:", indent);
        for relation_node in &node.relations {
            println!(
                "{}     {} -> {}",
                indent,
                relation_node.relation_trigger,
                relation_node.element_node.element.identifier
            );
        }
        println!();

        // Recursively print impacted elements
        for relation_node in &node.relations {
            print_impact_tree(&relation_node.element_node, depth + 1);
        }
    } else if depth > 0 {
        println!("{}   (No further impacts)", indent);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use globset::{Glob, GlobSet, GlobSetBuilder};

    fn build_glob_set(patterns: &[String]) -> GlobSet {
        let mut builder = GlobSetBuilder::new();
        for pattern in patterns {
            if let Ok(glob) = Glob::new(pattern) {
                builder.add(glob);
            } else {
                eprintln!("Invalid glob pattern: {}", pattern);
            }
        }
        builder.build().expect("Failed to build glob set")
    }

    #[test]
    fn test_cli_parsing_subcommand() {
        let args = Args::parse_from(&["reqvire", "export"]);
        assert!(matches!(args.command, Some(Commands::Export { output: _ })));
    }

    #[tokio::test]
    async fn test_handle_command() {
        // Mock CLI arguments
        let args = Args {
            command: Some(Commands::Export {
                output: Some("html".to_string()),
            }),
        };

        // Define test input paths

        let excluded_filename_patterns = vec![
            "**/README*.md".to_string(),
            "**/Logical*.md".to_string(),
            "**/Physical*.md".to_string(),
            "**/index.md".to_string(),
        ];

        // Run the handle_command function
        let result = handle_command(args, &build_glob_set(&excluded_filename_patterns)).await;

        // Assert that it runs without error
        assert!(
            result.is_ok(),
            "handle_command should execute without errors"
        );
    }
}
