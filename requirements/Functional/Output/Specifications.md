# Elements

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
- When direction is UPSTREAM (default):
 - Traverse derivedFrom relations in reverse direction (child to parents)
 - Continue until root ancestors reached (elements with no derivedFrom)
- When direction is DOWNSTREAM:
 - Traverse derive relations in forward direction (parent to children)
 - Continue until leaf descendants reached (elements with no derive to other requirements)
- Include the starting element in output

**Content Collection:**
- Collect element content field (main body text including Details section)
- For each refinedBy target:
 - ElementIdentifier (refinement element): Include element's content
 - FilePath pointing to .md file: Read and include file content
 - FilePath pointing to other file types: Include as markdown link
- For each attachment:
 - FilePath pointing to .md file: Read and include file content
 - FilePath pointing to other file types: Include as markdown link
 - ElementIdentifier: Include referenced element's content
- Skip external URL attachments

**Output Ordering:**
- Flat list structure (no nesting)
- When direction is UPSTREAM: ancestors first (depth 0 = root), then starting element
- When direction is DOWNSTREAM: starting element first (depth 0), then descendants at increasing depth
- Same-depth elements sorted alphabetically by name or file path

**Error Handling:**
- Element not found: Error with message
- Element not a requirement type: Error with message
- Attachment file not found: Warning, continue with other content
- Circular reference: Detect and break cycle

#### Metadata
 * type: specification
---

### Collect Output Format Specification

Output format specification for collect command text and JSON modes.

#### Details
**Text Format:**
Each collected content block followed by source citation and separator:

```
[Content from element or attachment]

— Source: [Element Name](file.md#element-id)


```

**Citation Formats:**
| Source Type | Citation Format |
|-------------|-----------------|
| Element | `— Source: [Element Name](file.md#element-id)` |
| Refinement Element (via refinedBy) | `— Source: [Refinement Name](file.md#refinement-id) refining [Element Name](file.md#element-id)` |
| Attachment Element | `— Source: [Refinement Name](file.md#refinement-id) attached to [Element Name](file.md#element-id)` |

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
 "name": "Attached Refinement",
 "identifier": "Refinements.md#deterministic-output-specification",
 "file_path": "Refinements.md",
 "element_type": "specification",
 "content": "Technical refinement details...",
 "depth": 0,
 "source_type": "attachment_element",
 "attached_to": "file.md#element-id"
 }
 ],
 "metadata": {
 "element_count": 5,
 "refinement_count": 3,
 "attachment_count": 1,
 "total_items": 9
 }
}
```

**Source Type Values:**
- `element` - Content from model element
- `refined_by_element` - Content from refinement element (via refinedBy relation)
- `attachment_element` - Content from attached refinement element

#### Metadata
 * type: specification
---

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
| attachment-element | #607D8B | 📎 |

#### Metadata
 * type: specification
---

### Comma-Separated Type Filter Parsing Refinement Specification

#### Details
`--filter-type` parsing behavior:
- Splits input by comma delimiter.
- Trims whitespace around each token.
- Normalizes type tokens to lowercase for case-insensitive matching.
- Validates each parsed type against supported element types.
- Reports clear errors for invalid type tokens.

Filtering behavior:
- Matches elements when any parsed type matches (OR semantics).
- Supports custom type syntax using `other-TYPENAME`.
- Preserves single-type query behavior as a compatible subset.

#### Metadata
 * type: specification

#### Relations
 * refine: [Comma-Separated Type Filter Parsing](Reporting.md#comma-separated-type-filter-parsing)
---

### Containment View Report Refinement Specification

#### Details
The containment view shows the physical organization of the model:
- Root folder → Subfolders → Files → Elements
- Sections skipped (elements shown directly under files)

The system is expected to generate reports in multiple formats:
- Mermaid diagrams for visualization
- JSON for programmatic access
- HTML export integration

The system is expected to include design documents:
- Files in DesignDocuments folders displayed alongside specifications
- Design documents visually distinguished from specification elements
- Clickable navigation to document files

#### Metadata
 * type: specification
---

### Deterministic Output Specification

Technical specification for ensuring deterministic, reproducible output across all report generation operations.

#### Details
All generated reports is expected to produce deterministic output with consistent ordering to enable reliable testing, version control, and reproducible builds.

**Ordering Rules:**

1. **Element Ordering**: Elements is expected to be sorted by identifier before iteration to ensure consistent processing order across all operations
2. **Relation Ordering**: Relations within each element is expected to be sorted by relation type name and then by target identifier before rendering
3. **Section Ordering**: Sections within files is expected to be sorted alphabetically when order is not semantically significant
4. **File Ordering**: Files within folders is expected to be sorted alphabetically

**Benefits of Determinism:**
- Running the same operation multiple times produces byte-identical output
- Automated tests can reliably compare expected and actual outputs using simple diff tools without special normalization
- Version control diffs are meaningful and reflect actual changes rather than random ordering variations
- Continuous integration pipelines produce consistent, reproducible results

**Applies to:**
- Model summary reporting (text and JSON formats)
- Verification tracing (upward traceability trees from verifications to requirements)
- Coverage reporting (verification coverage analysis)
- Change impact analysis (reports showing propagation of changes)
- Validation reporting (model validation error reports)
- Linting (model quality issue reports)

#### Metadata
 * type: specification
---

### Diagram Relation Filtering Specification

Technical specification for relation filtering in diagram generation to render only forward relations while ensuring complete element hierarchy representation.

#### Details
**Auto-Generated Diagram Identification:**
The system is expected to embed a unique identification marker "REQVIRE-AUTOGENERATED-DIAGRAM" as a comment within all auto-generated mermaid diagrams to distinguish them from user-created diagrams.

The marker must be:
- Embedded as a mermaid comment line using the `%% REQVIRE-AUTOGENERATED-DIAGRAM` format
- Present in every auto-generated diagram
- Not present in user-created custom diagrams

This enables the system to:
- Reliably identify auto-generated diagrams regardless of their location in documents
- Support mixed documents containing both auto-generated and custom diagrams

**Diagram Relation Filtering Rules:**
When generating diagrams, the system is expected to apply the following relation filtering rules:

1. **Diagram Relation Filtering**: Only relations specified in the DIAGRAM_RELATIONS list is expected to be rendered to prevent duplicate arrows representing the same logical relationship
2. **Complete Hierarchy Inclusion**: Start with file-local parent requirements but include all children even if they are defined outside of the file
3. **List-Based Rendering**: Relations is expected to be rendered according to the DIAGRAM_RELATIONS list which defines which relation from each opposite pair should be shown

**Filtering Benefits:**
The filtering ensures that:
- Bidirectional relationships (e.g., `derivedFrom`/`derive`) appear as single arrows using the relation specified in DIAGRAM_RELATIONS
- Hierarchical context is preserved by starting from local parents and showing all derived children regardless of file location
- Diagram readability is maintained while accurately representing the complete model structure

#### Metadata
 * type: specification
---

### File Diagram Attachment Display Refinement Specification

#### Details
File-diagram attachment rendering behavior:
- Renders attachments below the element name using `<br/>` separators.
- Prefixes each attachment entry with `📎`.
- Displays attached refinement element names.
- Emits clickable attachment links to refinement identifier targets.
- Renders each attachment on its own line.
- For elements without attachments, renders only the element name.

Label format example:
`Element Name<br/>📎 Deterministic Output Specification<br/>📎 Rate Limits`

#### Metadata
 * type: specification

#### Relations
 * refine: [File Diagram Attachment Display](DiagramGeneration.md#file-diagram-attachment-display)
---

### Flexible Search Type Filtering Refinement Specification

#### Details
Users is expected to be able to specify multiple element types in a single search operation using comma-separated values (e.g., `requirement,test-verification,behavior`).

This capability enables:
- Searching across related type categories (all requirement types, all verification types)
- Building complex queries without multiple search invocations
- Improved workflow efficiency for model analysis and reporting

#### Metadata
 * type: specification
---

### Implementation Coverage Output Structure Specification

Technical specification for implementation coverage report output structure.

#### Details
**Output Requirements (Text):**
- Summary subsection with totals, covered/uncovered counts, and percentage.
- Covered requirements list grouped by file with source classification and evidence identifiers.
- Uncovered requirements list grouped by file.

**Output Requirements (JSON):**
- `summary` includes:
 - `total_requirements_in_scope`
 - `covered_requirements`
 - `uncovered_requirements`
 - `implementation_coverage_percentage`
 - `coverage_sources` object with counts for `direct_satisfied`, `refinement_contract_satisfied_via_attachment`, `refinement_contract_satisfied_via_child`
- `covered_requirements` contains per-element:
 - `identifier`, `name`
 - `coverage_source`
 - `evidence` (identifier list)
- `uncovered_requirements` contains per-element:
 - `identifier`, `name`
- Coverage percentage values in summary is expected to be emitted with at most 2 decimal places.

#### Metadata
 * type: specification

#### Relations
 * refine: [Requirement Implementation Coverage Report](Reporting.md#requirement-implementation-coverage-report)
---

### Interactive Mermaid Diagram Node Behavior Refinement Specification

#### Details
Clickable mermaid diagrams links by default must use relative links to the git repository.

CLI flag options must be provided that can change default behavior to use stable github repository links:
* diagrams click links are not working on Github if not using stable github repository links
* from another side that pollutes PR diffs thus choice must be given to the user
* Commands that generate diagrams (`generate-diagrams`, `export`, `serve`) must expose `--links-with-blobs` CLI flag for that purpose
* The flag defaults to `false` (use relative paths)

When generating diagram node links and when `--links-with-blobs` flag is set to `true`, the system is expected to:
- Use stable git repository links (`{repository-url}/blob/{commit-hash}/{file-path}`) when git repository information is available
- Fallback to relative markdown links when git repository information is not available
- Use the current commit hash to ensure links remain stable even as the repository evolves
- Match the same link format used in traceability matrices and change impact reports
- Preserve interactive behavior across all generated diagrams

The `traces` command is expected to always use relative paths (hardcoded to `false`, no flag needed).

The `change-impact` command is expected to continue to use GitHub blob URLs by default (unchanged behavior).

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
- `attachments`: Array of refinement element identifier strings

**Error Handling:**
- Error responses include `error` field with message
- Successful responses omit error field entirely
- Exit code accompanies JSON (0=success, non-zero=error)

**File Output:**
- When `--output <FILE>` is provided alongside `--json`, write JSON to file instead of stdout
- Print confirmation message to stdout: `✅ Output saved to <filepath>`
- `--output` without `--json` is an error

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

### Mermaid Diagram Generation Specification

Technical specification for Mermaid diagram generation approach and structure.

#### Details
**Diagram Generation Approach:**
Diagram generation follows a file-based approach:
- One diagram is generated per specification file
- The diagram shows all elements in the file and their relationships
- External related resources are displayed as linked boxes to the actual resource

**Diagram Styling:**
The system is expected to implement diagram styling including:
- Containment structure with nested subgraphs for physical organization
- Element type-specific CSS classes for visual differentiation
- Relation-specific line styles and colors
- Interactive highlighting on hover
- Consistent background and border styling

**Model Structure Visualization:**
The system is expected to provide visualization of the complete model structure showing an element-centric view with nested relations:
- Display elements with their properties (identifier, name, type, file location)
- Show relations nested inside elements with full target details
- Support recursive nesting for element-to-element relations
- Handle file path and external URL relations
- Provide metadata about total elements and relations
- Use consistent visual styling with mermaid diagrams showing hash-based node identifiers

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
---

### Mermaid Interactive Features Specification

Technical specification for interactive Mermaid diagram navigation and filtering capabilities.

#### Details
**Model Navigation and Filtering:**
Users is expected to be able to generate and view model structure diagrams from any starting point:
- Default view shows root requirements (no hierarchical parent)
- Filter from specific element using --from flag
- Generate complete model structure with nested relations showing element details recursively
- Mermaid diagrams display all nested relations recursively

**Diagram Output:**
The system is expected to generate Mermaid diagrams embedded in markdown format for visual representation of the model structure.

**Interactive Capabilities:**
The visualization helps users:
- Understand the model's logical structure
- Navigate relationships between elements
- Explore the model from any starting point
- Filter and focus on specific subtrees of the model

#### Metadata
 * type: specification
---

### Model Diagram Output Formats Refinement Specification

#### Details
Model output format rules:
- Markdown format includes embedded Mermaid diagram with model structure.
- Markdown shows hierarchical structure using containment subgraphs (folders > files > elements).
- Mermaid diagrams use folder and file subgraphs to visually group elements by physical location.
- JSON format uses structured data with folders, files, sections, elements, relations, and attachments.
- Both formats represent the same filtered or complete model data.
- Element attachments are included as an array of refinement element identifier strings in both formats.

#### Metadata
 * type: specification

#### Relations
 * refine: [Model Diagram Output Formats](Reporting.md#model-diagram-output-formats)
---

### Requirement Implementation Coverage Logic Specification

Technical specification for requirement implementation coverage classification logic.

#### Details
Implementation coverage scope includes only elements of type `requirement`. Elements of type `user-requirement` are excluded from implementation coverage.

**Coverage Sources:**
- `direct_satisfied`: requirement has one or more `satisfiedBy` relations.
- `refinement_contract_satisfied_via_attachment`: requirement has no direct satisfaction but has owned refinements (`refinedBy`) attached by a directly satisfied requirement.
- `refinement_contract_satisfied_via_child`: requirement has no direct satisfaction but owns refinements and has directly satisfied derived descendants.
- `uncovered`: no implementation evidence from any source.

#### Metadata
 * type: specification

#### Relations
 * refine: [Requirement Implementation Coverage Report](Reporting.md#requirement-implementation-coverage-report)
---

### Requirement Submodels Report Specification

Technical specification for submodels report structure and deterministic ordering.

#### Details
**Submodel Boundary Principle:**
- Reqvire models are structured as independent hierarchical submodels, each with clear ownership, lifecycle, and stakeholder responsibility.
- Hierarchical relations are used only for internal decomposition within a submodel.
- Cross-submodel dependencies are expressed through explicit attachment contracts, not hierarchical coupling.
- This preserves boundary clarity, supports independent evolution of submodels, and keeps collect, change-impact, and coverage outputs deterministic and auditable.

**Refactor Rule:**
When a relation crosses intended submodel boundaries, either:
1. Move/reparent to restore hierarchical ownership, or
2. Replace cross-boundary hierarchy links with attachment-based refinement contracts.

**Refactor Procedure:**
Apply boundary refactoring recursively, top-down:
1. Start from each top root and inspect its first-level children.
2. For each first-level child, inspect all direct children and relation edges.
3. Continue recursively for each descendant branch until leaf requirements.
4. At each level, enforce:
 - hierarchical relations remain internal to that branch/submodel,
 - cross-branch dependencies are attachment contracts.
5. If a cross-boundary hierarchical relation is found, either:
 - move/reparent to restore ownership, or
 - replace with attachment-based refinement contract.
6. Re-run validation and submodel analysis after each boundary slice before continuing recursion.

**Internal Sub-Boundaries:**
A submodel may contain internal sub-boundaries (nested domains) with separate ownership and lifecycle.
Cross-internal-boundary dependencies should be modeled as explicit attachment contracts when they represent contractual dependency, not hierarchical ownership.

**Submodel Resolution Rules:**
- A submodel root is a requirement element with no hierarchical parent relation.
- Submodel membership is resolved using hierarchical relations only, in downstream direction.
- Each requirement is expected to be assigned to one resolved top root for report grouping.
- For scope-scoped report generation, the selected requirement defines report scope; it is not itself reported as a submodel entry.

**Report Content:**
- List all discovered submodels with:
 - root identifier and display name
 - root element type
 - requirement count in that submodel
- List cross-submodel requirement couplings:
 - source requirement
 - relation type
 - target requirement
 - source and target root context

**Cross-Submodel Coupling Scope:**
- Include requirement-to-requirement relations where source and target belong to different top roots.
- Use explicit relation targets only (no inferred transitive links).
- For scope-scoped report generation, include only couplings relevant to submodels inside the selected scope.

**Scope Resolution Rules:**
- In scope mode, select a requirement by name and compute the induced downstream subtree via hierarchical relations.
- Submodel roots for scope mode are first-level roots of this induced subtree where no parent in the induced subtree exists.
- If the selected boundary has no children in the induced subtree, `submodels` output is empty.

**Output Formats:**
*Text/Markdown Format:*
- Human-readable sectioned report with deterministic ordering
- Markdown links for source/target/root identifiers
- Summary section with totals
- When filtered by scope:
 - output submodels discovered within selected scope
 - do not output selected scope boundary as a submodel entry
 - summary counts are computed from filtered output only

*JSON Format:*
- Structured arrays for `submodels` and `cross_submodel_couplings`
- Summary object with deterministic count fields
- Stable sort order for reproducible automation output
- When filtered by scope:
 - JSON includes only filtered-scope submodel data and relevant couplings
 - selected scope boundary is excluded from `submodels` array

**Summary Section Semantics:**
- Summary reports:
 - `Submodels`: number of scoped top-level submodel roots returned
 - `Requirements`: number of requirements counted across returned scoped submodels
 - `Cross-Submodel Couplings`: number of qualifying couplings for the active scope
- In scoped mode, all summary fields are derived from scoped submodel and coupling sets only.

#### Metadata
 * type: specification
---

### Resources Report Format Specification

Technical specification for resources report structure and output formats.

#### Details
**Report Structure:**
The resources report is expected to consist of two sections:
1. Relations section
2. Attachments section

**Relations Section:**
- Files from InternalPath relation targets (satisfiedBy, trace, etc.)
- Shows relation type and source element for each reference
- Sorted by relation type, then by element identifier
- Each file lists all elements that reference it with their relation types

**Attachments Section:**
- Refinement element identifiers from attachment targets
- Shows source element for each reference
- Sorted by element identifier
- Each refinement identifier lists all elements that attach it

**Output Formats:**

*Text/Markdown Format:*
- Human-readable with markdown links
- Entries listed alphabetically by path (relations) and identifier (attachments)
- Element references shown as clickable markdown links
- Clear section headers separating Relations and Attachments

*JSON Format:*
- Structured data for programmatic use
- Same logical structure as text format
- Includes file paths, element identifiers, relation types
- Suitable for automated processing and integration

**HTML Export:**
- Resources view available in HTML export with navigation link
- Shows complete list of referenced files with element traceability
- Maintains same structure as text/JSON outputs
- Provides clickable navigation between resources and elements

#### Metadata
 * type: specification
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
- Attachments listed as refinement element identifiers

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

### Trace Relation Non-Directional Behavior Refinement Specification

#### Details
The trace relation behavior is expected to include:

1. **Circular Dependency Exclusion**:
- Trace relations is expected to not be traversed during circular dependency detection
- The cycle detection algorithm is expected to skip trace relations to prevent false positive cycles
- Trace relations exist solely for traceability and documentation purposes

2. **Non-Propagation Behavior**:
- Changes is expected to not propagate through trace relations
- Trace relations is expected to not be included in change impact analysis
- Impact trees is expected to not traverse trace relation connections

3. **Bidirectional Traceability**:
- Trace relations is expected to provide bidirectional navigational capability
- Users can navigate from source to target and target to source
- Both directions are semantically equivalent for traceability purposes

4. **Validation Behavior**:
- Trace relations is expected to be validated for target existence
- Trace relations is expected to not require type compatibility validation
- Trace relations can connect any element type to any other element type

This ensures that trace relations serve their intended purpose of establishing lightweight traceability connections without creating artificial dependency constraints or participating in architectural validation logic.

#### Metadata
 * type: specification
---

### Verification Trace Diagram Specification

Visual styling rules for verification trace diagrams.

#### Details
**Diagram Output:**
Verification trace diagrams is expected to use the same visual styling as other mermaid diagrams:
- Containment structure with folder and file subgraphs showing physical location of elements
- Element type-based CSS classes (userRequirement, systemRequirement, verification) for consistent coloring
- Directly verified requirements highlighted with appropriate styling

#### Metadata
 * type: specification
---
