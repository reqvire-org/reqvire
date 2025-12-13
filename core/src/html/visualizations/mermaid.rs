use maud::{html, Markup, PreEscaped};

/// Generate Mermaid diagram with pan/zoom support and ELK layout
///
/// # Arguments
/// * `diagram_code` - Mermaid diagram code (without ```mermaid wrapper)
///
/// # Example
/// ```ignore
/// let mermaid_code = "graph TD\n  A --> B";
/// let diagram = mermaid::render(mermaid_code);
/// ```
pub fn render(diagram_code: &str) -> Markup {
    html! {
        div class="mermaid-container w-full h-full" {
            div class="mermaid" {
                (PreEscaped(diagram_code))
            }
        }
        (scripts())
    }
}

/// Generate Mermaid initialization scripts (public for use in pages)
/// This matches the old template/model.html implementation exactly
pub fn scripts() -> Markup {
    html! {
        // External dependencies
        script src="https://cdn.jsdelivr.net/npm/hammerjs@2.0.8/hammer.min.js" {}
        script src="https://cdn.jsdelivr.net/npm/svg-pan-zoom@3.5.0/dist/svg-pan-zoom.min.js" {}

        // Mermaid initialization - matches old template exactly
        script type="module" {
            (PreEscaped(include_str!("../../../templates/mermaid-init.js")))
        }
    }
}
