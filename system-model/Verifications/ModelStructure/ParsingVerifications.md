# Elements

### Model Parsing and Structure Verification Objective

This objective groups verification that Reqvire parses element structure, subsections, fragments, governance metadata, contracts, and specification files consistently.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Contract Element Type Parsing Test](#contract-element-type-parsing-test)
  * derive: [Contract Relations Rejection Test](#contract-relations-rejection-test)
  * derive: [Element Size Estimate Model Build Verification](#element-size-estimate-model-build-verification)
  * derive: [Element Subsection Parsing Test](#element-subsection-parsing-test)
  * derive: [Fragment Normalization Test](#fragment-normalization-test)
  * derive: [Non-Reserved Subsections Content Test](#non-reserved-subsections-content-test)
  * derive: [Requirement Governance Metadata Verification](#requirement-governance-metadata-verification)
  * derive: [Specification File Identification Test](#specification-file-identification-test)
  * derive: [In-Memory Model Build Cache Verification](#in-memory-model-build-cache-verification)
---

### Contract Element Type Parsing Test

This test verifies that the system parses Contract element types (constraint, behavior, specification, state, input-output) from metadata and that type-based search filters return the expected elements.

#### Details

##### Acceptance Criteria
- With only valid fixtures present, `reqvire validate` succeeds.
- `reqvire search --json` reports:
  - `Test Constraint Element` with type `constraint`
  - `Test Behavior Element` with type `behavior`
  - `Test Specification Element` with type `specification`
  - `Test State Element` with type `state`
  - `Test Input Output Element` with type `input-output`
- `reqvire search --filter-type=constraint --json` returns exactly 1 element.
- `reqvire search --filter-type=behavior --json` returns exactly 1 element.
- `reqvire search --filter-type=specification --json` returns exactly 1 element.

##### Test Criteria
1. Remove invalid contract fixture and run `reqvire validate`; assert exit code is 0.
2. Run `reqvire search --json`; assert the three contract elements have exact types `constraint`, `behavior`, `specification`.
3. Run `reqvire search --filter-type=constraint --json`; assert element count is 1.
4. Run `reqvire search --filter-type=behavior --json`; assert element count is 1.
5. Run `reqvire search --filter-type=specification --json`; assert element count is 1.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../tests/test-contract-elements/test.sh)
  * verify: [Contract Element Structure Constraints](../../ModelStructure/ModelManagement.md#contract-element-structure-constraints)
---

### Contract Relations Rejection Test

This test verifies that the system rejects Contract elements that include a Relations subsection during validation.

#### Details

##### Acceptance Criteria
- When an invalid `constraint` element includes a Relations subsection, `reqvire validate` fails (non-zero exit code).
- When an invalid `behavior` element includes a Relations subsection, `reqvire validate` fails (non-zero exit code).
- When an invalid `specification` element includes a Relations subsection, `reqvire validate` fails (non-zero exit code).
- Validation output contains at least one of: `constraint`, `contract`, or `relations`.

##### Test Criteria
1. Write an invalid `constraint` element containing a Relations subsection, run `reqvire validate`, and assert non-zero exit.
2. Write an invalid `behavior` element containing a Relations subsection, run `reqvire validate`, and assert non-zero exit.
3. Write an invalid `specification` element containing a Relations subsection, run `reqvire validate`, and assert non-zero exit.
4. Assert validation output mentions contract/type/relations context.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../tests/test-contract-elements/test.sh)
  * verify: [Contract Element Structure Constraints](../../ModelStructure/ModelManagement.md#contract-element-structure-constraints)
---

### Element Size Estimate Model Build Verification

This verification shall prove that element size estimates are computed only when model building explicitly enables them.

#### Details
Expected checks:
- Build a fixture model without size estimates and verify serialized model elements do not include `size_estimate`.
- Build the same fixture model with `with_size_estimates` enabled and verify each serialized element includes `size_estimate`.
- Verify `size_estimate` contains `content_bytes`, `rendered_context_bytes`, and `estimated_tokens`.
- Verify `rendered_context_bytes` is computed without recursively including the `size_estimate` field itself.
- Verify source Markdown files are not modified by size-estimate model building.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../tests/test-model-command/test.sh)
  * verify: [Opt-In Element Size Estimate Model Build](../../ModelStructure/ModelManagement.md#opt-in-element-size-estimate-model-build)
---

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
- System shall support all element types: capability, requirement, ontology, semantic-contract, verification, test-verification, formal-proof-verification, analysis-verification, inspection-verification, demonstration-verification, source, constraint, behavior, specification, state, input-output, other
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
  * satisfiedBy: [test.sh](../../../tests/test-parsing-functionality/test.sh)
  * verify: [Default Requirement Type Assignment](../../ModelStructure/ModelManagement.md#default-requirement-type-assignment)
  * verify: [Relation Types and behaviors](../../ModelStructure/ModelManagement.md#relation-types-and-behaviors)
  * verify: [Reserved Subsections Support](../../ModelStructure/StructureAndParsing.md#reserved-subsections-support)
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
- `"My Capability Name"` → `"my-capability-name"`
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
   - Test elements with punctuation: `"Capability (v2.0)"`
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
   - Create element `"My Capability Name"`
   - Reference it as `"My Capability Name"`, `"my capability name"`, `"MY CAPABILITY NAME"`
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
  * satisfiedBy: [test.sh](../../../tests/test-parsing-functionality/test.sh)
  * verify: [Element Identity Model](../../ModelStructure/StructureAndParsing.md#element-identity-model)
---

### Non-Reserved Subsections Content Test

This test verifies that non-reserved subsections (subsections other than Relations, Details, Metadata, Contract Bindings) are correctly included in the element's content field.

#### Details

##### Acceptance Criteria
**Non-Reserved Subsection Handling:**
- System shall include non-reserved subsection headers (e.g., `#### Test Steps`, `#### Expected Results`) in element content
- System shall include content following non-reserved subsection headers in element content
- Non-reserved subsection content shall NOT be moved to page content
- Non-reserved subsections shall behave like `#### Details` (content goes into element's content field)

**Reserved Subsection Behavior:**
- Reserved subsections (Relations, Metadata, Contract Bindings) shall NOT be included in element content
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
  * satisfiedBy: [test.sh](../../../tests/test-search-all-capabilities/test.sh)
  * verify: [Reserved Subsections Support](../../ModelStructure/StructureAndParsing.md#reserved-subsections-support)
---

### Requirement Governance Metadata Verification

This verification shall prove that requirement governance metadata is parsed, validated against the governance metadata contract, and exposed as effective model evidence without losing whether each value is explicit, inherited, or default.

#### Details
Expected checks:
- Create a requirement hierarchy with parent metadata values for `status`, `priority`, `risk`, and `owner`.
- Create child requirements that omit some governance metadata keys.
- Verify explicit child values override inherited parent values key by key.
- Verify omitted child values inherit from the nearest requirement ancestor.
- Verify omitted values with no ancestor metadata use the specification defaults.
- Verify model evidence distinguishes explicit, inherited, and default governance metadata values.
- Verify invalid enum values for `status`, `priority`, and `risk` are rejected with clear diagnostics naming the invalid key and accepted values.
- Verify `owner` accepts a free-form string.
- Verify inherited or default `status: approved` is not treated as explicit approval evidence.
- Verify non-governance-bearing elements that declare `status`, `priority`, `risk`, or `owner` metadata are rejected with clear diagnostics.
- Verify governance context for a contract element is resolved from its owning requirement through `define` / `definedBy`, not from metadata authored on the contract.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../tests/test-requirement-governance-metadata/test.sh)
  * verify: [Contract Element Structure Constraints](../../ModelStructure/ModelManagement.md#contract-element-structure-constraints)
  * verify: [Requirement Governance Metadata](../../ModelStructure/ModelManagement.md#requirement-governance-metadata)
---

### Specification File Identification Test

This test verifies that the system only parses markdown files where the first H1 heading is exactly `# Elements` or `# Element`, and silently ignores all other markdown files.

#### Details

##### Acceptance Criteria
**File Identification:**
- System shall parse markdown files where first H1 heading is `# Elements`
- System shall parse markdown files where first H1 heading is `# Element`
- System shall ignore markdown files where first H1 heading is not `# Elements` or `# Element`
- System shall ignore markdown files with no H1 heading
- Files without a supported model heading shall be silently skipped (no error)

**Leading Content Handling:**
- System shall allow blank lines before supported model headings
- System shall allow frontmatter (YAML between `---` markers) before supported model headings
- System shall allow HTML comments before supported model headings
- System shall check the first H1 heading encountered, ignoring non-heading content

**Backward Compatibility:**
- Files with different H1 headings (e.g., `# User Stories`, `# System Design`) shall be ignored
- This behavior applies in addition to `.gitignore` and `.reqvireignore` exclusions
- Page title/header is not stored in the model; multi-element files output as `# Elements`, and single-element files output as `# Element`

##### Test Criteria
1. **Valid specification file parsing:**
   - Create file with `# Elements` as first H1
   - Run reqvire search
   - Verify elements from file are in model

2. **Valid single-element file parsing:**
   - Create file with `# Element` as first H1
   - Run reqvire search
   - Verify the file contributes exactly one element to the model

3. **Invalid specification file skipping:**
   - Create file with different H1 (e.g., `# Other Title`)
   - Run reqvire search
   - Verify elements from file are NOT in model
   - Verify no error is reported

4. **No H1 heading:**
   - Create markdown file starting with `## Section` (no H1)
   - Run reqvire search
   - Verify file is ignored

5. **Leading blank lines:**
   - Create file with blank lines before `# Elements`
   - Run reqvire search
   - Verify file is parsed correctly

6. **Combined with ignore patterns:**
   - Create valid `# Elements` file matching .gitignore pattern
   - Verify file is still excluded by ignore pattern
   - Both checks must pass for file to be parsed

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../tests/test-gitignore-integration/test.sh)
  * verify: [Specification File Identification](../../ModelStructure/StructureAndParsing.md#specification-file-identification)
---


### In-Memory Model Build Cache Verification

This verification shall prove that the in-memory model build cache returns cached models on unchanged workspaces and rebuilds after content changes or CRUD invalidation.

#### Details

##### Acceptance Criteria
- Two consecutive `reqvire.read_element` (or `reqvire.search`) calls over an unchanged workspace return equal results, demonstrating a cache hit without re-parsing.
- Modifying, adding, or removing a `.md` file changes the workspace fingerprint and triggers a rebuild so the new content is reflected.
- After a CRUD write (e.g. `reqvire.add_element`), the cache is invalidated and the next read reflects the newly added element.
- Changing `with_size_estimates` or `lenient` build options produces a different cache key and an appropriately different model.
- Git-commit scan paths (`--git-commit`) do not use the cache.

##### Test Criteria
1. Start a `reqvire mcp` server against a fixture workspace.
2. Issue two identical `reqvire.read_element` (or `reqvire.search`) calls back-to-back; assert both return the same element set (cache hit, no re-parse).
3. Issue a `reqvire.add_element` CRUD call to add a new element; assert the call succeeds.
4. Issue another `reqvire.search`; assert the newly added element is present, proving `invalidate()` cleared the cache and forced a rebuild.
5. Modify, add, or remove a `.md` file and issue a read; assert the result reflects the change (fingerprint change forces rebuild).
6. Run a `reqvire change-impact --git-commit=<hash>` scan and confirm it does not consult the cache.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../tests/test-cache-integration/test.sh)
  * verify: [In-Memory Model Build Cache](../../ModelStructure/ModelManagement.md#in-memory-model-build-cache)
---
