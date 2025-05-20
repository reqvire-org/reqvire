use clap::{ArgGroup, Parser, CommandFactory};
use std::path::PathBuf;
use anyhow::Result;
use log::{info};
use serde::Serialize;
use reqvire::error::ReqvireError;
use reqvire::{html_export, linting, ModelManager};
use reqvire::index_generator;
use globset::GlobSet;
use reqvire::reports;
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
    name = "reqvire",
    // Any of the filters requires --model-summary
    group(
        ArgGroup::new("filters")
            .args(&[
                "filter_file",
                "filter_section",
                "filter_type",
                "filter_name",                
                "filter_content",
                "filter_is_not_verified",
                "filter_is_not_satisfied",
            ])
            .requires("model_summary")
            .multiple(true)            
    )
)]
pub struct Args {

    /// Convert Markdown to HTML with embedded styles and save to output location
    #[clap(long)]
    pub html: bool,
       
    /// Enable linting to find potential improvements (non-blocking)
    /// By default, fixes will be applied automatically
    #[clap(long)]
    pub lint: bool,
    
    /// When linting, only show suggestions without applying fixes
    #[clap(long, requires = "lint")]
    pub dry_run: bool,
    
    /// Generate traceability information without processing other files
    /// Creates matrices and reports showing relationships between elements in the model
    #[clap(long)]
    pub traces: bool,
    
    /// Output traceability matrix as SVG without hyperlinks and with full element names
    /// Cannot be used with --json
    #[clap(long, requires = "traces", conflicts_with = "json")]
    pub svg: bool,
    
    /// Output validation results in JSON format
    /// Useful for CI/CD pipelines and automation
    #[clap(long)]
    pub json: bool,

    /// Validate model
    #[clap(long)]
    pub validate: bool,
                       
    /// Generate mermaid diagrams in markdown files showing requirements relationships
    /// The diagrams will be placed at the top of each requirements document
    #[clap(long)]
    pub generate_diagrams: bool,
    
    /// Generate index document with links and summaries to all documents
    #[clap(long)]
    pub generate_index: bool,

    /// Output model registry and summary
    #[clap(long)]
    pub model_summary: bool,

        
    /// Only include files whose path matches this glob pattern
    /// e.g. `src/**/*Reqs.md`
    #[clap(
        long,
        value_name = "GLOB",
        requires = "model_summary",
        help_heading = "MODEL SUMMARY FILTERS"
    )]
    pub filter_file: Option<String>,

    /// Only include elements whose name matches this regular expression
    #[clap(
        long,
        value_name = "REGEX",
        requires = "model_summary",
        help_heading = "MODEL SUMMARY FILTERS"
    )]
    pub filter_name: Option<String>,
    
    /// Only include sections whose name matches this glob pattern
    /// e.g. `System requirement*`
    #[clap(
        long,
        value_name = "GLOB",
        requires = "model_summary",
        help_heading = "MODEL SUMMARY FILTERS"
    )]
    pub filter_section: Option<String>,

    /// Only include elements of the given type
    /// e.g. `user-requirement`, `system-requirement`, `verification`, `file`, or other custom type
    #[clap(
        long,
        value_name = "TYPE",
        requires = "model_summary",
        help_heading = "MODEL SUMMARY FILTERS"
    )]
    pub filter_type: Option<String>,

    /// Only include elements whose content matches this regular expression
    #[clap(
        long,
        value_name = "REGEX",
        requires = "model_summary",
        help_heading = "MODEL SUMMARY FILTERS"
    )]
    pub filter_content: Option<String>,

    /// Only include requirements that have at least one “verifiedBy” relation
    #[clap(
        long,
        requires = "model_summary",
        help_heading = "MODEL SUMMARY FILTERS"
    )]
    pub filter_is_not_verified: bool,

    /// Only include requirements that have at least one “satisfiedBy” relation
    #[clap(
        long,
        requires = "model_summary",
        help_heading = "MODEL SUMMARY FILTERS"
    )]
    pub filter_is_not_satisfied: bool,
                
    /// Output traceability matrix as SVG without hyperlinks and with full element names
    /// Cannot be used with --json
    #[clap(long, hide = true, requires = "model_summary", conflicts_with_all = &["json", "svg"])]
    pub cypher: bool,                
                      
    /// Path to a custom configuration file (YAML format)
    /// If not provided, the system will look for reqvire.yml, reqvire.yaml, 
    /// .reqvire.yml, or .reqvire.yaml in the current directory
    #[clap(long, short = 'c')]
    pub config: Option<PathBuf>,
    
    /// Output LLM context document
    /// Generates a comprehensive context document with information about Reqvire
    /// methodology, document structure, relation types, and CLI usage to help
    /// Large Language Models understand and work with Reqvire-based projects
    #[clap(long)]
    pub llm_context: bool,
    
    
    /// Analise change impact and provides report
    #[clap(long)]
    pub change_impact: bool,
    
    /// Git commit hash to use when comparing models
    #[clap(long, requires = "change_impact", default_value = "HEAD")]    
    pub git_commit: String,
    
        /// Process only files within a specific subdirectory relative to git root (hidden flag for testing)
    #[clap(long, hide = true)]
    pub subdirectory: Option<String>,

}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }

    pub fn print_help() {
        let mut cmd = Args::command();
        cmd.print_help().unwrap();
        println!();
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
    user_requirements_root_folder: &Option<PathBuf>
) -> Result<i32,ReqvireError> {

    let mut model_manager = ModelManager::new();


    // Handle LLM context
    if args.llm_context {
        // Include the LLM context content directly in the binary
        let llm_context = include_str!("llm_context.md");
        println!("{}", llm_context);
        Args::print_help();
        return Ok(0);
    }else{
  
        let parse_result = model_manager.parse_and_validate(
            None, 
            user_requirements_root_folder,
            excluded_filename_patterns
        );

                
        if args.validate {
            match parse_result {
                Ok(errors) => {
                    if errors.is_empty() {
                        println!("✅ Validation completed successfully with no errors.");
                    } else {
                        print_validation_results(&errors, args.json);
                    }
                    return Ok(0);
                }
                Err(e) => {
                    eprintln!("❌ Validation failed: {}", e);
                    return Ok(0);
                }
            }  
        }else if args.generate_index {
            info!("Generating index.....");

            let _index_context = index_generator::generate_readme_index(
                &model_manager.element_registry, 
                &output_folder_path
            ).map_err(|e| {
                ReqvireError::ProcessError(format!("❌ Failed to generate README.md: {:?}", e))
            })?;

            return Ok(0);
                                            
        }else if args.generate_diagrams {
            info!("Generating mermaid diagrams");
            // Only collect identifiers and process files to add diagrams
            // Skip validation checks for diagram generation mode
            model_manager.process_diagrams(diagram_direction)?;
       
            info!("Requirements diagrams updated in source files");
            return Ok(0);
            
        }else if args.model_summary {
            let filters = Filters::new(
                args.filter_file.as_deref(),
                args.filter_name.as_deref(),
                args.filter_section.as_deref(),
                args.filter_type.as_deref(),
                args.filter_content.as_deref(),
                args.filter_is_not_verified,
                args.filter_is_not_satisfied,
            ).map_err(|e| {
                ReqvireError::ProcessError(format!("❌ Failed to construct filters: {}", e))
            })?;
            
            let output_format = if args.cypher {
                reports::SummaryOutputFormat::Cypher
            } else if args.json {
                reports::SummaryOutputFormat::Json
            } else {
                reports::SummaryOutputFormat::Text
            };

            
            reports::print_registry_summary(&model_manager.element_registry,output_format, &filters);
            return Ok(0);        
            
            
        }else if args.change_impact {
            
            let current_commit = git_commands::get_commit_hash().map_err(|_| {
                ReqvireError::ProcessError("❌ Failed to retrieve the current commit hash.".to_string())
            })?;

            let repo_root = git_commands::repository_root().map_err(|_| {
                ReqvireError::ProcessError("❌ Failed to determine repository root.".to_string())
            })?;

            let base_url = git_commands::get_repository_base_url().map_err(|_| {
                ReqvireError::ProcessError("❌ Failed to determine repository base url.".to_string())
            })?;

            
            let mut refference_model_manager = ModelManager::new();      
            let _not_interested=refference_model_manager.parse_and_validate(Some(&args.git_commit), user_requirements_root_folder, excluded_filename_patterns);
                                    
            let report=change_impact::compute_change_impact(
                &model_manager.element_registry, 
                &refference_model_manager.element_registry,
                &repo_root
            )
            .map_err(|e| ReqvireError::ProcessError(format!("❌ Failed to generate change impact report: {:?}", e)))?;
            
            report.print(&base_url, &current_commit, &args.git_commit, args.json);
                
            return Ok(0);
            
                                      
        }else if args.lint {
            linting::run_linting(excluded_filename_patterns, args.dry_run, args.subdirectory.as_deref())?;
            return Ok(0);
            
            
        }else if args.traces {
            let matrix_config = matrix_generator::MatrixConfig::default();
            
            let matrix_output = reqvire::matrix_generator::generate_matrix(
                &model_manager.element_registry,
                &matrix_config,
                if args.json {
                    matrix_generator::MatrixFormat::Json
                } else if args.svg {
                    matrix_generator::MatrixFormat::Svg
                } else {
                    matrix_generator::MatrixFormat::Markdown
                },                
            );
            
            println!("{}", matrix_output);
            return Ok(0);
            
           
        } else if args.html {
            let processed_count = html_export::export_markdown_to_html(output_folder_path)?;
            info!("{} markdown files converted to HTML", processed_count);
            return Ok(0);
        }else{
            Args::print_help();  

        }
    
    }
    Ok(1)
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
    fn test_cli_parsing() {
        let args = Args::parse_from(&["reqvire", "--html"]);
        assert!(args.html);
    }
    
    #[test]
    fn test_handle_command() {
        // Mock CLI arguments
        let args = Args {
            llm_context: false,
            html: false,
            lint: false,
            dry_run: false,
            json: false,
            cypher: false,
            svg: false,
            traces: false,
            generate_diagrams: false,
            generate_index: false,
            model_summary: false,
            filter_file: None,
            filter_section: None,
            filter_type: None,
            filter_name: None,             
            filter_content: None,
            filter_is_not_verified:false,
            filter_is_not_satisfied:false,                    
            validate: false,
            config: None, // No custom config file for the test
            change_impact: false, // Add the missing field
            git_commit: "HEAD".to_string(), // Add the missing field
            subdirectory: None
        };


        // Define test input and output pathse
        let specification_folder_path = PathBuf::from("test/specifications");
        let output_folder_path = PathBuf::from("test/output");
        let external_folders_path = vec![PathBuf::from("test/external")];
        
        let excluded_filename_patterns=vec![
            "**/README*.md".to_string(),
            "**/Logical*.md".to_string(),
            "**/Physical*.md".to_string(),
            "**/index.md".to_string()
        ];
                        
        // No need to create a mock model manager - it's created inside handle_command


    
        // Run the handle_command function
        let user_requirements_root = None;
        let result = handle_command(
            args,
            &output_folder_path,
            &build_glob_set(&excluded_filename_patterns),
            "TD",
            &user_requirements_root
        );

        // Assert that it runs without error
        assert!(result.is_ok(), "handle_command should execute without errors");
    }
}
