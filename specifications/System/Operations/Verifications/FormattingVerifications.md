# Elements

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
- System shall always output `# Elements` as the page header
- System shall add `## Elements` section header when elements exist without section header
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

**Attachment Formatting:**
- System shall format element identifier attachments with human-readable element names
- System shall look up actual element name from registry for attachment display names
- System shall NOT use identifier fragment as display name (e.g., NOT `[my-behavior]` but `[My Behavior]`)
- System shall preserve file attachment display names (filename)

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

7. **Attachment display name preservation**
   - Element identifier attachments use actual element name as display text
   - Format does NOT replace human-readable names with identifier fragments
   - File attachments preserve filename as display text

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Format Command](../../../Interfaces/CLI.md#format-command)
  * verify: [Document Structure Normalization](../Formatting.md#document-structure-normalization)
  * verify: [Structure and Addressing in Markdown Documents](../../Core/StructureAndParsing.md#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [test.sh](../../../../tests/test-advanced-format/test.sh)
---
