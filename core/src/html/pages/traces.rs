use maud::{html, Markup, PreEscaped};

/// Render verification traces page
///
/// Displays upward traceability from verifications through requirement hierarchies.
/// Uses the roll-up strategy where verifying a leaf requirement provides coverage
/// to all ancestors through derivedFrom relations.
///
/// # Arguments
/// * `html_content` - Pre-converted HTML content from traces.md (contains Mermaid diagrams)
/// * `nav_prefix` - Relative path prefix for navigation links
///
/// # Returns
/// Complete HTML page with navigation and traces visualization
pub fn render(html_content: &str, nav_prefix: &str) -> Markup {
    let content = html! {
        (PreEscaped(html_content))
        // Include Mermaid scripts for diagram rendering
        (crate::html::visualizations::mermaid::scripts())
    };

    crate::html::layouts::base("Verification Traces", content, nav_prefix)
}
