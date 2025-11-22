# Requirements

### Containment View Report Generation

The system shall generate containment view reports showing the physical hierarchical structure of the model (folders → files → elements) in multiple output formats including Mermaid diagrams, JSON, and HTML export integration.

#### Details
<details><summary>View Full Specification</summary>

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
- Represent as tree: `Folder → [Subfolders, Files]`
- Files contain: `File → [Elements]`
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
- Root: `root["📁 Reqvire root"]`
- Folders: `folderId["📁 Folder Name"]`
- Files: subgraphs with format `fileId["📄 File Name"]`
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
- Include in navigation menu as "Containment" (after "Home")

**Integration with Existing Export:**
- Follow existing HTML export styling and structure
- Use same CSS classes for element types
- Maintain consistent navigation patterns
- Apply post-processing for .md to .html conversions

**Requirements:**
- Generated during `reqvire export` command
- Updates automatically when model changes
- Deterministic output for version control
- Interactive Mermaid diagram with pan/zoom

</details>

#### Relations
  * derivedFrom: [Containment View Report](#containment-view-report)
---

### Forward-Only Relation Traversal

When filtering by root element, system shall traverse only forward relations down to leaf elements.

#### Details
- Shall follow only forward relations (derive, satisfiedBy, verifiedBy, trace)
- Shall start from specified root element (looked up by name)
- Shall recursively traverse outgoing relations to leaf elements
- Shall NOT traverse backward (no bidirectional traversal)
- Unfiltered diagrams (no --from) shall show complete model with all elements

#### Relations
  * derivedFrom: [CLI Model Diagram Command](../../Interfaces/CLI.md#cli-model-diagram-command)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](Verifications/ReportingVerifications.md#model-command-verification)
---

### Search Report Generator

The system shall implement a search report generator that produces comprehensive element searches with filtering, supporting both full and abbreviated output modes.

#### Details
The search report must include:

**File-level Information:**
- File path and name
- Number of sections per file
- Number of elements per file
- Page content (frontmatter content before first section header) - omitted in short mode

**Section-level Information:**
- Section name and hierarchy
- Number of elements per section - omitted in short mode
- Section content (content between section header and first element, excluding generated diagrams) - omitted in short mode

**Element Information:**
- Element identifier, name, type, and section
- Element content - omitted in short mode
- Verified and satisfied relations counts - omitted in short mode
- Complete list of relations with targets and types
- Complete list of attachments as file paths - omitted in short mode

**Global Counts:**
- Total files, pages, sections, and elements - omitted in short mode
- Requirements by type (system, user) - omitted in short mode
- Verifications by type (test, analysis, inspection, demonstration) - omitted in short mode
- Missing relations (unverified and unsatisfied requirements) - omitted in short mode

**Output Formats:**
- Human-readable text format with hierarchical display
- Human-readable abbreviated text format (with --short flag): one-line per element showing `[type] identifier - name`
- JSON format for programmatic processing
- JSON abbreviated format (with --short flag): omits content, page_content, verified_relations_count, satisfied_relations_count, element_count, total_elements, global_counters

The system must support comprehensive filtering by file path, element name, type, element content, page content, and relation presence. All filters are applied conjunctively.

#### Relations
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
---

### Verification Coverage Report Generator

The system shall provide a verification coverage report generator that analyzes leaf requirements verification status, test-verification satisfaction status, and orphaned verifications to produce coverage metrics and detailed reports.

#### Details
The coverage report generator must:
- Identify all leaf requirements (requirements without forward relations to other requirements) in the model
- Determine leaf requirement verification status based on presence of verifiedBy relations
- Identify all verification elements in the model with breakdown by verification type
- Determine test-verification satisfaction status based on presence of satisfiedBy relations
- Identify orphaned verifications (verification elements without any verify relations to requirements)
- Calculate coverage percentages: (verified_leaf_requirements/total_leaf_requirements * 100), (satisfied_test_verifications/total_test_verifications * 100), and (orphaned_verifications/total_verifications * 100)
- Group results by file and section for organization
- Support both human-readable text and machine-readable JSON output formats

The report structure shall include:
- Summary section with leaf requirements, test-verification, and orphaned verification counts and percentages
- Verified leaf requirements section grouped by file and section
- Unverified leaf requirements section with details (flagged for attention)
- Satisfied test-verification elements section grouped by file and section
- Unsatisfied test-verification elements section with details (flagged for attention)
- Orphaned verifications section with details (flagged for attention as they may be redundant or incorrectly configured)
- Analysis, inspection, and demonstration verification elements are considered satisfied by default

#### Relations
  * satisfiedBy: [report_coverage.rs](../../../core/src/report_coverage.rs)
  * derivedFrom: [Verification Coverage Report](#verification-coverage-report)
  * derivedFrom: [Search Report Generator](#search-report-generator)
---

### Custom Element Type Tracking

The system SHALL track and display custom element types (any type not in the standard categories) in model summary reports, providing counts for each custom type in both text and JSON output formats.

#### Details
The custom element type tracking feature must:

**Custom Type Definition:**
- Identify any element type that is not one of the standard types: requirement, user-requirement, verification, test-verification, analysis-verification, inspection-verification, or demonstration-verification
- Track custom types separately from standard element type counters
- Store custom types in a HashMap with type name as key and count as value

**Text Output:**
- Display custom element types in the "Element Types" section of the summary
- Format: `Custom (type-name): count`
- Sort custom types alphabetically by type name
- Only display custom types section when at least one custom type exists

**JSON Output:**
- Include custom element types in global_counters as `custom_element_types` field
- Structure: `{"custom_element_types": {"type-name": count, ...}}`
- Skip serialization of the field when no custom types exist (using `skip_serializing_if`)
- Maintain alphabetical sorting of type names in JSON output

**Exclusions:**
- Standard requirement types (requirement, user-requirement) SHALL NOT be counted as custom
- Standard verification types (verification, test-verification, analysis-verification, inspection-verification, demonstration-verification) SHALL NOT be counted as custom
- File-type elements SHALL NOT be counted as custom types

#### Relations
  * derivedFrom: [Search Report Generator](#search-report-generator)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
---

### Model Diagram Output Formats

System shall support markdown and JSON output formats.

#### Details
- Markdown format shall include embedded Mermaid diagram with model structure
- Markdown shall show hierarchical structure (folders > files > sections > elements)
- JSON format shall use structured data with folders, files, sections, elements, relations, and attachments
- Both formats shall represent the same filtered or complete model data
- Element attachments shall be included as an array of file paths in both formats

#### Relations
  * derivedFrom: [CLI Model Diagram Command](../../Interfaces/CLI.md#cli-model-diagram-command)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](Verifications/ReportingVerifications.md#model-command-verification)
---

### Search Fine Grained Filtering

The system shall implement comprehensive fine-grained filtering for the search report generator following the specifications below.

#### Details
<details><summary>View Full Specification</summary>

## Summary

This specification defines the functional requirements for a filtering subsystem used within the `search` reporting feature. The system must allow clients to selectively include or exclude elements from the search output based on metadata, content, and traceability properties.

The filters shall be composable and applied conjunctively (i.e., all active filters must match for an element to be included). The filtering system must support both human-readable text output and structured machine-readable output (e.g., JSON), as well as abbreviated short mode output.

---

## Filtering Scope

Filtering shall operate on the level of individual `Element` objects in the model registry. Each `Element` has the following relevant properties:

- `file_path: String`
- `name: String`
- `element_type: ElementType`
- `content: String`
- `relations: Vec<Relation>`
- `page_content: String` (from parent file)

---

## Supported Filters

The filtering system **must support the following filters**, which may be active simultaneously.

### 1. File Path Filter (Glob)

**Purpose:** Restrict search to elements defined in files whose paths match a given glob pattern.

**Input:** A single string pattern using glob syntax (e.g., `"src/**/*Spec.md"`)

**Match Target:** `Element.file_path`

**Behavior:** Case-sensitive glob match. If the glob does not match any file, no elements are included.

---

### 2. Name Filter (Regex)

**Purpose:** Include only elements whose `name` matches a regular expression.

**Input:** A valid Rust-compatible regular expression (e.g., `"autonomous.*"`)

**Match Target:** `Element.name`

**Behavior:** Case-sensitive match by default. The filter is considered invalid if the regex fails to compile.

---

### 3. Type Filter (Exact Match)

**Purpose:** Include only elements of a specific type.

**Input:** One of the following valid string identifiers:

- `"user-requirement"`
- `"system-requirement"`
- `"verification"`
- `"file"`
- Any user-defined type (e.g., `"interface"`, `"design"`)

**Match Target:** `Element.element_type`

**Behavior:** Matching must be exact. Internally, the filter string shall be mapped to an `ElementType` via a deterministic lookup function.

---

### 4. Content Filter (Regex)

**Purpose:** Include only elements whose body content matches a regular expression.

**Input:** A valid regex pattern applied to the element's `content`.

**Match Target:** `Element.content`

**Behavior:** Case-sensitive regex match. Invalid patterns must cause an immediate user-facing error.

---

### 5. Page Content Filter (Regex)

**Purpose:** Include only elements whose parent file's page content (frontmatter) matches a regular expression.

**Input:** A valid regex pattern applied to the file's page content.

**Match Target:** Page content (frontmatter) of the element's parent file

**Behavior:** Case-sensitive regex match. Invalid patterns must cause an immediate user-facing error.

---

### 6. Have Relations Filter (Comma-separated list)

**Purpose:** Include only elements that have ALL specified relation types.

**Input:** Comma-separated list of relation type names (e.g., `"verifiedBy,satisfiedBy"`)

**Match Target:** `Element.relations`

**Behavior:** When specified, element must have at least one relation of each specified type to be included. Invalid relation type names shall cause an error with a list of valid relation types.

---

### 7. Not Have Relations Filter (Comma-separated list)

**Purpose:** Include only elements that do NOT have ALL specified relation types.

**Input:** Comma-separated list of relation type names (e.g., `"verifiedBy"`)

**Match Target:** `Element.relations`

**Behavior:** When specified, element must NOT have all the specified relation types to be included. If element has all specified relation types, it is excluded. Invalid relation type names shall cause an error with a list of valid relation types.

---

## Filter Composition

All filters are applied **conjunctively**. That is, an element is included in the search results **only if all active filters return `true`** for that element.

---

## Error Handling

- Invalid regular expressions must produce a fatal error with a descriptive message.
- Invalid glob patterns should fail at startup with appropriate feedback.
- Unknown or malformed `type` filters should be rejected with a list of accepted values.
- Invalid relation type names in `--have-relations` or `--not-have-relations` shall produce an error listing valid relation types.

---

## Output Behavior

Filtered results must be consistent across all output modes (text, JSON, short text, short JSON). The final search results must include only elements passing the full filter set, and global counters should reflect the filtered subset.

---

## Performance Considerations

The filtering system must evaluate filters with minimal passes over element data. Repeated relation scans should be avoided in favor of single-pass accumulation.

---

## Test Cases (Examples)

| Filter Combination | Expected Result |
|--------------------|------------------|
| `type = verification` | Only verification elements |
| `filter-file = "System*"` + `name = ".*GPS.*"` | Elements in System files with GPS in name |
| `have-relations = verifiedBy,satisfiedBy` | Elements that have both verifiedBy AND satisfiedBy relations |
| `not-have-relations = verifiedBy` | Elements that do NOT have any verifiedBy relations |
| `filter-page-content = "architecture"` | Elements in files whose frontmatter contains "architecture" |

---

</details>

#### Relations
  * satisfiedBy: [filters.rs](../../../core/src/filters.rs)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * derivedFrom: [Search Report Generator](#search-report-generator)
  * verifiedBy: [Search Command Tests](Verifications/ReportingVerifications.md#search-command-tests)
---

### Tracing Structural Changes

When tracing structural changes, the system shall analyze the System model and diffs to identify affected components and generate a report of impacted elements and structures, so that the user can review the changes and decide on further actions.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Trace Changes in System Model](../../UserStories.md#trace-changes-in-system-model)
---

### Validation Report Generator

The system shall implement a validation report generator that compiles and formats validation results from all validators, providing a unified view of model quality with categorized issues, remediation suggestions, and compliance metrics.

#### Relations
  * derivedFrom: [Provide Validation Reports](#provide-validation-reports)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
---

### Model Reports

When requested the system shall provide human readable and machine readable System model reports.

#### Details
All generated reports shall produce deterministic output with consistent ordering to enable reliable testing, version control, and reproducible builds.

The system shall ensure deterministic output by:

1. **Element Ordering**: Elements shall be sorted by identifier before iteration to ensure consistent processing order across all operations
2. **Relation Ordering**: Relations within each element shall be sorted by relation type name and then by target identifier before rendering
3. **Section Ordering**: Sections within files shall be sorted alphabetically when order is not semantically significant
4. **File Ordering**: Files within folders shall be sorted alphabetically

This determinism ensures that:
- Running the same operation multiple times produces byte-identical output
- Automated tests can reliably compare expected and actual outputs using simple diff tools without special normalization
- Version control diffs are meaningful and reflect actual changes rather than random ordering variations
- Continuous integration pipelines produce consistent, reproducible results

This requirement applies to all report operations, including:
- Model summary reporting - Summary reports in text and JSON formats
- Verification tracing - Upward traceability trees from verifications to requirements
- Coverage reporting - Verification coverage analysis
- Change impact analysis - Reports showing propagation of changes
- Validation reporting - Model validation error reports
- Linting - Model quality issue reports

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Provide Reports](../../UserStories.md#provide-reports)
---

### Model Structure and Summaries

When requested the system shall generate reports summarizing the structure and relationships in the System model, including counts and types of connections also supporting json and cypher output.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
---

### Containment View Report

The system shall provide a containment view report that displays the physical hierarchical structure of the model showing the containment relationships between folders, files, and elements.

#### Details
The containment view shows the **physical organization** of the model, complementing the existing relation-centric model view that shows derivedFrom/verifiedBy relationships.

The containment hierarchy represents:
- **Root folder** → **Subfolders** → **Files** → **Elements**
- Sections are skipped in this view (elements are shown directly under files)

This view helps users:
- Understand model organization and file structure
- Navigate the physical layout of specifications
- Identify where elements are located
- Visualize the containment structure in diagrams

The containment view must support multiple output formats (text, JSON, Mermaid diagram) and be included in HTML export.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
---

### Provide Validation Reports

The system shall generate detailed validation reports, highlighting any inconsistencies or errors in the System model structure.

#### Details
Validation shall be performed automatically when any command requires the parsed model, eliminating the need for a separate validation command. Commands that operate on raw files shall skip validation to allow operation on potentially invalid documents.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Align with Industry Standards](../../UserStories.md#align-with-industry-standards)
  * derivedFrom: [Model Reports](#model-reports)
---

### Verification Coverage Report

The system shall generate verification coverage reports focusing on leaf requirements (requirements that do not have forward relations to any other requirement), showing the percentage and details of verified and unverified leaf requirements, including breakdowns by file, section, and verification type.

#### Details
The verification coverage report shall provide:
- Total count of leaf requirements with breakdown by requirement type
- Count and percentage of verified leaf requirements (those with verifiedBy relations pointing to existing verification artifacts)
- Count and percentage of unverified leaf requirements
- Total count of verification artifacts with breakdown by verification type (test, analysis, inspection, demonstration)
- Count and percentage of satisfied test-verification artifacts (those with satisfiedBy relations pointing to existing test implementations)
- Count and percentage of orphaned verification artifacts (those without any verify relations to requirements)
- Detailed list of verified leaf requirements grouped by file and section
- Detailed list of unverified leaf requirements with impact analysis
- Detailed list of orphaned verifications (flagged for attention as they may be redundant or incorrectly configured)
- Output in both human-readable text and machine-readable JSON formats

The report helps track verification completeness and identify gaps in requirement verification coverage, supporting quality assurance and compliance activities.

**Coverage Philosophy:**
- **Leaf requirements** (requirements that don't derive other requirements) MUST be verified
- **Parent/intermediate requirements** MAY be verified but it's not a hard requirement as they might be covered in verification of leaf requirements
- One verification may verify multiple leaf requirements (N:1 relationship)
- The change impact analysis system propagates changes from parent requirements down to leaf requirements and their verifications
- System engineers/architects are responsible for ensuring verification scopes are broad enough to cover parent requirements when there's no dedicated parent verification
- AI systems can help create comprehensive verification scopes and prevent verification overlap

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
---

### Verification Upward Traceability

The system shall provide upward traceability visualization from verifications to root requirements, showing the complete requirement hierarchy and indicating which requirements are directly verified.

#### Details
**Used for identifying redundant verifications**: When a verification directly verifies both a leaf requirement and its parent requirement, this creates a redundant relation that adds noise into the model and may be removed from the parent - verifying the leaf requirement is sufficient since it traces upward to the parent. This keeps verification placement at the most specific level.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Verification Coverage Report](#verification-coverage-report)
---

