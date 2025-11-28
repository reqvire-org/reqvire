# Elements

### Element Ordering Verification

This test verifies that the format command reorders elements following the Element Ordering Behavior, ensuring parent elements appear before their children based on file-local derivedFrom hierarchy.

#### Details

##### Acceptance Criteria
**Parent-Child Ordering:**
- Elements with file-local derivedFrom relations shall be ordered with parents before children
- Grandchildren shall appear after their parent elements
- The ordering shall be applied recursively through the entire hierarchy

**Sibling Alphabetical Ordering:**
- Elements at the same hierarchy level (siblings) shall be sorted alphabetically
- Root elements (no file-local parent) shall be sorted alphabetically among themselves
- Children of the same parent shall be sorted alphabetically

**Hierarchy Grouping:**
- Elements shall be grouped by their file-local parent chains
- Each parent's subtree shall be kept together (parent, then all descendants)
- Cross-file derivedFrom relations shall not affect ordering

##### Test Criteria
1. **Basic parent-child ordering**
   - Create file with child element before parent element
   - Run format command
   - Verify parent appears before child in output

2. **Multi-level hierarchy**
   - Create file with grandchild, child, parent in wrong order
   - Run format command
   - Verify order: parent → child → grandchild

3. **Sibling alphabetical sorting**
   - Create file with siblings "Child Z" and "Child A" under same parent
   - Run format command
   - Verify order: parent → Child A → Child Z

4. **Multiple root elements**
   - Create file with multiple root elements (no file-local parents) in wrong alphabetical order
   - Run format command
   - Verify roots are sorted alphabetically

5. **Mixed hierarchy and standalone**
   - Create file with hierarchy group and standalone elements
   - Run format command
   - Verify hierarchy groups are kept together and all roots sorted alphabetically

6. **Cross-file relations ignored**
   - Create file where element derives from element in different file
   - Run format command
   - Verify cross-file derivedFrom does not affect local ordering

#### Metadata
  * type: test-verification

#### Attachments
  * [Element Ordering Behavior](../Refinements.md#element-ordering-behavior)

#### Relations
  * verify: [Element Ordering Normalization](../Formatting.md#element-ordering-normalization)
  * verify: [Element Manipulation File Persistence](../ElementManipulation.md#element-manipulation-file-persistence)
  * verify: [Format Command](../../../Interfaces/CLI.md#format-command)
  * satisfiedBy: [test.sh](../../../../tests/test-element-ordering/test.sh)
---

### Format Command Requirements Verification

This test verifies the format command requirements from SystemRequirements and UserRequirements, focusing on normalizing and standardizing System models for consistency and readability.

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

### Relation Ordering Verification

This test verifies that the format command sorts relations alphabetically for deterministic output.

#### Details

##### Acceptance Criteria
**Primary Sort by Relation Type:**
- Relations shall be sorted alphabetically by relation type name
- derivedFrom shall appear before satisfiedBy
- satisfy shall appear before trace
- trace shall appear before verifiedBy

**Secondary Sort by Target Identifier:**
- Within the same relation type, relations shall be sorted alphabetically by target identifier
- For example: `derivedFrom: A` shall appear before `derivedFrom: B`

**Deterministic Output:**
- Same input shall always produce same output
- Output shall not depend on HashMap iteration order or parsing order

##### Test Criteria
1. **Relation type alphabetical ordering**
   - Create element with relations in non-alphabetical type order
   - Run format --fix
   - Verify relations are sorted by type name alphabetically

2. **Same-type relation ordering**
   - Create element with multiple relations of same type to different targets
   - Run format --fix
   - Verify relations of same type are sorted by target identifier

3. **Deterministic output across runs**
   - Run format --fix multiple times
   - Verify output is identical each time

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Relation Ordering Normalization](../Formatting.md#relation-ordering-normalization)
  * satisfiedBy: [test.sh](../../../../tests/test-advanced-format/test.sh)
---

### Full Relations Insertion Verification

This test verifies that the --with-full-relations flag correctly inserts all registered relations (user-created and auto-generated) into elements during formatting.

#### Details

##### Acceptance Criteria
**Auto-Generated Relations Insertion:**
- System shall insert inverse relations when --with-full-relations is provided
- satisfiedBy relations generated from satisfy shall be inserted
- derive relations generated from derivedFrom shall be inserted
- verify relations generated from verifiedBy shall be inserted
- tracedFrom relations generated from trace shall be inserted

**Relation Ordering:**
- All relations (user-created and auto-generated) shall be sorted according to the Relation Ordering Normalization requirement
- Relations shall be sorted alphabetically by type, then by target identifier

**Idempotency:**
- Running format --with-full-relations twice shall produce same output
- Already present relations shall not be duplicated

**Selective Behavior:**
- Without --with-full-relations flag, only user-created relations are persisted
- With --with-full-relations flag, all relations from registry are persisted

##### Test Criteria
1. **Basic inverse relation insertion**
   - Create element A with satisfy relation to element B
   - Run format --with-full-relations --fix
   - Verify element B now has satisfiedBy relation to element A

2. **Multiple inverse relation types**
   - Create elements with derivedFrom, satisfiedBy, verifiedBy, trace relations
   - Run format --with-full-relations --fix
   - Verify all inverse relations (derive, satisfy, verify, tracedFrom) are inserted

3. **No duplication**
   - Run format --with-full-relations --fix twice
   - Verify relations are not duplicated

4. **Default behavior unchanged**
   - Create element with satisfy relation
   - Run format --fix (without --with-full-relations)
   - Verify inverse relations are NOT inserted

5. **Preview mode**
   - Run format --with-full-relations (without --fix)
   - Verify diff shows proposed relation insertions
   - Verify files are not modified

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Full Relations Insertion](../Formatting.md#full-relations-insertion)
  * verify: [Format Command](../../../Interfaces/CLI.md#format-command)
  * satisfiedBy: [test.sh](../../../../tests/test-format-full-relations/test.sh)
---
