# Elements

### Details Block Formatting Test

Element with details block that should not be formatted.
<details>
<summary>Click to expand</summary>

#### This Should Not Get Blank Line
Content inside details block.
####Another Header Without Space
More content that should remain untouched.
</details>

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability Test Advanced Format Userstories Md](#test-capability-test-advanced-format-userstories-md)
---

### Exact Blank Line Count Test

Test that exactly one blank line is added before Metadata when repositioned.
Content line 1.
Content line 2.
Content line 3.

#### Details
Detail content here.

#### Metadata
  * type: test-verification
  * implementation-status: implemented
---

### Generate Diagrams

As a **System Engineer**, I want to generate diagrams for different system viewpoints, so that I can communicate system architecture effectively and understand dependencies and impacts across the system.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability Test Advanced Format Userstories Md](#test-capability-test-advanced-format-userstories-md)
  * trace: [MOE_UA](MOEs.md#moe_ua)
---

### Managing MBSE Models

As an **System Engineer**, I want to manage MBSE models effectively, so that I can ensure they align with project requirements and deliverable goals.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Requirements Processing](SystemRequirements/Requirements.md#requirements-processing)
  * specify: [Test Capability Test Advanced Format Userstories Md](#test-capability-test-advanced-format-userstories-md)
  * trace: [MOE_UA](MOEs.md#moe_ua)
---

### Metadata Repositioning Test

Verify that metadata repositioning maintains blank line spacing.

#### Details
This test verifies that when metadata is repositioned from early position to end position, a blank line is properly added before the metadata section.

##### Acceptance Criteria
  * [ ] Metadata is moved to end position
  * [ ] Blank line exists before metadata after repositioning
  * [ ] Content structure is preserved

#### Metadata
  * type: test-verification
  * implementation-status: implemented
---

### Missing Blank Line Test

This element has content but no blank line before metadata.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability Test Advanced Format Userstories Md](#test-capability-test-advanced-format-userstories-md)
---

### Root Capability

Top-level capability used as hierarchy parent for system requirements in this fixture.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability Test Advanced Format Userstories Md](#test-capability-test-advanced-format-userstories-md)
---

### External URL Preservation Test

This element tests that external URLs in relations are preserved exactly as written.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Capability](#root-capability)
  * satisfiedBy: [PlansAndPricing](https://docs.google.com/spreadsheets/d/1YgO2THkvPQEtvoP2c4JU6q8Fx4K9rWJ9V5bhpGvo7DI/edit?gid=203583324#gid=203583324)
---

### Test Capability Test Advanced Format Userstories Md

Test capability root for migrated requirement fixtures.


Personas:
1. System Engineer: Focused on managing system models, ensuring alignment with project requirements, and validating structures.
2. SOI Developer: Implements capabilities and makes system changes based on MBSE models, ensuring consistency between design and code.
3. Contributor: An external community member contributing to Reqvire by improving models, creating capabilities, or providing feedback.
4. Manager: Oversees the MBSE processes, tracks progress, ensures alignment with objectives, and generates reports for decision-making.

#### Metadata
  * type: capability
---
