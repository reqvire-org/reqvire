# Elements

### Component-Based HTML Architecture Refinement Specification



#### Details
The HTML generation system shall be organized into reusable components:

**Shared components:**
- Navigation menu (with mobile/desktop variants)
- Page header with metadata
- Footer
- Mobile menu toggle

**Reusable layouts:**
- Base layout for standard pages
- Diagram layout for full-height visualizations

**Page-specific modules:**
- Index/Containment page
- Model view page
- Traces page
- TraceFlow page
- Coverage page
- Resources page
- Individual specification pages

Each component shall be defined once and reused across all generated pages to eliminate code duplication.

#### Metadata
  * type: specification
---

### D3.js Containment Tree Specification

Specification for the D3.js interactive containment tree visualization.

#### Details
The containment page (containment.html) shall display an interactive D3.js collapsible tree showing the containment hierarchy:
1. Root node representing the model root
2. Folder nodes that can be expanded/collapsed
3. File nodes containing element children
4. Element nodes with type-specific icons and colors
5. Attachment nodes as children of elements (element and file attachments)
6. Clickable elements that navigate to their definitions
7. Expand All / Collapse All buttons for tree control

The containment view serves as the primary entry point for HTML documentation, providing an interactive visual overview of the model structure.

#### Metadata
  * type: specification
---

### HTML Branding Specification

Specification for Reqvire branding elements in HTML export.

#### Details
**Logo and Branding:**
- The navigation bar shall display the Reqvire logo on the left side before the navigation links
- A favicon shall be included for browser tab identification
- Apple touch icons shall be included for mobile device support
- All brand assets shall be exported to an assets folder during HTML export

**HTML Design:**
The system shall design and implement HTML pages with consistent layout, styling, and navigation for browsing the System model.

#### Metadata
  * type: specification
---

### HTML Export Pipeline Specification

Technical specification for HTML export generation pipeline.

#### Details
**Working Directory Setup:**
- Create temporary working directory (e.g., in /tmp)
- Generate markdown files from registry with full relations (user-created and auto-generated inverse relations)
- Copy all related system elements (following satisfiedBy and other relations)
- Copy all attachment files to temporary directory preserving structure

**Generation Pipeline (in temporary directory):**
Execute all generation commands treating temporary directory as repository root:
1. Generate all Mermaid diagrams in markdown files
2. Generate index.md (interactive D3.js tree showing containment hierarchy - main entry point)
3. Generate model.md (model-centric visualization with nested relations from root requirements)
4. Generate traces.md (verification upward traceability)
5. Generate coverage.md (verification coverage report)

**HTML Conversion:**
- Convert all markdown files to HTML with embedded styles
- Process Mermaid diagrams for web rendering
- Convert internal .md links to .html links
- Preserve directory structure

**Output:**
- Accept optional `--output` option to specify output directory
- When `--output` is not specified, export to a temporary directory and print the path
- When `--output` is specified, create output folder if not existing
- Copy generated HTML and all artifacts from temp directory to output directory
- Clean up temporary working directory (except when output is temp directory)

**Source Protection:**
- Never modify original repository files
- All generation happens in isolated temporary directory

**Git Directory Exclusion:**
- The .git directory shall never be exported to the output folder
- This prevents internal git metadata from polluting the exported documentation

**Export Related System Elements:**
- Ensure that any related system elements are also copied into output folder to ensure consistency of exported model

#### Metadata
  * type: specification
---

### Serve Command Refinement Specification

#### Details
Serve command behavior:
- Accept `--host <HOST>` option to specify the bind address (default: localhost)
- Accept `--port <PORT>` option to specify the server port (default: 8080)
- Use a random temporary directory for HTML export
- Run HTML Export to generate complete documentation in the temporary directory
- Start an HTTP server serving static files from the temporary directory
- Display clickable server URL for user to open in browser
- Display instructions to press Ctrl-C to stop server
- Continue serving until terminated by the user (Ctrl-C)

#### Metadata
  * type: specification

#### Relations
  * refine: [Serve Command](Features.md#serve-command)
---

### Attachment Export Refinement Specification

#### Details
Attachment export behavior during HTML export:
- Collects attachment references from `element.attachments` across the model.
- Copies each attachment file into the output directory while preserving relative paths.
- Skips duplicate file copies when the same file is referenced by multiple elements.
- Emits progress information for attachment copy operations.

This keeps exported documentation complete for offline browsing with referenced files.

#### Metadata
  * type: specification

#### Relations
  * refine: [Attachment Export](Features.md#attachment-export)
---

### Containment View Attachment Links Refinement Specification

#### Details
Containment view attachment rendering behavior:
- For each element with attachments, renders attachments as child nodes in the D3 tree.
- Uses wrench icon (`🔧`) and type `attachment-element` for element attachments.
- Uses paperclip icon (`📎`) and type `attachment-file` for file attachments.
- Element-attachment nodes navigate to the referenced element.
- File-attachment nodes display filename and path reference.

#### Metadata
  * type: specification

#### Relations
  * refine: [Containment View Attachment Links](Features.md#containment-view-attachment-links)
---

### Diagram Attachment Display Refinement Specification

#### Details
Diagram attachment rendering behavior in Mermaid output:
- Renders attachment links under the element name inside node labels.
- Prefixes each attachment with paperclip icon (`📎`).
- Displays filename rather than full path for compact diagrams.
- Produces clickable links to the referenced document.
- Uses Mermaid multiline label formatting (`<br/>`).

Example node:
```
elementId["Element Name<br/>📎 DesignDoc.md"]
```

#### Metadata
  * type: specification

#### Relations
  * refine: [Diagram Attachment Display](Features.md#diagram-attachment-display)
---

### Model-Centric View Generation Refinement Specification

#### Details
Model-centric view generation behavior:
- Uses root requirements (no hierarchical parent) as top-level entries.
- Expands relations recursively with full target element details.
- Includes summary metadata for element and relation counts.
- Generates Mermaid diagrams for nested relation structures.
- Produces markdown output that is later rendered as `model.html`.

#### Metadata
  * type: specification

#### Relations
  * refine: [Model-Centric View Generation](Features.md#model-centric-view-generation)
---

### Model View Element Navigation Refinement Specification

#### Details
Model-view element navigation behavior:
- Element-name headers render as hyperlinks.
- Links target element source file plus fragment identifier.
- Link format: `[Element Name](file_path#element-fragment)`.
- Navigation enables direct jump from model view to definition.

#### Metadata
  * type: specification

#### Relations
  * refine: [Model View Element Navigation](Features.md#model-view-element-navigation)
---

### Responsive HTML Generation Refinement Specification

#### Details
Responsive HTML behavior:
- Supports viewport widths from 320px (mobile) through 1920px+ (desktop).
- Uses mobile-first CSS with progressive enhancement.
- Provides hamburger navigation for viewports under 768px.
- Scales typography and spacing by responsive breakpoints.

Breakpoints:
- `sm`: 640px and up.
- `md`: 768px and up.
- `lg`: 1024px and up.
- `xl`: 1280px and up.

#### Metadata
  * type: specification

#### Relations
  * refine: [Responsive HTML Generation](HTMLGeneration.md#responsive-html-generation)
---

### CSS Framework Integration Refinement Specification

#### Details
CSS framework integration behavior:
- Uses Tailwind CSS via CDN in current export flow.
- Applies mobile-first utility classes for responsive layout behavior.
- Uses responsive modifiers (`sm`, `md`, `lg`, `xl`) for adaptation.
- Defines Reqvire theme colors including primary, requirement, and verification tones.

Tailwind usage provides:
- Utility-first styling for predictable page composition.
- Built-in responsive modifiers (for example `md:hidden`, `lg:flex`).
- Consistent spacing, color, and typography scales.

#### Metadata
  * type: specification

#### Relations
  * refine: [CSS Framework Integration](HTMLGeneration.md#css-framework-integration)
---

### Type-Safe HTML Generation Refinement Specification

#### Details
Type-safe HTML generation behavior:
- Uses `maud` macros for compile-time HTML generation.
- Relies on Rust type checks to validate structure during compilation.
- Prevents malformed tags, unclosed nodes, and invalid nesting.
- Produces well-formed HTML5 output for generated pages.

This shifts most structural HTML errors to compile time instead of runtime.

#### Metadata
  * type: specification

#### Relations
  * refine: [Type-Safe HTML Generation](HTMLGeneration.md#type-safe-html-generation)
---

### HTML Navigation Bar Specification

Specification for the fixed navigation bar in HTML pages.

#### Details
The system shall provide a fixed navigation bar in all HTML pages with links to key model artifacts for easy access.

The navigation bar must include (left to right):
- Containment: Link to containment.html (interactive D3.js tree - main entry point)
- Model: Link to model.html (model-centric view with nested relations)
- Traces: Link to traces.html (verification upward traceability report)
- Coverage: Link to coverage.html (verification coverage report)

The navigation bar must be:
- Always visible (fixed position) while scrolling
- Consistent across all HTML pages
- Clearly visible and accessible

#### Metadata
  * type: specification
---

### Web Interface Refinement Specification



#### Details
The browse interface allows users to:
- View HTML-rendered specifications and requirements
- Navigate through diagrams and visualizations
- Access verification traces and coverage reports
- Explore the complete model structure through an integrated web interface

This capability enables both human users (via browser) and AI agents (via MCP server) to efficiently explore and understand the System model without manually navigating file structures.

All generated HTML content shall produce deterministic output with consistent ordering to enable reliable version control and reproducible builds.

The system shall ensure deterministic HTML output by:
- Sorting elements by identifier before rendering
- Sorting relations by type and target identifier
- Maintaining consistent navigation and page ordering
- Generating stable diagram node and relation ordering

This determinism ensures that:
- Running HTML generation multiple times produces byte-identical output
- Version control diffs reflect actual content changes
- Continuous integration pipelines produce reproducible results

#### Metadata
  * type: specification
---

### Web Interface Style Specification

Styling conventions for HTML export web interface.

#### Details
**Page Layout:**
- Navigation bar at top with links to views
- Content area with responsive width
- Sidebar for element tree (optional)

**Typography:**
- System font stack for readability
- Monospace for code and identifiers
- Heading hierarchy matches markdown levels

**Color Palette (MONO Theme):**
The web interface uses a monochrome grayscale theme for consistent, professional appearance.

| Usage | Color | Notes |
|-------|-------|-------|
| Navigation background | #1c1c1c | Dark gray for main nav bar |
| Navigation hover | #2a2a2a | Subtle highlight on hover |
| Primary/Buttons | #333333 | Lighter gray for interactive elements |
| Primary hover | #4a4a4a | Button hover state |
| Page background | #FAFAFA | Light gray page background |
| Content background | #FFFFFF | White content cards |
| Text primary | #212121 | Dark gray for headings |
| Text secondary | #424242 | Medium gray for body text |
| Text muted | #757575 | Light gray for secondary info |
| Links | #4a4a4a | Grayscale links |
| Borders | #EEEEEE | Light borders |
| Highlight | #d0d0d0 | Selection/highlight color |

**Element Cards:**
| Element Type | Border Color | Background |
|--------------|--------------|------------|
| Requirement | #0066FF | #D0E0FF |
| User-requirement | #0066FF | #D0E0FF |
| Verification | #CC9900 | #FFF7B3 |
| Behavior | #9900CC | #E0D0FF |
| Specification | #009900 | #DFFFD0 |
| Constraint | #CC0000 | #FFD0D0 |

**Navigation:**
- Breadcrumb trail for element hierarchy
- Clickable relation links
- Collapsible sections for long content

#### Metadata
  * type: specification
---
