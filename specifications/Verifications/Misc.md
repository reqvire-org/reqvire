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
   - output contains "Options:" section with `-h, --help`, `-V, --version`

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
  * verify: [Format Command](../SystemRequirements/Requirements.md#format-command)
  * verify: [Validate Command](../SystemRequirements/Requirements.md#validate-command)
  * verify: [HTML Export](../SystemRequirements/Requirements.md#html-export)
  * verify: [CLI Generate Diagrams Flag](../SystemRequirements/Requirements.md#cli-generate-diagrams-flag)
  * verify: [CLI Remove Diagrams Flag](../SystemRequirements/Requirements.md#cli-remove-diagrams-flag)
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
- System shall default to dry-run mode (preview changes without applying them)
- System shall require --fix flag to actually apply formatting changes to files
- System shall display changes in git diff style with line numbers and colors
- System shall show diff output in both preview mode and when applying changes with --fix
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
   - Default mode (no --fix flag) shows preview without making changes
   - --fix flag applies changes correctly
   - Format command shows diff output in both preview and --fix modes
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
  * verify: [Structure of Markdown Documents](../SpecificationsRequirements.md#structure-of-markdown-documents)
  * satisfiedBy: [test.sh](../../tests/test-advanced-format/test.sh)
---

### Serve Command Verification

This test verifies that the serve command exports HTML to a temporary directory and starts an HTTP server that serves the model documentation.

#### Details

##### Acceptance Criteria:
- System shall export HTML artifacts to a temporary directory
- System shall start HTTP server on specified host and port
- System shall display clickable terminal link to the server URL
- System shall serve index.html when accessing root URL
- System shall serve all exported HTML files with correct paths
- System shall serve static assets (SVG diagrams, CSS, etc.)
- System shall return 404 for non-existent files
- System shall set correct Content-Type headers for different file types
- System shall run in quiet mode (suppress verbose export output)
- System shall not automatically open browser window
- System shall display instructions for Ctrl-C stop

##### Test Criteria:
- Command starts successfully and displays server URL with instructions
- Server responds to HTTP requests on specified port
- Root URL (/) serves index.html
- HTML files are served with text/html content type
- SVG files are served with image/svg+xml content type
- Non-existent paths return 404 status
- Export verbose output is suppressed (quiet mode active)

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Serve Command](../SystemRequirements/Requirements.md#serve-command)
  * satisfiedBy: [test.sh](../../tests/test-serve-command/test.sh)
---

### Lint Command Verification

This test verifies that the lint command analyzes model quality, detects issues in requirements relations, and provides categorized output distinguishing between auto-fixable issues and those requiring manual review.

#### Details

##### Acceptance Criteria:
**Basic Command Functionality:**
- System shall provide lint command that analyzes model quality
- System shall default to dry-run mode (report issues without applying fixes)
- System shall support --fixable flag to show only auto-fixable issues
- System shall support --auditable flag to show only issues requiring manual review
- System shall support --fix flag to automatically apply fixes for auto-fixable issues
- System shall support --json flag for structured JSON output
- System shall show ALL issues when no filter flags are provided
- System shall exit with code 0 when no issues are found or fixes are successfully applied
- System shall exit with non-zero code on errors

**Redundant Verify Relations Detection:**
- System shall detect when a verification verifies both a leaf requirement and its ancestor
- System shall reuse verification trace tree logic for detection
- System shall report these as auto-fixable issues
- System shall show affected verification elements with file paths
- System shall list specific redundant verify relations
- System shall explain why relations are redundant

**Maybe-Redundant Hierarchical Relations Detection:**
- System shall detect when an element has derivedFrom to both a requirement and its ancestor
- System shall use virtual verification connected to all leaf requirements for detection
- System shall reuse trace tree building logic for consistency
- System shall report these as needs manual review issues
- System shall show affected elements with their derivedFrom relations
- System shall not suggest which relations to remove

**Output Formatting:**
- System shall categorize output into "Auto-fixable Issues" and "Needs Manual Review" sections
- Each issue shall include element identifier, file path, specific relations, and rationale
- JSON output shall include issue categorization, type, affected elements, relation details, and rationale
- System shall not suggest what to remove in output

**Auto-fix Capability:**
- System shall apply fixes only for auto-fixable issues when --fix is used
- System shall remove redundant verify relations from verification elements
- System shall remove safe redundant hierarchical derivation relations following single-chain constraint
- System shall NOT auto-remove hierarchical relations with multiple converging paths (requires manual review)
- System shall preserve all other content and formatting in files
- System shall report all changes made (files modified, relations removed)
- System shall not modify issues categorized as needs manual review

##### Test Criteria:
1. **Command execution**
   - Lint command runs successfully with default behavior
   - --fixable flag shows only auto-fixable issues
   - --auditable flag shows only manual review issues
   - --fix flag applies fixes for auto-fixable issues
   - --json flag produces structured JSON output

2. **Redundant verify detection**
   - Correctly identifies redundant verify relations using trace tree logic
   - Reports verification elements with redundant relations
   - Categorizes as auto-fixable

3. **Hierarchical redundancy detection**
   - Creates virtual verification connected to leaf requirements
   - Correctly identifies potentially redundant derivedFrom relations
   - Categorizes as needs manual review

4. **Output quality**
   - Clear categorization into auto-fixable and manual review sections
   - Complete information for each issue
   - JSON output has correct structure
   - No suggestions on what to remove

5. **Auto-fix functionality**
   - Removes redundant verify relations from markdown files
   - Removes safe redundant hierarchical derivation relations (single-chain only)
   - Does NOT auto-remove hierarchical relations with multiple converging paths
   - Preserves file content and formatting
   - Reports changes accurately
   - Does not modify manual review issues

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Redundant Verify Relations Detection](../SystemRequirements/Requirements.md#redundant-verify-relations-detection)
  * verify: [Safe Redundant Hierarchical Relations Auto-Removal](../SystemRequirements/Requirements.md#safe-redundant-hierarchical-relations-auto-removal)
  * verify: [Lint Output Formatting](../SystemRequirements/Requirements.md#lint-output-formatting)
  * verify: [Lint Auto-fix Capability](../SystemRequirements/Requirements.md#lint-auto-fix-capability)
---