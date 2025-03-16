use clap::{Parser, CommandFactory};
use std::path::PathBuf;
use anyhow::Result;
use log::{error, info};
use serde::Serialize;
use crate::error::ReqFlowError;
use crate::{html_export, init, linting, model::ModelManager};
use globset::GlobSet;

#[derive(Parser, Debug)]
#[clap(author,version, about = "Reqflow MBSE model management tool", long_about = None)]
pub struct Args {

    /// Convert Markdown to HTML with embedded styles
    #[clap(long)]
    pub html: bool,
       
    /// Enable linting to find potential improvements (non-blocking)
    /// By default, fixes will be applied automatically
    #[clap(long)]
    pub lint: bool,
    
    /// When linting, only show suggestions without applying fixes
    #[clap(long, requires = "lint")]
    pub dry_run: bool,
    
    /// Generate traceability matrix without processing other files
    /// Creates a matrix showing relationships between elements in the model
    #[clap(long)]
    pub generate_matrix: bool,
    
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
    
    /// Path to a custom configuration file (YAML format)
    /// If not provided, the system will look for reqflow.yml, reqflow.yaml, 
    /// .reqflow.yml, or .reqflow.yaml in the current directory
    #[clap(long, short = 'c')]
    pub config: Option<PathBuf>,
    
    /// Output LLM context document
    /// Generates a comprehensive context document with information about ReqFlow
    /// methodology, document structure, relation types, and CLI usage to help
    /// Large Language Models understand and work with ReqFlow-based projects
    #[clap(long)]
    pub llm_context: bool,
    
    /// Initialize a new ReqFlow project
    /// Bootstraps a basic project structure with example requirements and configuration
    #[clap(long)]
    pub init: bool,
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
    validation_type: String,
    errors: Vec<String>,
    fixed: bool, // Kept for API compatibility but always false now
}

/// Helper function to print validation results
fn print_validation_results(validation_type: &str, errors: &[ReqFlowError], json_output: bool) {
    if json_output {
        let json_result = ValidationResult {
            validation_type: validation_type.to_string(),
            errors: errors.iter().map(|e| e.to_string()).collect(),
            fixed: false,
        };
        println!("{}", serde_json::to_string_pretty(&json_result).unwrap());
    } else {
        println!("❌ {} validation failed with {} errors.", validation_type, errors.len());
        for error in errors {
            println!("  - {}", error);
        }
    }
}

pub fn handle_command(
    args: Args,
    model_manager: &mut ModelManager, 
    specification_folder_path: &PathBuf,
    external_folders_path: &[PathBuf],    
    output_folder_path: &PathBuf,
    excluded_filename_patterns: &GlobSet,
    diagram_direction: &str
) -> Result<i32> {

    // Handle LLM context
    if args.llm_context {
        match std::fs::read_to_string("src/llm_context.md") {
            Ok(content) => {
                println!("{}", content);
                Args::print_help();
                return Ok(0);
            }
            Err(e) => {
                error!("Error reading LLM context file: {}", e);
                return Err(anyhow::anyhow!("Failed to read LLM context file"));
            }
        }
    }else if args.init {
       // Handle `init` command    
        match init::initialize_project(&specification_folder_path) {
            Ok(_) => return Ok(0), 
            Err(e) => {
                error!("Failed to initialize project: {}", e);
                return Err(anyhow::anyhow!("Project initialization failed: {}", e));
            }
        }
    }else{
  
        let parse_result=model_manager.parse_and_validate(&specification_folder_path, &external_folders_path,excluded_filename_patterns);

        if args.validate {
            match parse_result {
                Ok(errors) => {
                    if errors.is_empty() {
                        println!("✅ Validation completed successfully with no errors.");
                    } else {
                        print_validation_results("Validation Summary", &errors, args.json);
                    }
                }
                Err(e) => eprintln!("❌ Validation failed: {}", e),
            }                                 
        }else if args.generate_diagrams {
            info!("Generating mermaid diagrams in {:?}", specification_folder_path);
            // Only collect identifiers and process files to add diagrams
            // Skip validation checks for diagram generation mode
            model_manager.process_diagrams(diagram_direction, false)?;
       
            info!("Requirements diagrams updated in source files");
            return Ok(0);
        }else if args.lint {
            linting::run_linting(&specification_folder_path, &external_folders_path,excluded_filename_patterns, args.dry_run)?;
            return Ok(0);
        }else if args.generate_matrix {
            std::fs::create_dir_all(output_folder_path)?;
            //model_manager.generate_traceability_matrix(specification_folder_path, output_folder_path)?;
            info!("Traceability matrix saved to {:?}", output_folder_path);
            return Ok(0);
        } else if args.html {
            let processed_count = html_export::export_markdown_to_html(specification_folder_path, output_folder_path)?;
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
    use crate::config::Config;
    use crate::model::ModelManager;
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
        let args = Args::parse_from(&["reqflow", "--init"]);
        assert!(args.init);
    }
    
    #[test]
    fn test_handle_command() {
        // Mock CLI arguments
        let args = Args {
            llm_context: false,
            init: false,
            html: false,
            lint: false,
            dry_run: false,
            json: false,
            generate_matrix: false,
            generate_diagrams: false,
            validate: false,
            config: None, // No custom config file for the test
        };

        // Create a default config instance
        let config = Config::default();

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
                        
        // Create a mock model manager
        let mut model_manager = ModelManager::new(config.clone());

        // Run the handle_command function
        let result = handle_command(
            args,
            &mut model_manager,
            &specification_folder_path,
            &external_folders_path,            
            &output_folder_path,
            &build_glob_set(&excluded_filename_patterns)
        );

        // Assert that it runs without error
        assert!(result.is_ok(), "handle_command should execute without errors");
    }
}


