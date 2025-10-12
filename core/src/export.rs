use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use log::{debug,warn,info};
use crate::git_commands;
use std::io::Write;

use crate::error::ReqvireError;
use crate::html_export;
use crate::graph_registry::GraphRegistry;
use crate::filesystem;

/// Generates HTML wrapper for matrix.svg with pan/zoom functionality
fn generate_matrix_html() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>Verification Traceability Matrix</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            overflow: hidden;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
        }
        #controls {
            position: fixed;
            top: 10px;
            right: 10px;
            background: white;
            padding: 10px;
            border-radius: 4px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.2);
            z-index: 1000;
        }
        #controls button {
            margin: 0 5px;
            padding: 5px 10px;
            cursor: pointer;
            border: 1px solid #ccc;
            border-radius: 3px;
            background: white;
        }
        #controls button:hover {
            background: #f0f0f0;
        }
        .arrow-btn {
            position: fixed;
            background: white;
            border: 1px solid #ccc;
            border-radius: 50%;
            width: 40px;
            height: 40px;
            cursor: pointer;
            box-shadow: 0 2px 4px rgba(0,0,0,0.2);
            z-index: 1000;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 20px;
        }
        .arrow-btn:hover {
            background: #f0f0f0;
        }
        #arrow-up {
            top: 60px;
            left: 50%;
            transform: translateX(-50%);
        }
        #arrow-down {
            bottom: 20px;
            left: 50%;
            transform: translateX(-50%);
        }
        #arrow-left {
            left: 20px;
            top: 50%;
            transform: translateY(-50%);
        }
        #arrow-right {
            right: 20px;
            top: 50%;
            transform: translateY(-50%);
        }
        #svg-container {
            width: 100vw;
            height: 100vh;
        }
        #svg-container object {
            width: 100%;
            height: 100%;
        }
    </style>
</head>
<body>
    <div id="controls">
        <button id="zoom-in">Zoom In</button>
        <button id="zoom-out">Zoom Out</button>
        <button id="reset">Reset</button>
        <button id="center">Center</button>
    </div>
    <button id="arrow-up" class="arrow-btn">▲</button>
    <button id="arrow-down" class="arrow-btn">▼</button>
    <button id="arrow-left" class="arrow-btn">◄</button>
    <button id="arrow-right" class="arrow-btn">►</button>
    <div id="svg-container"></div>
    <script src="https://cdn.jsdelivr.net/npm/hammerjs@2.0.8/hammer.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/svg-pan-zoom@3.5.0/dist/svg-pan-zoom.min.js"></script>
    <script>
        var panZoomInstance = null;

        window.onload = function() {
            console.log('Page loaded');

            // Load SVG inline to enable proper touch event handling
            fetch('matrix.svg')
                .then(function(response) {
                    return response.text();
                })
                .then(function(svgText) {
                    var container = document.getElementById('svg-container');
                    container.innerHTML = svgText;

                    var svgElement = container.querySelector('svg');
                    if (!svgElement) {
                        console.error('SVG element not found');
                        return;
                    }

                    console.log('SVG loaded inline');

                    var eventsHandler;
                    eventsHandler = {
                        haltEventListeners: ['touchstart', 'touchend', 'touchmove', 'touchleave', 'touchcancel'],
                        init: function(options) {
                            var instance = options.instance;
                            var initialScale = 1;
                            var pannedX = 0;
                            var pannedY = 0;

                            this.hammer = Hammer(options.svgElement, {
                                inputClass: Hammer.SUPPORT_POINTER_EVENTS ? Hammer.PointerEventInput : Hammer.TouchInput
                            });

                            this.hammer.get('pinch').set({enable: true});

                            this.hammer.on('doubletap', function(ev){
                                instance.zoomIn();
                            });

                            this.hammer.on('panstart panmove', function(ev){
                                if (ev.type === 'panstart') {
                                    pannedX = 0;
                                    pannedY = 0;
                                }
                                instance.panBy({x: ev.deltaX - pannedX, y: ev.deltaY - pannedY});
                                pannedX = ev.deltaX;
                                pannedY = ev.deltaY;
                            });

                            this.hammer.on('pinchstart pinchmove', function(ev){
                                if (ev.type === 'pinchstart') {
                                    initialScale = instance.getZoom();
                                    instance.zoomAtPoint(initialScale * ev.scale, {x: ev.center.x, y: ev.center.y});
                                }
                                instance.zoomAtPoint(initialScale * ev.scale, {x: ev.center.x, y: ev.center.y});
                            });

                            options.svgElement.addEventListener('touchmove', function(e){ e.preventDefault(); });
                        },
                        destroy: function(){
                            this.hammer.destroy();
                        }
                    };

                    panZoomInstance = svgPanZoom(svgElement, {
                        zoomEnabled: true,
                        controlIconsEnabled: false,
                        fit: true,
                        center: true,
                        minZoom: 0.1,
                        maxZoom: 10,
                        customEventsHandler: eventsHandler
                    });

                    console.log('PanZoom initialized');

                    // Attach button event handlers
                    document.getElementById('zoom-in').addEventListener('click', function() {
                        if (panZoomInstance) panZoomInstance.zoomIn();
                    });
                    document.getElementById('zoom-out').addEventListener('click', function() {
                        if (panZoomInstance) panZoomInstance.zoomOut();
                    });
                    document.getElementById('reset').addEventListener('click', function() {
                        if (panZoomInstance) panZoomInstance.reset();
                    });
                    document.getElementById('center').addEventListener('click', function() {
                        if (panZoomInstance) panZoomInstance.center();
                    });

                    // Attach arrow button handlers
                    var panStep = 50;
                    document.getElementById('arrow-up').addEventListener('click', function() {
                        if (panZoomInstance) panZoomInstance.panBy({x: 0, y: panStep});
                    });
                    document.getElementById('arrow-down').addEventListener('click', function() {
                        if (panZoomInstance) panZoomInstance.panBy({x: 0, y: -panStep});
                    });
                    document.getElementById('arrow-left').addEventListener('click', function() {
                        if (panZoomInstance) panZoomInstance.panBy({x: panStep, y: 0});
                    });
                    document.getElementById('arrow-right').addEventListener('click', function() {
                        if (panZoomInstance) panZoomInstance.panBy({x: -panStep, y: 0});
                    });

                    console.log('All event handlers attached');
                })
                .catch(function(error) {
                    console.error('Error loading SVG:', error);
                });
        };
    </script>
</body>
</html>"#.to_string()
}

fn prepare_output_folder(output_folder: &Path) -> std::io::Result<()> {
    // Clean output folder
    if output_folder.exists() {
        fs::remove_dir_all(output_folder)?;
    }
    fs::create_dir_all(output_folder)?;

    // Create a .gitignore file that ignores everything except itself
    let gitignore_path = output_folder.join(".gitignore");
    let mut file = fs::File::create(gitignore_path)?;
    writeln!(
        file,
        "*\n!.gitignore"
    )?;

    Ok(())
}


/// Copies all model files from graph registry to temporary directory
pub fn copy_model_files_to_temp(
    registry: &GraphRegistry,
    temp_dir: &Path,
) -> Result<(), ReqvireError> {
    let git_root = git_commands::get_git_root_dir()
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."));

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
    let html_files = vec!["index.html", "traces.html", "coverage.html"];

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

/// Generates all artifacts (diagrams, index, traces, coverage, matrix, HTML) in temporary directory
///
/// Returns the path to the temporary directory containing all generated artifacts.
/// The caller is responsible for cleanup.
///
/// Steps:
/// 1. Creates temporary working directory
/// 2. Copies all model files to temp
/// 3. Initializes git repository in temp directory
/// 4. Changes to temp directory and reloads model
/// 5. Generates all artifacts (diagrams, index, traces, coverage, matrix)
/// 6. Converts markdown to HTML
/// 7. Restores original directory
///
/// Returns: PathBuf to temporary directory with all generated content
pub fn generate_artifacts_in_temp(
    registry: &GraphRegistry,
    excluded_patterns: &globset::GlobSet,
    diagram_direction: &str,
    diagrams_with_blobs: bool,
) -> Result<PathBuf, ReqvireError> {
    use std::env;

    // Step 1: Create temp directory
    info!("Creating temporary working directory...");
    let temp_dir = filesystem::create_temp_working_dir()?;
    info!("✅ Temporary directory: {}", temp_dir.display());

    // Step 2: Copy all model files to temp
    copy_model_files_to_temp(registry, &temp_dir)?;

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
        &None,
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
        diagram_direction,
        diagrams_with_blobs
    )?;

    info!("Generating index.md...");
    let index_content = crate::index_generator::generate_readme_index(
        &temp_model_manager.graph_registry,
        &PathBuf::from(".")
    )?;
    filesystem::write_file("index.md", index_content.as_bytes())?;

    info!("Generating matrix.svg...");
    let matrix_config = crate::matrix_generator::MatrixConfig::default();
    let matrix_svg = crate::matrix_generator::generate_matrix(
        &temp_model_manager.graph_registry,
        &matrix_config,
        crate::matrix_generator::MatrixFormat::Svg
    );
    filesystem::write_file("matrix.svg", matrix_svg.as_bytes())?;

    // Generate HTML wrapper for matrix.svg with pan/zoom functionality
    info!("Generating matrix.html...");
    let matrix_html = generate_matrix_html();
    filesystem::write_file("matrix.html", matrix_html.as_bytes())?;

    // Generate model structure diagram with Mermaid
    info!("Generating model.md...");
    let model_mermaid = crate::diagrams::generate_model_diagram(&temp_model_manager.graph_registry, None)?;
    filesystem::write_file("model.md", model_mermaid.as_bytes())?;

    info!("Generating traces.md...");
    let trace_generator = crate::verification_trace::VerificationTraceGenerator::new(
        &temp_model_manager.graph_registry,
        diagrams_with_blobs,
        None
    );
    let trace_report = trace_generator.generate();
    let traces_markdown = trace_generator.generate_markdown(&trace_report);
    filesystem::write_file("traces.md", traces_markdown.as_bytes())?;

    info!("Generating coverage.md...");
    let coverage_report = crate::reports::generate_coverage_report(&temp_model_manager.graph_registry);
    let coverage_text = coverage_report.format_text();
    filesystem::write_file("coverage.md", coverage_text.as_bytes())?;

    // Step 6: Convert markdown to HTML
    info!("Converting markdown to HTML...");
    let html_count = html_export::export_markdown_to_html(&temp_dir, &temp_dir)?;
    info!("✅ Converted {} markdown files to HTML", html_count);

    // Step 6.5: Post-process HTML files to convert .md references to .html
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
    diagram_direction: &str,
    diagrams_with_blobs: bool,
) -> Result<(), ReqvireError> {
    let temp_dir = generate_artifacts_in_temp(
        registry,
        excluded_patterns,
        diagram_direction,
        diagrams_with_blobs
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

