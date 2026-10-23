# Elements

### Formatting and Relation Normalization Verification Objective

This objective groups verification of shared formatting semantics: document form, element and relation ordering, canonical entries, governance preservation and explicit inverse materialization.

#### Details
Linked fixture scripts provide core algorithm evidence through their current harness. They require harness migration when the CLI surface changes; their links do not establish MCP transport, permission or automatic mutation-persistence coverage. Protocol formatting acceptance is planned separately under the MCP verification objective.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Element Ordering Verification](#element-ordering-verification)
  * derive: [Format Duplicate Removal Test](#format-duplicate-removal-test)
  * derive: [Formatting Operations Verification](#formatting-operations-verification)
  * derive: [Full Relations Insertion Verification](#full-relations-insertion-verification)
  * derive: [Relation Ordering Verification](#relation-ordering-verification)
  * derive: [Requirement Governance Metadata Formatting Verification](#requirement-governance-metadata-formatting-verification)
---

### Element Ordering Verification

This test verifies that the shared formatter reorders elements following the Element Ordering Behavior, ensuring parent elements appear before their children based on file-local derivedFrom hierarchy.

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
   - Apply the shared formatter
   - Verify parent appears before child in output

2. **Multi-level hierarchy**
   - Create file with grandchild, child, parent in wrong order
   - Apply the shared formatter
   - Verify order: parent → child → grandchild

3. **Sibling alphabetical sorting**
   - Create file with siblings "Child Z" and "Child A" under same parent
   - Apply the shared formatter
   - Verify order: parent → Child A → Child Z

4. **Multiple root elements**
   - Create file with multiple root elements (no file-local parents) in wrong alphabetical order
   - Apply the shared formatter
   - Verify roots are sorted alphabetically

5. **Mixed hierarchy and standalone**
   - Create file with hierarchy group and standalone elements
   - Apply the shared formatter
   - Verify hierarchy groups are kept together and all roots sorted alphabetically

6. **Cross-file relations ignored**
   - Create file where element derives from element in different file
   - Apply the shared formatter
   - Verify cross-file derivedFrom does not affect local ordering

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-element-ordering/test.sh)
  * verify: [Element Ordering Normalization](../../../Operations/Formatting/FormattingRequirements.md#element-ordering-normalization)
---

### Format Duplicate Removal Test

Verify canonical-entry preservation and rejection of invalid duplicate source declarations.

#### Details
- A disk fixture containing duplicate relations or duplicate contract dependencies fails model admission; formatting does not silently repair it. Match diagnostics and compare source bytes to prove rejection preserves declarations.
- A cross-section duplicate fails shared semantic validation and remains an author decision rather than an automatic formatting repair.
- A valid fixture reloads and formats without introducing repeated relation or binding entries. Compare canonical output with the expected source fixture and require repeated application to preserve those bytes.
- The linked suite establishes existing duplicate-admission and valid-output behavior through the current harness; it does not claim an MCP endpoint can start from an invalid fixture.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-duplicate-detection/test.sh)
  * verify: [Format Duplicate Removal](../../../Operations/Formatting/FormattingRequirements.md#format-duplicate-removal)
---

### Formatting Operations Verification

Verify shared formatting and diff semantics independently of transport.

#### Details
- Preview produces deterministic file/hunk evidence without changing source bytes. Applying formatting matches the expected fixture tree and is idempotent.
- Preserve `# Elements` with H3 elements/H4 reserved subsections and `# Element` with H2 metadata, optional relations and the named body. Body headings in a single-element file remain content.
- Normalize whitespace, separators, reserved subsections, ordering and readable link labels while preserving page text, domain prose, external URLs, governance metadata and fenced payload bytes.
- Normalize structured relations, contract dependencies and concept references to source-relative destinations while retaining resolved identities and resource kinds.
- Default formatting retains the authored relation subset; explicit full-relation materialization inserts registered compatible inverses.
- Invalid semantic input returns shared diagnostics and guidance without an inferred dependency repair or source writes.
- Compare representative source/result fixtures with unified diffs. The linked suite submits typed MCP formatting requests; the interface fixture separately checks permission schemas and protocol results.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-advanced-format/test.sh)
  * verify: [Shared Operation Diff Evidence](../../../Interfaces/InterfacesRequirements.md#shared-operation-diff-evidence)
  * verify: [Structure and Addressing in Markdown Documents](../../../ModelStructure/StructureAndParsing.md#structure-and-addressing-in-markdown-documents)
  * verify: [Document Structure Normalization](../../../Operations/Formatting/FormattingRequirements.md#document-structure-normalization)
  * verify: [Formatting Output](../../../Operations/Formatting/FormattingRequirements.md#formatting-output)
  * verify: [Replace Absolute Links with Relative Links](../../../Operations/Formatting/FormattingRequirements.md#replace-absolute-links-with-relative-links)
---

### Full Relations Insertion Verification

This test verifies that the typed `with_full_relations` option correctly inserts all registered relations (user-created and auto-generated) into elements during formatting.

#### Details

##### Acceptance Criteria
**Auto-Generated Relations Insertion:**
- System shall insert inverse relations when `with_full_relations: true` is provided
- definedBy relations generated from a contract's define relation shall be inserted
- derive relations generated from derivedFrom shall be inserted
- verify relations generated from verifiedBy shall be inserted

**Relation Ordering:**
- All relations (user-created and auto-generated) shall be sorted according to the Relation Ordering Normalization requirement
- Relations shall follow the owned ordering contract, including concept-family priorities before alphabetical type/target ordering

**Idempotency:**
- Applying shared formatting with full-relation materialization enabled twice shall produce same output
- Already present relations shall not be duplicated

**Selective Behavior:**
- Without `with_full_relations` option, only user-created relations are persisted
- With `with_full_relations` option, all relations from registry are persisted

##### Test Criteria
1. **Basic inverse relation insertion**
   - Create a compatible contract with define pointing to its requirement owner
   - Apply the shared formatter with full-relation materialization enabled
   - Verify the owner now has the inverse definedBy relation to the contract

2. **Multiple inverse relation types**
   - Create a compatible same-family hierarchy and a requirement verified by a concrete verification with its objective parent
   - Apply the shared formatter with full-relation materialization enabled
   - Verify the model elements receive compatible derive and verify inverses. Artifact satisfaction does not authorize writing relation declarations into evidence files

3. **No duplication**
   - Apply the shared formatter with full-relation materialization enabled twice
   - Verify relations are not duplicated

4. **Default behavior unchanged**
   - Create a contract with define pointing to its requirement owner
   - Apply the shared formatter with default relation persistence
   - Verify inverse relations are NOT inserted

5. **Preview mode**
   - Preview shared formatting with full-relation materialization enabled
   - Verify diff shows proposed relation insertions
   - Verify files are not modified

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-format-full-relations/test.sh)
  * verify: [Full Relations Insertion](../../../Operations/Formatting/FormattingRequirements.md#full-relations-insertion)
---

### Prose Link Formatting Repair Boundary Verification

Representative automated verification that formatting preserves author decisions for a specification's prohibited prose link. One fixture scenario covers preview/fix, source-type-appropriate guidance, and an explicit valid repair using unified diffs.

#### Details

##### Acceptance Criteria
- Formatting does not silently turn a prohibited prose hyperlink into a relation, contract dependency, or Concept Reference.
- Responses preserve shared repair guidance and actual source/target context, including source-type-appropriate literal Markdown examples.

##### Test Criteria
1. Format one specification fixture containing a prohibited prose link in preview and fix modes. Require shared-rule rejection and unchanged source files, with no inferred relation, dependency, ownership transfer, or copied target content.
2. Require the shared diagnostic code, occurrence, resolved target, and non-empty guidance directing the contract dependency to the responsible requirement.
3. Apply an explicit repair using readable domain wording. Require formatting and validation to succeed and match the resulting file against a checked-in expected fixture.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Formatting and Relation Normalization Verification Objective](#formatting-and-relation-normalization-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-advanced-format/test.sh)
  * verify: [Format Consistency Enforcement](../../../Operations/Formatting/FormattingRequirements.md#format-consistency-enforcement)
---

### Relation Ordering Verification

Verify the shared formatter's relation ordering against representative fixtures.

#### Details
- The advanced-format fixture establishes alphabetical type ordering for ordinary elements and target ordering within one type.
- The native concept fixture deliberately scrambles `derivedFrom`, `broader`, `narrower`, `related`, `exactMatch` and `closeMatch`. Compare the complete formatting diff against the expected JSON fixture.
- The golden output puts `derivedFrom` before `broader` and `related` before mapping relations, distinguishing concept priority from universal alphabetical sorting.
- Keep the preview read-only and validate the fixture before rendering the diff. Existing core harness invocation does not prove MCP formatting permission or canonical transaction behavior; those checks remain in the MCP format acceptance plan.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-advanced-format/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-concept-elements/test.sh)
  * verify: [Relation Ordering Normalization](../../../Operations/Formatting/FormattingRequirements.md#relation-ordering-normalization)
---

### Requirement Governance Metadata Formatting Verification

This verification shall prove that formatting and rewrite operations preserve authored requirement governance metadata without persisting inherited or default governance metadata values.

#### Details
Expected checks:
- Create a parent requirement with explicit governance metadata.
- Create child requirements that omit one or more governance metadata keys.
- Apply the shared formatting operation to the valid fixture.
- Verify explicitly authored governance metadata remains in each element's `#### Metadata` subsection.
- Verify inherited governance metadata is not inserted into child requirement Markdown.
- Verify default governance metadata is not inserted into requirement Markdown.
- Verify governance metadata is not inserted into contract element Markdown.
- Verify metadata indentation and ordering remain deterministic after formatting.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-requirement-governance-metadata/test.sh)
  * verify: [Requirement Governance Metadata](../../../ModelStructure/ModelManagement.md#requirement-governance-metadata)
  * verify: [Format Consistency Enforcement](../../../Operations/Formatting/FormattingRequirements.md#format-consistency-enforcement)
---
