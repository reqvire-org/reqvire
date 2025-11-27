# Containment View Specification

## Overview

The containment view displays the physical organization of the model using interactive D3.js visualizations. This serves as the main entry point (index.html) for HTML documentation, providing a complete navigable view of the model structure including elements and their attachments.

Two visualization types are available:
- **Sunburst**: Circular nested hierarchy with zoom-in/zoom-out navigation
- **Icicle**: Horizontal rectangular bars with breadcrumb navigation

Users can switch between views using toggle buttons at the top of the page.

## Hierarchy Extraction

The containment hierarchy extraction must:

**Hierarchy Structure:**
- Start from the specifications root folder
- Traverse folder structure recursively
- For each folder: collect subfolders and files
- For each file: collect all elements (H3 headers with Metadata)
- For each element: collect attachments as children
- Skip sections (H2 headers) in the hierarchy representation

**Element Information:**
- Extract element identifier, name, and type
- Preserve file path and folder structure
- Maintain insertion order for elements within files
- Extract all attachments distinguishing between element and file attachments

**Data Structure:**
- Represent as tree: `Folder -> [Subfolders, Files]`
- Files contain: `File -> [Elements]`
- Elements contain: identifier, name, type, attachments
- Attachments displayed as children of elements

**Ordering:**
- Folders sorted alphabetically
- Files sorted alphabetically within folders
- Elements preserve document order within files
- Attachments preserve document order within elements

---

## Visualization Types

### Sunburst View

The Sunburst visualization displays the hierarchy as concentric rings:

**Structure:**
- Center represents the root (model root)
- Each ring represents a level of hierarchy
- Segment size proportional to number of descendants
- Color-coded by element type

**Interactive Features:**
- Click on any segment to zoom into that subtree
- Click center circle to zoom out (navigate to parent)
- Center text shows current focused node name
- Center text is a clickable link when node has a link
- Hover shows breadcrumb path with color-coded ancestors
- Smooth zoom animations

**Navigation:**
- Clicking a leaf node with a link navigates to that element
- Clicking the center text link navigates to the focused element
- Breadcrumb displays path from root to hovered element

### Icicle View

The Icicle/Partition visualization displays the hierarchy as horizontal rectangular bars:

**Structure:**
- Left-to-right hierarchy (root on left)
- Each column represents a level of hierarchy
- Bar height proportional to number of descendants
- Color-coded by element type

**Interactive Features:**
- Click on any bar to zoom into that subtree
- Click on breadcrumb items to navigate back up the hierarchy
- Breadcrumb path is clickable for quick navigation
- Current focused node shown in bold in breadcrumb
- Element link displayed when zoomed into a node with a link
- Smooth zoom animations

**Navigation:**
- Clicking a leaf bar with a link navigates to that element
- Clicking the element link navigates to the focused element
- Breadcrumb items are clickable to zoom out to any ancestor

---

## Color Scheme

Both visualizations use consistent colors matching Mermaid diagrams:

| Type | Color | Description |
|------|-------|-------------|
| folder | #9E9E9E | Gray for directories |
| file | #B8860B | Dark goldenrod for files |
| design-document | #607D8B | Blue-gray for design docs |
| user-requirement | #7E57C2 | Purple for user requirements |
| system-requirement | #673AB7 | Deep purple for system requirements |
| verification | #4CAF50 | Green for verifications |
| test-verification | #4CAF50 | Green for test verifications |
| refinement | #FF9800 | Orange for refinements |
| attachment-element | #8D6E63 | Brown for element attachments |
| attachment-file | #8D6E63 | Brown for file attachments |

---

## View Toggle

The page includes a toggle mechanism:

**Toggle Buttons:**
- Two buttons: "Sunburst" and "Icicle"
- Active button highlighted with primary color
- Clicking switches the visible view

**View Instructions:**
- Each view has context-specific instructions shown below the toggle
- Sunburst: "Click on segments to zoom in. Click center circle to zoom out. Click the center name link to navigate to the element."
- Icicle: "Click on bars to zoom in. Click breadcrumb path to navigate back. Click the element link to open it."

**Technical Implementation:**
- Both views render on page load (required for D3 dimension calculations)
- Icicle view hidden after DOMContentLoaded
- Switching views toggles CSS display property

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
          "link": "requirements/UserStories.html",
          "children": [
            {
              "name": "User Authentication",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#user-authentication",
              "children": [
                {
                  "name": "auth-design",
                  "type": "attachment-element",
                  "link": "requirements/Design.html#auth-design"
                },
                {
                  "name": "AuthSpec.pdf",
                  "type": "attachment-file",
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
- `link`: Optional navigation link for clickable nodes (uses .html extension)
- `children`: Array of child nodes (empty array omitted in serialization)

**Attachment Node Types:**
- `attachment-element`: Element identifier (navigable to element definition)
- `attachment-file`: File name only, link contains the full path

---

## Markdown Integration

The D3.js visualizations are embedded in markdown using code blocks:

**Sunburst:**
```
```d3-sunburst
{ "name": "Root", "type": "folder", "children": [...] }
```
```

**Icicle:**
```
```d3-icicle
{ "name": "Root", "type": "folder", "children": [...] }
```
```

**Processing:**
1. Code blocks are extracted before markdown link conversion
2. JSON data is preserved with .md extensions in names
3. During HTML conversion, blocks are replaced with D3.js visualizations
4. D3.js library loaded from CDN (d3js.org)

---

## HTML Export Integration

HTML export integration must:

**Index Page:**
- Generate as `containment.md` containing both D3.js visualizations
- Rename to `index.html` during export
- Serve as primary entry point for documentation

**Integration with Existing Export:**
- Follow existing HTML export styling and structure
- Use Reqvire color scheme for consistency
- Maintain consistent navigation patterns
- Include in navigation bar as "Index" (first link)

**Requirements:**
- Generated during `reqvire export` command
- Updates automatically when model changes
- Deterministic output for version control
- Both views render correctly with proper dimensions
