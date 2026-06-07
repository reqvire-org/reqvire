/// HTML generation module using Maud for type-safe HTML generation
///
/// This module provides component-based HTML generation with:
/// - Type-safe HTML via Maud macros (compile-time validation)
/// - Responsive source/specification document design with Tailwind CSS
/// - Reusable components (source-page route links, layouts)
/// - Compact source-page route links and contextual help modal
///
/// # Architecture
///
/// - `layouts`: Base layouts (standard page, diagram page)
/// - `components`: Reusable components (head, navigation)
/// - `styles`: source/specification page CSS generation (Tailwind CDN + custom overrides)
/// - `scripts`: JavaScript utilities
/// - `pages`: Page-specific generators (to be implemented in Phase 2/3)
/// - `visualizations`: Visualization components (to be implemented in Phase 2/3)
mod components;
mod layouts;
pub mod markdown;
mod scripts;
pub mod store;
mod styles;

pub mod pages;
pub mod visualizations;

// Re-export commonly used items
pub use layouts::{base, diagram_layout};
pub use maud::Markup;

use crate::error::ReqvireError;
use std::path::{Path, PathBuf};

/// Convert markdown file to HTML using component-based architecture
///
/// This is the main entry point for Phase 2 HTML generation.
/// It processes markdown content and generates appropriate HTML page based on filename.
///
/// # Arguments
/// * `file_path` - Path to the markdown file being converted
/// * `markdown_content` - Raw markdown content
/// * `title` - Page title
/// * `base_folder` - Base folder for calculating relative navigation paths
///
/// # Returns
/// Complete HTML document as a string
pub fn convert_to_html(
    file_path: &PathBuf,
    markdown_content: &str,
    title: &str,
    base_folder: &PathBuf,
) -> Result<String, ReqvireError> {
    // Process markdown to HTML content
    let html_content =
        markdown::markdown_to_html_content(file_path, markdown_content, base_folder)?;

    // Calculate relative path prefix for navigation links
    let nav_prefix = calculate_nav_prefix(file_path, base_folder);

    // Determine which page template to use based on filename
    let filename = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    let html_document = match filename {
        "SpecificationIndex.md" | "index.md" => {
            pages::index::render(&html_content, &nav_prefix).into_string()
        }
        _ => {
            // Default: standard page for specification files
            // Check if content contains diagrams and include appropriate scripts
            let has_mermaid = html_content.contains(r#"<div class="mermaid">"#);
            let has_d3 = html_content.contains("d3-sunburst")
                || html_content.contains("d3-icicle")
                || html_content.contains("d3-sankey");

            let content = if has_mermaid || has_d3 {
                // Content has visualizations - use diagram page for proper script inclusion
                let mut markup = maud::html! {
                    div class="prose max-w-none" {
                        (maud::PreEscaped(&html_content))
                    }
                };

                // Add visualization scripts if needed
                if has_mermaid {
                    markup = maud::html! {
                        (markup)
                        (visualizations::mermaid::scripts())
                    };
                }

                markup
            } else {
                // Plain content - no visualizations
                maud::PreEscaped(html_content)
            };

            generate_page(title, content, &nav_prefix)
        }
    };

    Ok(html_document)
}

/// Calculate the relative path prefix needed for navigation links
/// based on the depth of the current file relative to base_folder
fn calculate_nav_prefix(file_path: &Path, base_folder: &Path) -> String {
    // Get relative path from base_folder
    let relative_path = match file_path.strip_prefix(base_folder) {
        Ok(rel) => rel,
        Err(_) => {
            // If strip_prefix fails, assume file is at root
            return String::new();
        }
    };

    // Count the number of directory components (excluding the filename)
    let depth = relative_path.components().count().saturating_sub(1);

    if depth == 0 {
        // File is at root level
        String::new()
    } else {
        // Need to go up 'depth' levels
        "../".repeat(depth)
    }
}

/// Generate a standard page using the base layout
///
/// # Arguments
/// * `title` - Page title (appears in <title> tag and browser tab)
/// * `content` - HTML content markup to display in the page
/// * `nav_prefix` - Relative path prefix for navigation links (e.g., "../" for nested pages)
///
pub fn generate_page(title: &str, content: Markup, nav_prefix: &str) -> String {
    layouts::base(title, content, nav_prefix).into_string()
}

/// Generate a diagram page using the full-height diagram layout
///
/// # Arguments
/// * `title` - Page title
/// * `diagram` - Diagram visualization markup
/// * `nav_prefix` - Relative path prefix for navigation links
pub fn generate_diagram_page(title: &str, diagram: Markup, nav_prefix: &str) -> String {
    layouts::diagram_layout(title, diagram, nav_prefix).into_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use maud::html;

    #[test]
    fn test_generate_page() {
        let content = html! { p { "Test content" } };
        let page = generate_page("Test", content, "");

        assert!(page.contains("<!DOCTYPE html>"));
        assert!(page.contains("<title>Test</title>"));
        assert!(page.contains("Test content"));
        assert!(page.contains("reqvire-nav"));
    }

    #[test]
    fn test_generate_diagram_page() {
        let diagram = html! { div { "Diagram" } };
        let page = generate_diagram_page("Model", diagram, "../");

        assert!(page.contains("<!DOCTYPE html>"));
        assert!(page.contains("<title>Model</title>"));
        assert!(page.contains("Diagram"));
        assert!(page.contains("href=\"../index.html#/model\""));
    }

    #[test]
    fn test_navigation_links() {
        let content = html! { p { "Content" } };
        let page = generate_page("Test", content, "");

        // Source-page navigation targets canonical SPA views, not standalone Explorer pages.
        assert!(page.contains("reqvire-nav"));
        assert!(page.contains("href=\"index.html#/model\""));
        assert!(page.contains("Model"));
        assert!(page.contains("Traces"));
        assert!(page.contains("Ontologies"));
        assert!(page.contains("KN2"));
        assert!(!page.contains("href=\"index.html#/knowledge-graph\""));
        assert!(!page.contains("#/traceflow"));
        assert!(!page.contains("#/coverage"));
        assert!(!page.contains("#/resources"));
    }

    #[test]
    fn test_page_structure() {
        let content = html! { p { "Content" } };
        let page = generate_page("Test", content, "");

        // Check page has basic structure
        assert!(page.contains("reqvire-nav"));
        assert!(page.contains("container"));
        assert!(page.contains("content"));
    }

    #[test]
    fn test_viewport_and_footer() {
        let content = html! { p { "Content" } };
        let page = generate_page("Test", content, "");

        // Check viewport meta tag is present and no generated footer wastes space.
        assert!(page.contains("viewport"));
        assert!(!page.contains("Generated by"));
    }

    #[test]
    fn test_nav_prefix() {
        let content = html! { p { "Content" } };
        let page = generate_page("Test", content, "../../");

        // Check navigation links use prefix
        assert!(page.contains("href=\"../../index.html#/model\""));
    }
}
