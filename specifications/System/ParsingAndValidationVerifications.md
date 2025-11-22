# Requirements

### Element Subsection Parsing Test

This test verifies that the system correctly extracts and parses element subsections (Metadata, Relations, Details) and element content from markdown documents.

#### Details

##### Acceptance Criteria
**Subsection Extraction:**
- System shall identify and extract Metadata subsection (level 4 heading)
- System shall identify and extract Relations subsection (level 4 heading)
- System shall identify and extract Details subsection (level 4 heading) when present
- System shall parse element content (text before first subsection)
- System shall exclude subsection headers and content from main element content

**Metadata Parsing:**
- System shall extract element type from `* type:` metadata entry
- System shall support all element types: requirement, user-requirement, verification, test-verification, analysis-verification, inspection-verification, demonstration-verification, other
- System shall assign default type 'requirement' when no type metadata present

**Relations Parsing:**
- System shall extract relation type (derivedFrom, verifiedBy, verify, satisfiedBy)
- System shall extract relation target (element identifier with file path and fragment)
- System shall normalize target fragment identifiers
- System shall support multiple relations of same or different types
- System shall validate relation targets exist in model

**Content Extraction:**
- System shall extract element description text before subsections
- System shall preserve markdown formatting in content
- System shall NOT include subsection headers in content
- System shall NOT include subsection body text in content

**Details Subsection:**
- System shall extract Details subsection content when present
- System shall preserve multi-paragraph Details content
- System shall store Details separately from main content

##### Test Criteria
1. **Metadata subsection parsing:**
   - Create elements with various element types in Metadata
   - Query model via JSON output
   - Verify `element_type` field matches metadata
   - Test all supported element types

2. **Relations subsection parsing:**
   - Create elements with multiple relations
   - Query model via JSON output
   - Verify `relations` array contains all relations
   - Verify each relation has `relation_type` and `target` fields
   - Verify target fragments are normalized

3. **Content extraction:**
   - Create element with description text before subsections
   - Query model via JSON output
   - Verify `content` field contains description
   - Verify content does NOT include subsection headers
   - Verify content does NOT include metadata or relations

4. **Details subsection parsing:**
   - Create element with Details subsection
   - Query model via JSON output
   - Verify `details` field is populated
   - Verify details content is separate from main content
   - Test multi-paragraph details

5. **JSON structure validation:**
   - Verify JSON output contains `elements` array
   - Verify each element has required fields: `element_id`, `name`, `file_path`, `section`, `element_type`, `content`
   - Verify optional fields present when applicable: `details`, `relations`, `metadata`

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Reserved Subsections Support](Subsections.md#reserved-subsections-support)
  * verify: [Supported Element Types](ModelManagement.md#supported-element-types)
  * verify: [Relation Types and behaviors](ModelManagement.md#relation-types-and-behaviors)
  * verify: [Default Requirement Type Assignment](ModelManagement.md#default-requirement-type-assignment)
  * satisfiedBy: [test.sh](../../tests/test-parsing-functionality/test.sh)
---

### Specification File Identification Test

This test verifies that the system only parses markdown files where the first H1 heading is exactly `# Requirements`, and silently ignores all other markdown files.

#### Details

##### Acceptance Criteria
**File Identification:**
- System shall parse markdown files where first H1 heading is `# Requirements`
- System shall ignore markdown files where first H1 heading is not `# Requirements`
- System shall ignore markdown files with no H1 heading
- Files without `# Requirements` heading shall be silently skipped (no error)

**Leading Content Handling:**
- System shall allow blank lines before `# Requirements` heading
- System shall allow frontmatter (YAML between `---` markers) before `# Requirements` heading
- System shall allow HTML comments before `# Requirements` heading
- System shall check the first H1 heading encountered, ignoring non-heading content

**Backward Compatibility:**
- Files with different H1 headings (e.g., `# User Stories`, `# System Design`) shall be ignored
- This behavior applies in addition to `.gitignore` and `.reqvireignore` exclusions
- Page title/header is not stored in the model (always output as `# Requirements`)

##### Test Criteria
1. **Valid specification file parsing:**
   - Create file with `# Requirements` as first H1
   - Run reqvire search
   - Verify elements from file are in model

2. **Invalid specification file skipping:**
   - Create file with different H1 (e.g., `# Other Title`)
   - Run reqvire search
   - Verify elements from file are NOT in model
   - Verify no error is reported

3. **No H1 heading:**
   - Create markdown file starting with `## Section` (no H1)
   - Run reqvire search
   - Verify file is ignored

4. **Leading blank lines:**
   - Create file with blank lines before `# Requirements`
   - Run reqvire search
   - Verify file is parsed correctly

5. **Combined with ignore patterns:**
   - Create valid `# Requirements` file matching .gitignore pattern
   - Verify file is still excluded by ignore pattern
   - Both checks must pass for file to be parsed

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Specification File Identification](StructureAndParsing.md#specification-file-identification)
  * satisfiedBy: [test.sh](../../tests/test-gitignore-integration/test.sh)
---

### Fragment Normalization Test

This test verifies that the system correctly normalizes element name fragments according to GitHub's fragment identifier rules for use in Element IDs and cross-references.

#### Details

##### Acceptance Criteria
**GitHub Fragment Normalization Rules:**
- System shall convert all letters to lowercase
- System shall replace spaces with hyphens (-)
- System shall remove all punctuation characters except hyphens and underscores
- System shall remove all whitespace characters except spaces (which become hyphens)
- System shall trim leading and trailing whitespace before processing
- System shall preserve alphanumeric characters, hyphens, and underscores

**Normalization Examples:**
- `"My Feature Name"` → `"my-feature-name"`
- `"Version 1.2.3"` → `"version-123"` (dots removed)
- `"Installation (Windows)"` → `"installation-windows"` (parentheses removed)
- `"C++ API Reference"` → `"c-api-reference"` (plus signs removed)
- `"my_variable_name"` → `"my_variable_name"` (underscores preserved)
- `"Multiple    Spaces"` → `"multiple----spaces"` (each space becomes hyphen)

**Element ID Generation:**
- System shall use normalized fragments to generate Element IDs
- Element IDs shall be stable across element relocations
- Element IDs shall be globally unique within the model

**Cross-Reference Resolution:**
- System shall normalize fragment portions of identifiers during relation resolution
- System shall match elements using normalized fragments
- System shall handle case-insensitive element name lookups

##### Test Criteria
1. **Basic normalization verification:**
   - Create elements with various naming patterns
   - Verify Element IDs use normalized fragments
   - Test lowercase conversion
   - Test space-to-hyphen conversion
   - Test punctuation removal

2. **Special character handling:**
   - Test elements with punctuation: `"Feature (v2.0)"`
   - Test elements with symbols: `"C++ API"`
   - Test elements with dots: `"Version 1.2.3"`
   - Verify all punctuation is removed correctly

3. **Underscore and hyphen preservation:**
   - Test elements with underscores: `"my_variable_name"`
   - Test elements with hyphens: `"pre-release-build"`
   - Verify both are preserved in normalized form

4. **Whitespace handling:**
   - Test multiple consecutive spaces: `"Multiple    Spaces"`
   - Test leading/trailing spaces: `"  Trimmed  "`
   - Verify each space becomes a hyphen
   - Verify trim operation works correctly

5. **Cross-reference resolution:**
   - Create element `"My Feature Name"`
   - Reference it as `"My Feature Name"`, `"my feature name"`, `"MY FEATURE NAME"`
   - Verify all variants resolve to same element
   - Verify relations are established correctly

6. **Element ID stability:**
   - Rename element markdown file (relocation)
   - Verify Element ID remains unchanged (uses normalized name)
   - Verify cross-references continue to work
   - Verify change detection identifies as relocation, not new element

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Element Identity Model](StructureAndParsing.md#element-identity-model)
  * satisfiedBy: [test.sh](../../tests/test-parsing-functionality/test.sh)
---
