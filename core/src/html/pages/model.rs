use maud::{html, Markup, PreEscaped};

/// Generate model view page with Mermaid diagrams
///
/// The model page displays a model-centric view starting from ontology roots and feature roots,
/// showing complete relation trees with embedded Mermaid diagrams for each section.
///
/// # Arguments
/// * `html_content` - Pre-converted HTML content from markdown model report (contains Mermaid diagrams)
/// * `nav_prefix` - Relative path prefix for navigation links
///
pub fn render(html_content: &str, nav_prefix: &str) -> Markup {
    let diagram = html! {
        div class="model-view w-full h-full p-4" {
            (PreEscaped(html_content))
        }
        // Include Mermaid scripts for diagram rendering
        (crate::html::visualizations::mermaid::scripts())
    };

    crate::html::layouts::diagram_layout("Model View", diagram, nav_prefix)
}
