# Miscellaneous Verifications

This document contains miscellaneous verification tests that don't fit into the other verification categories.

## Export and Context Verifications

### HTML Export Verification

This test verifies that the system exports specifications into HTML format with generated index and saves them in the designated output location.

#### Details

##### Acceptance Criteria:
- System should export specifications to HTML format
- HTML files should be saved in the designated output location
- HTML output should maintain the structure and content of the original specifications
- System shall generate index.md in the temporary working directory during HTML export
- index.md shall be converted to index.html in the output directory
- index.html shall contain links to all specification documents
- index.html shall be properly structured with sections
- index.html shall include brief summaries of each document
- index.html shall serve as the primary entry point for HTML documentation
- Links in diagrams and text must be converted to use .html instead of .md
- Paths in HTML files should maintain the original relative structure
- System should work in environments without Git repositories

##### Test Criteria:
- Command exits with success (0) return code
- HTML files are generated at the expected location with .html extension
- Output directory contains index.html file
- index.html contains links to all specification documents
- index.html structure follows expected format
- HTML content preserves the structure and information from the source files
- Links in HTML files use .html extension instead of .md
- Mermaid click links are properly converted from .md to .html
- Both GitHub-style URLs and direct file paths in mermaid click links are handled correctly
- Paths should not have duplicated folder names (e.g., specifications/specifications)

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Export HTML specifications](../UserRequirements.md#export-html-specifications)
  * verify: [Index Generation](../SystemRequirements/Requirements.md#index-generation)
  * satisfiedBy: [test.sh](../../tests/test-html-export/test.sh)
---

### CLI Help Structure Verification

This test verifies that the CLI help output displays all commands and their options flattened in the main help, and that all CLI command requirements are covered.

#### Details

##### Acceptance Criteria
- Running `reqvire` (without arguments) displays the main help output as the default command
- Main help output shows Usage line: `reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]`
- Main help lists all available commands under "Commands:" section
- Main help lists all global options under "Options:" section
- Command-specific options are displayed under dedicated headings (e.g., "SUMMARY OPTIONS:", "FORMAT OPTIONS:")
- All command options are flattened and visible in the main help without needing to drill down
- Help output includes nested subcommands expanded (e.g., "matrix", "traces", "coverage")
- Each command's help text is descriptive and clear

##### Test Criteria
1. **Main help output structure**
   Command: `reqvire`
   - exits code **0**
   - output contains "Usage: reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]"
   - output contains "Commands:" section
   - output contains "Options:" section with `-c, --config`, `-h, --help`, `-V, --version`

2. **Command listing completeness**
   - All CLI commands are listed: html, format, validate, generate-diagrams, remove-diagrams, summary, section-summary, change-impact, verifications
   - Nested commands are expanded: "matrix", "traces", "coverage"

3. **Options flattening - all command-specific option sections present**
   - FORMAT OPTIONS section visible
   - SUMMARY OPTIONS section visible
   - SECTION-SUMMARY OPTIONS section visible
   - CHANGE IMPACT OPTIONS section visible
   - VERIFICATIONS MATRIX OPTIONS section visible
   - VERIFICATIONS TRACES OPTIONS section visible
   - VERIFICATIONS COVERAGE OPTIONS section visible

4. **Help text quality**
   - Each command has descriptive help text
   - Options have clear descriptions

#### Metadata
  * type: test-verification

#### Relations
  * verify: [CLI Interface Structure](../SystemRequirements/Requirements.md#cli-interface-structure)
  * verify: [Format Command](../SystemRequirements/Requirements.md#format-command)
  * verify: [Validate Command](../SystemRequirements/Requirements.md#validate-command)
  * verify: [HTML Export](../SystemRequirements/Requirements.md#html-export)
  * verify: [CLI Generate Diagrams Flag](../SystemRequirements/Requirements.md#cli-generate-diagrams-flag)
  * verify: [CLI Remove Diagrams Flag](../SystemRequirements/Requirements.md#cli-remove-diagrams-flag)
  * verify: [CLI Summary Report Command](../SystemRequirements/Requirements.md#cli-summary-report-command)
  * verify: [CLI Sections Summary Command](../SystemRequirements/Requirements.md#cli-sections-summary-command)
  * verify: [CLI Change Impact Report Command](../SystemRequirements/Requirements.md#cli-change-impact-report-command)
  * verify: [CLI Matrix Command](../SystemRequirements/Requirements.md#cli-matrix-command)
  * verify: [CLI Traces Command](../SystemRequirements/Requirements.md#cli-traces-command)
  * verify: [CLI Coverage Command](../SystemRequirements/Requirements.md#cli-coverage-command)
  * satisfiedBy: [test.sh](../../tests/test-cli-help-structure/test.sh)
---

### Format Command Requirements Verification

This test verifies the format command requirements from SystemRequirements and UserRequirements, focusing on normalizing and standardizing MBSE models for consistency and readability.

#### Details

##### Acceptance Criteria
**Format Command Functionality:**
- System shall provide format command that normalizes and standardizes markdown documents
- System shall support --dry-run flag to preview changes without applying them
- System shall display changes in git diff style with line numbers and colors
- System shall show diff output when applying formatting changes (not only in dry-run mode)
- System shall support --json flag for structured output of formatting results
- System shall preserve all document content while improving formatting consistency

**Content Preservation:**
- System shall preserve personas sections and other non-element content
- System shall maintain element ordering within sections
- System shall preserve section ordering throughout documents
- System shall preserve page content (frontmatter before first section)
- System shall preserve section content (content between section headers and first element)

**Formatting Consistency:**
- System shall trim excess whitespace from lines
- System shall normalize line endings consistently
- System shall insert proper separators between elements
- System shall normalize consecutive separators to single separators
- System shall normalize relation indentation to proper 2-space format
- System shall format relation links with human-readable names
- System shall clean up file references to show filename only for implementation files

**Document Structure Normalization:**
- System shall add level 1 page header (based on filename) when document lacks page header
- System shall add `## Requirements` section header when elements exist without section header
- System shall preserve existing page headers (starting with `# `)
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
   - Dry-run mode shows preview without making changes
   - Actual format command applies changes correctly
   - Format command shows diff output when applying changes (not only in dry-run)
   - JSON flag produces structured output with formatting results

2. **Content preservation verification**
   - Personas sections remain intact after formatting
   - Element content and structure preserved
   - Section ordering maintained correctly
   - Page and section content preserved

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
  * verify: [Format Command](../SystemRequirements/Requirements.md#format-command)
  * verify: [Document Structure Normalization](../SystemRequirements/Requirements.md#document-structure-normalization)
  * verify: [Model Formatting](../UserRequirements.md#model-formatting)
  * verify: [Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * verify: [Formatting Output](../UserRequirements.md#formatting-output)
  * verify: [Structure of Markdown Documents](../SpecificationsRequirements.md#structure-of-markdown-documents)
  * satisfiedBy: [test.sh](../../tests/test-advanced-format/test.sh)
---
