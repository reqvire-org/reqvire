use crate::git_commands;
use log::{debug, info, warn};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ReqvireError;
use crate::filesystem;
use crate::graph_registry::GraphRegistry;
use crate::html_export;

fn prepare_output_folder(output_folder: &Path) -> std::io::Result<()> {
    // Clean output folder
    if output_folder.exists() {
        fs::remove_dir_all(output_folder)?;
    }
    fs::create_dir_all(output_folder)?;

    Ok(())
}

/// Compiled Explorer SPA bundle, embedded at compile time by `build.rs`.
///
/// The exported/served `index.html` is this Vite/React/Radix bundle with the
/// immutable Project Store seed injected before the bundle script — not a
/// runtime-assembled page. Tailwind is compiled into `EXPLORER_BUNDLE_CSS`
/// (no CDN / runtime Tailwind). See `build.rs`.
const EXPLORER_INDEX_HTML: &str =
    include_str!(concat!(env!("OUT_DIR"), "/explorer_bundle/index.html"));
const EXPLORER_BUNDLE_JS: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/explorer_bundle/explorer.js"));
const EXPLORER_BUNDLE_CSS: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/explorer_bundle/explorer.css"));

/// Assets folder embedded at compile time
const ASSETS: &[(&str, &[u8])] = &[
    ("logo.png", include_bytes!("../templates/assets/logo.png")),
    (
        "logo-long.png",
        include_bytes!("../templates/assets/logo-long.png"),
    ),
    (
        "favicon.ico",
        include_bytes!("../templates/assets/favicon.ico"),
    ),
    (
        "apple-touch-icon.png",
        include_bytes!("../templates/assets/apple-touch-icon.png"),
    ),
    (
        "android-chrome-192x192.png",
        include_bytes!("../templates/assets/android-chrome-192x192.png"),
    ),
];

/// Copies assets folder to output directory
fn copy_assets_folder(output_dir: &Path) -> Result<(), ReqvireError> {
    let assets_dir = output_dir.join("assets");
    fs::create_dir_all(&assets_dir).map_err(ReqvireError::IoError)?;

    for (filename, content) in ASSETS {
        let dest_path = assets_dir.join(filename);
        fs::write(&dest_path, content).map_err(ReqvireError::IoError)?;
        debug!("Copied asset: {}", filename);
    }

    info!("✅ Copied {} assets", ASSETS.len());
    Ok(())
}

/// Writes the exported `index.html` as the compiled Explorer SPA bundle with
/// the Project Store seed injected immediately before the bundle script, and
/// emits the bundle's `assets/explorer.{js,css}`.
///
/// Explorer views render as native SPA routes from the browser-local Project
/// Store seed. Export does not emit standalone Explorer/report pages.
fn write_explorer_index(
    output_dir: &Path,
    store: &crate::html::store::ExplorerProjectStore,
) -> Result<(), ReqvireError> {
    let seed = crate::html::store::project_store_script(store)?;

    // Insert the seed script right before the Explorer bundle's module script so
    // `window`/`#reqvire-project-store` is present before the SPA boots.
    let bundle_marker = "<script type=\"module\"";
    let html = match EXPLORER_INDEX_HTML.find(bundle_marker) {
        Some(pos) => format!(
            "{}{}\n    {}",
            &EXPLORER_INDEX_HTML[..pos],
            seed,
            &EXPLORER_INDEX_HTML[pos..]
        ),
        None => format!("{seed}\n{EXPLORER_INDEX_HTML}"),
    };

    fs::write(output_dir.join("index.html"), html).map_err(ReqvireError::IoError)?;

    let assets_dir = output_dir.join("assets");
    fs::create_dir_all(&assets_dir).map_err(ReqvireError::IoError)?;
    fs::write(assets_dir.join("explorer.js"), EXPLORER_BUNDLE_JS).map_err(ReqvireError::IoError)?;
    fs::write(assets_dir.join("explorer.css"), EXPLORER_BUNDLE_CSS)
        .map_err(ReqvireError::IoError)?;
    Ok(())
}

fn copy_local_images_from_markdown_content(
    markdown_content: &str,
    markdown_relative_path: &Path,
    temp_dir: &Path,
    git_root: &Path,
    subdir_prefix: Option<&Path>,
    copied_files: &mut HashSet<String>,
) -> Result<(), ReqvireError> {
    let Some(markdown_parent) = markdown_relative_path.parent() else {
        return Ok(());
    };

    for relative_asset_path in html_export::extract_local_asset_paths(markdown_content) {
        let source_absolute_path = git_root.join(markdown_parent).join(&relative_asset_path);
        if !source_absolute_path.is_file() {
            debug!(
                "Skipping missing local linked asset during temp export: {} (from {})",
                source_absolute_path.display(),
                markdown_relative_path.display()
            );
            continue;
        }

        let Ok(canonical_source_path) = fs::canonicalize(&source_absolute_path) else {
            debug!(
                "Skipping unreadable local linked asset during temp export: {}",
                source_absolute_path.display()
            );
            continue;
        };

        let Ok(source_relative_path) = canonical_source_path.strip_prefix(git_root) else {
            debug!(
                "Skipping local linked asset outside repository root during temp export: {}",
                canonical_source_path.display()
            );
            continue;
        };

        let copied_key = source_relative_path.to_string_lossy().to_string();
        if copied_files.contains(&copied_key) {
            continue;
        }

        let dest_path = if let Some(prefix) = subdir_prefix {
            if let Ok(stripped) = source_relative_path.strip_prefix(prefix) {
                temp_dir.join(stripped)
            } else {
                temp_dir.join(source_relative_path)
            }
        } else {
            temp_dir.join(source_relative_path)
        };

        filesystem::copy_file_with_structure(&canonical_source_path, &dest_path)?;
        copied_files.insert(copied_key);
        debug!(
            "Copied local linked asset for temp export: {} -> {}",
            canonical_source_path.display(),
            dest_path.display()
        );
    }

    Ok(())
}

/// Generates model markdown files with full relations and copies referenced files to temporary directory
/// This generates markdown from the registry (with all relations) instead of copying original files
pub fn flush_model_to_temp(
    registry: &GraphRegistry,
    temp_dir: &Path,
    current_dir: &Path,
    git_root: &Path,
) -> Result<(), ReqvireError> {
    // Determine if we're in a subdirectory and get the relative path prefix to strip
    let subdir_prefix = if current_dir.starts_with(git_root) && current_dir != git_root {
        current_dir.strip_prefix(git_root).ok()
    } else {
        None
    };

    info!("Generating model files with full relations to temporary directory...");

    let mut copied_files = HashSet::new();

    // Generate markdown files from registry with full relations (user-created + auto-generated)
    let grouped_elements = registry.group_elements_by_location();
    let mut markdown_files_written = 0;

    for (file_path, elements) in grouped_elements {
        // Generate markdown content with full relations
        let markdown_content = registry.generate_file_markdown(&file_path, &elements, true);

        // Strip subdirectory prefix from destination path if running from subdirectory
        let dest_path = if let Some(prefix) = subdir_prefix {
            if let Ok(stripped) = Path::new(file_path.as_str()).strip_prefix(prefix) {
                temp_dir.join(stripped)
            } else {
                temp_dir.join(&file_path)
            }
        } else {
            temp_dir.join(&file_path)
        };

        // Create parent directories if needed
        if let Some(parent_dir) = dest_path.parent() {
            fs::create_dir_all(parent_dir).map_err(ReqvireError::IoError)?;
        }

        // Write the generated markdown file
        fs::write(&dest_path, &markdown_content).map_err(ReqvireError::IoError)?;

        copied_files.insert(file_path.clone());
        copy_local_images_from_markdown_content(
            &markdown_content,
            Path::new(&file_path),
            temp_dir,
            git_root,
            subdir_prefix,
            &mut copied_files,
        )?;
        markdown_files_written += 1;
        debug!(
            "Generated model file: {} -> {}",
            file_path,
            dest_path.display()
        );
    }

    info!(
        "✅ Generated {} markdown files with full relations",
        markdown_files_written
    );

    // Copy all files referenced in relations (InternalPath)
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
                    debug!(
                        "Copied relation target: {} -> {}",
                        path.display(),
                        dest.display()
                    );
                }
            }
        }

        // Copy all attachment files (only for FilePath attachments, not ElementIdentifier)
        for attachment in &node.element.attachments {
            if let crate::element::AttachmentTarget::FilePath(path) = &attachment.target {
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
                    debug!(
                        "Copied attachment: {} -> {}",
                        path.display(),
                        dest.display()
                    );
                }
            }
            // Element identifier attachments don't need to be copied - they reference other elements
        }
    }

    info!(
        "✅ Total {} files in temporary directory",
        copied_files.len()
    );
    Ok(())
}

/// Copies HTML output from temp directory to final output directory
/// Skips .md files only if a corresponding .html file exists
pub fn copy_html_output(temp_dir: &Path, output_dir: &Path) -> Result<(), ReqvireError> {
    info!("Copying HTML output to {}...", output_dir.display());

    prepare_output_folder(output_dir)?;

    // Recursively copy files, skipping .md files that have .html equivalents
    copy_html_and_assets(temp_dir, output_dir, temp_dir)?;

    info!("✅ HTML output copied to {}", output_dir.display());
    Ok(())
}

/// Helper function to recursively copy files, skipping .md files that have .html equivalents
#[allow(clippy::only_used_in_recursion)]
fn copy_html_and_assets(src: &Path, dst: &Path, temp_root: &Path) -> Result<(), ReqvireError> {
    fs::create_dir_all(dst).map_err(ReqvireError::IoError)?;

    for entry in fs::read_dir(src).map_err(ReqvireError::IoError)? {
        let entry = entry.map_err(ReqvireError::IoError)?;
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
    use regex::Regex;

    let html_files = vec!["index.html"];

    // Only convert .md to .html in heading id attributes and heading text content
    // IMPORTANT: Do NOT convert .md in script tags - D3 JSON data and JS code need .md preserved
    let id_attr_regex = Regex::new(r#"(id="[^"]*?)\.md""#).unwrap();

    for file_name in html_files {
        let file_path = temp_dir.join(file_name);
        if !file_path.exists() {
            continue;
        }

        let content = fs::read_to_string(&file_path).map_err(ReqvireError::IoError)?;

        // Convert .md references to .html ONLY in specific contexts:
        // 1. ID attributes: id="file:-path/file.md" → id="file:-path/file.html"
        // 2. Heading text ending tags: .md</h1>, .md</h2>, etc.
        //
        // We must NOT convert:
        // - Script content (D3 JSON data with "name": "file.md")
        // - JavaScript code with .replace(".html", ".md")
        let processed = id_attr_regex.replace_all(&content, r#"${1}.html""#);
        let processed = processed
            .replace(".md</h1>", ".html</h1>")
            .replace(".md</h2>", ".html</h2>")
            .replace(".md</h3>", ".html</h3>")
            .replace(".md</h4>", ".html</h4>")
            .replace(".md</h5>", ".html</h5>")
            .replace(".md</h6>", ".html</h6>");

        fs::write(&file_path, processed).map_err(ReqvireError::IoError)?;

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

    // Step 2: Generate model files with full relations to temp
    flush_model_to_temp(registry, &temp_dir, current_dir, git_root)?;

    // Step 3: Initialize git repository in temp directory
    info!("Initializing git repository in temporary directory...");
    std::process::Command::new("git")
        .arg("init")
        .current_dir(&temp_dir)
        .output()
        .map_err(|e| {
            ReqvireError::PathError(format!("Failed to initialize git repo in temp: {}", e))
        })?;

    // Step 4: Change to temp directory and create new model manager
    let original_dir = env::current_dir().map_err(ReqvireError::IoError)?;

    env::set_current_dir(&temp_dir).map_err(ReqvireError::IoError)?;

    // Clear git cache so paths resolve to temp directory instead of original repo
    git_commands::clear_git_cache();

    // Create new model manager and parse from temp directory
    info!("Loading model from temporary directory...");
    let mut temp_model_manager = crate::ModelManager::new();
    let parse_result = temp_model_manager.parse_and_validate(None, excluded_patterns);

    // Check for validation errors
    if let Err(e) = parse_result {
        // Log the error before returning
        match &e {
            ReqvireError::ValidationError(errors) => {
                eprintln!(
                    "\n❌ Validation failed in temporary directory with {} error(s):",
                    errors.len()
                );
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
    crate::diagrams::process_diagrams(&temp_model_manager.graph_registry, diagrams_with_blobs)?;

    // Generate ontology data artifact from the same graph-registry collector used by the CLI.
    info!("Generating ontologies.ttl...");
    let ontologies_report =
        crate::semantic_contract::build_semantic_index(&temp_model_manager.graph_registry);
    filesystem::write_file(
        "ontologies.ttl",
        ontologies_report.to_turtle_string().as_bytes(),
    )?;
    info!("✅ Generated ontologies.ttl");
    let explorer_project_store = crate::html::store::build_project_store(
        &temp_model_manager.graph_registry,
        &ontologies_report,
    );

    // Step 6: Convert markdown to HTML
    info!("Converting remaining markdown to HTML...");
    let html_count = html_export::export_markdown_to_html(&temp_dir, &temp_dir)?;
    info!("✅ Converted {} markdown files to HTML", html_count);

    // Step 6.4: Write index.html as the compiled Explorer SPA bundle seeded with
    // the Project Store. Explorer/report views are canonical SPA routes.
    write_explorer_index(&temp_dir, &explorer_project_store)?;
    info!("✅ Wrote Explorer SPA bundle to index.html");

    // Step 6.5: Copy assets folder for HTML pages
    info!("Copying assets...");
    copy_assets_folder(&temp_dir)?;

    // Step 6.6: Post-process HTML files to convert .md references to .html
    info!("Post-processing HTML artifacts...");
    post_process_html_files(&temp_dir)?;

    // Step 7: Restore original directory
    env::set_current_dir(&original_dir).map_err(ReqvireError::IoError)?;

    // Clear git cache again so it refreshes for original directory
    git_commands::clear_git_cache();

    Ok(temp_dir)
}

/// Finalizes export by copying temp directory to output and cleaning up
pub fn finalize_export(
    temp_dir: &Path,
    output_dir: &Path,
    cleanup: bool,
) -> Result<(), ReqvireError> {
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
        git_root,
    )?;

    finalize_export(&temp_dir, output_dir, true)?;

    Ok(())
}

/// Converts Markdown → HTML *and* copies all registry-internal files into `output_folder`.
pub fn export_model(registry: &GraphRegistry, output_folder: &Path) -> Result<usize, ReqvireError> {
    // Try to get repository root as base directory
    let base_dir = match git_commands::get_git_root_dir() {
        Ok(git_root) => git_root,
        Err(_) => {
            // If Git repository root can't be found, use the current working directory
            std::env::current_dir().map_err(|e| {
                ReqvireError::PathError(format!("Failed to get current directory: {}", e))
            })?
        }
    };

    // prepare output folder
    prepare_output_folder(output_folder)?;

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
            Ok(_) => println!("✅ Exported: {:?} -> {}", src.display(), dst.display()),
            Err(e) => warn!("Failed to copy {:?}: {}", src, e),
        }
    }

    Ok(count)
}
