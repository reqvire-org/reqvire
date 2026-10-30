use crate::mcp;
use crate::serve;
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand, ValueEnum};
use reqvire::error::ReqvireError;
use reqvire::exclusions::ExclusionSet as GlobSet;
use reqvire::explorer_runtime;
use reqvire::format::{format_files, render_diff, render_diff_json};
use reqvire::git_commands;
use reqvire::migrations;
use reqvire::operations;
use reqvire::report;
use reqvire::semantic_contract::{self, SemanticExportFormat, SemanticExportLayer};
use reqvire::ModelBuildOptions;
use reqvire::ModelManager;
use serde::Serialize;
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
    /// Workspace directory to operate on
    #[clap(long, global = true, value_name = "DIR")]
    pub workspace: Option<PathBuf>,

    /// Override Repo manifest location (default: .repo/manifest.xml)
    #[clap(long, global = true, value_name = "FILE")]
    pub manifest: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum McpTransport {
    Http,
    Stdio,
}

#[derive(clap::Args, Debug)]
pub struct SearchArgs {
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

    /// Only include elements of the given type(s). Supports comma-separated list. Valid: capability, requirement, ontology, concept-scheme, concept, test-verification, formal-proof-verification, analysis-verification, inspection-verification, demonstration-verification, verification-objective, source, semantic-query, semantic-contract, constraint, behavior, specification, state, input-output. Custom: other-TYPENAME
    #[clap(long, value_name = "TYPE[,TYPE...]", help_heading = "SEARCH OPTIONS")]
    filter_type: Option<String>,

    /// Only include requirement-family elements with matching effective status values
    #[clap(
        long,
        value_name = "STATUS[,STATUS...]",
        help_heading = "SEARCH OPTIONS"
    )]
    filter_status: Option<String>,

    /// Only include requirement-family elements with matching effective priority values
    #[clap(
        long,
        value_name = "PRIORITY[,PRIORITY...]",
        help_heading = "SEARCH OPTIONS"
    )]
    filter_priority: Option<String>,

    /// Only include requirement-family elements with matching effective risk values
    #[clap(long, value_name = "RISK[,RISK...]", help_heading = "SEARCH OPTIONS")]
    filter_risk: Option<String>,

    /// Only include requirement-family elements whose effective owner matches this regular expression
    #[clap(long, value_name = "REGEX", help_heading = "SEARCH OPTIONS")]
    filter_owner: Option<String>,

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

    /// Only include elements that have contract_bindings
    #[clap(long, help_heading = "SEARCH OPTIONS")]
    has_contract_bindings: bool,

    /// Only include elements with contract_bindings matching this glob pattern (e.g., "*.pdf", "docs/**/*")
    #[clap(long, value_name = "GLOB", help_heading = "SEARCH OPTIONS")]
    filter_contract_bindings: Option<String>,

    /// Only include elements that have Contract References
    #[clap(long, help_heading = "SEARCH OPTIONS")]
    has_contract_references: bool,

    /// Only include elements with Contract Reference targets matching this glob (e.g., "*#error-response-specification")
    #[clap(long, value_name = "GLOB", help_heading = "SEARCH OPTIONS")]
    filter_contract_references: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Serve the embedded Explorer UI via HTTP server
    #[clap(
        override_help = "Serve the embedded Explorer UI via HTTP server\n\nThis is intended for release/npm Reqvire binaries. Source builds must build the Explorer bundle before compiling Rust.\n\nSERVE OPTIONS:\n      --host <HOST>             Bind address (default: localhost)\n      --port <PORT>             Server port (default: 8080)\n      --enable-mcp              Also serve the Reqvire MCP Streamable HTTP endpoint at /mcp\n      --enable-mutations        Advertise and allow mutation tools on the embedded MCP endpoint\n      --enable-commits          Commit successful mutations automatically (default: false; requires --enable-mutations)\n      --enable-github           Enable same-repository GitHub publication (requires --enable-mutations)\n      --github-remote <NAME>    Configured publication remote (default: origin; requires --enable-github)\n      --allow-origin <ORIGIN>   Additional browser origin (repeatable; requires --enable-mcp)\n      --allow-host <HOST[:PORT]>  Additional MCP endpoint hostname (repeatable; requires --enable-mcp)"
    )]
    Serve {
        /// Bind address
        #[clap(long, default_value = "localhost", help_heading = "SERVE OPTIONS")]
        host: String,

        /// Server port
        #[clap(long, default_value = "8080", help_heading = "SERVE OPTIONS")]
        port: u16,

        /// Also serve the Reqvire MCP Streamable HTTP endpoint at /mcp
        #[clap(long, help_heading = "SERVE OPTIONS")]
        enable_mcp: bool,

        /// Advertise and allow mutation tools on the embedded MCP endpoint
        #[clap(
            long = "enable-mutations",
            requires = "enable_mcp",
            help_heading = "SERVE OPTIONS"
        )]
        mcp_enable_mutations: bool,

        /// Commit successful mutations automatically (default: false; requires --enable-mutations)
        #[clap(
            long = "enable-commits",
            requires = "mcp_enable_mutations",
            help_heading = "SERVE OPTIONS"
        )]
        mcp_enable_commits: bool,

        /// Enable same-repository GitHub publication (requires --enable-mutations)
        #[clap(
            long = "enable-github",
            requires = "mcp_enable_mutations",
            help_heading = "SERVE OPTIONS"
        )]
        mcp_enable_github: bool,

        /// Configured publication remote (default: origin; requires --enable-github)
        #[clap(
            long = "github-remote",
            requires = "mcp_enable_github",
            value_name = "NAME",
            help_heading = "SERVE OPTIONS"
        )]
        mcp_github_remote: Option<String>,

        /// Additional MCP browser origin (repeatable; loopback remains allowed)
        #[clap(
            long = "allow-origin",
            value_name = "ORIGIN",
            requires = "enable_mcp",
            help_heading = "SERVE OPTIONS"
        )]
        allowed_origins: Vec<crate::mcp_http::AllowedOrigin>,

        /// Additional MCP endpoint host or host:port (repeatable; requires --enable-mcp)
        #[clap(
            long = "allow-host",
            value_name = "HOST[:PORT]",
            requires = "enable_mcp",
            help_heading = "SERVE OPTIONS"
        )]
        allowed_hosts: Vec<crate::mcp_http::AllowedHost>,
    },

    /// Export the Explorer SPA as a static site to a directory
    #[clap(
        override_help = "Export the Explorer SPA as a static site to a directory\n\nWrites the embedded Explorer bundle and generated model data to <OUTPUT>.\nThe result is a self-contained static site suitable for GitHub Pages or any static host.\n\nEXPORT OPTIONS:\n      --output <DIR>    Output directory (default: site)"
    )]
    Export {
        /// Output directory
        #[clap(
            long,
            short = 'o',
            default_value = "site",
            help_heading = "EXPORT OPTIONS"
        )]
        output: String,
    },

    /// Start Reqvire MCP server
    #[clap(
        name = "mcp",
        override_help = "Start Reqvire MCP server\n\nMCP OPTIONS:\n      --transport <TRANSPORT>   MCP transport (default: http; values: http, stdio)\n      --host <HOST>             HTTP bind address (HTTP-only; default: 127.0.0.1)\n      --port <PORT>             HTTP server port (HTTP-only; default: 8081)\n      --enable-mutations        Advertise and allow mutation tools\n      --enable-commits          Commit successful mutations automatically (default: false; requires --enable-mutations)\n      --enable-github           Enable same-repository GitHub publication (requires --enable-mutations)\n      --github-remote <NAME>    Configured publication remote (default: origin; requires --enable-github)\n      --with-size-estimates     Include element size estimates in model evidence tools\n      --allow-origin <ORIGIN>   Additional browser origin (HTTP-only; repeatable; loopback remains allowed)\n      --allow-host <HOST[:PORT]> Additional endpoint host or host:port (HTTP-only; repeatable)"
    )]
    Mcp {
        /// MCP transport (default: http; values: http, stdio)
        #[clap(long, value_enum, default_value = "http", help_heading = "MCP OPTIONS")]
        transport: McpTransport,

        /// HTTP bind address (HTTP-only)
        #[clap(long, default_value = "127.0.0.1", help_heading = "MCP OPTIONS")]
        host: String,

        /// HTTP server port (HTTP-only)
        #[clap(long, default_value = "8081", help_heading = "MCP OPTIONS")]
        port: u16,

        /// Advertise and allow mutation tools
        #[clap(long, help_heading = "MCP OPTIONS")]
        enable_mutations: bool,

        /// Commit successful mutations automatically (default: false; requires --enable-mutations)
        #[clap(long, requires = "enable_mutations", help_heading = "MCP OPTIONS")]
        enable_commits: bool,

        /// Enable same-repository GitHub publication (requires --enable-mutations)
        #[clap(long, requires = "enable_mutations", help_heading = "MCP OPTIONS")]
        enable_github: bool,

        /// Configured publication remote (default: origin; requires --enable-github)
        #[clap(
            long,
            requires = "enable_github",
            value_name = "NAME",
            help_heading = "MCP OPTIONS"
        )]
        github_remote: Option<String>,

        /// Include element size estimates in model evidence tools
        #[clap(long, help_heading = "MCP OPTIONS")]
        with_size_estimates: bool,

        /// Additional browser origin (HTTP-only; repeatable; loopback remains allowed)
        #[clap(
            long = "allow-origin",
            value_name = "ORIGIN",
            help_heading = "MCP OPTIONS"
        )]
        allowed_origins: Vec<crate::mcp_http::AllowedOrigin>,

        /// Additional endpoint host or host:port (HTTP-only; repeatable)
        #[clap(
            long = "allow-host",
            value_name = "HOST[:PORT]",
            help_heading = "MCP OPTIONS"
        )]
        allowed_hosts: Vec<crate::mcp_http::AllowedHost>,
    },

    /// Preview or apply source migrations for known model-breaking changes
    #[clap(
        override_help = "Preview or apply source migrations for known model-breaking changes. By default, shows preview without applying changes\n\nMIGRATE OPTIONS:\n      --fix              Apply migration changes to files\n      --json             Output results in JSON format\n      --output <FILE>    Save JSON output to file (requires --json)\n\nCURRENT MIGRATIONS:\n    v0.15-documents-to-element-header - rewrite legacy single-element '# Documents' headers to '# Element'\n    v0.16-verification-objective - create one shared holder verification-objective in root VerificationObjectiveMigration.md and link standalone concrete verifications from that holder with derive\n    v1.0-contract-relations - rewrite legacy contract relation names to contract relation names: refinedBy -> definedBy and refine -> define\n    v1.1-contract-bindings-section - rewrite legacy 'Reused Contract Context'/'Attachments' reserved section headings to 'Contract Bindings'\n    v1.2-concept-reference-links - rewrite legacy Concept References entries from Label: IRI to Markdown links targeting native concept elements"
    )]
    Migrate {
        /// Apply migration changes to files
        #[clap(long, help_heading = "MIGRATE OPTIONS")]
        fix: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "MIGRATE OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "MIGRATE OPTIONS")]
        output: Option<String>,
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
        override_help = "Search and filter model elements with comprehensive filtering options\n\nSEARCH OPTIONS:\n      --json                            Output results in JSON format\n      --output <FILE>                   Save JSON output to file (requires --json)\n      --short                           Output abbreviated format (one-line per element)\n      --filter-file <GLOB>              Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`\n      --filter-name <REGEX>             Only include elements whose name matches this regular expression\n      --filter-type <TYPE>              Only include elements of the given type. Valid types: capability, requirement, ontology, concept-scheme, concept, test-verification, formal-proof-verification, analysis-verification, inspection-verification, demonstration-verification, verification-objective, source, semantic-query, semantic-contract, constraint, behavior, specification, state, input-output. For custom types use: other-TYPENAME\n      --filter-status <LIST>            Only include requirement-family elements with effective status values (draft, review, approved)\n      --filter-priority <LIST>          Only include requirement-family elements with effective priority values (low, medium, high, critical)\n      --filter-risk <LIST>              Only include requirement-family elements with effective risk values (low, medium, high, critical)\n      --filter-owner <REGEX>            Only include requirement-family elements whose effective owner matches this regex\n      --filter-content <REGEX>          Only include elements whose content matches this regular expression\n      --filter-page-content <REGEX>     Only include elements whose parent file page content matches this regular expression\n      --have-relations <LIST>           Only include elements that have ALL specified relations (comma-separated)\n      --not-have-relations <LIST>       Only include elements that do NOT have ALL specified relations (comma-separated)\n      --has-contract-bindings          Only include elements that have Contract Bindings\n      --filter-contract-bindings <GLOB> Match Contract Binding target identifiers\n      --has-contract-references        Only include elements that have Contract References\n      --filter-contract-references <GLOB> Match Contract Reference target identifiers"
    )]
    Search(Box<SearchArgs>),

    /// Analyze change impact and provide report
    #[clap(
        override_help = "Analyze change impact and provide report\n\nCHANGE IMPACT OPTIONS:\n      --git-commit <GIT_COMMIT>  Base Git commit (includes the historical manifest in Repo mode) [default: HEAD]\n      --json                     Output results in JSON format\n      --output <FILE>            Save JSON output to file (requires --json)"
    )]
    ChangeImpact {
        /// Base Git commit (includes the historical manifest in Repo mode) [default: HEAD]
        #[clap(long, default_value = "HEAD", help_heading = "CHANGE IMPACT OPTIONS")]
        git_commit: String,

        /// Output results in JSON format
        #[clap(long, help_heading = "CHANGE IMPACT OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "CHANGE IMPACT OPTIONS")]
        output: Option<String>,
    },

    /// Generate verification traces showing upward paths from verifications to capability-rooted requirements
    #[clap(
        override_help = "Generate verification traces showing upward paths from verifications to capability-rooted requirements\n\nTRACES OPTIONS:\n      --output <FILE>             Save JSON output to file\n      --filter-id <ID>            Only include verification with this specific identifier\n      --filter-name <REGEX>       Only include verifications whose name matches this regular expression\n      --filter-type <TYPE>        Only include verifications of the given type. Valid types: test-verification, formal-proof-verification, analysis-verification, inspection-verification, demonstration-verification"
    )]
    Traces {
        /// Save JSON output to file
        #[clap(long, value_name = "FILE", help_heading = "TRACES OPTIONS")]
        output: Option<String>,

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
        override_help = "Generate verification and implementation coverage report\n\nCOVERAGE OPTIONS:\n      --from <NAME>              Select a capability subtree by exact name or canonical identifier (default: whole model)\n      --json                      Output results in JSON format\n      --output <FILE>             Save JSON output to file (requires --json)"
    )]
    Coverage {
        /// Select a capability subtree by exact name or canonical identifier
        #[clap(long, value_name = "NAME", help_heading = "COVERAGE OPTIONS")]
        from: Option<String>,
        /// Output results in JSON format
        #[clap(long, help_heading = "COVERAGE OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "COVERAGE OPTIONS")]
        output: Option<String>,
    },

    /// Generate model-centric structure with nested relations
    ///
    /// By default, shows ontology roots, concept roots, and capability roots.
    /// Use --from <NAME> to start from specific element.
    /// Use --reverse for leaf-to-root traversal.
    ///
    #[clap(
        override_help = "Generate model-centric JSON structure with nested relations\n\nBy default, shows ontology roots, concept roots, and capability roots.\nUse --from <NAME> to start from specific element.\nUse --reverse for leaf-to-root traversal.\n\nMODEL OPTIONS:\n      --from <NAME>               Start from specific element by exact name or canonical identifier\n      --reverse                   Traverse from leaves to roots (follow backward relations)\n      --filter-type <TYPE>        Filter starting elements by type (comma-separated). Valid types: capability, requirement, ontology, concept-scheme, concept, test-verification, formal-proof-verification, analysis-verification, inspection-verification, demonstration-verification, verification-objective, source, semantic-query, semantic-contract, constraint, behavior, specification, state, input-output. For custom types use: other-TYPENAME\n      --with-size-estimates       Include element size estimates in JSON output\n      --output <FILE>             Save JSON output to file"
    )]
    Model {
        /// Start from specific element by exact name or canonical identifier
        #[clap(long, value_name = "NAME", help_heading = "MODEL OPTIONS")]
        from: Option<String>,

        /// Traverse from leaves to roots (follow backward relations)
        #[clap(long, help_heading = "MODEL OPTIONS")]
        reverse: bool,

        /// Filter starting elements by type (comma-separated). Valid: capability, requirement, ontology, concept-scheme, concept, test-verification, formal-proof-verification, analysis-verification, inspection-verification, demonstration-verification, verification-objective, source, semantic-query, semantic-contract, constraint, behavior, specification, state, input-output. Custom: other-TYPENAME
        #[clap(long, value_name = "TYPE", help_heading = "MODEL OPTIONS")]
        filter_type: Option<String>,

        /// Include element size estimates in JSON output
        #[clap(long, help_heading = "MODEL OPTIONS")]
        with_size_estimates: bool,

        /// Save JSON output to file
        #[clap(long, value_name = "FILE", help_heading = "MODEL OPTIONS")]
        output: Option<String>,
    },

    /// Analyze model quality and detect issues in requirements relations
    #[clap(
        override_help = "Analyze model quality and detect issues in requirements relations\n\nLINT OPTIONS:\n      --fixable                   Show only auto-fixable issues\n      --auditable                 Show only issues requiring manual review\n      --json                      Output results in JSON format\n      --output <FILE>             Save JSON output to file (requires --json)"
    )]
    Lint {
        /// Show only auto-fixable issues
        #[clap(long, help_heading = "LINT OPTIONS", conflicts_with = "auditable")]
        fixable: bool,

        /// Show only issues requiring manual review
        #[clap(long, help_heading = "LINT OPTIONS", conflicts_with = "fixable")]
        auditable: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "LINT OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "LINT OPTIONS")]
        output: Option<String>,
    },

    /// Generate containment view showing folder/file/element hierarchy
    #[clap(
        override_help = "Generate containment view JSON showing folder/file/element hierarchy\n\nCONTAINMENT OPTIONS:\n      --output <FILE>     Save JSON output to file\n      --short             Show only root elements (without hierarchical parents)"
    )]
    Containment {
        /// Save JSON output to file
        #[clap(long, value_name = "FILE", help_heading = "CONTAINMENT OPTIONS")]
        output: Option<String>,

        /// Show only root elements (without hierarchical parents in same file)
        #[clap(long, help_heading = "CONTAINMENT OPTIONS")]
        short: bool,
    },

    /// Generate resources report showing files referenced by the model
    #[clap(
        override_help = "Generate resources JSON report showing files referenced by the model\n\nRESOURCES OPTIONS:\n      --output <FILE>     Save JSON output to file"
    )]
    Resources {
        /// Save JSON output to file
        #[clap(long, value_name = "FILE", help_heading = "RESOURCES OPTIONS")]
        output: Option<String>,
    },

    /// Export semantic RDF layers
    #[clap(
        override_help = "Export semantic RDF layers\n\nSEMANTIC COMMANDS:\n    export   Export selected semantic RDF layers; no --layer exports all layers"
    )]
    Semantic {
        #[clap(subcommand)]
        command: SemanticCommands,
    },

    /// Work with standalone Reqvire concept schemes and thesaurus concepts
    #[clap(
        override_help = "Work with standalone Reqvire concept schemes and thesaurus concepts\n\nCONCEPTS COMMANDS:\n    export     Export generated SKOS concept scheme/thesaurus triples\n    validate   Validate standalone concept schemes, concept references, and mappings"
    )]
    Concepts {
        #[clap(subcommand)]
        command: ConceptCommands,
    },

    /// Compatibility alias for combined semantic export
    #[clap(
        override_help = "Compatibility alias for combined semantic export\n\nONTOLOGIES OPTIONS:\n      --jsonld                   Output JSON-LD RDF format instead of RDF/Turtle (.ttl)\n      --full                     Include Reqvire model context triples and ontology projection facts in the semantic export\n      --include-external         Include used local external ontology subset triples\n      --namespace-base <IRI>     Filter clean authored export to one ontology base or term namespace\n      --output <FILE>            Save output to file"
    )]
    Ontologies {
        /// Output JSON-LD format instead of RDF/Turtle (.ttl)
        #[clap(long, help_heading = "ONTOLOGIES OPTIONS")]
        jsonld: bool,

        /// Include Reqvire model context triples and ontology projection facts in the semantic export
        #[clap(long, help_heading = "ONTOLOGIES OPTIONS")]
        full: bool,

        /// Include used local external ontology subset triples
        #[clap(long, help_heading = "ONTOLOGIES OPTIONS")]
        include_external: bool,

        /// Filter clean authored export to one ontology base or term namespace
        #[clap(long, value_name = "IRI", help_heading = "ONTOLOGIES OPTIONS")]
        namespace_base: Option<String>,

        /// Save output to file
        #[clap(long, value_name = "FILE", help_heading = "ONTOLOGIES OPTIONS")]
        output: Option<String>,
    },

    /// Analyze independent capability-rooted submodels and cross-submodel couplings
    #[clap(
        override_help = "Analyze independent capability-rooted submodels and cross-submodel couplings\n\nSUBMODELS OPTIONS:\n      --from <NAME>      Scope report to a capability subtree or requirement subtree by exact name or canonical identifier\n      --json              Output results in JSON output format\n      --output <FILE>     Save JSON output to file (requires --json)"
    )]
    Submodels {
        /// Scope report to a capability subtree or requirement subtree by exact name or canonical identifier
        #[clap(long, value_name = "NAME", help_heading = "SUBMODELS OPTIONS")]
        from: Option<String>,

        /// Output results in JSON format
        #[clap(long, help_heading = "SUBMODELS OPTIONS")]
        json: bool,

        /// Save JSON output to file (requires --json)
        #[clap(long, value_name = "FILE", help_heading = "SUBMODELS OPTIONS")]
        output: Option<String>,
    },

    /// Collect content from capability, requirement, ontology, semantic-query, concept-scheme, or concept context
    #[clap(
        override_help = "Collect content from capability, requirement, ontology, semantic-query, concept-scheme, or concept context\n\nCOLLECT OPTIONS:\n      <ELEMENT_NAME>        Exact name or canonical identifier of the capability, requirement, ontology, semantic-query, concept-scheme, or concept element to collect from\n      --direction <DIR>     Traversal direction: UPSTREAM (default) or DOWNSTREAM\n      --json                Output results in JSON format\n      --output <FILE>       Save JSON output to file (requires --json)\n\nCOLLECTED CONTEXT:\n    Capability/requirement starts include traversed elements, authored concept references, requirement contracts, contract bindings, and contract references.\n    Ontology starts include ontology hierarchy and downstream semantic contracts that use reachable ontology.\n    Concept-scheme/concept starts include thesaurus context through concept hierarchy."
    )]
    Collect {
        /// Exact name or canonical identifier of the capability, requirement, ontology, semantic-query, concept-scheme, or concept element to collect from
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
}

#[derive(Subcommand, Debug)]
pub enum SemanticCommands {
    /// Manage authored SPARQL query artifacts
    Query {
        #[clap(subcommand)]
        command: QueryCommands,
    },
    /// Export selected semantic RDF layers
    #[clap(
        override_help = "Export selected semantic RDF layers\n\nSEMANTIC EXPORT OPTIONS:\n      --layer <LAYER>            Include layer: ontologies, shapes, concepts, queries, model, external-used, prefixes. Repeatable; omitted means all layers\n      --jsonld                   Output JSON-LD RDF format instead of RDF/Turtle (.ttl)\n      --namespace-base <IRI>     Filter clean authored export to one ontology base or term namespace. Cannot be combined with model\n      --output <FILE>            Save output to file"
    )]
    Export {
        /// Include layer: ontologies, shapes, concepts, queries, model, external-used, prefixes. Repeatable; omitted means all layers
        #[clap(
            long,
            value_enum,
            value_name = "LAYER",
            help_heading = "SEMANTIC EXPORT OPTIONS"
        )]
        layer: Vec<SemanticLayerArg>,

        /// Output JSON-LD format instead of RDF/Turtle (.ttl)
        #[clap(long, help_heading = "SEMANTIC EXPORT OPTIONS")]
        jsonld: bool,

        /// Filter clean authored export to one ontology base or term namespace. Cannot be combined with model
        #[clap(long, value_name = "IRI", help_heading = "SEMANTIC EXPORT OPTIONS")]
        namespace_base: Option<String>,

        /// Save output to file
        #[clap(long, value_name = "FILE", help_heading = "SEMANTIC EXPORT OPTIONS")]
        output: Option<String>,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum SemanticLayerArg {
    Queries,
    Ontologies,
    Shapes,
    Concepts,
    Model,
    ExternalUsed,
    Prefixes,
}

impl From<SemanticLayerArg> for SemanticExportLayer {
    fn from(value: SemanticLayerArg) -> Self {
        match value {
            SemanticLayerArg::Queries => Self::Queries,
            SemanticLayerArg::Ontologies => Self::Ontologies,
            SemanticLayerArg::Shapes => Self::Shapes,
            SemanticLayerArg::Concepts => Self::Concepts,
            SemanticLayerArg::Model => Self::Model,
            SemanticLayerArg::ExternalUsed => Self::ExternalUsed,
            SemanticLayerArg::Prefixes => Self::Prefixes,
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum ConceptCommands {
    /// Export generated SKOS concept scheme/thesaurus triples
    #[clap(
        override_help = "Export generated SKOS concept scheme/thesaurus triples\n\nCONCEPTS EXPORT OPTIONS:\n      --jsonld             Output JSON-LD RDF format instead of RDF/Turtle (.ttl)\n      --include-mappings   Include structural reqvire:mapsToConcept bridge triples that point to generated native concepts\n      --output <FILE>      Save output to file"
    )]
    Export {
        /// Output JSON-LD format instead of RDF/Turtle (.ttl)
        #[clap(long, help_heading = "CONCEPTS EXPORT OPTIONS")]
        jsonld: bool,

        /// Include structural reqvire:mapsToConcept bridge triples that point to generated native concepts
        #[clap(long, help_heading = "CONCEPTS EXPORT OPTIONS")]
        include_mappings: bool,

        /// Save output to file
        #[clap(long, value_name = "FILE", help_heading = "CONCEPTS EXPORT OPTIONS")]
        output: Option<String>,
    },

    /// Validate standalone concept schemes, concept references, and mappings
    #[clap(
        override_help = "Validate standalone concept schemes, concept references, and mappings\n\nCONCEPTS VALIDATE OPTIONS:\n      --json           Output validation result in JSON format\n      --output <FILE>  Save JSON output to file (requires --json)"
    )]
    Validate {
        /// Output validation result in JSON format
        #[clap(long, help_heading = "CONCEPTS VALIDATE OPTIONS")]
        json: bool,

        /// Save JSON output to file
        #[clap(long, value_name = "FILE", help_heading = "CONCEPTS VALIDATE OPTIONS")]
        output: Option<String>,
    },
}

impl Args {
    fn try_parse_args_from<I, T>(args: I) -> Result<Self, clap::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let matches = Self::command().try_get_matches_from(args)?;
        if let Some(mcp) = matches.subcommand_matches("mcp") {
            if mcp.get_one::<McpTransport>("transport") == Some(&McpTransport::Stdio) {
                for (id, flag) in [
                    ("host", "--host"),
                    ("port", "--port"),
                    ("allowed_origins", "--allow-origin"),
                    ("allowed_hosts", "--allow-host"),
                ] {
                    // Explicit default values are still HTTP-only options.
                    if mcp.value_source(id) == Some(clap::parser::ValueSource::CommandLine) {
                        return Err(Self::command().error(
                            clap::error::ErrorKind::ArgumentConflict,
                            format!(
                                "{flag} is HTTP-only and cannot be used with --transport stdio"
                            ),
                        ));
                    }
                }
            }
        }
        Self::from_arg_matches(&matches)
    }

    pub fn parse_args() -> Self {
        // Check if help was requested before parsing
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 && (args[1] == "--help" || args[1] == "-h" || args[1] == "help") {
            let cmd = Self::command();
            print_custom_help(&cmd);
            std::process::exit(0);
        }
        match Self::try_parse_args_from(&args) {
            Ok(args) => args,
            Err(error) => {
                use clap::error::{ContextKind, ContextValue};
                let replacement = match error.get(ContextKind::InvalidSubcommand) {
                    Some(ContextValue::String(command)) => match command.as_str() {
                        "add" => Some("reqvire.add_element"),
                        "rm" => Some("reqvire.remove_element"),
                        "mv" => Some("reqvire.move_element"),
                        "rename" => Some("reqvire.rename_element"),
                        "merge" => Some("reqvire.merge_elements"),
                        "mv-file" => Some("reqvire.move_file"),
                        "mv-folder" => Some("reqvire.move_folder"),
                        "link" => Some("reqvire.link"),
                        "unlink" => Some("reqvire.unlink"),
                        "relink" => Some("reqvire.relink"),
                        "mv-asset" => Some("reqvire.move_asset"),
                        "rm-asset" => Some("reqvire.remove_asset"),
                        "format" => Some("reqvire.format"),
                        _ => None,
                    },
                    _ => match error.get(ContextKind::InvalidArg) {
                        Some(ContextValue::String(argument))
                            if argument == "--fix" && args.iter().any(|arg| arg == "lint") =>
                        {
                            Some("reqvire.lint")
                        }
                        _ => None,
                    },
                };
                if let Some(tool) = replacement {
                    eprintln!("Use the typed MCP tool `{tool}` for this operation. Start `reqvire mcp --enable-mutations` for authoring and repair, including CRUD dry-run previews. MCP format preview and lint inspection work without mutation enablement; applying format or lint fixes requires `fix: true`. Use CLI `migrate` for source upgrades.");
                }
                error.exit()
            }
        }
    }

    pub fn print_help() {
        let cmd = Self::command();
        print_custom_help(&cmd);
    }
}

pub fn apply_workspace(workspace: Option<&PathBuf>) -> Result<(), ReqvireError> {
    let Some(workspace) = workspace else {
        return Ok(());
    };

    let canonical = std::fs::canonicalize(workspace).map_err(|e| {
        ReqvireError::PathError(format!(
            "Failed to resolve workspace '{}': {}",
            workspace.display(),
            e
        ))
    })?;

    if !canonical.is_dir() {
        return Err(ReqvireError::PathError(format!(
            "Workspace '{}' is not a directory",
            canonical.display()
        )));
    }

    std::env::set_current_dir(&canonical).map_err(|e| {
        ReqvireError::PathError(format!(
            "Failed to enter workspace '{}': {}",
            canonical.display(),
            e
        ))
    })?;

    git_commands::clear_git_cache();
    Ok(())
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

        println!("  {:<17} {}", name, about);
        // Check if this command has subcommands (like verifications)
        if subcommand.has_subcommands() {
            // List nested subcommands indented
            for nested in subcommand.get_subcommands() {
                let nested_name = format!("{} {}", name, nested.get_name());
                let nested_about = nested
                    .get_about()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                println!("    {:<15} {}", nested_name, nested_about);
            }
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    diagnostics: Vec<reqvire::ValidationDiagnostic>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    migration_candidates: Vec<migrations::MigrationCandidate>,
}

/// Helper function to print validation results
pub(crate) fn print_validation_results(errors: &[ReqvireError], json_output: bool) {
    let migration_plan = migrations::candidates_for_validation_errors(errors);

    if json_output {
        let mut error_strings: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        error_strings.sort(); // Sort for deterministic output
        let json_result = ValidationResult {
            errors: error_strings,
            diagnostics: ReqvireError::errors_to_diagnostics(errors),
            migration_candidates: migration_plan.candidates,
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&json_result).expect("failed to serialize JSON")
        );
    } else {
        println!("\n❌ {} validation failed with error(s):", errors.len());
        println!();
        for (i, error) in errors.iter().enumerate() {
            println!("  {}. {}", i + 1, error);
            println!();
        }
        if !migration_plan.is_empty() {
            println!("Migration candidates:");
            for candidate in migration_plan.candidates {
                println!("  - {}: {}", candidate.id, candidate.summary);
                println!("    Preview with `reqvire migrate`; apply with `reqvire migrate --fix`.");
            }
            println!();
        }
        println!();
    }
}

pub(crate) const fn wants_json(args: &Args) -> bool {
    match &args.command {
        Some(Commands::Semantic {
            command: SemanticCommands::Query { command },
        }) => match command {
            QueryCommands::List { json, .. }
            | QueryCommands::Validate { json, .. }
            | QueryCommands::Export { json, .. }
            | QueryCommands::Check { json, .. } => *json,
        },
        Some(Commands::Migrate { json, .. }) => *json,
        Some(Commands::Validate { json, .. }) => *json,
        Some(Commands::ChangeImpact { json, .. }) => *json,
        Some(Commands::Search(search)) => search.json,
        Some(Commands::Traces { .. }) => true,
        Some(Commands::Coverage { json, .. }) => *json,
        Some(Commands::Model { .. }) => true,
        Some(Commands::Lint { json, .. }) => *json,
        Some(Commands::Submodels { json, .. }) => *json,
        Some(Commands::Containment { .. }) => true,
        Some(Commands::Resources { .. }) => true,
        Some(Commands::Concepts {
            command: ConceptCommands::Validate { json, .. },
        }) => *json,
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

const fn semantic_export_format(jsonld: bool) -> SemanticExportFormat {
    if jsonld {
        SemanticExportFormat::JsonLd
    } else {
        SemanticExportFormat::Turtle
    }
}

fn write_or_print_semantic_output(
    output: Option<&String>,
    output_content: String,
) -> Result<(), ReqvireError> {
    if let Some(path) = output {
        std::fs::write(path, output_content).map_err(|e| {
            ReqvireError::ProcessError(format!("Failed to write output file '{}': {}", path, e))
        })?;
        println!("✅ Output saved to {}", path);
    } else {
        print!("{}", output_content);
    }
    Ok(())
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

    // Early validation: --output requires a machine/raw output mode.
    // Check before model parsing so we can fail fast with a clear error
    if let Some(ref cmd) = args.command {
        let (has_output, has_json) = match cmd {
            Commands::Migrate { output, json, .. } => (output.is_some(), *json),
            Commands::Validate { output, json, .. } => (output.is_some(), *json),
            Commands::Search(search) => (search.output.is_some(), search.json),
            Commands::ChangeImpact { output, json, .. } => (output.is_some(), *json),
            Commands::Traces { output, .. } => (output.is_some(), true),
            Commands::Coverage { output, json, .. } => (output.is_some(), *json),
            Commands::Model { output, .. } => (output.is_some(), true),
            Commands::Lint { output, json, .. } => (output.is_some(), *json),
            Commands::Containment { output, .. } => (output.is_some(), true),
            Commands::Resources { output, .. } => (output.is_some(), true),
            Commands::Ontologies { .. } => (false, false),
            Commands::Concepts { command } => match command {
                ConceptCommands::Export { output, .. } => (output.is_some(), true),
                ConceptCommands::Validate { output, json } => (output.is_some(), *json),
            },
            Commands::Submodels { output, json, .. } => (output.is_some(), *json),
            Commands::Collect { output, json, .. } => (output.is_some(), *json),
            _ => (false, false),
        };
        if has_output && !has_json {
            eprintln!("error: --output requires --json");
            return Ok(1);
        }
    }

    if let Some(Commands::Mcp {
        transport,
        host,
        port,
        enable_mutations,
        enable_commits,
        enable_github,
        github_remote,
        with_size_estimates,
        allowed_origins,
        allowed_hosts,
    }) = args.command
    {
        let options = mcp::McpOptions {
            enable_mutations,
            enable_commits,
            enable_github,
            github_remote: github_remote.as_deref().unwrap_or("origin"),
            with_size_estimates,
        };
        return match transport {
            McpTransport::Http => {
                mcp::serve_http(
                    options,
                    excluded_filename_patterns,
                    &host,
                    port,
                    &crate::mcp_http::HttpAccess::new(&allowed_origins, &allowed_hosts),
                )
                .await
            }
            McpTransport::Stdio => mcp::serve_stdio(options, excluded_filename_patterns).await,
        }
        .map(|_| 0);
    }

    if let Some(Commands::Serve {
        host,
        port,
        enable_mcp,
        mcp_enable_mutations,
        mcp_enable_commits,
        mcp_enable_github,
        mcp_github_remote,
        allowed_origins,
        allowed_hosts,
    }) = args.command
    {
        reqvire::utils::enable_quiet_mode();
        return serve::serve_explorer(
            &host,
            port,
            enable_mcp,
            mcp::McpOptions {
                enable_mutations: mcp_enable_mutations,
                enable_commits: mcp_enable_commits,
                enable_github: mcp_enable_github,
                github_remote: mcp_github_remote.as_deref().unwrap_or("origin"),
                with_size_estimates: false,
            },
            excluded_filename_patterns,
            &crate::mcp_http::HttpAccess::new(&allowed_origins, &allowed_hosts),
        )
        .await
        .map(|_| 0);
    }

    let mut model_manager = ModelManager::new();
    let is_lint_command = matches!(args.command, Some(Commands::Lint { .. }));
    let is_migrate_command = matches!(args.command, Some(Commands::Migrate { .. }));
    let with_size_estimates = matches!(
        args.command,
        Some(Commands::Model {
            with_size_estimates: true,
            ..
        })
    );
    let parse_result = model_manager.parse_and_validate_with_options(
        None,
        excluded_filename_patterns,
        ModelBuildOptions {
            lenient: is_lint_command || is_migrate_command,
            with_size_estimates,
        },
    );

    let json_output = wants_json(&args);

    if let Some(Commands::Semantic {
        command:
            SemanticCommands::Query {
                command: QueryCommands::Validate { name, iri, json },
            },
    }) = &args.command
    {
        let index = model_manager
            .semantic_index()
            .map(std::borrow::Cow::Borrowed)
            .unwrap_or_else(|| {
                std::borrow::Cow::Owned(semantic_contract::build_semantic_index(
                    &model_manager.graph_registry,
                ))
            });
        let diagnostics: Vec<_> = parse_result
            .as_ref()
            .err()
            .map(ReqvireError::diagnostics)
            .unwrap_or_default()
            .into_iter()
            .filter(|diagnostic| !diagnostic.guidance.is_empty())
            .collect();
        let model_errors = parse_result
            .as_ref()
            .err()
            .map(|e| {
                if diagnostics.is_empty() {
                    vec![e.to_string()]
                } else {
                    diagnostics.iter().map(ToString::to_string).collect()
                }
            })
            .unwrap_or_default();
        let mut report =
            index.query_validation_report(name.as_deref(), iri.as_deref(), model_errors)?;
        if !diagnostics.is_empty() {
            report["diagnostics"] = serde_json::to_value(diagnostics)?;
        }
        print_query_validation_report(&report, *json)?;
        return Ok(if report["valid"] == true { 0 } else { 1 });
    }
    // Handle validation failures for all commands (including validate)
    match &parse_result {
        Err(ReqvireError::ValidationError(errors)) => {
            print_validation_results(errors, json_output);
            return Ok(1);
        }
        Err(ReqvireError::ValidationDiagnostics { related_errors, .. }) => {
            print_validation_results(related_errors, json_output);
            return Ok(1);
        }
        Err(e) => {
            if json_output {
                let json_result = ValidationResult {
                    errors: vec![e.to_string()],
                    diagnostics: e.diagnostics(),
                    migration_candidates: Vec::new(),
                };
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json_result).expect("failed to serialize JSON")
                );
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
                let json_result = ValidationResult {
                    errors: vec![],
                    diagnostics: Vec::new(),
                    migration_candidates: Vec::new(),
                };
                let json_str =
                    serde_json::to_string_pretty(&json_result).expect("failed to serialize JSON");
                handle_json_output(&json_str, &output)?;
            } else {
                println!("✅ No validation issues found");
            }
            Ok(0)
        }
        Some(Commands::Search(search)) => {
            let SearchArgs {
                json,
                output,
                short,
                filter_file,
                filter_name,
                filter_type,
                filter_status,
                filter_priority,
                filter_risk,
                filter_owner,
                filter_content,
                filter_page_content,
                have_relations,
                not_have_relations,
                has_contract_bindings,
                filter_contract_bindings,
                has_contract_references,
                filter_contract_references,
            } = *search;
            // Build search filters
            let filters = reqvire::search::SearchFilters::new(
                filter_file.as_deref(),
                filter_name.as_deref(),
                filter_type.as_deref(),
                filter_status.as_deref(),
                filter_priority.as_deref(),
                filter_risk.as_deref(),
                filter_owner.as_deref(),
                filter_content.as_deref(),
                filter_page_content.as_deref(),
                have_relations.as_deref(),
                not_have_relations.as_deref(),
                has_contract_bindings,
                filter_contract_bindings.as_deref(),
            )?
            .with_contract_references(
                has_contract_references,
                filter_contract_references.as_deref(),
            )?;

            // Generate search report
            let report_output =
                operations::search_report(&model_manager.graph_registry, &filters, json, short)?;

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
            let (report, base_url, current_commit) = operations::change_impact_report(
                &model_manager,
                &git_commit,
                excluded_filename_patterns,
            )?;

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
        Some(Commands::Migrate { fix, json, output }) => {
            let dry_run = !fix;
            let (documents_summary, documents_diffs) =
                migrations::apply_documents_header_migration(excluded_filename_patterns, dry_run)?;
            let (contract_bindings_summary, contract_bindings_diffs) =
                migrations::apply_contract_bindings_section_migration(
                    excluded_filename_patterns,
                    dry_run,
                )?;
            let (concept_reference_summary, concept_reference_diffs) =
                migrations::apply_concept_reference_link_migration(
                    &model_manager.graph_registry,
                    excluded_filename_patterns,
                    dry_run,
                )?;

            if !dry_run
                && (documents_summary.files_changed > 0
                    || contract_bindings_summary.files_changed > 0
                    || concept_reference_summary.references_rewritten > 0)
            {
                model_manager.parse_and_validate_with_options(
                    None,
                    excluded_filename_patterns,
                    ModelBuildOptions {
                        lenient: true,
                        with_size_estimates: false,
                    },
                )?;
            }

            let contract_summary =
                migrations::apply_contract_relation_migration(&mut model_manager.graph_registry)?;
            let verification_summary = migrations::apply_verification_objective_holders(
                &mut model_manager.graph_registry,
            )?;
            let mut format_result = format_files(&model_manager.graph_registry, dry_run, false)?;
            format_result.files_changed += documents_diffs.len();
            format_result.diffs.extend(documents_diffs);
            format_result.files_changed += contract_bindings_diffs.len();
            format_result.diffs.extend(contract_bindings_diffs);
            format_result.files_changed += concept_reference_diffs.len();
            format_result.diffs.extend(concept_reference_diffs);
            format_result
                .diffs
                .sort_by(|a, b| a.file_path.cmp(&b.file_path));

            if json {
                let diff_value: serde_json::Value =
                    serde_json::from_str(&render_diff_json(&format_result)).map_err(|e| {
                        ReqvireError::ProcessError(format!(
                            "Failed to serialize migration diff: {}",
                            e
                        ))
                    })?;
                let json_str = serde_json::to_string_pretty(&serde_json::json!({
                    "migrations": {
                        "documents_header": documents_summary,
                        "contract_bindings_section": contract_bindings_summary,
                        "concept_reference_links": concept_reference_summary,
                        "contract_relations": contract_summary,
                        "verification_objective": verification_summary,
                    },
                    "changes": diff_value,
                }))
                .map_err(|e| {
                    ReqvireError::ProcessError(format!(
                        "Failed to serialize migration result: {}",
                        e
                    ))
                })?;
                handle_json_output(&json_str, &output)?;
            } else if documents_summary.files_changed == 0
                && contract_bindings_summary.files_changed == 0
                && concept_reference_summary.references_rewritten == 0
                && contract_summary.relations_rewritten == 0
                && verification_summary.derive_relations_added == 0
                && verification_summary.objectives_created == 0
            {
                println!("✅ No migration changes needed.");
            } else {
                if dry_run {
                    println!(
                        "Migration preview: {} legacy document header file(s), {} contract bindings heading file(s), {} concept reference rewrite(s), {} contract relation rewrite(s), {} objective holder(s), {} derive relation(s).\n",
                        documents_summary.files_changed,
                        contract_bindings_summary.files_changed,
                        concept_reference_summary.references_rewritten,
                        contract_summary.relations_rewritten,
                        verification_summary.objectives_created,
                        verification_summary.derive_relations_added
                    );
                } else {
                    println!(
                        "Applied migration: {} legacy document header file(s), {} contract bindings heading file(s), {} concept reference rewrite(s), {} contract relation rewrite(s), {} objective holder(s), {} derive relation(s).\n",
                        documents_summary.files_changed,
                        contract_bindings_summary.files_changed,
                        concept_reference_summary.references_rewritten,
                        contract_summary.relations_rewritten,
                        verification_summary.objectives_created,
                        verification_summary.derive_relations_added
                    );
                }
                render_diff(&format_result);
            }
            Ok(0)
        }
        Some(Commands::Traces {
            output,
            filter_id,
            filter_name,
            filter_type,
        }) => {
            // Generate verification traces report (upward paths from verifications to requirements)
            let report = operations::traces_report(
                &model_manager.graph_registry,
                filter_id.as_deref(),
                filter_name.as_deref(),
                filter_type.as_deref(),
            )?;

            let json_str = serde_json::to_string_pretty(&report).map_err(|e| {
                ReqvireError::ProcessError(format!("Failed to serialize report: {}", e))
            })?;
            handle_json_output(&json_str, &output)?;

            Ok(0)
        }
        Some(Commands::Coverage { from, json, output }) => {
            let coverage_report =
                operations::scoped_coverage_report(&model_manager.graph_registry, from.as_deref())?;
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
            with_size_estimates: _,
            output,
        }) => {
            // Parse filter types if provided
            let type_filter: Option<Vec<&str>> = filter_type
                .as_ref()
                .map(|s| s.split(',').map(|t| t.trim()).collect());

            // Generate model-centric report with optional filtering
            let report_output = operations::model_report(
                &model_manager.graph_registry,
                from.as_deref(),
                reverse,
                type_filter,
            )?;
            handle_json_output(&report_output, &output)?;
            Ok(0)
        }
        Some(Commands::Lint {
            fixable,
            auditable,
            json,
            output,
        }) => {
            let report = operations::lint_report(&model_manager.graph_registry);
            if json {
                handle_json_output(&report.to_json_string(fixable, auditable), &output)?;
            } else {
                report.print(false, fixable, auditable);
            }
            Ok(0)
        }
        Some(Commands::Export { output }) => {
            reqvire::utils::enable_quiet_mode();

            let explorer_assets = explorer_runtime::build_runtime_assets(&model_manager)?;

            let output_dir = std::path::Path::new(&output);
            explorer_runtime::export_to_dir(&explorer_assets, output_dir)?;

            println!("Exported Explorer site to: {}", output_dir.display());
            Ok(0)
        }
        Some(Commands::Containment { output, short }) => {
            let hierarchy =
                operations::containment_hierarchy(&model_manager.graph_registry, short)?;
            let json_str = serde_json::to_string_pretty(&hierarchy).map_err(|e| {
                ReqvireError::ElementError(format!("JSON serialization error: {}", e))
            })?;
            handle_json_output(&json_str, &output)?;
            Ok(0)
        }
        Some(Commands::Resources { output }) => {
            handle_json_output(
                &operations::resources_report(&model_manager.graph_registry).to_json_string(),
                &output,
            )?;
            Ok(0)
        }
        Some(Commands::Semantic { command }) => {
            let semantic_store = model_manager
                .semantic_store
                .as_ref()
                .ok_or_else(|| ReqvireError::ProcessError("Missing semantic index".into()))?;
            match command {
                SemanticCommands::Query { command } => {
                    return run_query_command(semantic_store.index(), command);
                }
                SemanticCommands::Export {
                    layer,
                    jsonld,
                    namespace_base,
                    output,
                } => {
                    let format = semantic_export_format(jsonld);
                    let layers = layer
                        .into_iter()
                        .map(SemanticExportLayer::from)
                        .collect::<Vec<_>>();
                    let output_content = semantic_store.serialize_export_layers(
                        format,
                        &layers,
                        namespace_base.as_deref(),
                    )?;
                    write_or_print_semantic_output(output.as_ref(), output_content)?;
                }
            }
            Ok(0)
        }
        Some(Commands::Concepts { command }) => match command {
            ConceptCommands::Export {
                jsonld,
                include_mappings,
                output,
            } => {
                let index = model_manager
                    .semantic_index()
                    .ok_or_else(|| ReqvireError::ProcessError("Missing semantic index".into()))?;
                let format = semantic_export_format(jsonld);
                let output_content = index.serialize_concepts(format, include_mappings)?;
                write_or_print_semantic_output(output.as_ref(), output_content)?;
                Ok(0)
            }
            ConceptCommands::Validate { json, output } => {
                if json {
                    let json_result = ValidationResult {
                        errors: vec![],
                        diagnostics: Vec::new(),
                        migration_candidates: Vec::new(),
                    };
                    let json_str = serde_json::to_string_pretty(&json_result)
                        .expect("failed to serialize JSON");
                    handle_json_output(&json_str, &output)?;
                } else {
                    println!("✅ Concept validation passed");
                }
                Ok(0)
            }
        },
        Some(Commands::Ontologies {
            jsonld,
            full,
            include_external,
            namespace_base,
            output,
        }) => {
            let semantic_store = model_manager
                .semantic_store
                .as_ref()
                .ok_or_else(|| ReqvireError::ProcessError("Missing semantic index".into()))?;
            let format = if jsonld {
                SemanticExportFormat::JsonLd
            } else {
                SemanticExportFormat::Turtle
            };
            let output_content = semantic_store.serialize_with_options_and_filter(
                format,
                full,
                include_external,
                namespace_base.as_deref(),
            )?;
            if let Some(path) = output {
                std::fs::write(&path, output_content).map_err(|e| {
                    ReqvireError::ProcessError(format!(
                        "Failed to write output file '{}': {}",
                        path, e
                    ))
                })?;
                println!("✅ Output saved to {}", path);
            } else {
                print!("{}", output_content);
            }
            Ok(0)
        }
        Some(Commands::Submodels { from, json, output }) => {
            let report =
                operations::submodels_report(&model_manager.graph_registry, from.as_deref())?;
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
                "UPSTREAM" => report::collect::CollectDirection::Upstream,
                "DOWNSTREAM" => report::collect::CollectDirection::Downstream,
                _ => {
                    eprintln!(
                        "error: invalid direction '{}'. Valid values: UPSTREAM, DOWNSTREAM",
                        direction
                    );
                    return Ok(1);
                }
            };
            let report_output = operations::collect_report(
                &model_manager.graph_registry,
                &element_name,
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
        Some(Commands::Mcp { .. } | Commands::Serve { .. }) => {
            unreachable!("Server commands are handled before model parsing")
        }
        None => {
            // This case is handled at the beginning of handle_command
            unreachable!("Command is None but should have been handled earlier");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use globset::Glob;
    use reqvire::exclusions::{ExclusionSet as GlobSet, ExclusionSetBuilder as GlobSetBuilder};

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
    fn mcp_transport_options_reject_explicit_http_flags_only_for_stdio() {
        for (flag, value) in [
            ("--host", "127.0.0.1"),
            ("--port", "8081"),
            ("--allow-origin", "https://app.example"),
            ("--allow-host", "mcp.example"),
        ] {
            for transport in ["http", "stdio"] {
                let result = Args::try_parse_args_from([
                    "reqvire",
                    "mcp",
                    "--transport",
                    transport,
                    flag,
                    value,
                ]);
                if transport == "stdio" {
                    let error = result.expect_err("HTTP-only option accepted with stdio");
                    assert_eq!(error.kind(), clap::error::ErrorKind::ArgumentConflict);
                    assert!(error.to_string().contains(flag));
                } else {
                    assert!(result.is_ok());
                }
            }
        }
        assert!(matches!(
            Args::try_parse_args_from(["reqvire", "mcp"])
                .expect("default HTTP transport should parse")
                .command,
            Some(Commands::Mcp {
                transport: McpTransport::Http,
                ..
            })
        ));
        assert!(Args::try_parse_args_from(["reqvire", "mcp", "--transport", "stdio"]).is_ok());
    }

    #[test]
    fn mcp_cors_cli_accepts_repeated_origins() {
        for command in [
            vec!["reqvire", "mcp"],
            vec!["reqvire", "serve", "--enable-mcp"],
        ] {
            let mut args = command;
            args.extend([
                "--allow-origin",
                "https://app.example",
                "--allow-origin",
                "http://192.0.2.10:3000",
            ]);
            assert!(Args::try_parse_from(args).is_ok());
        }
    }

    #[test]
    fn mcp_cors_cli_rejects_invalid_origins_and_missing_embedded_endpoint() {
        for prefix in [
            vec!["reqvire", "mcp"],
            vec!["reqvire", "serve", "--enable-mcp"],
        ] {
            for value in [
                "*",
                "null",
                "https://app.example/path",
                "https://app.example:65536",
            ] {
                let mut args = prefix.clone();
                args.extend(["--allow-origin", value]);
                let error = Args::try_parse_from(args)
                    .expect_err("reject invalid MCP origin configuration");
                assert_eq!(error.kind(), clap::error::ErrorKind::ValueValidation);
            }
        }
        let error =
            Args::try_parse_from(["reqvire", "serve", "--allow-origin", "https://app.example"])
                .expect_err("reject invalid MCP origin configuration");
        assert_eq!(
            error.kind(),
            clap::error::ErrorKind::MissingRequiredArgument
        );
    }

    #[test]
    fn mcp_host_cli_accepts_aliases_and_rejects_invalid_configuration() {
        for prefix in [
            vec!["reqvire", "mcp"],
            vec!["reqvire", "serve", "--enable-mcp"],
        ] {
            let mut args = prefix.clone();
            args.extend([
                "--host",
                "0.0.0.0",
                "--allow-host",
                "mcp.example",
                "--allow-host",
                "192.0.2.50:8081",
            ]);
            assert!(Args::try_parse_from(args).is_ok());
            for invalid in [
                "*",
                "0.0.0.0",
                "https://mcp.example",
                "mcp.example/path",
                "mcp.example:65536",
            ] {
                let mut args = prefix.clone();
                args.extend(["--allow-host", invalid]);
                assert_eq!(
                    Args::try_parse_from(args)
                        .expect_err("reject invalid host")
                        .kind(),
                    clap::error::ErrorKind::ValueValidation
                );
            }
        }
        assert_eq!(
            Args::try_parse_from(["reqvire", "serve", "--allow-host", "mcp.example"])
                .expect_err("require embedded MCP")
                .kind(),
            clap::error::ErrorKind::MissingRequiredArgument
        );
    }

    #[test]
    fn test_cli_parsing_subcommand() {
        let args = Args::parse_from(["reqvire", "serve", "--host", "127.0.0.1", "--port", "9000"]);
        assert!(matches!(
            args.command,
            Some(Commands::Serve { host, port, enable_mcp, mcp_enable_mutations, .. })
                if host == "127.0.0.1" && port == 9000 && !enable_mcp && !mcp_enable_mutations
        ));
    }

    #[test]
    fn parses_serve_with_embedded_mcp() {
        let args = Args::parse_from(["reqvire", "serve", "--enable-mcp", "--enable-mutations"]);
        assert!(matches!(
            args.command,
            Some(Commands::Serve {
                enable_mcp: true,
                mcp_enable_mutations: true,
                ..
            })
        ));
    }

    #[tokio::test]
    async fn test_handle_command() {
        // Mock CLI arguments
        let args = Args {
            workspace: None,
            manifest: None,
            command: Some(Commands::Validate {
                json: false,
                output: None,
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

    #[test]
    fn workspace_flag_is_global() {
        let args = Args::parse_from(["reqvire", "--workspace", "/tmp", "validate"]);
        assert_eq!(args.workspace, Some(PathBuf::from("/tmp")));
        assert!(matches!(args.command, Some(Commands::Validate { .. })));
    }

    #[test]
    fn apply_workspace_rejects_missing_directory() {
        let missing = PathBuf::from("/definitely/not/a/reqvire/workspace");
        let err = apply_workspace(Some(&missing)).expect_err("expected the operation to fail");
        assert!(err.to_string().contains("Failed to resolve workspace"));
    }
}

#[derive(Subcommand, Debug)]
pub enum QueryCommands {
    /// List native query artifacts and their source metadata
    List {
        /// Exact semantic-query source element name or canonical identifier
        #[clap(long, conflicts_with = "iri")]
        name: Option<String>,
        #[clap(long)]
        iri: Option<String>,
        #[clap(long)]
        namespace_base: Option<String>,
        #[clap(long)]
        json: bool,
    },
    /// Validate query artifacts without executing them
    Validate {
        /// Exact semantic-query source element name or canonical identifier
        #[clap(long, conflicts_with = "iri")]
        name: Option<String>,
        #[clap(long)]
        iri: Option<String>,
        #[clap(long)]
        json: bool,
    },
    /// Export a standalone SPARQL artifact
    Export {
        /// Exact semantic-query source element name or canonical identifier
        #[clap(long, required_unless_present = "iri", conflicts_with = "iri")]
        name: Option<String>,
        #[clap(long, required_unless_present = "name")]
        iri: Option<String>,
        #[clap(long, conflicts_with = "json")]
        output: Option<std::path::PathBuf>,
        #[clap(long)]
        json: bool,
    },
    /// Compare a generated query with an existing artifact
    Check {
        /// Exact semantic-query source element name or canonical identifier
        #[clap(long, required_unless_present = "iri", conflicts_with = "iri")]
        name: Option<String>,
        #[clap(long, required_unless_present = "name")]
        iri: Option<String>,
        #[clap(long)]
        artifact: std::path::PathBuf,
        #[clap(long)]
        json: bool,
    },
}

fn run_query_command(
    index: &semantic_contract::SemanticIndex,
    command: QueryCommands,
) -> Result<i32, ReqvireError> {
    match command {
        QueryCommands::List {
            name,
            iri,
            namespace_base,
            json,
        } => {
            let records =
                index.select_queries(name.as_deref(), iri.as_deref(), namespace_base.as_deref())?;
            if json {
                println!("{}", serde_json::to_string_pretty(&records)?);
            } else {
                for record in records {
                    println!(
                        "{}\t{}\t{}",
                        record.iri,
                        record.query_form.as_deref().unwrap_or("invalid"),
                        record.name
                    );
                }
            }
        }
        QueryCommands::Validate { name, iri, json } => {
            let report =
                index.query_validation_report(name.as_deref(), iri.as_deref(), Vec::new())?;
            print_query_validation_report(&report, json)?;
            return Ok(if report["valid"] == true { 0 } else { 1 });
        }
        QueryCommands::Export {
            name,
            iri,
            output,
            json,
        } => {
            let records = index.select_queries(name.as_deref(), iri.as_deref(), None)?;
            let record = records[0];
            let envelope = record.artifact()?;
            if let Some(path) = output {
                use std::io::Write;
                let parent = path
                    .parent()
                    .filter(|p| !p.as_os_str().is_empty())
                    .unwrap_or_else(|| std::path::Path::new("."));
                let mut temp = tempfile::NamedTempFile::new_in(parent)?;
                temp.write_all(record.content.as_bytes())?;
                temp.as_file().sync_all()?;
                temp.persist(&path).map_err(|e| {
                    ReqvireError::ProcessError(format!("Cannot write {}: {}", path.display(), e))
                })?;
                eprintln!("Wrote {}", path.display());
            } else if json {
                println!("{}", serde_json::to_string_pretty(&envelope)?);
            } else {
                print!("{}", record.content);
            }
        }
        QueryCommands::Check {
            name,
            iri,
            artifact,
            json,
        } => {
            let records = index.select_queries(name.as_deref(), iri.as_deref(), None)?;
            let expected = records[0].artifact()?;
            let (status, actual) = match std::fs::read(&artifact) {
                Ok(bytes) => (
                    if bytes == records[0].content.as_bytes() {
                        "matching"
                    } else {
                        "stale"
                    },
                    Some(reqvire::hashing::sha256_hex(&bytes)),
                ),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => ("missing", None),
                Err(error) => {
                    return Err(ReqvireError::ProcessError(format!(
                        "Cannot read {}: {}",
                        artifact.display(),
                        error
                    )))
                }
            };
            let result = serde_json::json!({"status":status,"expected_sha256":expected["sha256"],"actual_sha256":actual});
            if json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!("{}", result);
            }
            return Ok(if status == "matching" { 0 } else { 1 });
        }
    }
    Ok(0)
}

fn print_query_validation_report(
    report: &serde_json::Value,
    json: bool,
) -> Result<(), ReqvireError> {
    if json {
        println!("{}", serde_json::to_string_pretty(report)?);
    } else {
        for query in report["queries"].as_array().into_iter().flatten() {
            let valid = query["diagnostics"]
                .as_array()
                .is_some_and(|d| d.is_empty());
            println!(
                "{}: {}",
                query["name"].as_str().unwrap_or_default(),
                if valid { "valid" } else { "invalid" }
            );
            for diagnostic in query["diagnostics"].as_array().into_iter().flatten() {
                eprintln!("{}", diagnostic);
            }
        }
        for error in report["model_errors"].as_array().into_iter().flatten() {
            eprintln!("{}", error.as_str().unwrap_or_default());
        }
    }
    Ok(())
}
