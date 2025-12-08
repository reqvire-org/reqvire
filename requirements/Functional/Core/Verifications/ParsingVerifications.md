# Elements

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
- System shall support all element types: requirement, user-requirement, verification, test-verification, analysis-verification, inspection-verification, demonstration-verification, constraint, behavior, specification, other
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
  * satisfiedBy: [test.sh](../../../../tests/test-parsing-functionality/test.sh)
  * verify: [Default Requirement Type Assignment](../ModelManagement.md#default-requirement-type-assignment)
  * verify: [Relation Types and behaviors](../ModelManagement.md#relation-types-and-behaviors)
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
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
  * satisfiedBy: [test.sh](../../../../tests/test-parsing-functionality/test.sh)
  * verify: [Element Identity Model](../StructureAndParsing.md#element-identity-model)
---

### Non-Reserved Subsections Content Test

This test verifies that non-reserved subsections (subsections other than Relations, Details, Metadata, Attachments) are correctly included in the element's content field.

#### Details

##### Acceptance Criteria
**Non-Reserved Subsection Handling:**
- System shall include non-reserved subsection headers (e.g., `#### Test Steps`, `#### Expected Results`) in element content
- System shall include content following non-reserved subsection headers in element content
- Non-reserved subsection content shall NOT be moved to page content
- Non-reserved subsections shall behave like `#### Details` (content goes into element's content field)

**Reserved Subsection Behavior:**
- Reserved subsections (Relations, Metadata, Attachments) shall NOT be included in element content
- `#### Details` subsection header and its content shall be included in element content

**Format Command:**
- Format command shall preserve non-reserved subsections within their parent element
- Format command shall NOT move non-reserved subsection content to page level

##### Test Criteria
1. **Non-reserved subsection parsing:**
   - Create element with `#### Test Steps` and `#### Expected Results` subsections
   - Run reqvire search --json
   - Verify element content includes both subsection headers and their content

2. **Page content exclusion:**
   - Create element with non-reserved subsections
   - Run reqvire search --json
   - Verify page_content does NOT contain non-reserved subsection content

3. **Format preservation:**
   - Run reqvire format on file with non-reserved subsections
   - Verify format does not propose moving subsection content to page level
   - Run reqvire format --fix
   - Verify subsections remain under their parent element

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-search-all-features/test.sh)
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
---

### Refinement Element Type Parsing Test

This test verifies that the system correctly parses Refinement element types (constraint, behavior, specification) from metadata and handles their structural constraints.

#### Details

##### Acceptance Criteria
**Type Parsing:**
- System shall parse `type: constraint` as Refinement type Constraint
- System shall parse `type: behavior` as Refinement type Behavior
- System shall parse `type: specification` as Refinement type Specification
- Refinement types shall be stored distinctly from other element types
- Refinement types shall display as `"constraint"`, `"behavior"`, `"specification"` in output

**Search and Filtering:**
- System shall support filtering by `--filter-type="constraint"`
- System shall support filtering by `--filter-type="behavior"`
- System shall support filtering by `--filter-type="specification"`

##### Test Criteria
1. **Constraint type parsing:**
   - Create element with `type: constraint` metadata
   - Query model via JSON output
   - Verify `element_type` field is `"constraint"`

2. **Behavior type parsing:**
   - Create element with `type: behavior` metadata
   - Query model via JSON output
   - Verify `element_type` field is `"behavior"`

3. **Specification type parsing:**
   - Create element with `type: specification` metadata
   - Query model via JSON output
   - Verify `element_type` field is `"specification"`

4. **Search filtering:**
   - Create elements of all three Refinement types
   - Test `--filter-type="constraint"` returns only constraint elements
   - Test `--filter-type="behavior"` returns only behavior elements
   - Test `--filter-type="specification"` returns only specification elements

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Refinement Element Structure Constraints](../ModelManagement.md#refinement-element-structure-constraints)
---

### Refinement Relations Rejection Test

This test verifies that the system rejects Refinement elements that include a Relations subsection during validation.

#### Details

##### Acceptance Criteria
**Validation Rejection:**
- System shall report validation error when Refinement element has Relations subsection
- Error message shall indicate that Refinement elements cannot have relations
- Error shall include element name and file location
- Validation shall fail (non-zero exit code) when this error is detected

**Valid Refinement Elements:**
- Refinement elements without Relations subsection shall pass validation
- Refinement elements with Metadata and Details subsections shall pass validation
- Refinement elements with Attachments subsection shall pass validation

##### Test Criteria
1. **Relations rejection for constraint:**
   - Create constraint element with Relations subsection
   - Run reqvire validate
   - Verify validation fails with appropriate error message

2. **Relations rejection for behavior:**
   - Create behavior element with Relations subsection
   - Run reqvire validate
   - Verify validation fails with appropriate error message

3. **Relations rejection for specification:**
   - Create specification element with Relations subsection
   - Run reqvire validate
   - Verify validation fails with appropriate error message

4. **Valid Refinement elements:**
   - Create Refinement elements without Relations (only Metadata, Details, Attachments)
   - Run reqvire validate
   - Verify validation passes

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Refinement Element Structure Constraints](../ModelManagement.md#refinement-element-structure-constraints)
---

### Specification File Identification Test

This test verifies that the system only parses markdown files where the first H1 heading is exactly `# Elements`, and silently ignores all other markdown files.

#### Details

##### Acceptance Criteria
**File Identification:**
- System shall parse markdown files where first H1 heading is `# Elements`
- System shall ignore markdown files where first H1 heading is not `# Elements`
- System shall ignore markdown files with no H1 heading
- Files without `# Elements` heading shall be silently skipped (no error)

**Leading Content Handling:**
- System shall allow blank lines before `# Elements` heading
- System shall allow frontmatter (YAML between `---` markers) before `# Elements` heading
- System shall allow HTML comments before `# Elements` heading
- System shall check the first H1 heading encountered, ignoring non-heading content

**Backward Compatibility:**
- Files with different H1 headings (e.g., `# User Stories`, `# System Design`) shall be ignored
- This behavior applies in addition to `.gitignore` and `.reqvireignore` exclusions
- Page title/header is not stored in the model (always output as `# Elements`)

##### Test Criteria
1. **Valid specification file parsing:**
   - Create file with `# Elements` as first H1
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
   - Create file with blank lines before `# Elements`
   - Run reqvire search
   - Verify file is parsed correctly

5. **Combined with ignore patterns:**
   - Create valid `# Elements` file matching .gitignore pattern
   - Verify file is still excluded by ignore pattern
   - Both checks must pass for file to be parsed

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-gitignore-integration/test.sh)
  * verify: [Specification File Identification](../StructureAndParsing.md#specification-file-identification)
---
