use clap::{Parser, Subcommand, CommandFactory};
use std::path::PathBuf;
use anyhow::Result;
use log::{info};
use serde::Serialize;
use reqvire::error::ReqvireError;
use reqvire::ModelManager;
use globset::GlobSet;
use reqvire::reports;
use reqvire::diagrams;
use reqvire::export;
use reqvire::change_impact;
use reqvire::git_commands;
use reqvire::matrix_generator;
use reqvire::sections_summary;
use reqvire::verification_trace;
use reqvire::GraphRegistry;
use reqvire::graph_registry::{Page, Section};
use reqvire::element::Element;
use reqvire::format::{format_files, render_diff, render_diff_json};
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

    /// Path to a custom configuration file (YAML format)
    /// If not provided, the system will look for reqvire.yml, reqvire.yaml,
    /// .reqvire.yml, or .reqvire.yaml in the current directory
    #[clap(long, short = 'c', global = true)]
    pub config: Option<PathBuf>,

}



#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Export model to browsable HTML documentation with complete traceability
    Export {
        /// Output directory for HTML files
        #[clap(long, short = 'o', default_value = "html")]
        output: String,
    },

    /// Serve model as browsable HTML documentation via HTTP server
    #[clap(override_help = "Serve model as browsable HTML documentation via HTTP server\n\nSERVE OPTIONS:\n      --host <HOST>  Bind address (default: localhost)\n      --port <PORT>  Server port (default: 8080)")]
    Serve {
        /// Bind address
        #[clap(long, default_value = "localhost", help_heading = "SERVE OPTIONS")]
        host: String,

        /// Server port
        #[clap(long, default_value = "8080", help_heading = "SERVE OPTIONS")]
        port: u16,
    },

    /// Format markdown files by applying automatic normalization and stylistic fixes
    #[clap(override_help = "Format markdown files by applying automatic normalization and stylistic fixes\n\nFORMAT OPTIONS:\n      --dry-run  Show differences without applying changes\n      --json     Output results in JSON format")]
    Format {
        /// Show differences without applying changes
        #[clap(long, help_heading = "FORMAT OPTIONS")]
        dry_run: bool,

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
    

    /// Generate mermaid diagrams in markdown files showing requirements relationships The diagrams will be placed at the top of each requirements document
    GenerateDiagrams,

    /// Remove all generated mermaid diagrams from markdown files
    RemoveDiagrams,

    /// Output model registry and summary
    #[clap(override_help = "Output model registry and summary\n\nSUMMARY OPTIONS:\n      --json                        Output results in JSON format\n      --filter-file <GLOB>          Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`\n      --filter-name <REGEX>         Only include elements whose name matches this regular expression\n      --filter-section <GLOB>       Only include sections whose name matches this glob pattern e.g. `System requirement*`\n      --filter-type <TYPE>          Only include elements of the given type e.g. `user-requirement`, `system-requirement`, `verification`, `file`, or other custom type\n      --filter-content <REGEX>      Only include elements whose content matches this regular expression\n      --filter-is-not-verified      Only include requirements that do NOT have any \"verifiedBy\" relations\n      --filter-is-not-satisfied     Only include requirements that do NOT have any \"satisfiedBy\" relations")]
    Summary {
        /// Output results in JSON format
        #[clap(long, help_heading = "SUMMARY OPTIONS")]
        json: bool,

        /// Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`
        #[clap(long, value_name = "GLOB", help_heading = "SUMMARY OPTIONS")]
        filter_file: Option<String>,

        /// Only include elements whose name matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "SUMMARY OPTIONS")]
        filter_name: Option<String>,

        /// Only include sections whose name matches this glob pattern e.g. `System requirement*`
        #[clap(long, value_name = "GLOB", help_heading = "SUMMARY OPTIONS")]
        filter_section: Option<String>,

        /// Only include elements of the given type e.g. `user-requirement`, `system-requirement`, `verification`, `file`, or other custom type
        #[clap(long, value_name = "TYPE", help_heading = "SUMMARY OPTIONS")]
        filter_type: Option<String>,

        /// Only include elements whose content matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "SUMMARY OPTIONS")]
        filter_content: Option<String>,

        /// Only include requirements that do NOT have any "verifiedBy" relations
        #[clap(long, help_heading = "SUMMARY OPTIONS")]
        filter_is_not_verified: bool,

        /// Only include requirements that do NOT have any "satisfiedBy" relations
        #[clap(long, help_heading = "SUMMARY OPTIONS")]
        filter_is_not_satisfied: bool,

        /// Output traceability matrix as SVG without hyperlinks and with full element names Cannot be used with --json
        #[clap(long, hide = true, conflicts_with_all = &["json"], help_heading = "SUMMARY OPTIONS")]
        cypher: bool,
    },

    /// Output sections summary showing files, section names, and section content without individual elements
    #[clap(override_help = "Output sections summary showing files, section names, and section content without individual elements\n\nSECTION-SUMMARY OPTIONS:\n      --json                        Output results in JSON format\n      --filter-file <GLOB>          Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`\n      --filter-section <GLOB>       Only include sections whose name matches this glob pattern e.g. `System requirement*`\n      --filter-content <REGEX>      Only include sections whose content matches this regular expression")]
    SectionSummary {
        /// Output results in JSON format
        #[clap(long, help_heading = "SECTION-SUMMARY OPTIONS")]
        json: bool,

        /// Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`
        #[clap(long, value_name = "GLOB", help_heading = "SECTION-SUMMARY OPTIONS")]
        filter_file: Option<String>,

        /// Only include sections whose name matches this glob pattern e.g. `System requirement*`
        #[clap(long, value_name = "GLOB", help_heading = "SECTION-SUMMARY OPTIONS")]
        filter_section: Option<String>,

        /// Only include sections whose content matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "SECTION-SUMMARY OPTIONS")]
        filter_content: Option<String>,
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
    #[clap(override_help = "Generate verification traces showing upward paths from verifications to root requirements\n\nTRACES OPTIONS:\n      --json                      Output results in JSON format\n      --from-folder <PATH>        Generate links relative to this folder path\n      --filter-id <ID>            Only include verification with this specific identifier\n      --filter-name <REGEX>       Only include verifications whose name matches this regular expression\n      --filter-type <TYPE>        Only include verifications of the given type e.g. `test-verification`, `analysis-verification`")]
    Traces {
        /// Output results in JSON format
        #[clap(long, help_heading = "TRACES OPTIONS")]
        json: bool,

        /// Relative path to folder where output will be saved (for generating relative links in Mermaid diagrams)
        #[clap(long, value_name = "PATH", help_heading = "TRACES OPTIONS")]
        from_folder: Option<String>,

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
        let json_result = ValidationResult {
            errors: errors.iter().map(|e| e.to_string()).collect(),
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
        Some(Commands::Summary { json, .. }) => *json,
        Some(Commands::SectionSummary { json, .. }) => *json,
        Some(Commands::Matrix { json, .. }) => *json,
        Some(Commands::Traces { json, .. }) => *json,
        Some(Commands::Coverage { json }) => *json,
        _ => false,
    }
}

pub fn handle_command(
    args: Args,
    excluded_filename_patterns: &GlobSet,
    diagram_direction: &str,
    diagrams_with_blobs: bool,
    user_requirements_root_folder: &Option<PathBuf>
) -> Result<i32,ReqvireError> {

    // If no command provided, show help
    if args.command.is_none() {
        Args::print_help();
        return Ok(0);
    }

    let mut model_manager = ModelManager::new();
    let parse_result = model_manager.parse_and_validate(
        None,
        user_requirements_root_folder,
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
        Some(Commands::GenerateDiagrams) => {
            info!("Generating mermaid diagrams");
            // Only collect identifiers and process files to add diagrams
            // Skip validation checks for diagram generation mode
            diagrams::process_diagrams(&model_manager.graph_registry,diagram_direction,diagrams_with_blobs)?;

            info!("Requirements diagrams updated in source files");
            return Ok(0);
        },
        Some(Commands::RemoveDiagrams) => {
            info!("Removing generated mermaid diagrams");
            diagrams::remove_diagrams(&model_manager.graph_registry)?;
            info!("Generated diagrams removed from source files");
            return Ok(0);
        },
        Some(Commands::Summary {
            json,
            cypher,
            filter_file,
            filter_name,
            filter_section,
            filter_type,
            filter_content,
            filter_is_not_verified,
            filter_is_not_satisfied
        }) => {
            let filters = reports::Filters::new(
                filter_file.as_deref(),
                filter_name.as_deref(),
                filter_section.as_deref(),
                filter_type.as_deref(),
                filter_content.as_deref(),
                filter_is_not_verified,
                filter_is_not_satisfied,
            ).map_err(|e| {
                ReqvireError::ProcessError(format!("❌ Failed to construct filters: {}", e))
            })?;

            let output_format = if cypher {
                reports::SummaryOutputFormat::Cypher
            } else if json {
                reports::SummaryOutputFormat::Json
            } else {
                reports::SummaryOutputFormat::Text
            };

            reports::print_registry_summary(&model_manager.graph_registry,output_format, &filters);
            return Ok(0);
        },
        Some(Commands::SectionSummary {
            json,
            filter_file,
            filter_section,
            filter_content
        }) => {
            let filters = sections_summary::SectionsFilters::new(
                filter_file.as_deref(),
                filter_section.as_deref(),
                filter_content.as_deref(),
            ).map_err(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            }).unwrap();

            sections_summary::print_sections_summary(&model_manager.graph_registry, json, &filters);
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
            let _not_interested=refference_model_manager.parse_and_validate(Some(&git_commit), user_requirements_root_folder, excluded_filename_patterns);
                                    
            let report=change_impact::compute_change_impact(
                &model_manager.graph_registry, 
                &refference_model_manager.graph_registry
            )
            .map_err(|e| ReqvireError::ProcessError(format!("❌ Failed to generate change impact report: {:?}", e)))?;
             
            report.print(&base_url, &current_commit, &git_commit, json);
                
            return Ok(0);
        },
        Some(Commands::Format { dry_run, json }) => {
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
            filter_id,
            filter_name,
            filter_type
        }) => {
            // Generate verification traces report (upward paths from verifications to requirements)
            let generator = verification_trace::VerificationTraceGenerator::new(
                &model_manager.graph_registry,
                diagrams_with_blobs,
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
            let coverage_report = reports::generate_coverage_report(&model_manager.graph_registry);
            coverage_report.print(json);
            return Ok(0);
        },
        Some(Commands::Export { output }) => {
            let html_output_path = PathBuf::from(output);
            export::export_model_with_artifacts(
                &model_manager.graph_registry,
                &html_output_path,
                excluded_filename_patterns,
                diagram_direction,
                diagrams_with_blobs
            )?;

            return Ok(0);
        },
        Some(Commands::Serve { host, port }) => {
            // Enable quiet mode for serve command (suppress verbose export output)
            reqvire::utils::enable_quiet_mode();

            // Generate HTML artifacts in temporary directory
            let temp_dir = export::generate_artifacts_in_temp(
                &model_manager.graph_registry,
                excluded_filename_patterns,
                diagram_direction,
                diagrams_with_blobs
            )?;

            // Start HTTP server (runs until Ctrl-C)
            crate::serve::serve_directory(&temp_dir, &host, port)?;

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
            config: None,
        };

        // Define test input paths
        
        let excluded_filename_patterns=vec![
            "**/README*.md".to_string(),
            "**/Logical*.md".to_string(),
            "**/Physical*.md".to_string(),
            "**/index.md".to_string()
        ];
                        
        // Run the handle_command function
        let user_requirements_root = None;
        let result = handle_command(
            args,
            &build_glob_set(&excluded_filename_patterns),
            "TD",
            false,
            &user_requirements_root
        );

        // Assert that it runs without error
        assert!(result.is_ok(), "handle_command should execute without errors");
    }
}
