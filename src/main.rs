use anyhow::Result;
use clap::{Parser, CommandFactory};
use std::path::PathBuf;

mod config;
mod element;
mod error;
mod html;
mod linting;
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
#[path = "tests/linting_tests.rs"]
#[cfg(test)]
mod linting_tests;

use model::ModelManager;

/// ReqFlow is an agile Model-Based Systems Engineering (MBSE) framework
/// designed to integrate seamlessly with modern Git workflows.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path to the input folder containing Markdown files
    /// If not provided, this will be read from the configuration file
    #[clap(index = 1)]
    input_folder: Option<PathBuf>,

    /// Path to the output folder (not required for validate commands)
    /// If not provided, this will be read from the configuration file
    #[clap(index = 2)]
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
    
    /// Enable linting to find potential improvements (non-blocking)
    /// By default, fixes will be applied automatically
    #[clap(long)]
    lint: bool,
    
    /// When linting, only show suggestions without applying fixes
    #[clap(long, requires = "lint")]
    dry_run: bool,
    
    /// Generate traceability matrix without processing other files
    /// Creates a matrix showing relationships between elements in the model
    #[clap(long)]
    generate_matrix: bool,
    
    /// Output validation results in JSON format
    /// Useful for CI/CD pipelines and automation
    #[clap(long)]
    json: bool,
    
    /// Generate mermaid diagrams in markdown files showing requirements relationships
    /// The diagrams will be placed at the top of each requirements document
    #[clap(long)]
    generate_diagrams: bool,
    
    /// Path to a custom configuration file (YAML format)
    /// If not provided, the system will look for reqflow.yml, reqflow.yaml, 
    /// .reqflow.yml, or .reqflow.yaml in the current directory
    #[clap(long, short = 'c')]
    config: Option<PathBuf>,
    
    /// Output LLM context document
    /// Generates a comprehensive context document with information about ReqFlow
    /// methodology, document structure, relation types, and CLI usage to help
    /// Large Language Models understand and work with ReqFlow-based projects
    #[clap(long)]
    llm_context: bool,
}

fn main() -> Result<()> {
    // Initialize logger with a more verbose default level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let args = Args::parse();
    
    // Handle LLM context option
    if args.llm_context {
        // Read and output the LLM context file
        match std::fs::read_to_string("src/llm_context.md") {
            Ok(content) => {
                println!("{}", content);
                let mut cmd = Args::command();
                let help_text = cmd.render_help().to_string();
                println!("\n\n# ReqFlow CLI Reference\n\n```\n{}\n```", help_text);
                return Ok(());
            },
            Err(e) => {
                eprintln!("Error reading LLM context file: {}", e);
                return Err(anyhow::anyhow!("Failed to read LLM context file"));
            }
        }
    }
    
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
    
    if args.lint {
        config.linting.lint = true;
        config.linting.dry_run = args.dry_run;
    }
    
    if args.json {
        config.validation.json_output = true;
    }
    
    if args.generate_matrix {
        config.validation.generate_matrix = true;
    }
    
    if args.generate_diagrams {
        config.general.generate_diagrams = true;
    }
    
    // Get input and output folders from command line arguments or configuration
    let input_folder_path = args.input_folder
        .unwrap_or_else(|| PathBuf::from(&config.paths.specifications_folder));
    
    let output_folder_path = args.output_folder
        .unwrap_or_else(|| PathBuf::from(&config.paths.output_folder));

    // Create model manager with the configuration
    let mut model_manager = ModelManager::new_with_config(config.clone());
    
    // Determine if we're in validation mode
    let validation_mode = config.validation.validate_markdown || 
                          config.validation.validate_relations || 
                          config.validation.validate_all;
                          
    // Determine if we're in linting mode
    let linting_mode = config.linting.lint;
    
    // Determine if we're in matrix generation mode
    let matrix_mode = config.validation.generate_matrix;
    
    // Determine if we're in diagram generation mode
    let diagrams_mode = config.general.generate_diagrams;
    
    if diagrams_mode {
        // Run in diagram generation mode - only update the source files with diagrams
        
        // Set log level to INFO by default for diagram generation mode
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
            // Re-initialize logger with new setting
            env_logger::builder().init();
        }
        
        println!("Generating mermaid diagrams in requirements files in {:?}", input_folder_path);
        
        // Additional information to help diagnose path issues
        let abs_path = std::fs::canonicalize(&input_folder_path)
            .unwrap_or_else(|_| input_folder_path.clone());
        println!("Absolute path: {:?}", abs_path);
        
        // Always set verbose mode for diagram generation
        config.general.verbose = true;
        model_manager = ModelManager::new_with_config(config.clone());
        
        // Only collect identifiers and process files to add diagrams
        // Skip validation checks for diagram generation mode
        model_manager.collect_identifiers_only(&input_folder_path)?;
        
        // Process just to generate diagrams (without writing to output folder)
        // This will update source files in place with diagrams
        model_manager.process_diagrams(&input_folder_path)?;
        
        println!("Requirements diagrams generated and updated in source files");
    } else if validation_mode {
        // Run in validation mode
        if config.general.verbose {
            println!("Validating files in {:?}", input_folder_path);
        }
        
        // Only collect identifiers, don't process files
        model_manager.collect_identifiers_only(&input_folder_path)?;
        
        // Run the appropriate validations
        let mut exit_code = 0;
        
        if config.validation.validate_markdown || config.validation.validate_all {
            let markdown_errors = model_manager.validate_markdown_structure()?;
            if !markdown_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "markdown".to_string(),
                        errors: markdown_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: false,
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
            let relation_errors = model_manager.validate_relations()?;
            if !relation_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "relations".to_string(),
                        errors: relation_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: false,
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
            let filesystem_errors = model_manager.validate_filesystem_structure(&input_folder_path, &config)?;
            if !filesystem_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "filesystem".to_string(),
                        errors: filesystem_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: false,
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
            let dependency_errors = model_manager.validate_cross_component_dependencies()?;
            if !dependency_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "dependencies".to_string(),
                        errors: dependency_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: false,
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
    } else if linting_mode {
        // Run in linting mode
        if config.general.verbose {
            let dry_run_msg = if config.linting.dry_run { " (dry run)" } else { "" };
            
            println!("Linting requirements files in {:?}{}", 
                input_folder_path, 
                dry_run_msg);
        }
        
        // Run the linting with the configuration
        // If not in dry-run mode, we might need multiple passes to fix everything
        let mut lint_suggestions = Vec::new();
        let mut iteration = 0;
        let max_iterations = 5; // Prevent infinite loops
        
        loop {
            let current_suggestions = linting::lint_directory_with_config(
                &input_folder_path, 
                config.linting.dry_run,
                &config
            )?;
            
            // If no more issues or in dry-run mode, we're done
            if current_suggestions.is_empty() {
                break;
            }
            
            // Add to total suggestions found
            lint_suggestions.extend(current_suggestions.clone());
            
            // If in dry-run mode, don't attempt multiple passes
            if config.linting.dry_run {
                break;
            }
            
            // Check iteration count to prevent infinite loops
            iteration += 1;
            if iteration >= max_iterations {
                println!("⚠️ Reached maximum lint iterations ({}). Some fixes might require manual adjustment.", max_iterations);
                break;
            }
        }
        
        if lint_suggestions.is_empty() {
            println!("✅ No linting suggestions found. Your files are clean!");
        } else {
            println!("ℹ️ Found {} linting suggestions:", lint_suggestions.len());
            
            // Group suggestions by file and type for better readability
            let mut grouped_by_file: std::collections::HashMap<String, Vec<&linting::LintSuggestion>> = 
                std::collections::HashMap::new();
            
            for suggestion in &lint_suggestions {
                let file_path = suggestion.file_path.to_string_lossy().to_string();
                grouped_by_file.entry(file_path).or_default().push(suggestion);
            }
            
            // Print a summary first
            println!("Files with formatting issues:");
            for (file_path, suggestions) in &grouped_by_file {
                // Group by issue type within each file
                let mut type_counts: std::collections::HashMap<linting::LintType, usize> = 
                    std::collections::HashMap::new();
                
                for suggestion in suggestions {
                    *type_counts.entry(suggestion.suggestion_type.clone()).or_default() += 1;
                }
                
                // Check if this is a requirements file using our enhanced utility function
                let path = std::path::Path::new(file_path);
                let is_requirements_file = crate::utils::is_requirements_file_only(
                    path, 
                    &config, 
                    &std::path::PathBuf::from(input_folder_path.to_str().unwrap_or(".")), 
                    config.general.verbose
                );
                
                // Add a tag to indicate if this is a requirements file
                let file_type_tag = if is_requirements_file {
                    "[requirement document]"
                } else {
                    "[other document]" 
                };
                
                println!("  {} {} ({})", file_path, file_type_tag, suggestions.len());
                for (lint_type, count) in &type_counts {
                    let type_name = match lint_type {
                        linting::LintType::AbsoluteLink => "Absolute links",
                        linting::LintType::ExcessWhitespace => "Excess whitespace",
                        linting::LintType::InconsistentNewlines => "Inconsistent newlines",
                        linting::LintType::MissingSeparator => "Missing separators",
                        linting::LintType::InconsistentIndentation => "Inconsistent indentation",
                    };
                    println!("    - {}: {}", type_name, count);
                }
            }
            
            // Then print detailed git-style diffs
            println!("\nSuggested changes (git diff style):");
            
            // Sort suggestions by file and line number for easier reading
            let mut sorted_suggestions = lint_suggestions.clone();
            sorted_suggestions.sort_by(|a, b| {
                let file_cmp = a.file_path.to_string_lossy().cmp(&b.file_path.to_string_lossy());
                if file_cmp == std::cmp::Ordering::Equal {
                    a.line_number.unwrap_or(0).cmp(&b.line_number.unwrap_or(0))
                } else {
                    file_cmp
                }
            });
            
            for suggestion in &sorted_suggestions {
                // Print the colorized diff, ignoring any errors
                let _ = suggestion.print_colorized_diff();
            }
            
            if config.linting.dry_run {
                println!("Run without --dry-run to apply these fixes automatically");
            } else {
                println!("✅ All suggestions have been applied");
            }
        }
    } else if matrix_mode {
        // Run in matrix generation mode
        if config.general.verbose {
            println!("Generating traceability matrix from {:?} to {:?}", input_folder_path, output_folder_path);
        }
        
        // Create the output folder if it doesn't exist
        std::fs::create_dir_all(&output_folder_path)?;
        
        // First collect identifiers to build the element registry
        model_manager.collect_identifiers_only(&input_folder_path)?;
        
        // Generate the traceability matrix to be saved in the input directory (specifications root)
        // and HTML version to output directory if requested
        model_manager.generate_traceability_matrix(&input_folder_path, &output_folder_path, config.general.html_output)?;
        
        if config.general.html_output {
            println!("Traceability matrix generated: Markdown saved to {:?}, HTML saved to {:?}", input_folder_path, output_folder_path);
        } else {
            println!("Traceability matrix generated and saved to {:?}", input_folder_path);
        }
    } else if config.general.html_output {
        // Run in HTML conversion mode
        if config.general.verbose {
            println!("Converting markdown files to HTML from {:?} to {:?}", input_folder_path, output_folder_path);
        }
        
        // Create the output folder if it doesn't exist
        std::fs::create_dir_all(&output_folder_path)?;
        
        // Process the files and convert to HTML
        // The convert_to_html flag is set to true
        model_manager.process_files(
            &input_folder_path,
            &output_folder_path,
            true, // Convert to HTML
        )?;
        
        println!("Markdown files converted to HTML and saved to {:?}", output_folder_path);
    } else {
        // No specific mode was selected - print help and exit
        Args::command().print_help()?;
        println!("\n\nNo operation specified. Please select an operation to perform.");
        std::process::exit(1)
    }
    
    Ok(())
}

/// Structure for JSON output of validation results
#[derive(serde::Serialize)]
struct ValidationResult {
    validation_type: String,
    errors: Vec<String>,
    fixed: bool, // Kept for API compatibility but always false now
}