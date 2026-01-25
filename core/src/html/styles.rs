use maud::{html, Markup, PreEscaped};

/// Generate custom CSS that matches the old template system exactly
pub fn custom() -> Markup {
    html! {
        style {
            (PreEscaped(r#"
                :root {
                    /* Primary Colors - MONO theme */
                    --color-nav: #1c1c1c;
                    --color-nav-hover: #2a2a2a;
                    --color-primary: #333333;
                    --color-primary-hover: #4a4a4a;
                    --color-primary-active: #2a2a2a;

                    /* Element Type Colors */
                    --color-requirement: #4a4a4a;
                    --color-verification: #5a5a5a;
                    --color-other: #8a8a8a;

                    /* Status Colors - grayscale with subtle distinction */
                    --color-verified: #6a6a6a;
                    --color-pending: #9a9a9a;
                    --color-error: #4a4a4a;

                    /* Interactive Colors */
                    --color-highlight: #d0d0d0;
                    --color-link: #4a4a4a;
                    --color-link-hover: #2a2a2a;

                    /* Text Colors */
                    --color-text-primary: #212121;
                    --color-text-secondary: #424242;
                    --color-text-muted: #757575;
                    --color-background: #FAFAFA;
                    --color-border: #EEEEEE;
                }

                /* Ensure body takes full height */
                body {
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
                    line-height: 1.6;
                    margin: 0;
                    padding: 0;
                    background-color: #FAFAFA;
                    color: var(--color-text-secondary);
                }

                /* Navigation bar */
                .reqvire-nav {
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    height: 50px;
                    background-color: var(--color-nav);
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    z-index: 1000;
                    display: flex;
                    align-items: center;
                    padding: 0 20px;
                }

                .reqvire-nav a {
                    color: white;
                    text-decoration: none;
                    padding: 0 15px;
                    font-weight: 500;
                    transition: background-color 0.2s;
                }

                .reqvire-nav a:hover {
                    background-color: var(--color-nav-hover);
                    text-decoration: none;
                }

                .nav-logo {
                    margin-right: 20px;
                }

                .nav-logo:hover {
                    background-color: transparent !important;
                    text-decoration: none !important;
                }

                .nav-logo img {
                    height: 24px;
                    width: auto;
                    vertical-align: middle;
                }

                .nav-link {
                    color: white;
                    text-decoration: none;
                    padding: 10px 20px;
                    margin-right: 5px;
                    border-radius: 3px;
                    transition: background-color 0.2s, text-decoration 0.2s;
                }

                .nav-link:hover {
                    background-color: var(--color-nav-hover);
                    text-decoration: underline;
                    color: white;
                }

                .nav-link:active {
                    background-color: var(--color-nav-hover);
                    text-decoration: underline;
                    color: white;
                }

                .mobile-nav-link {
                    color: white;
                    text-decoration: none;
                    padding: 12px 20px;
                    display: block;
                    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
                    transition: background-color 0.2s;
                }

                .mobile-nav-link:hover {
                    background-color: var(--color-nav-hover);
                    color: white;
                }

                .reqvire-nav-spacer {
                    height: 50px;
                }

                /* Content container */
                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 20px;
                }

                .content {
                    background-color: white;
                    padding: 30px;
                    border-radius: 4px;
                    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
                }

                /* Content area typography and styling - matches old template exactly */
                /* Use higher specificity to override Tailwind reset */
                .bg-white h1, .content h1, h1 {
                    color: var(--color-text-primary) !important;
                    border-bottom: 2px solid #EEEEEE !important;
                    padding-bottom: 10px !important;
                    margin-top: 0 !important;
                    font-size: 2em !important;
                    font-weight: bold !important;
                }

                .bg-white h2, .content h2, h2 {
                    color: var(--color-text-primary) !important;
                    border-bottom: 1px solid #EEEEEE !important;
                    padding-bottom: 5px !important;
                    font-size: 1.5em !important;
                    font-weight: bold !important;
                    margin-top: 20px !important;
                }

                .bg-white h3, .content h3, h3 {
                    color: var(--color-text-primary) !important;
                    margin-top: 25px !important;
                    font-size: 1.17em !important;
                    font-weight: bold !important;
                }

                .bg-white h4, .content h4, h4 {
                    color: var(--color-text-secondary) !important;
                    font-weight: 600 !important;
                    font-size: 1em !important;
                    margin-top: 15px !important;
                }

                .bg-white a:not(.nav-link):not(.mobile-nav-link):not(.text-indigo-700),
                .content a:not(.nav-link):not(.mobile-nav-link):not(.text-indigo-700),
                div.bg-white > a {
                    color: var(--color-primary) !important;
                    text-decoration: none !important;
                    font-weight: 500 !important;
                    transition: text-decoration 0.15s ease !important;
                }

                .bg-white a:not(.nav-link):not(.mobile-nav-link):hover,
                .content a:not(.nav-link):not(.mobile-nav-link):hover {
                    text-decoration: underline !important;
                }

                .bg-white a:focus, .content a:focus {
                    outline: 2px solid var(--color-primary);
                    outline-offset: 2px;
                }

                .bg-white table, .content table, table {
                    border-collapse: collapse !important;
                    width: 100% !important;
                    margin: 20px 0 !important;
                }

                .bg-white table, .bg-white th, .bg-white td,
                .content table, .content th, .content td {
                    border: 1px solid #EEEEEE !important;
                }

                .bg-white th, .bg-white td,
                .content th, .content td {
                    padding: 8px 12px !important;
                    text-align: left !important;
                }

                .bg-white th, .content th {
                    background-color: #F5F5F5 !important;
                    color: #212121 !important;
                    font-weight: 600 !important;
                }

                .bg-white tr:hover td, .content tr:hover td {
                    background-color: #FFF8E1 !important;
                }

                .bg-white code, .content code, code {
                    background: #f6f8fa !important;
                    padding: 2px 4px !important;
                    border-radius: 3px !important;
                    font-family: SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace !important;
                    font-size: 90% !important;
                }

                .bg-white pre, .content pre, pre {
                    background: #f6f8fa !important;
                    border-radius: 3px !important;
                    padding: 16px !important;
                    overflow: auto !important;
                }

                .bg-white pre code, .content pre code {
                    background: transparent !important;
                    padding: 0 !important;
                }

                .bg-white blockquote, .content blockquote, blockquote {
                    margin: 0 !important;
                    padding: 0 15px !important;
                    color: var(--color-text-muted) !important;
                    border-left: 4px solid var(--color-primary) !important;
                }

                .bg-white p, .content p {
                    margin: 10px 0 !important;
                    line-height: 1.6 !important;
                }

                .bg-white ul, .content ul,
                .bg-white ol, .content ol {
                    margin: 10px 0 !important;
                    padding-left: 30px !important;
                }

                .bg-white li, .content li {
                    margin: 5px 0 !important;
                }

                /* Custom Mermaid diagram styles - matches old template */
                .mermaid {
                    margin: 20px 0;
                    text-align: center;
                    height: calc(100vh - 150px);
                    width: 100%;
                    overflow: hidden;
                    position: relative;
                    border: 1px solid #EEEEEE;
                    border-radius: 3px;
                    background-color: #FAFAFA;
                    display: flex;
                    align-items: flex-start;
                    justify-content: center;
                    /* Hide raw markdown text until rendered */
                    color: transparent;
                }

                .mermaid svg {
                    /* Show rendered diagram */
                    color: initial;
                }

                .mermaid svg {
                    width: 100%;
                    height: 100%;
                    max-width: 100%;
                    max-height: 100%;
                    position: relative;
                    z-index: 1;
                }

                /* Diagram navigation buttons */
                .diagram-nav-buttons {
                    position: absolute;
                    top: 5px;
                    left: 5px;
                    z-index: 999;
                    display: flex;
                    flex-direction: column;
                    gap: 2px;
                    background-color: rgba(255, 255, 255, 0.85);
                    padding: 3px;
                    border-radius: 3px;
                    box-shadow: 0 1px 3px rgba(0,0,0,0.15);
                }

                .diagram-nav-row {
                    display: flex;
                    gap: 2px;
                    justify-content: center;
                }

                .diagram-nav-btn {
                    width: 22px;
                    height: 22px;
                    background-color: var(--color-primary);
                    color: white;
                    border: none;
                    border-radius: 2px;
                    cursor: pointer;
                    font-size: 11px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }

                .diagram-nav-btn:hover {
                    background-color: var(--color-primary-hover);
                }

                .diagram-nav-btn:active {
                    background-color: var(--color-primary-active);
                }

                /* Smooth transitions for interactive elements */
                a, button {
                    transition: all 0.2s ease-in-out;
                }

                /* Responsive typography */
                @media (max-width: 640px) {
                    h1 { font-size: 1.5rem; }
                    h2 { font-size: 1.25rem; }
                    h3 { font-size: 1.125rem; }
                }

                /* Print styles */
                @media print {
                    nav, .no-print { display: none; }
                }
            "#))
        }
    }
}
