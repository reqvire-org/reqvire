use clap::{Parser, Subcommand, CommandFactory};
use std::path::PathBuf;
use anyhow::Result;
use log::{info};
use serde::Serialize;
use reqvire::error::ReqvireError;
use reqvire::{linting, ModelManager};
use reqvire::index_generator;
use globset::GlobSet;
use reqvire::reports;
use reqvire::diagrams;
use reqvire::export;
use reqvire::change_impact;
use reqvire::git_commands;
use reqvire::matrix_generator;
use reqvire::reports::Filters;


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
    
    /// Process only files within a specific subdirectory relative to git root (hidden flag for testing)
    #[clap(long, hide = true, global = true)]
    pub subdirectory: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert Markdown to HTML with embedded styles and save to output location
    Html {
        /// Output directory for HTML files
        #[clap(long, short = 'o', default_value = "html")]
        output: String,
    },
    
    /// Enable linting to find potential improvements (non-blocking) By default, fixes will be applied automatically
    #[clap(override_help = "Enable linting to find potential improvements (non-blocking) By default, fixes will be applied automatically\n\nLINT OPTIONS:\n      --dry-run  When linting, only show suggestions without applying fixes\n      --json     Output results in JSON format")]
    Lint {
        /// When linting, only show suggestions without applying fixes
        #[clap(long, help_heading = "LINT OPTIONS")]
        dry_run: bool,
        
        /// Output results in JSON format
        #[clap(long, help_heading = "LINT OPTIONS")]
        json: bool,
    },
    
    /// Generate traceability information without processing other files Creates matrices and reports showing relationships between elements in the model
    #[clap(override_help = "Generate traceability information without processing other files Creates matrices and reports showing relationships between elements in the model\n\nTRACES OPTIONS:\n      --svg   Output traceability matrix as SVG without hyperlinks and with full element names Cannot be used with --json\n      --json  Output results in JSON format")]
    Traces {
        /// Output traceability matrix as SVG without hyperlinks and with full element names Cannot be used with --json
        #[clap(long, conflicts_with = "json", help_heading = "TRACES OPTIONS")]
        svg: bool,
        
        /// Output results in JSON format
        #[clap(long, help_heading = "TRACES OPTIONS")]
        json: bool,
    },

    /// Validate model
    Validate {
        /// Output results in JSON format
        #[clap(long)]
        json: bool,
    },
                       
    /// Generate mermaid diagrams in markdown files showing requirements relationships The diagrams will be placed at the top of each requirements document
    GenerateDiagrams,
    
    /// Generate index document with links and summaries to all documents
    GenerateIndex,

    /// Output model registry and summary
    #[clap(override_help = "Output model registry and summary\n\nMODEL SUMMARY OPTIONS:\n      --json                        Output results in JSON format\n      --filter-file <GLOB>          Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`\n      --filter-name <REGEX>         Only include elements whose name matches this regular expression\n      --filter-section <GLOB>       Only include sections whose name matches this glob pattern e.g. `System requirement*`\n      --filter-type <TYPE>          Only include elements of the given type e.g. `user-requirement`, `system-requirement`, `verification`, `file`, or other custom type\n      --filter-content <REGEX>      Only include elements whose content matches this regular expression\n      --filter-is-not-verified      Only include requirements that have at least one \"verifiedBy\" relation\n      --filter-is-not-satisfied     Only include requirements that have at least one \"satisfiedBy\" relation")]
    ModelSummary {
        /// Output results in JSON format
        #[clap(long, help_heading = "MODEL SUMMARY OPTIONS")]
        json: bool,
        
        /// Only include files whose path matches this glob pattern e.g. `src/**/*Reqs.md`
        #[clap(long, value_name = "GLOB", help_heading = "MODEL SUMMARY OPTIONS")]
        filter_file: Option<String>,

        /// Only include elements whose name matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "MODEL SUMMARY OPTIONS")]
        filter_name: Option<String>,
        
        /// Only include sections whose name matches this glob pattern e.g. `System requirement*`
        #[clap(long, value_name = "GLOB", help_heading = "MODEL SUMMARY OPTIONS")]
        filter_section: Option<String>,

        /// Only include elements of the given type e.g. `user-requirement`, `system-requirement`, `verification`, `file`, or other custom type
        #[clap(long, value_name = "TYPE", help_heading = "MODEL SUMMARY OPTIONS")]
        filter_type: Option<String>,

        /// Only include elements whose content matches this regular expression
        #[clap(long, value_name = "REGEX", help_heading = "MODEL SUMMARY OPTIONS")]
        filter_content: Option<String>,

        /// Only include requirements that have at least one "verifiedBy" relation
        #[clap(long, help_heading = "MODEL SUMMARY OPTIONS")]
        filter_is_not_verified: bool,

        /// Only include requirements that have at least one "satisfiedBy" relation
        #[clap(long, help_heading = "MODEL SUMMARY OPTIONS")]
        filter_is_not_satisfied: bool,
                        
        /// Output traceability matrix as SVG without hyperlinks and with full element names Cannot be used with --json
        #[clap(long, hide = true, conflicts_with_all = &["json"], help_heading = "MODEL SUMMARY OPTIONS")]
        cypher: bool,
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
    }
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
        let name = subcommand.get_name();
        let about = subcommand.get_about().map(|s| s.to_string()).unwrap_or_default();
        println!("  {:<17} {}", name, about);
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

pub fn handle_command(
    args: Args,
    output_folder_path: &PathBuf,
    excluded_filename_patterns: &GlobSet,
    diagram_direction: &str,
    diagrams_with_blobs: bool,
    user_requirements_root_folder: &Option<PathBuf>
) -> Result<i32,ReqvireError> {
                        
    let mut model_manager = ModelManager::new();
    let parse_result = model_manager.parse_and_validate(
        None, 
        user_requirements_root_folder,
        excluded_filename_patterns
    );
    
    match args.command {
        Some(Commands::Validate { json }) => {
            match parse_result {
                Ok(errors) => {
                    if errors.is_empty() {
                        println!("✅ Validation completed successfully with no errors.");
                    } else {
                        print_validation_results(&errors, json);
                    }
                    return Ok(0);
                }
                Err(e) => {
                    eprintln!("❌ Validation failed: {}", e);
                    return Ok(0);
                }
            }  
        },
        Some(Commands::GenerateIndex) => {
            info!("Generating index.....");
            let _index_context = index_generator::generate_readme_index(
                &model_manager.element_registry, 
                    &output_folder_path
                ).map_err(|e| {
                    ReqvireError::ProcessError(format!("❌ Failed to generate README.md: {:?}", e))
                })?;

            return Ok(0);
        },
        Some(Commands::GenerateDiagrams) => {
            info!("Generating mermaid diagrams");
            // Only collect identifiers and process files to add diagrams
            // Skip validation checks for diagram generation mode
            diagrams::process_diagrams(&model_manager.element_registry,diagram_direction,diagrams_with_blobs)?;
           
            info!("Requirements diagrams updated in source files");
            return Ok(0);
        },
        Some(Commands::ModelSummary { 
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
            let filters = Filters::new(
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
               
            reports::print_registry_summary(&model_manager.element_registry,output_format, &filters);
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
                &model_manager.element_registry, 
                &refference_model_manager.element_registry
            )
            .map_err(|e| ReqvireError::ProcessError(format!("❌ Failed to generate change impact report: {:?}", e)))?;
             
            report.print(&base_url, &current_commit, &git_commit, json);
                
            return Ok(0);
        },
        Some(Commands::Lint { dry_run, json: _ }) => {
            linting::run_linting(excluded_filename_patterns, dry_run, args.subdirectory.as_deref())?;
            return Ok(0);
        },
        Some(Commands::Traces { json, svg }) => {
            let matrix_config = matrix_generator::MatrixConfig::default();
                
            let matrix_output = reqvire::matrix_generator::generate_matrix(
                &model_manager.element_registry,
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
        Some(Commands::Html { output }) => {
            let html_output_path = PathBuf::from(output);
            let processed_count = export::export_model(&model_manager.element_registry, &html_output_path)?;
            info!("{} markdown files converted to HTML", processed_count);   
            
            return Ok(0);
        },
        None => {
            Args::print_help();
            return Ok(0);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
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
        let args = Args::parse_from(&["reqvire", "html"]);
        assert!(matches!(args.command, Some(Commands::Html { output: _ })));
    }
    
    #[test]
    fn test_handle_command() {
        // Mock CLI arguments
        let args = Args {
            command: Some(Commands::Html { output: "html".to_string() }),
            config: None,
            subdirectory: None
        };

        // Define test input and output pathse
        let output_folder_path = PathBuf::from("test/output");
        
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
            &output_folder_path,
            &build_glob_set(&excluded_filename_patterns),
            "TD",
            false,
            &user_requirements_root
        );

        // Assert that it runs without error
        assert!(result.is_ok(), "handle_command should execute without errors");
    }
}