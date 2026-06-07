use maud::{html, Markup, PreEscaped};

/// Generate containment/index page with D3 Sunburst and Icicle toggle
///
/// The containment view shows the physical organization of the model with
/// two visualization options: Sunburst (circular) and Icicle (hierarchical bars).
///
/// # Arguments
/// * `html_content` - Pre-converted HTML content with D3 visualizations and toggle
/// * `nav_prefix` - Relative path prefix for navigation links
///
pub fn render(html_content: &str, nav_prefix: &str) -> Markup {
    let diagram = html! {
        div class="reqvire-explorer-shell containment-page w-full h-full p-4" {
            (PreEscaped(html_content))
        }
    };

    crate::html::layouts::diagram_layout("Containment View", diagram, nav_prefix)
}
