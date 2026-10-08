# Elements

### Model Parsing and Structure Verification Objective

This objective groups verification that Reqvire parses element structure, subsections, fragments, governance metadata, contracts, and specification files consistently, and reuses completed model builds with correct source freshness and publication.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Contract Element Type Parsing Test](#contract-element-type-parsing-test)
  * derive: [Contract Relations Rejection Test](#contract-relations-rejection-test)
  * derive: [Element Size Estimate Model Build Verification](#element-size-estimate-model-build-verification)
  * derive: [Element Subsection Parsing Test](#element-subsection-parsing-test)
  * derive: [Fragment Normalization Test](#fragment-normalization-test)
  * derive: [In-Memory Model Build Cache Verification](#in-memory-model-build-cache-verification)
  * derive: [Non-Reserved Subsections Content Test](#non-reserved-subsections-content-test)
  * derive: [Requirement Governance Metadata Verification](#requirement-governance-metadata-verification)
  * derive: [Specification File Identification Test](#specification-file-identification-test)
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

### Canonical Graph Storage Verification

Verify bounded canonical graph storage, current target resolution, and isolation of mutation candidates while preserving change-impact report behavior.

#### Details
- Rebuild cyclic and reconvergent helper graphs repeatedly, including size-estimate rebuilding and reversed registration order. Count serialized canonical element payloads and adjacency edges; counts remain equal to the fixture's vertices and retained edges without nested target payloads.
- Normalize relative identifiers and propagate inverse relations before building adjacency. Repeating relation-context preparation produces the same edges and retains authored/generated relation provenance.
- Clone an accepted graph, then edit, rename, move, relink, and remove candidate targets. Assert current target content and locations in reports, preserved report JSON fields and cycle guards, and unchanged accepted graph data.
- Run the existing change-impact and CRUD E2E golden comparisons for capability and contract propagation, element relocation, relation consistency, relinking, and cross-file moves.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Model Parsing and Structure Verification Objective](#model-parsing-and-structure-verification-objective)
  * verify: [Canonical Graph Storage](../../ModelStructure/ModelManagement.md#canonical-graph-storage)
  * satisfiedBy: [graph_registry.rs](../../../crates/reqvire-core/src/graph_registry.rs)
  * satisfiedBy: [test.sh](../../../tests/test-capability-change-impact/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-change-impact-contract-bindings/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-change-impact-detection/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-change-impact-element-relocation/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-crud-relation-consistency/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-relink-command/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-mv-cross-file-relation-integrity/test.sh)
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

### In-Memory Model Build Cache Verification

This verification checks read-result consistency and mutation visibility through the cached model-loading path. The dedicated cache input/freshness and publication verifications establish whether reuse and rebuild coordination actually occurred.

#### Details

##### Acceptance Criteria
- Two consecutive `reqvire.read_element` calls over an unchanged workspace return the same requested element. Equal payloads establish result consistency only, not a cache hit or absence of parsing.
- After a successful persisted `reqvire.add_element` with automatic commits disabled, a subsequent search exposes the added element while HEAD and index remain unchanged.
- In read-only MCP mode, after a direct Markdown edit adds an element, a subsequent search exposes that element. Mutation-enabled sessions retain their accepted snapshot until an accepted mutation or restart.
- A standalone `change-impact --git-commit` invocation completes against the fixture history. A separate instrumented core check establishes actual cache bypass.

##### Test Criteria
1. Start a mutation-enabled `reqvire mcp` server against a clean committed fixture workspace.
2. Issue two identical `reqvire.read_element` calls back-to-back; assert both resolve the requested element and return equal structured content.
3. Issue a `reqvire.add_element` CRUD call without the commit startup flag; assert the call succeeds without advancing HEAD, changing index contents, or returning a `commit` field.
4. Issue another `reqvire.search`; assert the newly added element is present.
5. Stop and reap the mutation server, start a read-only server, and verify mutation tools are absent. Warm its model cache, append another element directly to a Markdown source, and issue a read; assert the result reflects the change.
6. Run a standalone `reqvire change-impact --git-commit=<hash>` invocation and check its exit status.
7. Verify the test timing wrapper forwards termination and interruption to its child, waits for child shutdown, and preserves normal stdin, exit status, and timing records; a previous mutation server must not survive and answer the read-only phase.

##### Evidence Scope
The satisfiedBy evidence supplies these response and mutation assertions. Response equality and standalone command completion do not prove internal reuse or bypass. The dedicated input-freshness and publication verifications own the instrumented correctness assertions and their execution status.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../tests/test-cache-integration/test.sh)
  * verify: [In-Memory Model Build Cache](../../ModelStructure/ModelManagement.md#in-memory-model-build-cache)
---

### Model Cache Input Freshness Verification

Verify stable model construction identity, complete input invalidation, and recovery while preserving the authoritative parser's workspace and dependency semantics.

#### Details

##### Acceptance Criteria
- After one completed build, repeated unchanged reads reuse that model without another parse, validation, or semantic-store build. Exercise a regex-backed exclusion pattern that executes matching, multiple worker threads, and reconstruction of an equivalent matcher; response equality alone is insufficient evidence.
- Assert shared model/registry identity across cache hits and concurrent same-input loads, with no graph/page copies. Retain an old read handle across changed-input publication and invalidation; its original content and semantic state remain intact. Mutable candidate edits must not change either cached handle.
- Effective pattern identity distinguishes different rules and matching options even when pattern counts and the selected Markdown inventory are equal. Reordering or repeating any-match rules without changing their effective meaning preserves identity.
- Creating, editing, and removing applicable root `.gitignore` and `.reqvireignore` files changes the actual active matcher and selected model inventory on the next load without restart. Equivalent policy edits do not force construction solely because of matcher runtime state. Nested ignore files do not change the root-only policy.
- Source edits, additions, removals, and moves become visible. Equal-length content edits with preserved modification time remain detectable, including page-only changes excluded from the public parsed-element revision.
- Editing a consumed local external ontology in Turtle/TTL, RDF/XML, or JSON-LD refreshes its derived semantic state even when Markdown, public model revision, file length, modification time, and dirty/clean status are unchanged.
- Removing, making unreadable, or invalidating a consumed dependency produces the authoritative builder's applicable failure or diagnostics; restoring it allows recovery. Creation/removal of a higher-priority external-source resolution candidate replaces a previously used fallback under the existing resolution rules.
- Removal or eligibility changes of referenced evidence/resource targets trigger applicable validation rather than reuse of a previously valid model. Inputs used only for existence validation are distinguished from files whose contents contribute to cached semantic state.
- Workspace root/worktree scope, available Git metadata, effective exclusions, version context, and build-mode changes cannot reuse an incompatible model. Lenient results never satisfy strict requests; size-estimate modes retain their own output semantics.
- Historical Git-commit construction does not consume or populate the current-workspace cache.

##### Required Evidence
- Extend the existing cache integration suite with real long-lived MCP requests, committed fixtures/expected outputs, and explicit build/reuse observations. Use ignored files and at least one exclusion such as `**/*[0-9]*.md` that exercises mutable regex state while preserving the selected fixture sources.
- Use core-level tests for matching-option distinctions, equivalent matcher reconstruction, mode separation, historical-build bypass, and referenced-target worktree eligibility transitions where controlled workspace setup or internal observations are needed.
- Record parser/build invocation counts or equivalent structured test instrumentation; neither latency thresholds nor equal payloads prove reuse. Establish input changes independently and assert visible results against a fresh authoritative build. Keep server logs outside the fixture's scanned source/Git state.
- Keep fixtures continuously dirty or explicitly preserve Git metadata when isolating dependency freshness, so a dirty-state transition cannot accidentally supply the invalidation being tested.

##### Evidence Status
Regression execution passes for worker-independent exclusion identity, equivalent policy ordering, live exclusion reloads, external dependency freshness, source-resolution precedence, evidence removal, and recovery. Controls also pass for ordinary source edits, build-mode separation, workspace/Git context, historical-cache isolation, and referenced-target eligibility loss and recovery. The shared HTTP suite passes all 190 checks; instrumented core tests establish actual reuse rather than only equal responses.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Model Parsing and Structure Verification Objective](#model-parsing-and-structure-verification-objective)
  * satisfiedBy: [model_cache_tests.rs](../../../crates/reqvire-core/src/model_cache_tests.rs)
  * satisfiedBy: [test.sh](../../../tests/test-cache-integration/test.sh)
  * verify: [In-Memory Model Build Cache](../../ModelStructure/ModelManagement.md#in-memory-model-build-cache)
---

### Model Cache Publication Verification

Verify coordinated model construction and publication of complete current state across concurrent loads, invalidation, and controlled writes.

#### Details

##### Acceptance Criteria
- Concurrent cold requests for identical workspace inputs and build mode perform one model build and receive its completed result. Distinct build modes cannot share incompatible graph, diagnostics, or size-estimate state.
- Each resolved construction attempt builds one semantic index, reused by validation and RDF capture in strict, lenient, and size-estimate modes. Warm reads and semantic exports do not build another index or initialize query stores.
- A strict semantic failure retains query diagnostics without publishing a query store. Repair and repeated parsing replace the failed state. Changed mutation candidates receive fresh semantic validation, and invalid edits leave persisted sources unchanged.
- Changing an external ontology between semantic validation and RDF capture cannot mix validation from one version with RDF from another. Cache publication detects the changed dependency and retries with a newly built index.
- While a build is paused before publication, invalidate its workspace and complete a newer persisted update/build. Releasing the older build cannot replace the newer cached state; waiting dependent reads resolve current state or an explicit applicable error.
- A relevant source, configuration, or dependency change during construction supersedes the candidate. The published identity and model reflect the inputs actually consumed; a candidate cannot be tagged with one input observation while containing another.
- A read that has captured a completed model retains consistent graph, page, and semantic query state while later publication occurs. Subsequent dependent reads after a successful write use the completed new state.
- A completed model initially has no prepared query-store variants. First use prepares only the requested variant; repeated and concurrent use through snapshot clones shares that completed store. All public variants retain their expected default/named graph contents and external visibility.
- First use after dependency deletion, and initialization paused across source edits and newer publication, use the captured RDF. Mutating a separate working registry cannot alter an older snapshot's deferred store. Current-model loads still reject missing or invalid dependencies and recover after repair.
- A failed store initialization returns the same applicable error to concurrent callers without exposing partial graphs; another snapshot can initialize successfully.
- Controlled writes cannot expose partially persisted files to a newly published model. Reuse of a post-write model requires its semantic state and persisted-input identity to agree with its graph; invalidation and rebuild remain a valid fallback.
- Failed construction or superseded attempts release all waiting requests and permit retry/recovery. Continuous external changes produce a bounded retry outcome rather than an endless wait or stale success. Old valid data is not reported as current for invalid new inputs.
- Rejected/previews with unchanged sources do not publish mutation candidates. If a failure occurs after any persistence, affected cache state is invalidated and the operation error remains visible.

##### Required Evidence
- Add deterministic core tests using barriers or controlled build hooks to hold input capture, build completion, invalidation, and publication at known points. Assert build counts and the actual retained/resulting model, not scheduling delays.
- Exercise both successful and failing shared builds and assert every waiter completes under a bounded test deadline. Verify the next valid load recovers.
- Inject a persistence failure only after observing changed bytes successfully written to the first affected file. Assert the operation error remains visible, affected cached state is invalidated, subsequent reads follow authoritative validation, and repaired sources recover.
- Compare graph/page data and SPARQL-visible triples for the accepted generation; verify a captured older result remains internally consistent. Generation instrumentation is internal and does not require new public interface fields.
- Observe prepared-store counts and store identity independently of response equality. Pause first-use initialization with a deterministic hook and verify old/new SPARQL results against their respective captured RDF. Check initialization failures with malformed captured RDF in an internal fixture.
- Instrument semantic-index builds and pause immediately after semantic validation. Assert per-attempt index counts, retained invalid-query diagnostics, refreshed mutation validation, and agreement between validated external labels and captured RDF across an intervening file edit.
- Consumer-owned verifications establish transport write gating and derived-artifact integration; this verification owns the core construction/publication invariants.

##### Evidence Status
Regression execution passes for coordinated construction, superseded publication, input rechecks, bounded failure under continuous edits, and invalidation after partial persistence. The core cache tests include a contender arriving between cache lookup and build registration, first-use queries after dependency removal, and deferred initialization paused across newer publication. Instrumented tests establish one semantic index per resolved construction attempt across build modes, retained diagnostics after strict semantic failure, reset on repeated parsing, and fresh validation for mutated candidates. An external edit between validation and RDF capture preserves the validated version within the attempt and forces a fresh cache build before publication. Captured graph/page/SPARQL state and derived semantic artifacts remain consistent across later changes. Store tests establish deferred preparation, shared concurrent initialization, graph visibility, and error propagation. The consumer adapter tests establish read isolation during controlled persistence. Persistence errors remain visible, partial inputs receive authoritative validation, and repaired inputs recover.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Model Parsing and Structure Verification Objective](#model-parsing-and-structure-verification-objective)
  * satisfiedBy: [mcp_cache_tests.rs](../../../crates/reqvire-cli/src/mcp_cache_tests.rs)
  * satisfiedBy: [mcp_session_tests.rs](../../../crates/reqvire-cli/src/mcp_session_tests.rs)
  * satisfiedBy: [semantic_store.rs](../../../crates/reqvire-core/src/semantic_store.rs)
  * satisfiedBy: [model_cache_tests.rs](../../../crates/reqvire-core/src/model_cache_tests.rs)
  * verify: [In-Memory Model Build Cache](../../ModelStructure/ModelManagement.md#in-memory-model-build-cache)
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
