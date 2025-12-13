use maud::{html, Markup, PreEscaped};

/// Render TraceFlow page with interactive Sankey diagram
///
/// Visualizes verification traceability flow from stakeholder needs through system
/// specifications to verifications. Link width indicates number of connections.
///
/// # Arguments
/// * `html_content` - Pre-converted HTML content from traceflow.md (contains D3 Sankey)
/// * `nav_prefix` - Relative path prefix for navigation links
///
/// # Returns
/// Complete HTML page with navigation and TraceFlow Sankey visualization
pub fn render(html_content: &str, nav_prefix: &str) -> Markup {
    let content = html! {
        (PreEscaped(html_content))
    };

    // Use diagram_layout for full-height interactive Sankey visualization
    crate::html::layouts::diagram_layout("TraceFlow", content, nav_prefix)
}
