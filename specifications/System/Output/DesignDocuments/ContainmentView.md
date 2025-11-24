# Containment View Specification

## Hierarchy Extraction

The containment hierarchy extraction must:

**Hierarchy Structure:**
- Start from the specifications root folder
- Traverse folder structure recursively
- For each folder: collect subfolders and files
- For each file: collect all elements (H3 headers with Metadata)
- Skip sections (H2 headers) in the hierarchy representation

**Element Information:**
- Extract element identifier, name, and type
- Preserve file path and folder structure
- Maintain insertion order for elements within files

**Data Structure:**
- Represent as tree: `Folder -> [Subfolders, Files]`
- Files contain: `File -> [Elements]`
- Elements contain: identifier, name, type

**Ordering:**
- Folders sorted alphabetically
- Files sorted alphabetically within folders
- Elements preserve document order within files

---

## Mermaid Diagram Output

The Mermaid diagram generation must:

**Graph Structure:**
- Use `graph TD` (top-down layout)
- Folder nodes with connections to child folders and files
- File subgraphs containing element nodes
- Tree structure with explicit parent-child connections

**Node Format:**
- Root: `root["folder-icon Reqvire root"]`
- Folders: `folderId["folder-icon Folder Name"]`
- Files: subgraphs with format `fileId["file-icon File Name"]`
- Elements: `hashId["Element Name"]` within file subgraphs

**Connections:**
- `parent --> child` for folder/file hierarchy
- No connections between elements within files

**Element Display Modes:**
- Default: Show ALL elements in each file
- With `--short` flag: Show only root elements (those without hierarchical parents in same file)

**Element Nodes:**
- Use 16-character hash IDs for node uniqueness
- Display element name as node label
- Apply CSS classes based on element type

**Styling:**
- `userRequirement` - pink fill (#f9d6d6), red stroke (#f55f5f)
- `systemRequirement` - light pink fill (#fce4e4), pink stroke (#e68a8a)
- `requirement` - light pink fill (#fce4e4), pink stroke (#e68a8a)
- `verification` - light green fill (#d6f9d6), green stroke (#5fd75f)
- `folder` - light blue fill (#e8f4f8), blue stroke (#4a90a4)
- `file` - light yellow fill (#fff8e1), orange stroke (#f9a825)
- `default` - gray fill (#f5f5f5), dark stroke (#333333)

**Clickable Links:**
- Add `click` directives for each element node
- Link to element location: `click hashId "path.md#fragment"`
- Use relative paths from diagram location
- Normalize fragments to lowercase with hyphens

**Requirements:**
- Valid Mermaid syntax
- Deterministic node ordering
- Consistent hash ID generation
- Unique file IDs based on full path (not just filename)

---

## JSON Output (Optional)

The JSON structure must include:

**Root Level:**
```json
{
  "root_folder": "specifications",
  "folders": [ ... ],
  "files": [ ... ],
  "element_count": 123
}
```

**Folder Objects:**
```json
{
  "path": "specifications/SystemRequirements",
  "name": "SystemRequirements",
  "subfolders": [ ... ],
  "files": [ ... ]
}
```

**File Objects:**
```json
{
  "path": "specifications/Requirements.md",
  "name": "Requirements.md",
  "elements": [ ... ]
}
```

**Element Objects:**
```json
{
  "identifier": "specifications/Requirements.md#auth-system",
  "name": "Authentication System",
  "type": "requirement"
}
```

**Requirements:**
- Valid JSON format with proper escaping
- Deterministic key ordering
- Include metadata counts (folders, files, elements)

---

## HTML Export Integration

HTML export integration must:

**Containment View Page:**
- Create dedicated page: `containment.html`
- Generate `containment.md` with Mermaid diagram
- Convert to HTML during export process
- Include in navigation menu as "Containment" (after "Index")

**Integration with Existing Export:**
- Follow existing HTML export styling and structure
- Use same CSS classes for element types
- Maintain consistent navigation patterns
- Apply post-processing for .md to .html conversions

**Assets Export:**
- Export `assets/` folder with logo, favicon, and touch icons
- Include favicon link in all HTML pages
- Include logo in navigation bar before Index link
- Assets embedded at compile time for portability

**Requirements:**
- Generated during `reqvire export` command
- Updates automatically when model changes
- Deterministic output for version control
- Interactive Mermaid diagram with pan/zoom
