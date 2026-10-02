# Element

## Metadata
  * type: specification

## Relations
  * define: [Model-Centric View Generation](Capabilities.md#model-centric-view-generation)

## ContainmentView

# Model Containment Modes Specification

## Overview

Model browsing modes display the physical organization and graph structure of the model inside the canonical `index.html#/model` Explorer route. List and Grid browse the Project Store hierarchy, while Graph renders the project knowledge graph in the same Model workspace. There is no separate primary Containment Explorer route.

Four workspace modes are available:
- **List**: Tabular folder/file/element browsing with sortable fields
- **Grid**: Card-based folder/file/element browsing
- **Graph**: Interactive project graph over elements, resources, relations, contract_bindings, and trace facts
- **Flow**: Capability-first, top-to-bottom element cards descending through requirements, verifications, and evidence, with shared element details and complete path highlighting through splits and merges

In Flow, left-tree selection updates the relation scope displayed in the main workspace and keeps the Model route active. Activating an element card's name or body opens the shared element-detail modal. Pointer hover and keyboard focus on a card highlight its full directed paths through splits and merges.

Users switch between Model modes using the shared workspace layout selector. Flow uses the same card-and-connection pattern as the native trace preview, with horizontal/vertical layout, pan/zoom, and source navigation. The project-tree selection scopes Flow to folder/file elements and immediate relation endpoints while retaining their capability ancestry, or the complete directed paths of a selected element, as specified by the Model Browser and Graph Specification.

## Hierarchy Extraction

The containment hierarchy extraction must:

**Hierarchy Structure:**
- Start from the specifications root folder
- Traverse folder structure recursively
- For each folder: collect subfolders and files
- For each file: collect all elements (H3 headers with Metadata)
- For each element: collect contract_bindings as children
- Skip sections (H2 headers) in the hierarchy representation

**Element Information:**
- Extract element identifier, name, and type
- Preserve file path and folder structure
- Maintain insertion order for elements within files
- Extract all contract_bindings distinguishing between element and file contract_bindings

**Data Structure:**
- Represent as tree: `Folder -> [Subfolders, Files]`
- Files contain: `File -> [Elements]`
- Elements contain: identifier, name, type, contract_bindings
- Contract Bindings displayed as children of elements

**Ordering:**
- Folders sorted alphabetically
- Files sorted alphabetically within folders
- Elements preserve document order within files
- Contract Bindings preserve document order within elements

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
- Edges represent relation facts, contract_bindings facts, concept-reference facts, verification/satisfaction facts, and trace overlays; concept references target SKOS concept nodes and do not create concept-reference nodes
- Left-pane controls expose graph filters, overlays, layout reset, and selected element actions

**Interactive Capabilities:**
- Clicking a node pins it as the current graph selection
- Clicking empty canvas clears the pinned graph selection
- Selected graph nodes expose an element link in the left Explorer pane that opens the shared element-detail modal
- Full-graph layout is computed from currently visible nodes and edges with bounded data-dependent ForceAtlas settings so hidden filters and overlays do not distort the visible graph baseline
- Selected-node centering uses Sigma's display-coordinate mapping after focus layout updates so camera movement and focus animation converge without using raw Graphology coordinates as camera state
- Graph labels, hover tooltips, and focused-neighborhood highlighting follow the shared Knowledge Graph behavior

---

### Flow View

**Structure:**
- The project overview starts with root capabilities and follows child capabilities, requirements, verification, and evidence.
- Cards place the element name beside its type glyph and show context below the name.
- Shared endpoints appear once, with their incoming and outgoing relation labels and arrow directions preserved.

**Interactive Capabilities:**
- Selecting a folder, file, or element in the left tree renders its relation scope in the right workspace; tree selection and element inspection are separate actions.
- Clicking a card name or body opens the shared element-detail modal. Closing it returns to the same Flow context.
- Hovering or keyboard-focusing a card accents the complete visible paths into and out of that element, including every split and merge, and fades unrelated paths. Related lines and arrowheads change together.
- Moving the pointer away clears temporary highlighting. Focus path pins the highlight until cleared. These interactions preserve card positions and the viewport.
- The direction control switches between Top to bottom and Left to right while retaining relation direction. Pan, zoom, Fit, and actual-size controls support navigation.
- Initial framing and Fit include the full layout of cards, connections, and labels. The minimum zoom adapts to layout and canvas size so large flows fit; users can zoom farther out or return to 100%. Scope, direction, and canvas resizing refresh framing.
- Expand flow opens a full-page overlay with the same scope, direction, and path focus. Close or Escape returns to the embedded view and its expand control. Element details open above the expanded canvas and return to it when closed.
- Source and focus controls perform their respective actions independently of opening element details.

---

## Visual Semantics

All Model route visualizations use the Explorer design-system semantic palette.

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

The Model route includes compact mode controls in the shared workspace toolbar:

**Toggle Buttons:**
- Four compact icon buttons: "List", "Grid", "Graph", and "Flow"
- Active button uses the shared selected-control background and selected-control foreground tokens
- Clicking switches the visible view

**View Instructions:**
- View instructions belong in the shared Explorer help surfaces rather than in a page header or content preamble
- List/Grid: "Browse folders and files. Select modeled elements to inspect details."
- Graph: "Select graph nodes to focus relations. Use the selected element link to open details."
- Flow: "Select a tree item to explore its flow. Hover a card to follow its paths and select its name to inspect details."

**Technical Implementation:**
- Model List, Grid, Graph, and Flow render as native Explorer mode states over the Project Store filesystem and knowledge-graph projections.
- Graph uses the shared Sigma/Graphology knowledge-graph renderer behavior inside the Model workspace, including visible-graph ForceAtlas layout and focused-neighborhood no-overlap animation.
- Model mode changes are handled by React Explorer UI state inside the canonical `index.html#/model` route, with no separate containment route.
- The route uses the shared headerless Explorer shell with vertical `Explorer` edge strip, left-pane project tree, workspace mode controls, selected-item modal detail, and right tool rail.

---

## JSON Data Format

Both visualizations consume JSON data in this format:

```json
{
  "name": "Model",
  "type": "folder",
  "children": [
    {
      "name": "reqvire",
      "type": "folder",
      "children": [
        {
          "name": "system-model",
          "type": "folder",
          "children": [
            {
              "name": "UserStories.md",
              "type": "file",
              "link": "#/content/system-model/UserStories.md",
              "children": [
                {
                  "name": "Authentication",
                  "type": "capability",
                  "link": "#/content/system-model/UserStories.md#authentication",
                  "children": [
                    {
                      "name": "auth-design",
                      "type": "contract-bindings-element",
                      "link": "#/content/system-model/Design.md#auth-design"
                    },
                    {
                      "name": "AuthSpec.pdf",
                      "type": "contract-bindings-file",
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
  ]
}
```

**Field Descriptions:**
- `name`: Display name for the node (files keep .md extension for display)
- `type`: Node type determining color and behavior
- `link`: Optional source or Explorer route link for clickable nodes
- `children`: Array of child nodes (empty array omitted in serialization)

**Contract Bindings Node Types:**
- `contract-bindings-element`: Element identifier (navigable to element definition)
- `contract-bindings-file`: File name only, link contains the full path

---

## Explorer Data Integration

The Model workspace consumes Project Store records directly:

**Processing:**
1. Files, folders, elements, resources, and graph edges are normalized during serve runtime generation
2. JSON data preserves workspace-root-relative source paths and canonical element identifiers
3. The compiled Explorer bundle renders List, Grid, Graph, and Flow modes from the shared Project Store
4. The served Explorer shall not depend on route-local Markdown code blocks or CDN-loaded visualization scripts for native Model modes

---

## Explorer Integration

Explorer integration must:

**Index Page:**
- Expose List, Grid, Graph, and Flow modes inside the canonical `index.html#/model` Explorer route
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
- List, Grid, Graph, and Flow modes render correctly with proper dimensions
