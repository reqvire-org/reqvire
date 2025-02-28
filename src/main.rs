use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod config;
mod element;
mod error;
mod html;
mod markdown;
mod model;
mod relation;
mod utils;
mod validation;
#[cfg(test)]
mod tests;
#[path = "tests/validation_tests.rs"]
#[cfg(test)]
mod validation_tests;
#[path = "tests/config_tests.rs"]
#[cfg(test)]
mod config_tests;

use model::ModelManager;

/// ReqFlow is an agile Model-Based Systems Engineering (MBSE) framework
/// designed to integrate seamlessly with modern Git workflows.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path to the input folder containing Markdown files
    #[clap(index = 1)]
    input_folder: PathBuf,

    /// Path to the output folder (not required for validate commands)
    #[clap(index = 2, required_unless_present_any = &["validate_markdown", "validate_relations", "validate_all"])]
    output_folder: Option<PathBuf>,

    /// Convert Markdown to HTML with embedded styles
    #[clap(long)]
    html: bool,

    /// Enable verbose output
    #[clap(short, long)]
    verbose: bool,
    
    /// Validate Markdown structure only
    /// Checks for:
    /// - Unique element names within documents
    /// - Proper heading hierarchy
    /// - Well-formed relation sections
    #[clap(long, conflicts_with = "html")]
    validate_markdown: bool,
    
    /// Validate relations only
    /// Checks for:
    /// - Valid relation types according to spec
    /// - Valid relation targets (elements and files exist)
    /// - Proper relation formatting
    #[clap(long, conflicts_with = "html")]
    validate_relations: bool,
    
    /// Run all validations
    /// Includes markdown structure validation, relation validation,
    /// and cross-component dependency validation
    #[clap(long, conflicts_with = "html")]
    validate_all: bool,
    
    /// Fixes common issues automatically
    /// Can be combined with validation commands
    #[clap(long)]
    fix: bool,
    
    /// Output validation results in JSON format
    /// Useful for CI/CD pipelines and automation
    #[clap(long)]
    json: bool,
    
    /// Path to a custom configuration file (YAML format)
    /// If not provided, the system will look for reqflow.yml, reqflow.yaml, 
    /// .reqflow.yml, or .reqflow.yaml in the current directory
    #[clap(long, short = 'c')]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    // Initialize logger with a more verbose default level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let args = Args::parse();

    // Load configuration
    let mut config = match &args.config {
        Some(config_path) => {
            match config::Config::from_file(config_path) {
                Ok(cfg) => {
                    println!("Loaded configuration from {:?}", config_path);
                    cfg
                },
                Err(e) => {
                    eprintln!("Error loading configuration from {:?}: {}", config_path, e);
                    eprintln!("Using default configuration");
                    config::Config::default()
                }
            }
        },
        None => {
            config::Config::load()
        }
    };
    
    // Update config with command line arguments
    if args.html {
        config.general.html_output = true;
    }
    
    if args.verbose {
        config.general.verbose = true;
    }
    
    if args.validate_markdown {
        config.validation.validate_markdown = true;
    }
    
    if args.validate_relations {
        config.validation.validate_relations = true;
    }
    
    if args.validate_all {
        config.validation.validate_all = true;
    }
    
    if args.fix {
        config.validation.fix_automatically = true;
    }
    
    if args.json {
        config.validation.json_output = true;
    }

    // Create model manager with the configuration
    let mut model_manager = ModelManager::new_with_config(config.clone());
    
    // Determine if we're in validation mode
    let validation_mode = config.validation.validate_markdown || 
                          config.validation.validate_relations || 
                          config.validation.validate_all;
    
    if validation_mode {
        // Run in validation mode
        if config.general.verbose {
            println!("Validating files in {:?}", args.input_folder);
        }
        
        // Only collect identifiers, don't process files
        model_manager.collect_identifiers_only(&args.input_folder)?;
        
        // Run the appropriate validations
        let mut exit_code = 0;
        
        if config.validation.validate_markdown || config.validation.validate_all {
            let markdown_errors = model_manager.validate_markdown_structure(config.validation.fix_automatically)?;
            if !markdown_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "markdown".to_string(),
                        errors: markdown_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: config.validation.fix_automatically,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Markdown validation failed with {} errors:", markdown_errors.len());
                    for error in markdown_errors {
                        println!("  - {}", error);
                    }
                }
            } else if config.general.verbose || !config.validation.json_output {
                println!("✅ Markdown structure validation passed");
            }
        }
        
        if config.validation.validate_relations || config.validation.validate_all {
            let relation_errors = model_manager.validate_relations(config.validation.fix_automatically)?;
            if !relation_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "relations".to_string(),
                        errors: relation_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: config.validation.fix_automatically,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Relation validation failed with {} errors:", relation_errors.len());
                    for error in relation_errors {
                        println!("  - {}", error);
                    }
                }
            } else if config.general.verbose || !config.validation.json_output {
                println!("✅ Relations validation passed");
            }
        }
        
        if config.validation.validate_all {
            // Filesystem structure validation (directory layout, file naming, etc.)
            let filesystem_errors = model_manager.validate_filesystem_structure(&args.input_folder, 
                                                               config.validation.fix_automatically,
                                                               &config)?;
            if !filesystem_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "filesystem".to_string(),
                        errors: filesystem_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: config.validation.fix_automatically,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Filesystem structure validation failed with {} errors:", filesystem_errors.len());
                    for error in filesystem_errors {
                        println!("  - {}", error);
                    }
                }
            } else if config.general.verbose || !config.validation.json_output {
                println!("✅ Filesystem structure validation passed");
            }
            
            // Cross-component dependency validation
            let dependency_errors = model_manager.validate_cross_component_dependencies(config.validation.fix_automatically)?;
            if !dependency_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "dependencies".to_string(),
                        errors: dependency_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: config.validation.fix_automatically,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Cross-component dependency validation failed with {} errors:", dependency_errors.len());
                    for error in dependency_errors {
                        println!("  - {}", error);
                    }
                }
            } else if config.general.verbose || !config.validation.json_output {
                println!("✅ Cross-component dependency validation passed");
            }
        }
        
        // Exit with appropriate code based on validation results
        if exit_code != 0 {
            std::process::exit(exit_code);
        }
        
        println!("All validations passed successfully!");
    } else {
        // Normal processing mode
        if config.general.verbose {
            println!("Processing files from {:?} to {:?}", args.input_folder, args.output_folder.as_ref().unwrap());
        }

        // Process files normally
        model_manager.process_files(&args.input_folder, 
                                  args.output_folder.as_ref().unwrap(), 
                                  config.general.html_output)?;

        println!("Files processed and saved to {:?}", args.output_folder.as_ref().unwrap());
    }
    
    Ok(())
}

/// Structure for JSON output of validation results
#[derive(serde::Serialize)]
struct ValidationResult {
    validation_type: String,
    errors: Vec<String>,
    fixed: bool,
}