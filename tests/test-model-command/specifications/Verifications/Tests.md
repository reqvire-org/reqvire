# Elements


### Model Generation Test

Test verifies that model diagrams can be generated from CLI.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Model Diagram Generation](../SystemRequirements.md#model-diagram-generation)

---

### From Flag Filtering Test

Test verifies --from flag filters model starting from specified element.

#### Details
Test procedure:
1. Run model command WITH --from <element-name> flag with --json
2. Compare output against expected_filtered_output.json
3. Verify output starts from specified element at top level
4. Verify relations are nested recursively from that starting point
5. Verify only forward-related elements appear in nested structure
6. Verify metadata.filtered_from contains element name

Expected files:
- tests/test-model-command/expected_filtered_output.json

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Model Filtering Capability](../SystemRequirements.md#model-filtering-capability)
  * verify: [Forward Relation Traversal](../SystemRequirements.md#forward-relation-traversal)

---

### Default Filtering Test

Test verifies default behavior filters to root requirements when --from flag is NOT specified.

#### Details
Test procedure:
1. Run model command WITHOUT --from flag with --json
2. Compare output against expected_default_output.json
3. Verify only root requirements (no hierarchical parent) appear at top level
4. Verify their derived children appear nested in relations
5. Verify output is model-centric (not folder-centric)

Expected files:
- tests/test-model-command/expected_default_output.json

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Default Root Filtering](../SystemRequirements.md#default-root-filtering)

---

### Output Format Test

Test verifies nested JSON structure and output formats against expected files.

#### Details
Test procedure:
1. Run model command with --json flag
2. Compare output against expected_output.json
3. Verify JSON structure matches expected:
   - Elements array with all required fields (identifier, name, element_type, file_path, section, section_index)
   - Relations nested inside elements with target details
   - Three target types handled correctly: element (recursive), file path, external URL
   - Metadata counts elements/relations without duplicates
4. Run model command without --json flag
5. Compare markdown output against expected_output.md
6. Verify mermaid diagrams present and correctly formatted with all nested relations

Expected files:
- tests/test-model-command/expected_output.json
- tests/test-model-command/expected_output.md

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Markdown Output Format](../SystemRequirements.md#markdown-output-format)
  * verify: [JSON Output Format](../SystemRequirements.md#json-output-format)

---

### Reverse Traversal Test

Test verifies --reverse flag traverses from leaves to roots.

#### Details
Test procedure:
1. Run model command with --reverse flag and --json
2. Verify output starts from leaf elements
3. Verify metadata.direction shows "Reverse"
4. Verify relations use backward relation types (derivedFrom, satisfy, verify)
5. Verify traversal goes from verification/leaf toward root requirements

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Forward Relation Traversal](../SystemRequirements.md#forward-relation-traversal)

---

### Filter Type Test

Test verifies --filter-type flag filters starting elements by type.

#### Details
Test procedure:
1. Run model command with --filter-type=test-verification and --json
2. Verify only test-verification elements appear at top level
3. Verify metadata.type_filter contains the specified types
4. Run model command with --reverse --filter-type=test-verification
5. Verify verifications are starting points for upward traversal

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Model Filtering Capability](../SystemRequirements.md#model-filtering-capability)

---
