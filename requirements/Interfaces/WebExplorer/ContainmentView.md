# Element

## Metadata
  * type: specification

## Relations
  * define: [Model-Centric View Generation](Capabilities.md#model-centric-view-generation)

## ContainmentView

# Model Containment Modes Specification

## Overview

Model browsing modes display the physical organization and graph structure of the model inside the canonical `index.html#/model` Explorer route. List and Grid browse the Project Store hierarchy, while Graph renders the project knowledge graph in the same Model workspace. There is no separate primary Containment Explorer route.

Three workspace modes are available:
- **List**: Tabular folder/file/element browsing with sortable fields
- **Grid**: Card-based folder/file/element browsing
- **Graph**: Interactive project graph over elements, resources, relations, reused_contract_context, and trace facts

Users switch between Model modes using compact controls in the left Explorer pane.

## Hierarchy Extraction

The containment hierarchy extraction must:

**Hierarchy Structure:**
- Start from the specifications root folder
- Traverse folder structure recursively
- For each folder: collect subfolders and files
- For each file: collect all elements (H3 headers with Metadata)
- For each element: collect reused_contract_context as children
- Skip sections (H2 headers) in the hierarchy representation

**Element Information:**
- Extract element identifier, name, and type
- Preserve file path and folder structure
- Maintain insertion order for elements within files
- Extract all reused_contract_context distinguishing between element and file reused_contract_context

**Data Structure:**
- Represent as tree: `Folder -> [Subfolders, Files]`
- Files contain: `File -> [Elements]`
- Elements contain: identifier, name, type, reused_contract_context
- Reused Contract Context displayed as children of elements

**Ordering:**
- Folders sorted alphabetically
- Files sorted alphabetically within folders
- Elements preserve document order within files
- Reused Contract Context preserve document order within elements

---

## Workspace Modes

### List View

The List view displays folders, files, and modeled elements in a dense tabular browser.

**Structure:**
- Breadcrumb identifies the current folder or file focus
- Rows represent folders, files, resources when relevant, and modeled elements
- Columns expose name, type, element count, and path/source details
- Type glyphs and badges use the shared Explorer type palette

**Interactive Capabilities:**
- Clicking a folder drills into that folder
- Clicking a file selects it and shows its modeled elements
- Secondary source actions open the source content route
- Modeled element rows open the shared element-detail modal

### Grid View

The Grid view displays the same folder/file/element hierarchy as compact tiles.

**Structure:**
- Tiles represent folders, files, and modeled elements
- Counts, type badges, and source paths use the same data as List mode
- Selection state is shared with the left project tree

**Interactive Capabilities:**
- Clicking a tile selects or opens the corresponding folder/file/element
- File tiles show modeled-element previews through the shared Project Store records
- Element tiles open the shared element-detail modal

### Graph View

The Graph view renders the project knowledge graph inside the Model workspace.

**Structure:**
- Nodes represent modeled elements and opt-in resource/evidence targets
- Edges represent relation facts, reused_contract_context facts, concept-reference facts, verification/satisfaction facts, and trace overlays
- Left-pane controls expose graph filters, overlays, layout reset, and selected element actions

**Interactive Capabilities:**
- Clicking a node pins it as the current graph selection
- Clicking empty canvas clears the pinned graph selection
- Selected graph nodes expose an element link in the left Explorer pane that opens the shared element-detail modal
- Graph labels, hover tooltips, and focused-neighborhood highlighting follow the shared Knowledge Graph behavior

---

## Visual Semantics

Both Model route visualizations use the Explorer design-system semantic palette rather than local color literals.

| Role | Visual contract |
|------|-----------------|
| folder | Shared folder icon and folder surface token |
| source file | Shared source-file icon and file surface token |
| capability | Capability role token and capability glyph |
| requirement | Requirement role token and requirement glyph |
| verification-objective | Verification-objective role token and plain square marker |
| verification | Concrete verification role token and verification glyph |
| contract | Contract role token with subtype-specific glyph |
| resource | Resource role token for referenced implementation, evidence, or document targets |
| other/default | Muted/default role token for unresolved or generic infrastructure nodes |

The concrete color values are owned by the Explorer design-system tokens. This design document names roles and usage only.

---

## Model Mode Controls

The Model route includes compact mode controls in the shared left Explorer pane:

**Toggle Buttons:**
- Three compact icon buttons: "List", "Grid", and "Graph"
- Active button uses the shared selected-control background and selected-control foreground tokens
- Clicking switches the visible view

**View Instructions:**
- View instructions belong in the shared Explorer help surfaces rather than in a page header or content preamble
- List/Grid: "Browse folders and files. Select modeled elements to inspect details."
- Graph: "Select graph nodes to focus relations. Use the selected element link to open details."

**Technical Implementation:**
- Model List, Grid, and Graph render as native Explorer mode states over the Project Store filesystem and knowledge-graph projections.
- Graph uses the shared Sigma/Graphology knowledge-graph renderer behavior inside the Model workspace.
- Model mode changes are handled by React Explorer UI state inside the canonical `index.html#/model` route, with no separate containment route.
- The route uses the shared headerless Explorer shell with vertical `Explorer` edge strip, expanded left-pane Model mode controls, central workspace, selected-item modal detail, and right tool rail.

---

## JSON Data Format

Both visualizations consume JSON data in this format:

```json
{
  "name": "Reqvire root",
  "type": "folder",
  "children": [
    {
      "name": "requirements",
      "type": "folder",
      "children": [
        {
          "name": "UserStories.md",
          "type": "file",
          "link": "#/content/requirements/UserStories.md",
          "children": [
            {
              "name": "Authentication",
              "type": "capability",
              "link": "#/content/requirements/UserStories.md#authentication",
              "children": [
                {
                  "name": "auth-design",
                  "type": "reused-contract-context-element",
                  "link": "#/content/requirements/Design.md#auth-design"
                },
                {
                  "name": "AuthSpec.pdf",
                  "type": "reused-contract-context-file",
                  "link": "docs/AuthSpec.pdf"
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

**Field Descriptions:**
- `name`: Display name for the node (files keep .md extension for display)
- `type`: Node type determining color and behavior
- `link`: Optional source or Explorer route link for clickable nodes
- `children`: Array of child nodes (empty array omitted in serialization)

**Reused Contract Context Node Types:**
- `reused-contract-context-element`: Element identifier (navigable to element definition)
- `reused-contract-context-file`: File name only, link contains the full path

---

## Explorer Data Integration

The Model workspace consumes Project Store records directly:

**Processing:**
1. Files, folders, elements, resources, and graph edges are normalized during serve runtime generation
2. JSON data preserves repository-relative source paths and canonical element identifiers
3. The compiled Explorer bundle renders List, Grid, and Graph modes from the shared Project Store
4. The served Explorer shall not depend on route-local Markdown code blocks or CDN-loaded visualization scripts for native Model modes

---

## Explorer Integration

Explorer integration must:

**Index Page:**
- Expose List, Grid, and Graph modes inside the canonical `index.html#/model` Explorer route
- Seed from the central Project Store containment/file sections rather than a page-local data island
- Keep `index.html` as the primary Explorer shell and browser entry point for model browsing

**Integration with Existing Explorer:**
- Follow the shared Explorer shell styling and structure
- Use the shared Explorer design-system role palette and surface tokens for consistency
- Maintain the shared headerless Explorer shell navigation pattern
- Include as Model mode controls, not as a separate left Explorer primary view

**Requirements:**
- Generated during Explorer serve runtime generation
- Updates automatically when model changes
- Deterministic output for version control
- List, Grid, and Graph modes render correctly with proper dimensions
