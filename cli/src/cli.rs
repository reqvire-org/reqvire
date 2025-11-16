use clap::{Parser, Subcommand, CommandFactory};
use std::path::PathBuf;
use anyhow::Result;
use log::{info};
use serde::Serialize;
use reqvire::error::ReqvireError;
use reqvire::ModelManager;
use globset::GlobSet;
use reqvire::report_coverage;
use reqvire::report_model;
use reqvire::diagrams;
use reqvire::export;
use reqvire::change_impact;
use reqvire::git_commands;
use reqvire::matrix_generator;
use reqvire::verification_trace;
use crate::serve;
use reqvire::lint;
use reqvire::GraphRegistry;
use reqvire::graph_registry::{Page, Section};
use reqvire::element::Element;
use reqvire::format::{format_files, render_diff, render_diff_json};
use reqvire::diff::{render_crud_result, render_crud_json};
use reqvire::crud;
use std::collections::HashMap;
use std::path::Path;


#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Reqvire requirements & treacibility management tool",
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
        /// Output directory for HTML files
        #[clap(long, short = 'o', default_value = "html", help_heading = "EXPORT OPTIONS")]
        output: String,
    },

    /// Serve model as browsable HTML documentation via HTTP server
    #[clap(override_help = "Serve model as browsable HTML documentation via HTTP server\n\nSERVE OPTIONS:\n      --host <HOST>          Bind address (default: localhost)\n      --port <PORT>          Server port (default: 8080)")]
    Serve {
        /// Bind address
        #[clap(long, default_value = "localhost", help_heading = "SERVE OPTIONS")]
        host: String,

        /// Server port
        #[clap(long, default_value = "8080", help_heading = "SERVE OPTIONS")]
        port: u16,
    },

    /// Format and normalize requirements files. By default, shows preview without applying changes
    #[clap(override_help = "Format and normalize requirements files. By default, shows preview without applying changes\n\nFORMAT OPTIONS:\n      --fix      Apply formatting changes to files\n      --json     Output results in JSON format")]
    Format {
        /// Apply formatting changes to files
        #[clap(long, help_heading = "FORMAT OPTIONS")]
        fix: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "FORMAT OPTIONS")]
        json: bool,
    },

    /// Validate model
    #[clap(override_help = "Validate model\n\nVALIDATION OPTIONS:\n      --json     Output results in JSON format")]
    Validate {
        /// Output results in JSON format
        #[clap(long, help_heading = "VALIDATION OPTIONS")]
        json: bool,
    },
    

    /// Generate mermaid diagrams in markdown files showing requirements relationships. Diagrams are placed at the top of each section
    #[clap(override_help = "Generate mermaid diagrams in markdown files showing requirements relationships. Diagrams are placed at the top of each section\n\nGENERATE-DIAGRAMS OPTIONS:\n      --links-with-blobs     Use GitHub blob URLs in diagram links instead of relative paths")]
    GenerateDiagrams {
        /// Use GitHub blob URLs in diagram links instead of relative paths
        #[clap(long, help_heading = "GENERATE-DIAGRAMS OPTIONS")]
        links_with_blobs: bool,
    },

    /// Remove all generated mermaid diagrams from markdown files
    RemoveDiagrams,

    /// Search and filter model elements with comprehensive filtering options
    #[clap(override_help = "Search and filter model elements with comprehensive filtering options\n\nSEARCH OPTIONS:\n      --json                            Output results in JSON format\n      --short                           Output abbreviated format (one-line per element)\n      --filter-file <GLOB>              Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`\n      --filter-name <REGEX>             Only include elements whose name matches this regular expression\n      --filter-section <GLOB>           Only include sections whose name matches this glob pattern e.g. `System requirement*`\n      --filter-type <TYPE>              Only include elements of the given type e.g. `user-requirement`, `system-requirement`, `verification`\n      --filter-content <REGEX>          Only include elements whose content matches this regular expression\n      --filter-section-content <REGEX>  Only include elements whose parent section content matches this regular expression\n      --filter-page-content <REGEX>     Only include elements whose parent file page content matches this regular expression\n      --have-relations <LIST>           Only include elements that have ALL specified relations (comma-separated)\n      --not-have-relations <LIST>       Only include elements that do NOT have ALL specified relations (comma-separated)")]
    Search {
        /// Output results in JSON format
        #[clap(long, help_heading = "SEARCH OPTIONS")]
        json: bool,

        /// Output abbreviated format (one-line per element in text, omit fields in JSON)
        #[clap(long, help_heading = "SEARCH OPTIONS")]
        short: bool,

        /// Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`
        #[clap(long, value_name = "GLOB", help_heading = "SEARCH OPTIONS")]
        filter_file: Option<String>,

        /// Only include elements whose name matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "SEARCH OPTIONS")]
        filter_name: Option<String>,

        /// Only include sections whose name matches this glob pattern e.g. `System requirement*`
        #[clap(long, value_name = "GLOB", help_heading = "SEARCH OPTIONS")]
        filter_section: Option<String>,

        /// Only include elements of the given type e.g. `user-requirement`, `system-requirement`, `verification`
        #[clap(long, value_name = "TYPE", help_heading = "SEARCH OPTIONS")]
        filter_type: Option<String>,

        /// Only include elements whose content matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "SEARCH OPTIONS")]
        filter_content: Option<String>,

        /// Only include elements whose parent section content matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "SEARCH OPTIONS")]
        filter_section_content: Option<String>,

        /// Only include elements whose parent file page content matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "SEARCH OPTIONS")]
        filter_page_content: Option<String>,

        /// Only include elements that have ALL specified relations (comma-separated, e.g., "verifiedBy,satisfiedBy")
        #[clap(long, value_name = "LIST", help_heading = "SEARCH OPTIONS")]
        have_relations: Option<String>,

        /// Only include elements that do NOT have ALL specified relations (comma-separated, e.g., "verifiedBy")
        #[clap(long, value_name = "LIST", help_heading = "SEARCH OPTIONS")]
        not_have_relations: Option<String>,
    },

    /// Analise change impact and provides report
    #[clap(override_help = "Analise change impact and provides report\n\nCHANGE IMPACT OPTIONS:\n      --git-commit <GIT_COMMIT>  Git commit hash to use when comparing models [default: HEAD]\n      --json                     Output results in JSON format")]
    ChangeImpact {
        /// Git commit hash to use when comparing models
        #[clap(long, default_value = "HEAD", help_heading = "CHANGE IMPACT OPTIONS")]
        git_commit: String,

        /// Output results in JSON format
        #[clap(long, help_heading = "CHANGE IMPACT OPTIONS")]
        json: bool,
    },

    /// Generate verification traceability matrix showing requirements and their verification status
    #[clap(override_help = "Generate verification traceability matrix showing requirements and their verification status\n\nMATRIX OPTIONS:\n      --svg                       Output traceability matrix as SVG (cannot be used with --json)\n      --json                      Output results in JSON format")]
    Matrix {
        /// Output traceability matrix as SVG without hyperlinks and with full element names Cannot be used with --json
        #[clap(long, conflicts_with = "json", help_heading = "MATRIX OPTIONS")]
        svg: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "MATRIX OPTIONS")]
        json: bool,
    },

    /// Generate verification traces showing upward paths from verifications to root requirements
    #[clap(override_help = "Generate verification traces showing upward paths from verifications to root requirements\n\nTRACES OPTIONS:\n      --json                      Output results in JSON format\n      --from-folder <PATH>        Generate links relative to this folder path\n      --links-with-blobs          Use GitHub blob URLs in diagram links instead of relative paths\n      --filter-id <ID>            Only include verification with this specific identifier\n      --filter-name <REGEX>       Only include verifications whose name matches this regular expression\n      --filter-type <TYPE>        Only include verifications of the given type e.g. `test-verification`, `analysis-verification`")]
    Traces {
        /// Output results in JSON format
        #[clap(long, help_heading = "TRACES OPTIONS")]
        json: bool,

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

        /// Only include verifications of the given type e.g. `test-verification`, `analysis-verification`
        #[clap(long, value_name = "TYPE", help_heading = "TRACES OPTIONS")]
        filter_type: Option<String>,
    },

    /// Generate verification coverage report for leaf requirements
    #[clap(override_help = "Generate verification coverage report for leaf requirements\n\nCOVERAGE OPTIONS:\n      --json                      Output results in JSON format")]
    Coverage {
        /// Output results in JSON format
        #[clap(long, help_heading = "COVERAGE OPTIONS")]
        json: bool,
    },

    /// Generate model-centric structure with nested relations
    ///
    /// By default, shows root requirements (no hierarchical parent).
    /// Use --from <NAME> to start from specific element.
    ///
    /// Output formats:
    /// - JSON: Nested structure with element details in relations
    /// - Markdown: Mermaid diagrams with all nested relationships
    #[clap(override_help = "Generate model-centric structure with nested relations\n\nBy default, shows root requirements (no hierarchical parent).\nUse --from <NAME> to start from specific element.\n\nOutput formats:\n  - JSON: Nested structure with element details in relations\n  - Markdown: Mermaid diagrams with all nested relationships\n\nMODEL OPTIONS:\n      --from <NAME>               Start from specific element by name\n      --json                      Output results in JSON format (nested structure)")]
    Model {
        /// Start from specific element by name
        #[clap(long, value_name = "NAME", help_heading = "MODEL OPTIONS")]
        from: Option<String>,

        /// Output results in JSON format (nested structure)
        #[clap(long, help_heading = "MODEL OPTIONS")]
        json: bool,
    },

    /// Analyze model quality and detect issues in requirements relations
    #[clap(override_help = "Analyze model quality and detect issues in requirements relations\n\nLINT OPTIONS:\n      --fixable                   Show only auto-fixable issues\n      --auditable                 Show only issues requiring manual review\n      --fix                       Apply automatic fixes for auto-fixable issues\n      --json                      Output results in JSON format")]
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
    },

    /// Add new element to model from Markdown definition
    #[clap(override_help = "Add new element to model from Markdown definition\n\nADD OPTIONS:\n      --to-file <FILE>           Target file path (relative to git repository root)\n      --to-section <SECTION>     Target section name\n      --index <INDEX>            Index within section (0-based, defaults to end)\n      --dry-run                  Preview changes without applying\n      --json                     Output results in JSON format\n\nUSAGE:\n    reqvire add <file> [<section>] [<index>]           # reads element from stdin\n    reqvire add <file> [<section>] [<index>] <element>  # element as last argument\n    reqvire add --to-file=<file> --to-section=<section> --index=<n> < element.md")]
    Add {
        /// Target file path (relative to git repository root)
        #[clap(long, value_name = "FILE", help_heading = "ADD OPTIONS")]
        to_file: Option<String>,

        /// Target section name
        #[clap(long, value_name = "SECTION", help_heading = "ADD OPTIONS")]
        to_section: Option<String>,

        /// Index within section (0-based, defaults to end)
        #[clap(long, value_name = "INDEX", help_heading = "ADD OPTIONS")]
        index: Option<usize>,

        /// Preview changes without applying
        #[clap(long, help_heading = "ADD OPTIONS")]
        dry_run: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "ADD OPTIONS")]
        json: bool,

        /// Positional arguments: [file] [section] [index] [element-markdown]
        #[clap(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Remove element from model
    #[clap(override_help = "Remove element from model\n\nRM OPTIONS:\n      --dry-run     Preview changes without applying\n      --json        Output results in JSON format\n\nUSAGE:\n    reqvire rm <element-id>")]
    Rm {
        /// Element identifier
        element_id: String,

        /// Preview changes without applying
        #[clap(long, help_heading = "RM OPTIONS")]
        dry_run: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "RM OPTIONS")]
        json: bool,
    },

    /// Move element to different location
    #[clap(override_help = "Move element to different location\n\nMV OPTIONS:\n      --to-file <FILE>           Target file path (relative to git repository root)\n      --to-section <SECTION>     Target section name\n      --index <INDEX>            Index within section (0-based, defaults to end)\n      --dry-run                  Preview changes without applying\n      --json                     Output results in JSON format\n\nUSAGE:\n    reqvire mv <element-id> <file> [<section>] [<index>]\n    reqvire mv <element-id> --to-file=<file> --to-section=<section> --index=<n>")]
    Mv {
        /// Element identifier
        element_id: String,

        /// Target file path (relative to git repository root)
        #[clap(long, value_name = "FILE", help_heading = "MV OPTIONS")]
        to_file: Option<String>,

        /// Target section name
        #[clap(long, value_name = "SECTION", help_heading = "MV OPTIONS")]
        to_section: Option<String>,

        /// Index within section (0-based, defaults to end)
        #[clap(long, value_name = "INDEX", help_heading = "MV OPTIONS")]
        index: Option<usize>,

        /// Preview changes without applying
        #[clap(long, help_heading = "MV OPTIONS")]
        dry_run: bool,

        /// Output results in JSON format
        #[clap(long, help_heading = "MV OPTIONS")]
        json: bool,

        /// Positional arguments: [file] [section] [index]
        #[clap(trailing_var_arg = true)]
        args: Vec<String>,
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

    println!("Usage: {} [OPTIONS] <COMMAND> [COMMAND OPTIONS]", cmd.get_name());
    println!();

    // Print commands
    println!("Commands:");
    for subcommand in cmd.get_subcommands() {
        // Skip hidden commands
        if subcommand.is_hide_set() {
            continue;
        }

        let name = subcommand.get_name();
        let about = subcommand.get_about().map(|s| s.to_string()).unwrap_or_default();

        // Check if this command has subcommands (like verifications)
        if subcommand.has_subcommands() {
            println!("  {:<17} {}", name, about);
            // List nested subcommands indented
            for nested in subcommand.get_subcommands() {
                let nested_name = format!("{} {}", name, nested.get_name());
                let nested_about = nested.get_about().map(|s| s.to_string()).unwrap_or_default();
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
            let long = arg.get_long().map(|l| format!("--{}", l)).unwrap_or_default();
            let short = arg.get_short().map(|s| format!("-{}, ", s)).unwrap_or_default();
            let value_name = if arg.get_action().takes_values() {
                let value = arg.get_value_names()
                    .and_then(|v| v.get(0))
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
                        let long = arg.get_long().map(|l| format!("--{}", l)).unwrap_or_default();
                        let value_name = if arg.get_action().takes_values() {
                            let value = arg.get_value_names()
                                .and_then(|v| v.get(0))
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
                    let long = arg.get_long().map(|l| format!("--{}", l)).unwrap_or_default();
                    let value_name = if arg.get_action().takes_values() {
                        let value = arg.get_value_names()
                            .and_then(|v| v.get(0))
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
    errors: Vec<String>
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
        println!("");        
        for (i, error) in errors.iter().enumerate() {
            println!("  {}. {}", i + 1, error);
            println!("");
        }
        println!();
    }
}

fn wants_json(args: &Args) -> bool {
    match &args.command {
        Some(Commands::Format { json, .. }) => *json,
        Some(Commands::Validate { json }) => *json,
        Some(Commands::ChangeImpact { json, .. }) => *json,
        Some(Commands::Search { json, .. }) => *json,
        Some(Commands::Matrix { json, .. }) => *json,
        Some(Commands::Traces { json, .. }) => *json,
        Some(Commands::Coverage { json }) => *json,
        Some(Commands::Model { json, .. }) => *json,
        Some(Commands::Lint { json, .. }) => *json,
        _ => false,
    }
}

pub fn handle_command(
    args: Args,
    excluded_filename_patterns: &GlobSet,
) -> Result<i32,ReqvireError> {

    // If no command provided, show help
    if args.command.is_none() {
        Args::print_help();
        return Ok(0);
    }

    let mut model_manager = ModelManager::new();
    let parse_result = model_manager.parse_and_validate(
        None,
        excluded_filename_patterns
    );

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
        Some(Commands::Validate { json }) => {
            // For validate command, if we get here it means no validation errors
            if json {
                let json_result = ValidationResult {
                    errors: vec![],
                };
                println!("{}", serde_json::to_string_pretty(&json_result).unwrap());
            } else {
                println!("✅ No validation issues found");
            }
            return Ok(0);
        },
        Some(Commands::GenerateDiagrams { links_with_blobs }) => {
            info!("Generating mermaid diagrams");
            // Only collect identifiers and process files to add diagrams
            // Skip validation checks for diagram generation mode
            diagrams::process_diagrams(&model_manager.graph_registry, links_with_blobs)?;

            info!("Requirements diagrams updated in source files");
            return Ok(0);
        },
        Some(Commands::RemoveDiagrams) => {
            info!("Removing generated mermaid diagrams");
            diagrams::remove_diagrams(&model_manager.graph_registry)?;
            info!("Generated diagrams removed from source files");
            return Ok(0);
        },
        Some(Commands::Search {
            json,
            short,
            filter_file,
            filter_name,
            filter_section,
            filter_type,
            filter_content,
            filter_section_content,
            filter_page_content,
            have_relations,
            not_have_relations,
        }) => {
            // Build search filters
            let filters = reqvire::search::SearchFilters::new(
                filter_file.as_deref(),
                filter_name.as_deref(),
                filter_section.as_deref(),
                filter_type.as_deref(),
                filter_content.as_deref(),
                filter_section_content.as_deref(),
                filter_page_content.as_deref(),
                have_relations.as_deref(),
                not_have_relations.as_deref(),
            )?;

            // Generate search report
            let output = reqvire::search::generate_search_report(
                &model_manager.graph_registry,
                &filters,
                json,
                short,
            )?;

            println!("{}", output);
            return Ok(0);
        },
        Some(Commands::ChangeImpact { json, git_commit }) => {
            let base_url = git_commands::get_repository_base_url().map_err(|_| {
                ReqvireError::ProcessError("❌ Failed to determine repository base url.".to_string())
            })?;

            let current_commit = git_commands::get_commit_hash().map_err(|_| {
                ReqvireError::ProcessError("❌ Failed to retrieve the current commit hash.".to_string())
            })?;
                 
            let mut refference_model_manager = ModelManager::new();
            // Use lenient mode for reference registry to handle historical commits with validation issues
            let _not_interested=refference_model_manager.parse_and_validate_with_mode(Some(&git_commit), excluded_filename_patterns, true);
                                    
            let report=change_impact::compute_change_impact(
                &model_manager.graph_registry, 
                &refference_model_manager.graph_registry
            )
            .map_err(|e| ReqvireError::ProcessError(format!("❌ Failed to generate change impact report: {:?}", e)))?;
             
            report.print(&base_url, &current_commit, &git_commit, json);
                
            return Ok(0);
        },
        Some(Commands::Format { fix, json }) => {
            // Default is dry-run mode (preview only), --fix flag applies changes
            let dry_run = !fix;
            let format_result = format_files(&model_manager.graph_registry, dry_run)?;

            if json {
                println!("{}", render_diff_json(&format_result));
            } else {
                render_diff(&format_result);
            }
            return Ok(0);
        },
        Some(Commands::Matrix { json, svg }) => {
            // Generate traceability matrix with verification roll-up strategy
            let matrix_config = matrix_generator::MatrixConfig::default();
            let matrix_output = reqvire::matrix_generator::generate_matrix(
                &model_manager.graph_registry,
                &matrix_config,
                if json {
                    matrix_generator::MatrixFormat::Json
                } else if svg {
                    matrix_generator::MatrixFormat::Svg
                } else {
                    matrix_generator::MatrixFormat::Markdown
                },
            );
            println!("{}", matrix_output);
            return Ok(0);
        },
        Some(Commands::Traces {
            json,
            from_folder,
            links_with_blobs,
            filter_id,
            filter_name,
            filter_type
        }) => {
            // Generate verification traces report (upward paths from verifications to requirements)
            let generator = verification_trace::VerificationTraceGenerator::new(
                &model_manager.graph_registry,
                links_with_blobs,
                from_folder.clone()
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
                let json_output = serde_json::to_string_pretty(&report)
                    .map_err(|e| ReqvireError::ProcessError(format!("Failed to serialize report: {}", e)))?;
                println!("{}", json_output);
            } else {
                let markdown_output = generator.generate_markdown(&report);
                println!("{}", markdown_output);
            }

            return Ok(0);
        },
        Some(Commands::Coverage { json }) => {
            let coverage_report = report_coverage::generate_coverage_report(&model_manager.graph_registry);
            coverage_report.print(json);
            return Ok(0);
        },
        Some(Commands::Model { from, json }) => {
            // Generate model-centric report with optional filtering
            let output = report_model::generate_model_report(
                &model_manager.graph_registry,
                from.as_deref(),
                json,
                "LR"  // Left-to-right diagrams for markdown output
            )?;
            println!("{}", output);
            return Ok(0);
        },
        Some(Commands::Lint { fixable, auditable, fix, json }) => {
            // Run lint analysis
            let lint_report = lint::analyze_model(&model_manager.graph_registry);

            if fix {
                // Apply automatic fixes
                match lint_report.apply_fixes(&mut model_manager.graph_registry) {
                    Ok(relations_removed) => {
                        if relations_removed > 0 {
                            // Rewrite all files with updated relations
                            let format_result = format_files(&model_manager.graph_registry, false)?;

                            if !json {
                                println!("✅ Fixed {} redundant verify relation(s)\n", relations_removed);
                                println!("Formatted {} file(s) with removed relations.\n", format_result.files_changed);
                            }

                            // Show remaining issues that need manual review
                            if !lint_report.needs_manual_review.is_empty() {
                                lint_report.print(json, false, true);  // Only show auditable issues
                            }
                        } else {
                            if !json {
                                println!("No auto-fixable issues found.\n");
                            }
                            lint_report.print(json, fixable, auditable);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to apply fixes: {}", e);
                        return Ok(1);
                    }
                }
            } else {
                // Just print the report based on flags
                lint_report.print(json, fixable, auditable);
            }

            return Ok(0);
        },
        Some(Commands::Export { output }) => {
            info!("Exporting model to HTML folder: {}", &output);
            let output_path = PathBuf::from(&output);
            export::export_model_with_artifacts(
                &model_manager.graph_registry,
                &output_path,
                excluded_filename_patterns,
                false // always generate links without blobs for Export
            )?;
            info!("✅ Export completed successfully");
            return Ok(0);
        },
        Some(Commands::Serve { host, port }) => {
            // Enable quiet mode for serve command (suppress verbose export output)
            reqvire::utils::enable_quiet_mode();

            let temp_dir = export::generate_artifacts_in_temp(
                &model_manager.graph_registry,
                excluded_filename_patterns,
                false // always generate links without blobs for Serve
            )?;

            // Start HTTP server (runs until Ctrl-C)
            info!("Starting HTTP server at http://{}:{}/", host, port);
            serve::serve_directory(&temp_dir, &host, port)?;

            // Cleanup temporary directory after server stops
            std::fs::remove_dir_all(&temp_dir)?;

            return Ok(0);
        },
        Some(Commands::Add { to_file, to_section, index, dry_run, json, args }) => {
            // Parse arguments
            let target_file = to_file.as_ref()
                .or(args.get(0))
                .ok_or_else(|| ReqvireError::ProcessError(
                    "Target file required. Usage: reqvire add <file> [section]".to_string()
                ))?;

            let target_section = to_section.as_ref()
                .or(args.get(1))
                .map(|s| s.as_str())
                .unwrap_or("Requirements");

            // Read element markdown from stdin
            use std::io::Read;
            let mut element_markdown = String::new();
            std::io::stdin().read_to_string(&mut element_markdown)?;

            if element_markdown.trim().is_empty() {
                return Err(ReqvireError::ProcessError(
                    "Element markdown is empty. Pipe element content to stdin.".to_string()
                ));
            }

            // Call CRUD operation
            let git_root = git_commands::get_git_root_dir()?;
            let result = crud::add_element(
                &mut model_manager,
                &element_markdown,
                target_file,
                target_section,
                index,
                excluded_filename_patterns,
                &git_root,
                dry_run,
            )?;

            // Output result
            if json {
                println!("{}", render_crud_json(&result));
            } else {
                render_crud_result(&result);
            }

            return Ok(0);
        },
        Some(Commands::Rm { element_id, dry_run, json }) => {
            // Call CRUD operation
            let git_root = git_commands::get_git_root_dir()?;
            let result = crud::remove_element(
                &mut model_manager,
                &element_id,
                &git_root,
                dry_run,
            )?;

            // Output result
            if json {
                println!("{}", render_crud_json(&result));
            } else {
                render_crud_result(&result);
            }

            return Ok(0);
        },
        Some(Commands::Mv { element_id, to_file, to_section, index, dry_run, json, args }) => {
            // Parse arguments
            let target_file = to_file.as_ref()
                .or(args.get(0))
                .ok_or_else(|| ReqvireError::ProcessError(
                    "Target file required. Usage: reqvire mv <element-id> <file> [section]".to_string()
                ))?;

            let target_section = to_section.as_ref()
                .or(args.get(1))
                .map(|s| s.as_str())
                .unwrap_or("Requirements");

            // Call CRUD operation
            let git_root = git_commands::get_git_root_dir()?;
            let result = crud::move_element(
                &mut model_manager,
                &element_id,
                target_file,
                target_section,
                index,
                excluded_filename_patterns,
                &git_root,
                dry_run,
            )?;

            // Output result
            if json {
                println!("{}", render_crud_json(&result));
            } else {
                render_crud_result(&result);
            }

            return Ok(0);
        },
        Some(Commands::Shell) => {
            run_shell(&mut model_manager)?;
            return Ok(0);
        },
        Some(Commands::Sout) => {
            run_sout(&model_manager.graph_registry)?;
            return Ok(0);
        },
        None => {
            // This case is handled at the beginning of handle_command
            unreachable!("Command is None but should have been handled earlier");
        }
    }
}

fn run_sout(graph_registry: &GraphRegistry) -> Result<(), ReqvireError> {
    use std::collections::BTreeMap;

    // Collect all file paths from pages, sections, and elements
    let mut file_map: BTreeMap<String, (Option<&Page>, Vec<&Section>, Vec<&Element>)> = BTreeMap::new();

    // Collect pages
    for (file_path, page) in &graph_registry.pages {
        file_map.entry(file_path.clone()).or_default().0 = Some(page);
    }

    // Collect sections grouped by file
    for (section_key, section) in &graph_registry.sections {
        file_map.entry(section_key.file_path.clone()).or_default().1.push(section);
    }

    // Collect elements grouped by file
    for element_node in graph_registry.nodes.values() {
        let element = &element_node.element;
        file_map.entry(element.file_path.clone()).or_default().2.push(element);
    }

    // Output content for each file in sorted order
    for (file_path, (page, mut sections, mut elements)) in file_map {
        println!("📄 {}", file_path);
        println!();

        // Output page content if exists
        if let Some(page) = page {
            if !page.frontmatter_content.trim().is_empty() {
                println!("{}", page.frontmatter_content);
                println!();
            }
        }

        // Sort sections by section_order
        sections.sort_by_key(|s| s.section_order);

        // Output sections
        for section in sections {
            if !section.content.trim().is_empty() {
                println!("{}", section.content);
                println!();
            }
        }

        // Sort elements by section_order_index for consistent ordering
        elements.sort_by_key(|e| e.section_order_index);

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
                    println!("  * {}: [{}]({})", relation.relation_type.name, relation.target.text, relation.target.link.as_str());
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
    println!("  list-elements [filter]                     - List all elements or filter by pattern");
    println!("  show-element <element_id>                  - Show detailed information about an element");
    println!("  move-element <element_id> <file> <section> - Move element to existing location");
    println!("  create-section <file> <section>            - Create new section in existing file");
    println!("  create-file <file> <section>               - Create new file with section");
    println!("  list-locations                             - Show all available file/section locations");
    println!("  get-move-impact <element_id>               - Show elements affected by moving an element");
    println!("  impact-tree <element_id>                   - Show change impact tree for an element");
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

fn process_shell_command(graph_registry: &mut GraphRegistry, command: &str) -> Result<(), ReqvireError> {
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
                elements.into_iter()
                    .filter(|elem| elem.identifier.contains(filter) || elem.name.contains(filter))
                    .collect()
            };

            println!("Found {} elements:", filtered.len());
            for element in filtered {
                println!("  {} ({:?}): {}", element.identifier, element.element_type, element.name);
            }
        }
        "show-element" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError("Usage: show-element <element_id>".to_string()));
            }
            let element_id = parts[1];
            if let Some(element) = graph_registry.get_element(element_id) {
                println!("Element ID: {}", element.identifier);
                println!("Name: {}", element.name);
                println!("Type: {:?}", element.element_type);
                println!("File: {}", element.file_path);
                println!("Section: {}", element.section);
                println!("Content: {}", element.content);
                if !element.relations.is_empty() {
                    println!("Relations:");
                    for relation in &element.relations {
                        println!("  {} -> {}", relation.relation_type.name, relation.target.text);
                    }
                }
            } else {
                println!("Element '{}' not found", element_id);
            }
        }
        "move-element" => {
            if parts.len() < 4 {
                return Err(ReqvireError::ProcessError("Usage: move-element <element_id> <file> <section>".to_string()));
            }
            let element_id = parts[1];
            let file_path = parts[2];
            let section = parts[3];

            graph_registry.move_element_to_location(element_id, file_path, section)?;
            println!("Element '{}' moved to {}#{}", element_id, file_path, section);
        }
        "create-section" => {
            if parts.len() < 3 {
                return Err(ReqvireError::ProcessError("Usage: create-section <file> <section>".to_string()));
            }
            let file_path = parts[1];
            let section = parts[2];

            graph_registry.create_virtual_section(file_path, section)?;
            println!("Virtual section '{}' created in file '{}'", section, file_path);
        }
        "create-file" => {
            if parts.len() < 3 {
                return Err(ReqvireError::ProcessError("Usage: create-file <file> <section>".to_string()));
            }
            let file_path = parts[1];
            let section = parts[2];

            graph_registry.create_virtual_file(file_path, section)?;
            println!("Virtual file '{}' created with section '{}'", file_path, section);
        }
        "list-locations" => {
            let locations = graph_registry.get_available_locations();
            println!("Available locations:");
            for (file, section) in locations {
                println!("  {}#{}", file, section);
            }
        }
        "get-move-impact" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError("Usage: get-move-impact <element_id>".to_string()));
            }
            let element_id = parts[1];
            let impact = graph_registry.get_move_impact(element_id);
            if impact.is_empty() {
                println!("No elements would be affected by moving '{}'", element_id);
            } else {
                println!("Elements affected by moving '{}': {}", element_id, impact.join(", "));
            }
        }
        "impact-tree" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError("Usage: impact-tree <element_id>".to_string()));
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
                return Err(ReqvireError::ProcessError("Usage: flush <output_dir>".to_string()));
            }
            let output_dir = Path::new(parts[1]);

            let (md_count, file_count) = graph_registry.flush_to_directory(output_dir)?;
            println!("Flushed {} markdown files and {} other files to '{}'", md_count, file_count, output_dir.display());
        }
        "flush-files" => {
            if parts.len() < 3 {
                return Err(ReqvireError::ProcessError("Usage: flush-files <file1,file2,...> <output_dir>".to_string()));
            }
            let file_list = parts[1];
            let output_dir = Path::new(parts[2]);

            let file_paths: Vec<String> = file_list.split(',').map(|s| s.trim().to_string()).collect();
            let (md_count, file_count) = graph_registry.flush_files_to_directory(&file_paths, output_dir)?;
            println!("Flushed {} markdown files and {} other files to '{}'", md_count, file_count, output_dir.display());
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
                return Err(ReqvireError::ProcessError("Usage: add-element <element_id> <element_name> <file_path> [section]".to_string()));
            }
            let element_id = parts[1];
            let element_name = parts[2];
            let file_path = parts[3];
            let section = parts.get(4).map_or("Main", |v| v);

            let element = reqvire::element::Element::new(
                element_name,
                element_id,
                file_path,
                section,
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
                return Err(ReqvireError::ProcessError("Usage: remove-element <element_id>".to_string()));
            }
            let element_id = parts[1];

            match graph_registry.remove_element(element_id) {
                Ok(()) => println!("Successfully removed element '{}'", element_id),
                Err(e) => println!("Failed to remove element '{}': {}", element_id, e),
            }
        }
        "add-relation" => {
            if parts.len() < 4 {
                return Err(ReqvireError::ProcessError("Usage: add-relation <source_id> <target_id> <relation_type>".to_string()));
            }
            let source_id = parts[1];
            let target_id = parts[2];
            let relation_type = parts[3];

            match graph_registry.add_relation(source_id, target_id, relation_type) {
                Ok(()) => println!("Successfully added relation '{}' from '{}' to '{}'", relation_type, source_id, target_id),
                Err(e) => println!("Failed to add relation: {}", e),
            }
        }
        "remove-relation" => {
            if parts.len() < 4 {
                return Err(ReqvireError::ProcessError("Usage: remove-relation <source_id> <target_id> <relation_type>".to_string()));
            }
            let source_id = parts[1];
            let target_id = parts[2];
            let relation_type = parts[3];

            match graph_registry.remove_relation(source_id, target_id, relation_type) {
                Ok(()) => println!("Successfully removed relation '{}' from '{}' to '{}'", relation_type, source_id, target_id),
                Err(e) => println!("Failed to remove relation: {}", e),
            }
        }
        "list-relations" => {
            if parts.len() < 2 {
                return Err(ReqvireError::ProcessError("Usage: list-relations <element_id>".to_string()));
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
            println!("  Average relations per element: {:.2}",
                     if element_count > 0 { relation_count as f64 / element_count as f64 } else { 0.0 });
        }
        _ => {
            println!("Unknown command: '{}'. Type 'help' for available commands.", parts[0]);
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
    println!("{}   Location: {}#{}", indent, element.file_path, element.section);

    // Print relations that caused this impact
    if !node.relations.is_empty() {
        println!("{}   Impacts through:", indent);
        for relation_node in &node.relations {
            println!("{}     {} -> {}",
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

    #[test]
    fn test_handle_command() {
        // Mock CLI arguments
        let args = Args {
            command: Some(Commands::Export { output: "html".to_string() }),
        };

        // Define test input paths

        let excluded_filename_patterns=vec![
            "**/README*.md".to_string(),
            "**/Logical*.md".to_string(),
            "**/Physical*.md".to_string(),
            "**/index.md".to_string()
        ];

        // Run the handle_command function
        let result = handle_command(
            args,
            &build_glob_set(&excluded_filename_patterns),
        );

        // Assert that it runs without error
        assert!(result.is_ok(), "handle_command should execute without errors");
    }
}
