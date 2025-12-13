use maud::{html, Markup, PreEscaped};

/// Render resources page
///
/// Shows all files referenced by the model through relations and attachments,
/// including implementation files (satisfiedBy), traced documents (trace),
/// and attachment files (design specs, images).
///
/// # Arguments
/// * `html_content` - Pre-converted HTML content from resources.md
/// * `nav_prefix` - Relative path prefix for navigation links
///
/// # Returns
/// Complete HTML page with navigation and resources listing
pub fn render(html_content: &str, nav_prefix: &str) -> Markup {
    let content = html! {
        (PreEscaped(html_content))
    };

    crate::html::layouts::base("Resources", content, nav_prefix)
}
