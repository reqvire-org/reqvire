use maud::{html, Markup, DOCTYPE};

/// Base layout for standard pages with Tailwind responsive design
pub fn base(title: &str, content: Markup, nav_prefix: &str) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
                link rel="icon" type="image/x-icon" href={(nav_prefix)"assets/favicon.ico"};
                link rel="apple-touch-icon" href={(nav_prefix)"assets/apple-touch-icon.png"};

                // Tailwind CSS CDN
                script src="https://cdn.tailwindcss.com" {}

                // Custom CSS for Mermaid, content styling, and color scheme
                (super::styles::custom())
            }
            body class="reqvire-html-body" {
                // Fixed Explorer header
                nav class="reqvire-nav fixed top-0 left-0 right-0 h-[50px] shadow-md z-[1000] flex items-center px-5" style="background-color: #1c1c1c;" {
                    // Logo
                    a href={(nav_prefix)"index.html"} class="nav-logo flex items-center pr-4" {
                        img src={(nav_prefix)"assets/logo.png"} alt="Reqvire" height="24";
                    }
                    div class="reqvire-view-switcher" aria-label="Explorer views" {
                        a href={(nav_prefix)"index.html"} class="nav-link" { "Containment" }
                        a href={(nav_prefix)"model.html"} class="nav-link" { "Model" }
                        a href={(nav_prefix)"traces.html"} class="nav-link" { "Traces" }
                        a href={(nav_prefix)"ontologies.html"} class="nav-link" { "Ontologies" }
                        a href={(nav_prefix)"knowledgegraph.html"} class="nav-link" { "Knowledge Graph" }
                        a href={(nav_prefix)"kn2.html"} class="nav-link" { "KN2" }
                    }
                    button class="reqvire-help-button" type="button" aria-label="Open view help" onclick="openReqvireExplorerHelp()" { "?" }
                }

                // Spacer for fixed nav
                div class="h-[50px]" {}

                // Main content with responsive fluid width
                div class="w-full max-w-[95%] mx-auto p-5" {
                    div class="bg-white p-8 rounded shadow-sm border border-gray-200" {
                        (content)
                    }
                }

                (help_modal(title))
            }
        }
    }
}

/// Diagram layout for full-height visualizations with Tailwind responsive design
pub fn diagram_layout(title: &str, diagram: Markup, nav_prefix: &str) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
                link rel="icon" type="image/x-icon" href={(nav_prefix)"assets/favicon.ico"};
                link rel="apple-touch-icon" href={(nav_prefix)"assets/apple-touch-icon.png"};

                // Tailwind CSS CDN
                script src="https://cdn.tailwindcss.com" {}

                // Custom CSS for Mermaid, content styling, and color scheme
                (super::styles::custom())

                // Include Mermaid scripts for diagrams
                (super::visualizations::mermaid::scripts())
            }
            body class="reqvire-html-body" {
                // Fixed Explorer header
                nav class="reqvire-nav fixed top-0 left-0 right-0 h-[50px] shadow-md z-[1000] flex items-center px-5" style="background-color: #1c1c1c;" {
                    // Logo
                    a href={(nav_prefix)"index.html"} class="nav-logo flex items-center pr-4" {
                        img src={(nav_prefix)"assets/logo.png"} alt="Reqvire" height="24";
                    }
                    div class="reqvire-view-switcher" aria-label="Explorer views" {
                        a href={(nav_prefix)"index.html"} class="nav-link" { "Containment" }
                        a href={(nav_prefix)"model.html"} class="nav-link" { "Model" }
                        a href={(nav_prefix)"traces.html"} class="nav-link" { "Traces" }
                        a href={(nav_prefix)"ontologies.html"} class="nav-link" { "Ontologies" }
                        a href={(nav_prefix)"knowledgegraph.html"} class="nav-link" { "Knowledge Graph" }
                        a href={(nav_prefix)"kn2.html"} class="nav-link" { "KN2" }
                    }
                    button class="reqvire-help-button" type="button" aria-label="Open view help" onclick="openReqvireExplorerHelp()" { "?" }
                }

                // Spacer for fixed nav
                div class="h-[50px]" {}

                // Main content with responsive fluid width
                div class="w-full max-w-[95%] mx-auto p-5" {
                    div class="bg-white p-8 rounded shadow-sm border border-gray-200" {
                        (diagram)
                    }
                }

                (help_modal(title))
            }
        }
    }
}

fn help_text(title: &str) -> (&'static str, &'static str) {
    match title {
        "Containment View" => (
            "Containment",
            "Shows repository/file containment so you can zoom into folders, files, elements, and attachments. Use Sunburst or Icicle depending on whether radial or rectangular hierarchy scanning is easier.",
        ),
        "Model View" => (
            "Model",
            "Shows the semantic model from ontology roots and capability roots with nested relations, including capabilities, requirements, refinements, verifications, and implementation evidence.",
        ),
        "Knowledge Graph" => (
            "Knowledge Graph",
            "Shows the actual parsed project graph: elements, relation facts, attachments, concept references, file targets, and external references that exist in the current model.",
        ),
        "KN2" => (
            "KN2",
            "Experimental Cytoscape.js project graph page for comparing layouts, centrality sizing, subgraph clustering, and compound-node rendering against the current Knowledge Graph view.",
        ),
        "Verification Traces" => (
            "Traces",
            "Shows upward verification traceability from verification elements through requirement hierarchy and capability context, including direct and rolled-up verification paths.",
        ),
        "Ontologies" => (
            "Ontologies",
            "Shows authored ontology and SHACL terms as an interactive semantic graph. Use search, filters, graph focus, and the inspector to explore classes, properties, individuals, constraints, and generated ontology constructs.",
        ),
        "TraceFlow" => (
            "TraceFlow",
            "Supporting report artifact that visualizes verification traceability flow as a Sankey diagram.",
        ),
        "Coverage" => (
            "Coverage",
            "Supporting report artifact that summarizes verification and implementation coverage.",
        ),
        "Resources" => (
            "Resources",
            "Supporting report artifact that lists files referenced by relations and attachments.",
        ),
        _ => (
            "Reqvire Explorer",
            "Use the header switcher to move between Containment, Model, Traces, Ontologies, and Knowledge Graph. Supporting report artifact pages may still exist but are not primary Explorer destinations.",
        ),
    }
}

fn help_modal(title: &str) -> Markup {
    let (help_title, help_body) = help_text(title);

    html! {
        div id="reqvire-help-modal" class="reqvire-help-modal" aria-hidden="true" {
            div class="reqvire-help-dialog" role="dialog" aria-modal="true" aria-labelledby="reqvire-help-title" {
                div class="reqvire-help-header" {
                    h2 id="reqvire-help-title" { (help_title) }
                    button type="button" aria-label="Close view help" onclick="closeReqvireExplorerHelp()" { "x" }
                }
                p { (help_body) }
            }
        }
        script {
            (maud::PreEscaped(r#"
                function openReqvireExplorerHelp() {
                    var modal = document.getElementById('reqvire-help-modal');
                    if (!modal) return;
                    modal.classList.add('is-open');
                    modal.setAttribute('aria-hidden', 'false');
                }
                function closeReqvireExplorerHelp() {
                    var modal = document.getElementById('reqvire-help-modal');
                    if (!modal) return;
                    modal.classList.remove('is-open');
                    modal.setAttribute('aria-hidden', 'true');
                }
                document.addEventListener('keydown', function(event) {
                    if (event.key === 'Escape') closeReqvireExplorerHelp();
                });
                document.addEventListener('click', function(event) {
                    if (event.target && event.target.id === 'reqvire-help-modal') {
                        closeReqvireExplorerHelp();
                    }
                });
            "#))
        }
    }
}
