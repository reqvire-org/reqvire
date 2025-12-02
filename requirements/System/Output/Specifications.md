# Elements

### Color Scheme Specification

Comprehensive color coding for terminal output, HTML export, and diagram generation.

#### Details
**Terminal Colors (ANSI):**
| Color | Meaning | Usage |
|-------|---------|-------|
| Red | Error | Validation errors, failed operations |
| Yellow | Warning | Lint issues needing review, deprecations |
| Green | Success | Added content, passed checks |
| Cyan | Info | Element names, identifiers |
| White/Default | Normal | Regular content |

**Primary Colors (HTML):**
| Color Name | Hex Code | Usage |
|------------|----------|-------|
| Indigo | #3F51B5 | Navigation bar background, primary branding |
| Indigo Hover | #7986CB | Navigation hover states |
| Indigo Active | #303F9F | Navigation active/pressed states |
| Off-White | #FAFAFA | Body background |
| White | #FFFFFF | Content background, navigation text |

**Element Type Colors:**
| Element Type | Color Name | Hex Code | Usage |
|--------------|------------|----------|-------|
| Requirement | Deep Purple | #673AB7 | Core requirements, goals |
| User Requirement | Light Purple | #7E57C2 | User-level requirements |
| Verification | Emerald Green | #4CAF50 | Validation criteria, testing |
| Refinement | Orange | #FF9800 | Behaviors, constraints, specifications |
| Other | Cool Gray | #9E9E9E | Other element types |

**Status Indicator Colors:**
| Status | Color Name | Hex Code | Usage |
|--------|------------|----------|-------|
| Verified/Passing | Forest Green | #4CAF50 | Verified requirements, passing tests |
| Pending/Warning | Amber | #FFB74D | Unverified items, warnings |
| Failed/Error | Red | #F44336 | Error messages, validation errors |

**Interactive State Colors:**
| State | Hex Code | Usage |
|-------|----------|-------|
| Hover Highlight | #FFAB91 | Diagram node/edge hover effect |
| Node Hover Shadow | rgba(255,171,145,0.7) | Drop-shadow on node hover |
| Link Color | #3F51B5 | Hyperlinks |

**D3.js Containment Tree Colors:**
| Node Type | Hex Code | Icon |
|-----------|----------|------|
| folder | #9E9E9E | 📁 |
| file | #FFCA28 | 📄 |
| user-requirement | #7E57C2 | 👤 |
| requirement | #673AB7 | 📐 |
| verification | #4CAF50 | ✅ |
| refinement | #FF9800 | 🔧 |
| design-document | #8D6E63 | 📝 |
| attachment-file | #607D8B | 📎 |

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Web Interface Color Scheme](../../Interfaces/WebInterface.md#web-interface-color-scheme)
---

### Diff Output Format Specification

Git-style diff format for change previews.

#### Details
**Used by commands:** format, lint, add, rm, mv, rename, mv-file, change-impact

**Format:**
```diff
--- a/<file_path>
+++ b/<file_path>
@@ -<old_start>,<old_count> +<new_start>,<new_count> @@
-<removed line>
+<added line>
 <context line>
```

**Colors:**
- Red: Removed lines (-)
- Green: Added lines (+)
- Cyan: Hunk headers (@@)
- White: Context lines

**Context:**
- Show 3 lines before and after changes
- Collapse large unchanged sections

#### Metadata
  * type: specification
---

### Error Message Format Specification

Structure for error and warning messages.

#### Details
**Format:**
```
<file_path>:<line_number>: <level>: <message>
  <context_line>
  ^--- <pointer to issue>
```

**Fields:**
- `file_path`: Git-root-relative path
- `line_number`: 1-based line number
- `level`: error | warning | info
- `message`: Concise description
- `context_line`: Source line (optional)
- `suggestion`: How to fix (optional)

**Grouping:**
- Group errors by file
- Sort by line number within file

#### Metadata
  * type: specification
---

### JSON Output Structure

Standard JSON output structure for CLI commands that support the `--json` flag.

#### Details
JSON output conventions:

**Structure:**
- Root object with semantic field names (not abbreviated)
- Arrays for collections (elements, relations, files)
- Nested objects for hierarchical data
- Consistent field naming using snake_case

**Common Fields:**
- `identifier`: Full element identifier (file#fragment)
- `name`: Display name of element
- `type`: Element type string
- `file_path`: Relative path from git root
- `relations`: Array of relation objects with `type` and `target` fields
- `attachments`: Array of attachment strings (file paths or element identifiers)

**Error Handling:**
- Error responses include `error` field with message
- Successful responses omit error field entirely
- Exit code accompanies JSON (0=success, non-zero=error)

#### Metadata
  * type: specification
---

### Markdown Report Style Specification

Style guidelines for markdown text report output (model, coverage, traces, containment commands).

#### Details
**Document Structure:**
- Title as H1 header
- Major sections as H2 headers
- Subsections as H3 headers
- Element listings as bullet points or tables

**Formatting Conventions:**
- Element names in backticks: `Element Name`
- File paths in backticks: `path/to/file.md`
- Identifiers in backticks: `file.md#element-id`
- Relation types in bold: **derivedFrom**, **verifiedBy**
- Counts and percentages: `15 (75%)`

**Tables:**
- Use markdown tables for structured data
- Align columns appropriately (left for text, right for numbers)
- Include header row with separator

**Lists:**
- Hierarchical bullet lists for tree structures
- Numbered lists for sequential steps
- Indentation shows nesting (2 spaces per level)

**Code Blocks:**
- Mermaid diagrams in ```mermaid blocks
- JSON output in ```json blocks

#### Metadata
  * type: specification
---

### Mermaid Diagram Style Specification

Styling conventions for Mermaid diagrams in CLI output and HTML export.

#### Details
**Containment Structure:**
- Folder subgraphs: `subgraph hashId["folder-icon Folder Name"]`
- File subgraphs: `subgraph hashId["file-icon File Name"]`
- Elements rendered inside their containing file subgraph
- Collapsible in interactive mode

**Node Shapes:**
| Element Type | Shape | Example |
|--------------|-------|---------|
| Requirement | Rectangle | `[Requirement Name]` |
| Verification | Stadium | `([Verification Name])` |
| File | Folder shape | `{{File.md}}` |
| Folder | Hexagon | `{{folder/}}` |

**Diagram Node Classes (CSS):**
| Class Name | Fill Color | Stroke Color | Usage |
|------------|------------|--------------|-------|
| userRequirement | #D1C4E9 | #7E57C2 | Top-level user requirements |
| systemRequirement | #E1D8EE | #673AB7 | System-level requirements |
| requirement | #ECEFF1 | #673AB7 | Generic requirements |
| verified | #D1C4E9 | #7E57C2 | Directly verified requirements |
| verification | #DCEDC8 | #4CAF50 | Verification elements |
| folder | #FAFAFA | #9E9E9E | Folder containers |
| file | #FFFFFF | #9E9E9E | File containers |
| attachment | #EFEBE9 | #8D6E63 | Design documents |
| impacted | #FFAAAA | - | Change impact nodes |
| changed | #FFDD57 | - | Changed nodes |

**Relation Line Styles:**
| Relation Type | Color | Line Style |
|---------------|-------|------------|
| Derive/DerivedFrom | #673AB7 | Dashed |
| Verify/VerifiedBy | #4CAF50 | Dashed |
| Satisfy/SatisfiedBy | #673AB7 | Solid |
| Trace | #9E9E9E | Dashed |

**Interactive Highlighting:**
| Effect | Implementation |
|--------|----------------|
| Hovered node | drop-shadow(0 0 8px rgba(255,171,145,0.7)) |
| Connected edges | stroke: #FFAB91, increased width |

**Diagram Background:**
- Canvas: #FAFAFA (off-white)
- Border: 1px solid #EEEEEE

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Interactive Mermaid Diagrams](DiagramGeneration.md#interactive-mermaid-diagrams)
---

### SysML Rendering Specification

SysML notation standards for relationship rendering in diagrams.

#### Details
Each relationship type is represented using SysML standard notation with specific arrow direction.

**Derive Relations:**
| Relation | Stereotype | Line Style | Arrow Direction |
|----------|------------|------------|-----------------|
| derive | «deriveReqt» | dashed | Parent → Child (derived) |
| derivedFrom | «deriveReqt» | dashed | Child → Parent (source) |

**Verify Relations:**
| Relation | Stereotype | Line Style | Arrow Direction |
|----------|------------|------------|-----------------|
| verify | «verify» | dashed | Verification → Requirement |
| verifiedBy | «verify» | dashed | Requirement → Verification |

**Satisfy Relations:**
| Relation | Stereotype | Line Style | Arrow Direction |
|----------|------------|------------|-----------------|
| satisfy | «satisfy» | solid | Implementation → Requirement |
| satisfiedBy | «satisfy» | solid | Requirement → Implementation |

**Trace Relations:**
| Relation | Stereotype | Line Style | Arrow Direction |
|----------|------------|------------|-----------------|
| trace | «trace» | dashed | Tracing → Traced (neutral) |

**Arrowhead Style:**
All relation types use open (hollow) arrowheads per SysML specification.

#### Metadata
  * type: specification

#### Relations
  * satisfy: [SysML-Compatible Relationship Rendering](DiagramGeneration.md#sysml-compatible-relationship-rendering)
---

### Collect Content Specification

Technical specification for content collection from requirement chains.

#### Details
**Input Validation:**
- Element name is required positional argument
- Element must exist in the model
- Element must be a requirement type (requirement or user-requirement)
- Error with non-zero exit if element not found or invalid type

**Traversal Rules:**
- Start from specified requirement element
- Traverse derivedFrom relations in reverse direction (child to parents)
- Continue until root ancestors reached (elements with no derivedFrom)
- Include the starting element in output

**Content Collection:**
- Collect element content field (main body text including Details section)
- For each attachment:
  - FilePath pointing to .md file: Read and include file content
  - FilePath pointing to other file types: Include as markdown link
  - ElementIdentifier: Include referenced element's content
- Skip external URL attachments

**Output Ordering:**
- Flat list structure (no nesting)
- Ancestors first (depth 0 = root), then descendants
- Same-level elements sorted alphabetically by name or file path

**Error Handling:**
- Element not found: Error with message
- Element not a requirement type: Error with message
- Attachment file not found: Warning, continue with other content
- Circular reference: Detect and break cycle

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Collect Content from Requirement Chain](Reporting.md#collect-content-from-requirement-chain)
---

### Collect Output Format Specification

Output format specification for collect command text and JSON modes.

#### Details
**Text Format:**
Each collected content block followed by source citation and separator:

```
[Content from element or attachment]

— Source: [Element Name](file.md#element-id)

---

```

**Citation Formats:**
| Source Type | Citation Format |
|-------------|-----------------|
| Element | `— Source: [Element Name](file.md#element-id)` |
| Attachment File | `— Source: [filename.md](path/to/file.md) attached to [Element Name](file.md#element-id)` |
| Refinement Element | `— Source: [Refinement Name](file.md#refinement-id) satisfying [Element Name](file.md#element-id)` |

**JSON Format:**
```json
{
  "starting_element": "file.md#element-id",
  "items": [
    {
      "name": "Element Name",
      "identifier": "file.md#element-id",
      "file_path": "path/to/file.md",
      "element_type": "requirement",
      "content": "The collected content...",
      "depth": 0,
      "source_type": "element"
    },
    {
      "name": "Attached File",
      "identifier": "path/to/attachment.md",
      "file_path": "path/to/attachment.md",
      "element_type": "attachment",
      "content": "Content from attachment file...",
      "depth": 0,
      "source_type": "attachment_file",
      "attached_to": "file.md#element-id"
    }
  ],
  "metadata": {
    "element_count": 5,
    "attachment_count": 2,
    "total_items": 7
  }
}
```

**Source Type Values:**
- `element` - Content from model element
- `attachment_file` - Content from attached file
- `attachment_element` - Content from attached refinement element

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Collect Content from Requirement Chain](Reporting.md#collect-content-from-requirement-chain)
---

### Text Output Formatting

Human-readable text output conventions for CLI commands.

#### Details
Default text output (when neither `--json` nor other format flags specified):

**Hierarchical Display:**
- Group elements by file, then by section
- Use indentation to show containment
- Display element name with type indicator

**Element Information:**
- Full element name and identifier
- Element type in brackets: `[requirement]`, `[test-verification]`
- Relations listed with target identifiers
- Attachments listed as file paths or element names

**Formatting:**
- Color output when terminal supports it (errors in red, warnings in yellow)
- Git-style diff format for change previews
- Line numbers for file references

**Consistency:**
- Deterministic ordering (alphabetical by identifier)
- Consistent spacing and alignment
- No trailing whitespace

#### Metadata
  * type: specification
---
