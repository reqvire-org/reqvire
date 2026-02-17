use maud::{html, Markup, PreEscaped};

/// Generate coverage report page with markdown content converted to HTML
///
/// # Arguments
/// * `html_content` - Pre-converted HTML content from markdown coverage report
/// * `nav_prefix` - Relative path prefix for navigation links
///
pub fn render(html_content: &str, nav_prefix: &str) -> Markup {
    let content = html! {
        div class="coverage-report" {
            (PreEscaped(html_content))
        }
    };

    crate::html::layouts::base("Coverage Report", content, nav_prefix)
}
