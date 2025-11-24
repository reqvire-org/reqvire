use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use log::{debug,warn,info};
use crate::git_commands;

use crate::error::ReqvireError;
use crate::html_export;
use crate::graph_registry::GraphRegistry;
use crate::filesystem;


fn prepare_output_folder(output_folder: &Path) -> std::io::Result<()> {
    // Clean output folder
    if output_folder.exists() {
        fs::remove_dir_all(output_folder)?;
    }
    fs::create_dir_all(output_folder)?;

    Ok(())
}

/// Assets folder embedded at compile time
const ASSETS: &[(&str, &[u8])] = &[
    ("logo.png", include_bytes!("../templates/assets/logo.png")),
    ("favicon.ico", include_bytes!("../templates/assets/favicon.ico")),
    ("apple-touch-icon.png", include_bytes!("../templates/assets/apple-touch-icon.png")),
    ("android-chrome-192x192.png", include_bytes!("../templates/assets/android-chrome-192x192.png")),
];

/// Page descriptions for HTML export pages
const PAGE_DESCRIPTION_CONTAINMENT: &str = r#"# Containment

The containment view shows the physical organization of the model—how elements are structured within folders and files. This hierarchical view reflects the file system layout and helps navigate to specific specification documents. In MBSE, containment represents where model elements are stored, separate from their logical relationships."#;

const PAGE_DESCRIPTION_MODEL: &str = r#"# Model

The model view displays the logical structure starting from root requirements—requirements without parent derivations. Each element shows its complete relation tree: derived child requirements, verifications, and implementations. This follows MBSE principles where stakeholder needs flow down through requirement hierarchies to verifiable, implementable specifications."#;

const PAGE_DESCRIPTION_WHOLE_MODEL: &str = r#"# Whole Model

This diagram visualizes the complete model as a single interconnected graph showing all elements and their relationships. Hover over any node to highlight its connected elements—ancestors (upstream) and descendants (downstream). Use this bird's-eye view to understand the overall requirements architecture and identify traceability chains across the model."#;

const PAGE_DESCRIPTION_TRACES: &str = r#"# Verification Traces

Verification traces show upward traceability from each verification through the requirement hierarchy. Using the **roll-up strategy**, verifying a leaf requirement automatically provides coverage to all its ancestors through derivedFrom relations—you don't need to verify every level. Each diagram marks directly verified requirements, helping identify redundant verify relations where both a requirement and its ancestor are explicitly verified."#;

const PAGE_DESCRIPTION_COVERAGE: &str = r#"# Verification Coverage

Coverage analysis focuses on **leaf requirements**—the lowest-level requirements that don't derive others. In MBSE, these are the implementable specifications. The **roll-up strategy** means verifying leaves provides automatic coverage to their ancestors through derivedFrom chains. This report shows verified vs. unverified leaf percentages by file and type, identifying where verification effort is needed."#;

/// Copies assets folder to output directory
fn copy_assets_folder(output_dir: &Path) -> Result<(), ReqvireError> {
    let assets_dir = output_dir.join("assets");
    fs::create_dir_all(&assets_dir)
        .map_err(|e| ReqvireError::IoError(e))?;

    for (filename, content) in ASSETS {
        let dest_path = assets_dir.join(filename);
        fs::write(&dest_path, content)
            .map_err(|e| ReqvireError::IoError(e))?;
        debug!("Copied asset: {}", filename);
    }

    info!("✅ Copied {} assets", ASSETS.len());
    Ok(())
}


/// Copies all model files from graph registry to temporary directory
pub fn copy_model_files_to_temp(
    registry: &GraphRegistry,
    temp_dir: &Path,
    current_dir: &Path,
    git_root: &Path,
) -> Result<(), ReqvireError> {

    // Determine if we're in a subdirectory and get the relative path prefix to strip
    let subdir_prefix = if current_dir.starts_with(&git_root) && current_dir != git_root {
        current_dir.strip_prefix(&git_root).ok()
    } else {
        None
    };

    info!("Copying model files to temporary directory...");

    let mut copied_files = HashSet::new();

    // Copy all model markdown files from pages
    for file_path in registry.pages.keys() {
        let src = git_root.join(file_path);

        // Strip subdirectory prefix from destination path if running from subdirectory
        let dest_path = if let Some(prefix) = subdir_prefix {
            if let Ok(stripped) = Path::new(file_path.as_str()).strip_prefix(prefix) {
                temp_dir.join(stripped)
            } else {
                temp_dir.join(file_path)
            }
        } else {
            temp_dir.join(file_path)
        };

        if src.exists() && !copied_files.contains(file_path.as_str()) {
            filesystem::copy_file_with_structure(&src, &dest_path)?;
            copied_files.insert(file_path.clone());
            debug!("Copied model file: {} -> {}", file_path, dest_path.display());
        }
    }

    // Copy all files referenced in relations
    for node in registry.nodes.values() {
        for relation in &node.element.relations {
            if let crate::relation::LinkType::InternalPath(path) = &relation.target.link {
                let src = git_root.join(path);
                let path_str = path.to_string_lossy().to_string();

                if src.is_file() && !copied_files.contains(&path_str) {
                    // Strip subdirectory prefix from destination path if running from subdirectory
                    let dest = if let Some(prefix) = subdir_prefix {
                        if let Ok(stripped) = path.strip_prefix(prefix) {
                            temp_dir.join(stripped)
                        } else {
                            temp_dir.join(path)
                        }
                    } else {
                        temp_dir.join(path)
                    };

                    filesystem::copy_file_with_structure(&src, &dest)?;
                    copied_files.insert(path_str);
                    debug!("Copied relation target: {} -> {}", path.display(), dest.display());
                }
            }
        }

        // Copy all attachment files
        for attachment in &node.element.attachments {
            let path = &attachment.file_path;
            let src = git_root.join(path);
            let path_str = path.to_string_lossy().to_string();

            if src.is_file() && !copied_files.contains(&path_str) {
                // Strip subdirectory prefix from destination path if running from subdirectory
                let dest = if let Some(prefix) = subdir_prefix {
                    if let Ok(stripped) = path.strip_prefix(prefix) {
                        temp_dir.join(stripped)
                    } else {
                        temp_dir.join(path)
                    }
                } else {
                    temp_dir.join(path)
                };

                filesystem::copy_file_with_structure(&src, &dest)?;
                copied_files.insert(path_str);
                debug!("Copied attachment: {} -> {}", path.display(), dest.display());
            }
        }
    }

    info!("✅ Copied {} files to temporary directory", copied_files.len());
    Ok(())
}

/// Copies HTML output from temp directory to final output directory
/// Skips .md files only if a corresponding .html file exists
pub fn copy_html_output(
    temp_dir: &Path,
    output_dir: &Path,
) -> Result<(), ReqvireError> {
    info!("Copying HTML output to {}...", output_dir.display());

    prepare_output_folder(output_dir)?;

    // Recursively copy files, skipping .md files that have .html equivalents
    copy_html_and_assets(temp_dir, output_dir, temp_dir)?;

    info!("✅ HTML output copied to {}", output_dir.display());
    Ok(())
}

/// Helper function to recursively copy files, skipping .md files that have .html equivalents
fn copy_html_and_assets(src: &Path, dst: &Path, temp_root: &Path) -> Result<(), ReqvireError> {
    fs::create_dir_all(dst)
        .map_err(|e| ReqvireError::IoError(e))?;

    for entry in fs::read_dir(src).map_err(|e| ReqvireError::IoError(e))? {
        let entry = entry.map_err(|e| ReqvireError::IoError(e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            // Skip .git directory
            if entry.file_name() == ".git" {
                continue;
            }
            // Recursively copy directories
            copy_html_and_assets(&src_path, &dst_path, temp_root)?;
        } else if src_path.is_file() {
            // Skip .md files only if corresponding .html exists
            let should_copy = if let Some(ext) = src_path.extension() {
                if ext == "md" {
                    // Check if .html version exists
                    let html_path = src_path.with_extension("html");
                    !html_path.exists()
                } else {
                    true
                }
            } else {
                true
            };

            if should_copy {
                filesystem::copy_file_with_structure(&src_path, &dst_path)?;
                debug!("Copied: {}", dst_path.display());
            } else {
                debug!("Skipped .md file (HTML exists): {}", src_path.display());
            }
        }
    }
    Ok(())
}

/// Post-processes generated HTML files to convert .md references to .html in display text
/// This fixes text like "File: path/to/file.md" that appears in HTML content
fn post_process_html_files(temp_dir: &Path) -> Result<(), ReqvireError> {
    let html_files = vec!["index.html", "traces.html", "coverage.html", "containment.html"];

    for file_name in html_files {
        let file_path = temp_dir.join(file_name);
        if !file_path.exists() {
            continue;
        }

        let content = fs::read_to_string(&file_path)
            .map_err(|e| ReqvireError::IoError(e))?;

        // Convert .md references to .html in HTML text content and id attributes
        // This handles heading text and other display text containing file paths
        // Example: <h2 id="file:-path/to/file.md">File: path/to/file.md</h2>
        // becomes: <h2 id="file:-path/to/file.html">File: path/to/file.html</h2>
        let processed = content
            .replace(".md\"", ".html\"")  // Fix id attributes and quoted strings
            .replace(".md</h1>", ".html</h1>")
            .replace(".md</h2>", ".html</h2>")
            .replace(".md</h3>", ".html</h3>")
            .replace(".md</h4>", ".html</h4>")
            .replace(".md</h5>", ".html</h5>")
            .replace(".md</h6>", ".html</h6>");

        fs::write(&file_path, processed)
            .map_err(|e| ReqvireError::IoError(e))?;

        debug!("Post-processed HTML: {}", file_name);
    }

    Ok(())
}

/// Generates all artifacts (diagrams, index, traces, coverage, HTML) in temporary directory
///
/// Returns the path to the temporary directory containing all generated artifacts.
/// The caller is responsible for cleanup.
///
/// Steps:
/// 1. Creates temporary working directory
/// 2. Copies all model files to temp
/// 3. Initializes git repository in temp directory
/// 4. Changes to temp directory and reloads model
/// 5. Generates all artifacts (diagrams, index, traces, coverage)
/// 6. Converts markdown to HTML
/// 7. Restores original directory
///
/// Returns: PathBuf to temporary directory with all generated content
pub fn generate_artifacts_in_temp(
    registry: &GraphRegistry,
    excluded_patterns: &globset::GlobSet,
    diagrams_with_blobs: bool,
    current_dir: &Path,
    git_root: &Path,
) -> Result<PathBuf, ReqvireError> {
    use std::env;

    // Step 1: Create temp directory
    info!("Creating temporary working directory...");
    let temp_dir = filesystem::create_temp_working_dir()?;
    info!("✅ Temporary directory: {}", temp_dir.display());

    // Step 2: Copy all model files to temp
    copy_model_files_to_temp(registry, &temp_dir, current_dir, git_root)?;

    // Step 3: Initialize git repository in temp directory
    info!("Initializing git repository in temporary directory...");
    std::process::Command::new("git")
        .arg("init")
        .current_dir(&temp_dir)
        .output()
        .map_err(|e| ReqvireError::PathError(format!("Failed to initialize git repo in temp: {}", e)))?;

    // Step 4: Change to temp directory and create new model manager
    let original_dir = env::current_dir()
        .map_err(|e| ReqvireError::IoError(e))?;

    env::set_current_dir(&temp_dir)
        .map_err(|e| ReqvireError::IoError(e))?;

    // Clear git cache so paths resolve to temp directory instead of original repo
    git_commands::clear_git_cache();

    // Create new model manager and parse from temp directory
    info!("Loading model from temporary directory...");
    let mut temp_model_manager = crate::ModelManager::new();
    let parse_result = temp_model_manager.parse_and_validate(
        None,
        excluded_patterns
    );

    // Check for validation errors
    if let Err(e) = parse_result {
        // Log the error before returning
        match &e {
            ReqvireError::ValidationError(errors) => {
                eprintln!("\n❌ Validation failed in temporary directory with {} error(s):", errors.len());
                for (i, error) in errors.iter().enumerate() {
                    eprintln!("  {}. {}", i + 1, error);
                }
            }
            _ => eprintln!("❌ Parsing failed in temporary directory: {}", e),
        }
        // Restore original directory before returning error
        let _ = env::set_current_dir(&original_dir);
        return Err(e);
    }

    // Step 5: Generate all artifacts in temp directory
    info!("Generating diagrams...");
    crate::diagrams::process_diagrams(
        &temp_model_manager.graph_registry,
        diagrams_with_blobs
    )?;

    info!("Generating index.md...");
    let index_content = crate::index_generator::generate_readme_index(
        &temp_model_manager.graph_registry,
        &PathBuf::from(".")
    )?;
    filesystem::write_file("index.md", index_content.as_bytes())?;


    // Generate model-centric view (root requirements with nested relations)
    info!("Generating model.md...");
    let model_report = crate::report_model::generate_model_report(
        &temp_model_manager.graph_registry,
        None,  // No filtering - use root requirements
        false, // Markdown output
        "TD"   // Top-down diagrams for HTML export
    )?;
    let model_content = format!(
        "{}\n\n{}",
        PAGE_DESCRIPTION_MODEL,
        model_report
    );
    filesystem::write_file("model.md", model_content.as_bytes())?;

    // Generate whole model diagram (all elements and relations)
    info!("Generating whole-model.md...");
    let whole_model_mermaid = crate::diagrams::generate_model_diagram(&temp_model_manager.graph_registry, None)?;
    let whole_model_content = format!(
        "{}\n\n{}",
        PAGE_DESCRIPTION_WHOLE_MODEL,
        whole_model_mermaid
    );
    filesystem::write_file("whole-model.md", whole_model_content.as_bytes())?;

    info!("Generating traces.md...");
    let trace_generator = crate::verification_trace::VerificationTraceGenerator::new(
        &temp_model_manager.graph_registry,
        false,  // Always use relative links for traces in HTML export
        None
    );
    let trace_report = trace_generator.generate();
    let traces_markdown = trace_generator.generate_markdown(&trace_report);
    let traces_content = format!(
        "{}\n\n{}",
        PAGE_DESCRIPTION_TRACES,
        traces_markdown
    );
    filesystem::write_file("traces.md", traces_content.as_bytes())?;

    info!("Generating coverage.md...");
    let coverage_report = crate::report_coverage::generate_coverage_report(&temp_model_manager.graph_registry);
    let coverage_text = coverage_report.format_text();
    let coverage_content = format!(
        "{}\n\n{}",
        PAGE_DESCRIPTION_COVERAGE,
        coverage_text
    );
    filesystem::write_file("coverage.md", coverage_content.as_bytes())?;

    info!("Generating containment.md...");
    let containment_diagram = crate::diagrams::generate_containment_diagram(&temp_model_manager.graph_registry, false)?;
    let containment_content = format!(
        "{}\n\n{}",
        PAGE_DESCRIPTION_CONTAINMENT,
        containment_diagram
    );
    filesystem::write_file("containment.md", containment_content.as_bytes())?;

    // Step 6: Convert markdown to HTML
    info!("Converting markdown to HTML...");
    let html_count = html_export::export_markdown_to_html(&temp_dir, &temp_dir)?;
    info!("✅ Converted {} markdown files to HTML", html_count);

    // Step 6.5: Copy assets folder for HTML pages
    info!("Copying assets...");
    copy_assets_folder(&temp_dir)?;

    // Step 6.6: Post-process HTML files to convert .md references to .html
    info!("Post-processing HTML artifacts...");
    post_process_html_files(&temp_dir)?;

    // Step 7: Restore original directory
    env::set_current_dir(&original_dir)
        .map_err(|e| ReqvireError::IoError(e))?;

    // Clear git cache again so it refreshes for original directory
    git_commands::clear_git_cache();

    Ok(temp_dir)
}

/// Finalizes export by copying temp directory to output and cleaning up
pub fn finalize_export(temp_dir: &Path, output_dir: &Path, cleanup: bool) -> Result<(), ReqvireError> {
    // Copy everything to output directory
    copy_html_output(temp_dir, output_dir)?;

    // Cleanup temp directory if requested
    if cleanup {
        filesystem::remove_dir_all(temp_dir)?;
        info!("✅ Cleaned up temporary directory");
    }

    info!("✅ HTML export complete: {}", output_dir.display());
    Ok(())
}

/// Exports comprehensive HTML documentation with all model artifacts (complete pipeline)
pub fn export_model_with_artifacts(
    registry: &GraphRegistry,
    output_dir: &Path,
    excluded_patterns: &globset::GlobSet,
    diagrams_with_blobs: bool,
    current_dir: &Path,
    git_root: &Path,
) -> Result<(), ReqvireError> {
    let temp_dir = generate_artifacts_in_temp(
        registry,
        excluded_patterns,
        diagrams_with_blobs,
        current_dir,
        git_root
    )?;

    finalize_export(&temp_dir, output_dir, true)?;

    Ok(())
}

/// Converts Markdown → HTML *and* copies all registry-internal files into `output_folder`.
pub fn export_model(
    registry: &GraphRegistry,
    output_folder: &Path,
) -> Result<usize, ReqvireError> {

    // Try to get repository root as base directory
    let base_dir = match git_commands::get_git_root_dir() {
        Ok(git_root) => git_root,
        Err(_) => {
            // If Git repository root can't be found, use the current working directory
            std::env::current_dir()
                .map_err(|e| ReqvireError::PathError(format!("Failed to get current directory: {}", e)))?
        }
    };
    
    // prepare output folder
    prepare_output_folder(&output_folder)?;

    let count = html_export::export_markdown_to_html(&base_dir, output_folder)?;
    
    debug!("{} markdown files converted to HTML", count);

    let internal_paths: HashSet<PathBuf> = registry.get_internal_path_targets();

    for src in internal_paths {
        // src is e.g. "core/src/linting/newlines.rs"
        if !src.is_file() {
            warn!("Skipping missing/non-file path: {:?}", src);
            continue;
        }

        // Build the destination: output_folder/core/src/linting/newlines.rs
        let dst = output_folder.join(&src);

        // Ensure parent dirs exist: output_folder/core/src/linting
        if let Some(parent) = dst.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                warn!("Failed to create directory {:?}: {}", parent, e);
                continue;
            }
        }

        // Copy the file
        match fs::copy(&src, &dst) {
            Ok(_)  => println!("✅ Exported: {:?} -> {}", src.display(), dst.display()),
            Err(e) => warn!("Failed to copy {:?}: {}", src, e),
        }
    }

    Ok(count)
}

