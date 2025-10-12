# Verification Tests

## Model Command Tests

### Model Generation Test

Test verifies that model diagrams can be generated from CLI.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Model Diagram Generation](../SystemRequirements.md#model-diagram-generation)

---

### Model Filtering Test

Test verifies that model diagrams can be filtered from a specific root element.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Model Filtering Capability](../SystemRequirements.md#model-filtering-capability)
  * verify: [Forward Relation Traversal](../SystemRequirements.md#forward-relation-traversal)

---

### Output Format Test

Test verifies that both markdown and JSON output formats are supported.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Markdown Output Format](../SystemRequirements.md#markdown-output-format)
  * verify: [JSON Output Format](../SystemRequirements.md#json-output-format)

---
