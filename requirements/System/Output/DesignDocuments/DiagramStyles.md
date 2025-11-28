# Elements

### Mermaid Diagram Interaction Behavior

Interactive behavior for Mermaid diagrams in HTML export.

#### Details
**Click Behavior:**
- Nodes are clickable links to element definition
- Links use relative paths from output location
- External links open in new tab

**Zoom/Pan:**
- Mouse wheel for zoom
- Click and drag for pan
- Reset button to restore initial view

#### Metadata
  * type: behavior
---

### Mermaid Diagram Style Specification

Styling conventions for Mermaid diagrams in CLI output and HTML export.

#### Details
**Node Shapes:**
| Element Type | Shape | Example |
|--------------|-------|---------|
| Requirement | Rectangle | `[Requirement Name]` |
| Verification | Stadium | `([Verification Name])` |
| File | Folder shape | `{{File.md}}` |
| Folder | Hexagon | `{{folder/}}` |

**Edge Styles:**
| Relation | Line Style | Arrow |
|----------|------------|-------|
| derivedFrom | Solid | Arrow |
| verifiedBy | Dashed | Arrow |
| satisfiedBy | Dotted | Arrow |
| trace | Dotted | No arrow |

**Colors (CSS Classes):**
| Class | Color | Usage |
|-------|-------|-------|
| requirement | #D0E0FF | Requirement nodes |
| verification | #FFF7B3 | Verification nodes |
| implementation | #DFFFD0 | Implementation nodes |
| impacted | #FFAAAA | Change impact nodes |
| changed | #FFDD57 | Changed nodes |

**Subgraph Styling:**
- Folders as subgraphs with light gray background
- Files as nested subgraphs
- Collapsible in interactive mode

#### Metadata
  * type: specification
---
