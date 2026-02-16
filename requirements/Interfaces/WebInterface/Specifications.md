# Elements

### Component-Based HTML Architecture Refinement Specification

Specification extracted from requirement "Component-Based HTML Architecture".

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

Specification extracted from requirement "Web Interface".

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
