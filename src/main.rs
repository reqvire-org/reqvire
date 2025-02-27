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
}

fn main() -> Result<()> {
    // Initialize logger with a more verbose default level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let args = Args::parse();

    // Create model manager that will load model data
    let mut model_manager = ModelManager::new();
    
    // Determine if we're in validation mode
    let validation_mode = args.validate_markdown || args.validate_relations || args.validate_all;
    
    if validation_mode {
        // Run in validation mode
        if args.verbose {
            println!("Validating files in {:?}", args.input_folder);
        }
        
        // Only collect identifiers, don't process files
        model_manager.collect_identifiers_only(&args.input_folder)?;
        
        // Run the appropriate validations
        let mut exit_code = 0;
        
        if args.validate_markdown || args.validate_all {
            let markdown_errors = model_manager.validate_markdown_structure(args.fix)?;
            if !markdown_errors.is_empty() {
                exit_code = 1;
                
                if args.json {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "markdown".to_string(),
                        errors: markdown_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: args.fix,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Markdown validation failed with {} errors:", markdown_errors.len());
                    for error in markdown_errors {
                        println!("  - {}", error);
                    }
                }
            } else if args.verbose || !args.json {
                println!("✅ Markdown structure validation passed");
            }
        }
        
        if args.validate_relations || args.validate_all {
            let relation_errors = model_manager.validate_relations(args.fix)?;
            if !relation_errors.is_empty() {
                exit_code = 1;
                
                if args.json {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "relations".to_string(),
                        errors: relation_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: args.fix,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Relation validation failed with {} errors:", relation_errors.len());
                    for error in relation_errors {
                        println!("  - {}", error);
                    }
                }
            } else if args.verbose || !args.json {
                println!("✅ Relations validation passed");
            }
        }
        
        if args.validate_all {
            // Filesystem structure validation (directory layout, file naming, etc.)
            let filesystem_errors = model_manager.validate_filesystem_structure(&args.input_folder, args.fix)?;
            if !filesystem_errors.is_empty() {
                exit_code = 1;
                
                if args.json {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "filesystem".to_string(),
                        errors: filesystem_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: args.fix,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Filesystem structure validation failed with {} errors:", filesystem_errors.len());
                    for error in filesystem_errors {
                        println!("  - {}", error);
                    }
                }
            } else if args.verbose || !args.json {
                println!("✅ Filesystem structure validation passed");
            }
            
            // Cross-component dependency validation
            let dependency_errors = model_manager.validate_cross_component_dependencies(args.fix)?;
            if !dependency_errors.is_empty() {
                exit_code = 1;
                
                if args.json {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "dependencies".to_string(),
                        errors: dependency_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: args.fix,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Cross-component dependency validation failed with {} errors:", dependency_errors.len());
                    for error in dependency_errors {
                        println!("  - {}", error);
                    }
                }
            } else if args.verbose || !args.json {
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
        if args.verbose {
            println!("Processing files from {:?} to {:?}", args.input_folder, args.output_folder.as_ref().unwrap());
        }

        // Process files normally
        model_manager.process_files(&args.input_folder, args.output_folder.as_ref().unwrap(), args.html)?;

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