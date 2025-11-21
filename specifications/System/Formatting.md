# Requirements

### Model Formatting

The system shall provide formatting capabilities to normalize and standardize MBSE models for consistency and readability.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../UserStories.md#validating-structures)
---

### Formatting Output

The system shall display formatting changes suggestion in similar manner as git diffs.

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
---

### Format Consistency Enforcement

The system shall provide formatting capability to ensure consistent formatting in requirements documents.

#### Details
The system shall implement the following formatting fixes:

**Excess Whitespace:**
- Detect and fix excess whitespace after element headers, subsection headers, and relation identifiers
- Maintain consistent formatting across all requirements documents

**Inconsistent Newlines:**
- Detect and fix excess or missing newlines before element headers and subsection headers
- Normalize to exactly two newlines before subsections (e.g., "#### Details")
- Maintain consistent formatting across all requirements documents

**Missing Separators:**
- Detect consecutive element sections that lack a separator line (---) between them
- Insert the separator to maintain consistent visual separation in the documentation
- Automatically insert separator lines between elements if not already present
- Normalize consecutive separators to single separators

**Reserved Subsections:**
- Identify and fix inconsistent indentation and bullet types in relation lists and other reserved subsections
- Ensure consistent indentation in relation lists (2-space format)
- Normalize relation entries to proper 2-space indentation format
- Standardize to a consistent format across all requirements documents

**Output Formatting:**
- Display changes with sequential line numbering that reflects final file positions
- Provide context lines with proper line number continuity

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * derivedFrom: [Align with Industry Standards](../UserStories.md#align-with-industry-standards)
  * satisfiedBy: [format.rs](../../core/src/format.rs)
  * verifiedBy: [Format Command Requirements Verification](#format-command-requirements-verification)
---

### Format Command Requirements Verification

This test verifies the format command requirements from SystemRequirements and UserRequirements, focusing on normalizing and standardizing MBSE models for consistency and readability.

#### Details

##### Acceptance Criteria
**Format Command Functionality:**
- System shall provide format command that normalizes and standardizes markdown documents
- System shall default to dry-run mode (preview changes without applying them)
- System shall require --fix flag to actually apply formatting changes to files
- System shall display changes in git diff style with line numbers and colors
- System shall show diff output in both preview mode and when applying changes with --fix
- System shall support --json flag for structured output of formatting results
- System shall preserve all document content while improving formatting consistency

**Content Preservation:**
- System shall preserve personas sections and other non-element content
- System shall maintain element ordering within files
- System shall preserve page content (frontmatter before first element)

**Formatting Consistency:**
- System shall trim excess whitespace from lines
- System shall normalize line endings consistently
- System shall insert proper separators between elements
- System shall normalize consecutive separators to single separators
- System shall normalize relation indentation to proper 2-space format
- System shall format relation links with human-readable names
- System shall clean up file references to show filename only for implementation files

**Document Structure Normalization:**
- System shall always output `# Requirements` as the page header
- System shall add `## Requirements` section header when elements exist without section header
- System shall preserve existing section headers (starting with `## `)
- System shall correctly distinguish level 1 headers from level 2+ headers

**Change Preview Quality:**
- System shall show file identification clearly in change output
- System shall display line references with consistent width based on maximum line number

**Known Limitations:**
- Diff output may not correctly display blank line additions in all cases (blank lines shown with ␤ character may have incorrect line numbering or positioning)
- System shall visualize trailing whitespace removal with special characters
- System shall use colors to distinguish additions (green) and removals (red)
- System shall group changes by file with clear separators
- System shall only show lines that have changes, omitting unchanged content
- System shall provide context lines before and after changes for better readability
- System shall maintain sequential line numbering that reflects final file positions
- System shall ensure line number continuity throughout diff output

**Relation Link Enhancement:**
- System shall convert simple identifiers (non-markdown format) to proper markdown link format
- System shall convert absolute links to relative links where appropriate
- System shall preserve already correct relative links without modification
- System shall replace fragment-only same-file references with full element names
- System shall convert implementation file paths to clean filename references
- System shall preserve external URLs without modification

##### Test Criteria
1. **Basic format functionality**
   - Format command runs successfully on test markdown files
   - Default mode (no --fix flag) shows preview without making changes
   - --fix flag applies changes correctly
   - Format command shows diff output in both preview and --fix modes
   - JSON flag produces structured output with formatting results

2. **Content preservation verification**
   - Personas sections remain intact after formatting
   - Element content and structure preserved
   - Element ordering maintained correctly within files
   - Page content preserved

3. **Change detection and preview**
   - Changes are clearly identified by file
   - Line references include consistent number width
   - Trailing whitespace removal is visualized
   - Color coding distinguishes addition/removal changes
   - Context lines provide readable context around changes
   - Line numbering maintains sequential continuity reflecting final file positions

4. **Relation link quality**
   - Simple identifiers are converted to proper markdown link format
   - Absolute links are converted to relative links where appropriate
   - Already correct relative links remain unchanged
   - Same-file references use human-readable element names
   - Implementation file references show clean filenames
   - Fragment references use proper notation

5. **Formatting consistency**
   - Excess whitespace is trimmed appropriately
   - Separators are inserted correctly between elements
   - Consecutive separators are normalized to single separators
   - Relation indentation is normalized to proper 2-space format
   - Line endings are normalized

6. **Line numbering accuracy**
   - Line numbers in diff output are sequential and consistent
   - Context lines maintain proper numbering continuity
   - Added lines show correct position in final file
   - Line numbering reflects final file structure accurately

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Format Command](../Interfaces/CLI.md#format-command)
  * verify: [Document Structure Normalization](#document-structure-normalization)
  * verify: [Structure And Addressing In Markdown Documents](StructureAndParsing.md#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [test.sh](../../tests/test-advanced-format/test.sh)
---

### Replace Absolute Links with Relative Links

The system shall replace absolute links with relative links, where applicable and contextually appropriate, to conform to repository standards and enhance portability.

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * verifiedBy: [Format Command Requirements Verification](#format-command-requirements-verification)
---

### Git-Style Diff Output for Format

The system shall display formatting change suggestions in a git-style diff format, color-coded when possible, to clearly show what modifications will be or have been made to the documents.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Formatting Output](#formatting-output)
  * satisfiedBy: [format.rs](../../core/src/format.rs)
  * satisfiedBy: [diff.rs](../../core/src/diff.rs)
  * verifiedBy: [Format Command Requirements Verification](#format-command-requirements-verification)
---

### File Pattern Exclusion for Format

The system shall respect configured excluded filename patterns when performing formatting operations, ensuring that files intentionally excluded from processing do not receive inappropriate formatting suggestions.

#### Relations
  * derivedFrom: [Ignoring Unstructured Documents](Configuration.md#ignoring-unstructured-documents)
  * satisfiedBy: [utils.rs](../../core/src/utils.rs)
---

### Document Structure Normalization

When generating formatted output, the system shall ensure all documents follow a consistent hierarchical structure.

#### Details
When generating formatted output, the system shall:
- Always output `# Requirements` as the page header (all specification files must have this header)
- Add a default section header `## Requirements` when elements exist without an explicit section header
- Preserve existing section headers when present (starting with `## `)
- Correctly distinguish between level 1 headers (`# `) and level 2 or deeper headers (`##`, `###`)

**Default Header Names:**
- Page header: Always `# Requirements` (required for all specification files)
- Section header: "Requirements" (the default section name used by parser)

**Normalization Rules:**
1. If document has `# Requirements` then `###` (no `##`): Add section header only
2. If document has `# Requirements` and `##`: No header additions needed

#### Relations
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
---

