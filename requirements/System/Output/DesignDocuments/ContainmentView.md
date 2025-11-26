# Containment View Specification

## Overview

The containment view displays the physical organization of the model as an interactive D3.js collapsible tree. This serves as the main entry point (index.html) for HTML documentation, providing a complete navigable view of the model structure including elements and their attachments.

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

## D3.js Tree Output

The D3.js tree visualization must:

**Tree Structure:**
- Root node representing the model root
- Folder nodes that can be expanded/collapsed
- File nodes containing element children
- Element nodes as leaf nodes

**Node Types and Icons:**
| Type | Icon | Color |
|------|------|-------|
| folder | 📁 | #9E9E9E (gray) |
| file | 📄 | #FFCA28 (yellow) |
| user-requirement | 👤 | #7E57C2 (purple) |
| system-requirement | 📐 | #673AB7 (deep purple) |
| requirement | 📐 | #673AB7 (deep purple) |
| verification | ✅ | #4CAF50 (green) |
| refinement | 🔧 | #FF9800 (orange) |
| design-document | 📝 | #8D6E63 (brown) |
| attachment-element | 🔧 | #FF9800 (orange) |
| attachment-file | 📎 | #607D8B (blue-gray) |

**Attachment Types:**
- `attachment-element`: References to refinement elements, shown with wrench icon and navigable link
- `attachment-file`: References to external files (PDFs, docs), shown with paperclip icon

**Interactive Features:**
- Click folder/file/element nodes to expand/collapse children
- Click element nodes to navigate to their definition
- Click element attachments to navigate to the refinement element
- Expand All button to show entire tree
- Collapse All button to collapse to root level
- Smooth animations for expand/collapse transitions

**Navigation:**
- Element clicks navigate to `file.html#element-fragment`
- Element attachment clicks navigate to refinement element definition
- File attachments show path for reference
- All links use `.html` extension for HTML export

---

## JSON Data Format

The D3.js tree consumes JSON data in this format:

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
          "name": "UserStories.html",
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
- `name`: Display name for the node
- `type`: Node type (folder, file, user-requirement, attachment-element, attachment-file, etc.)
- `link`: Optional navigation link for clickable nodes
- `children`: Array of child nodes (empty array omitted in serialization)

**Attachment Node Types:**
- `attachment-element`: Element identifier (navigable to element definition)
- `attachment-file`: File name only, link contains the full path

---

## Markdown Integration

The D3.js tree is embedded in markdown using a code block:

```
```d3-tree
{ "name": "Root", "type": "folder", "children": [...] }
```
```

**Processing:**
1. `d3-tree` code blocks are extracted before markdown processing
2. JSON data is preserved as-is
3. During HTML conversion, blocks are replaced with D3.js visualization
4. D3.js library loaded from CDN (d3js.org)

---

## HTML Export Integration

HTML export integration must:

**Index Page:**
- Generate as `index.md` containing D3.js tree
- Convert to `index.html` during export
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
- Interactive tree with expand/collapse functionality
