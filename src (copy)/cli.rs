use clap::{Parser, CommandFactory};
use std::path::PathBuf;
use anyhow::Result;
use log::{error, info};

use crate::{config::Config, html_export, init, linting, model::ModelManager, validation};

#[derive(Parser, Debug)]
#[clap(author,version, about = "Reqflow MBSE model management tool", long_about = None)]
pub struct Args {
    /// Path to the input folder containing Markdown files
    /// If not provided, this will be read from the configuration file
    #[clap(index = 1)]
    pub input_folder: Option<PathBuf>,

    /// Path to the output folder (not required for validate commands)
    /// If not provided, this will be read from the configuration file
    #[clap(index = 2)]
    pub output_folder: Option<PathBuf>,

    /// Convert Markdown to HTML with embedded styles
    #[clap(long)]
    pub html: bool,

    /// Enable verbose output
    #[clap(short, long)]
    pub verbose: bool,
    
    /// Validate Markdown structure only
    /// Checks for:
    /// - Unique element names within documents
    /// - Proper heading hierarchy
    /// - Well-formed relation sections
    #[clap(long, conflicts_with = "html")]
    pub validate_markdown: bool,
    
    /// Validate relations only
    /// Checks for:
    /// - Valid relation types according to spec
    /// - Valid relation targets (elements and files exist)
    /// - Proper relation formatting
    #[clap(long, conflicts_with = "html")]
    pub validate_relations: bool,
    
    /// Run all validations
    /// Includes markdown structure validation, relation validation,
    /// and cross-component dependency validation
    #[clap(long, conflicts_with = "html")]
    pub validate_all: bool,
    
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
pub fn handle_command(
    args: Args,
    model_manager: &mut ModelManager, 
    config: &Config,
    input_folder_path: &PathBuf,
    output_folder_path: &PathBuf,
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
    }

    // Handle `init` command
    if args.init {
        let target_dir = args.input_folder
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        match init::initialize_project(&target_dir) {
            Ok(_) => return Ok(0), 
            Err(e) => {
                error!("Failed to initialize project: {}", e);
                return Err(anyhow::anyhow!("Project initialization failed: {}", e));
            }
        }
    }

    // Determine execution mode
    let validation_mode = config.validation.validate_markdown || config.validation.validate_relations || config.validation.validate_all;
    let linting_mode = config.linting.lint;
    
    let matrix_mode = config.validation.generate_matrix;

    let diagrams_mode = config.general.generate_diagrams;

    if diagrams_mode {
        info!("Generating mermaid diagrams in {:?}", input_folder_path);
        // Only collect identifiers and process files to add diagrams
        // Skip validation checks for diagram generation mode
        model_manager.process_diagrams(input_folder_path)?;
        info!("Requirements diagrams updated in source files");
        return Ok(0);
    } 
    if validation_mode {
        let exit_code = validation::run_validation_checks(model_manager, config, input_folder_path)?;
        if exit_code != 0 {
            return Ok(exit_code); 
        }
        info!("All validations passed successfully!");
        return Ok(0);
    } 
    if linting_mode {
        linting::run_linting(input_folder_path, config)?;
        return Ok(0);
    } 
    if matrix_mode {
        std::fs::create_dir_all(output_folder_path)?;
        model_manager.generate_traceability_matrix(input_folder_path, output_folder_path)?;
        info!("Traceability matrix saved to {:?}", output_folder_path);
        return Ok(0);
    } 
    if config.general.html_output {
        let processed_count = html_export::export_markdown_to_html(input_folder_path, output_folder_path, config.general.verbose)?;
        info!("{} markdown files converted to HTML", processed_count);
        return Ok(0);
    } 
    
    Args::print_help();
    Ok(1) // 
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::config::Config;
    use crate::model::ModelManager;


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
            verbose: false,
            validate_markdown: false,
            validate_relations: false,
            validate_all: false,
            lint: false,
            dry_run: false,
            json: false,
            generate_matrix: false,
            generate_diagrams: false,
            input_folder: Some(PathBuf::from("test/specifications")),
            output_folder: Some(PathBuf::from("test/output")),
            config: None, // No custom config file for the test
        };

        // Create a default config instance
        let config = Config::default();

        // Define test input and output paths
        let input_folder_path = PathBuf::from("test/specifications");
        let output_folder_path = PathBuf::from("test/output");

        // Create a mock model manager
        let mut model_manager = ModelManager::new_with_config(config.clone());

        // Run the handle_command function
        let result = handle_command(
            args,
            &mut model_manager,
            &config,
            &input_folder_path,
            &output_folder_path
        );

        // Assert that it runs without error
        assert!(result.is_ok(), "handle_command should execute without errors");
    }
}


