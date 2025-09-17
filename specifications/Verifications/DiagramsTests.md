# Diagram Tests

This document verifies the requirements for Reqvire's diagram generation functionality.

## Diagram Generation Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  de8ab093f22a5cd7["Visualize Model Relationships Verification"];
  class de8ab093f22a5cd7 verification;
  click de8ab093f22a5cd7 "DiagramsTests.md#visualize-model-relationships-verification";
  3df49fd1a91c3db7["tests/test-diagram-generation/test.sh"];
  class 3df49fd1a91c3db7 default;
  click 3df49fd1a91c3db7 "../../tests/test-diagram-generation/test.sh";
  de8ab093f22a5cd7 -->|satisfiedBy| 3df49fd1a91c3db7;
  2d2cf67bb8070da8["Diagram Generation Test"];
  class 2d2cf67bb8070da8 verification;
  click 2d2cf67bb8070da8 "DiagramsTests.md#diagram-generation-test";
  3df49fd1a91c3db7["tests/test-diagram-generation/test.sh"];
  class 3df49fd1a91c3db7 default;
  click 3df49fd1a91c3db7 "../../tests/test-diagram-generation/test.sh";
  2d2cf67bb8070da8 -->|satisfiedBy| 3df49fd1a91c3db7;
  ba2fd3fb7c42cdb3["Diagram Relation Filtering Verification"];
  class ba2fd3fb7c42cdb3 verification;
  click ba2fd3fb7c42cdb3 "DiagramsTests.md#diagram-relation-filtering-verification";
  f6a73030d877fc0f["tests/test-diagram-filtering/test.sh"];
  class f6a73030d877fc0f default;
  click f6a73030d877fc0f "../../tests/test-diagram-filtering/test.sh";
  ba2fd3fb7c42cdb3 -->|satisfiedBy| f6a73030d877fc0f;
  d4e5f6a7b8c9d0e1["Remove Generated Diagrams Verification"];
  class d4e5f6a7b8c9d0e1 verification;
  click d4e5f6a7b8c9d0e1 "DiagramsTests.md#remove-generated-diagrams-verification";
  a1b2c3d4e5f6g7h8["tests/test-remove-diagrams/test.sh"];
  class a1b2c3d4e5f6g7h8 default;
  click a1b2c3d4e5f6g7h8 "../../tests/test-remove-diagrams/test.sh";
  d4e5f6a7b8c9d0e1 -->|satisfiedBy| a1b2c3d4e5f6g7h8;
  f8849dfe948d04fa["Automated Diagram Generation on PR Merge Verification"];
  class f8849dfe948d04fa verification;
  click f8849dfe948d04fa "DiagramsTests.md#automated-diagram-generation-on-pr-merge-verification";
  98af8a1cf9c86822[".github/workflows/generate_diagrams.yml"];
  class 98af8a1cf9c86822 default;
  click 98af8a1cf9c86822 "../../.github/workflows/generate_diagrams.yml";
  f8849dfe948d04fa -->|satisfiedBy| 98af8a1cf9c86822;
  7f86e99e7804366a["Diagram Relation Filtering"];
  class 7f86e99e7804366a requirement;
  click 7f86e99e7804366a "../SystemRequirements/Requirements.md#diagram-relation-filtering";
  dad7eeb932afdb92["diagrams.rs"];
  class dad7eeb932afdb92 default;
  click dad7eeb932afdb92 "../../core/src/diagrams.rs";
  7f86e99e7804366a -->|satisfiedBy| dad7eeb932afdb92;
  7f86e99e7804366a -.->|verifiedBy| ba2fd3fb7c42cdb3;
  98a581084d5542fa["Automate Diagram Generation"];
  class 98a581084d5542fa requirement;
  click 98a581084d5542fa "../UserRequirements.md#automate-diagram-generation";
  98a581084d5542fa -.->|verifiedBy| de8ab093f22a5cd7;
  98a581084d5542fa -.->|verifiedBy| 2d2cf67bb8070da8;
  3e3df7ad427a88fa["Automated Diagram Generation on PR Merge"];
  class 3e3df7ad427a88fa requirement;
  click 3e3df7ad427a88fa "../SystemRequirements/Requirements.md#automated-diagram-generation-on-pr-merge";
  98a581084d5542fa -.->|deriveReqT| 3e3df7ad427a88fa;
  98af8a1cf9c86822["generate_diagrams.yml"];
  class 98af8a1cf9c86822 default;
  click 98af8a1cf9c86822 "../../.github/workflows/generate_diagrams.yml";
  3e3df7ad427a88fa -->|satisfiedBy| 98af8a1cf9c86822;
  3e3df7ad427a88fa -.->|verifiedBy| f8849dfe948d04fa;
```

---

### Diagram Generation Test

This test verifies that the system can automatically generate and embed mermaid diagrams in requirements documents.

#### Metadata
  * type: verification

#### Details 

##### Acceptance Criteria
- System should process requirements files and add/update embedded mermaid diagrams
- System should create diagrams that represent relationships between elements
- System should preserve any existing custom mermaid diagrams in the documents
- System should update automatically generated diagrams when requirements change
- System should properly visualize all relationship types (verifies, trace, refines, contains, derives, satisfies, etc.)
- System should render relationships with appropriate arrows and formatting

##### Test Criteria
- Command exits with success (0) return code
- Modified files contain the expected mermaid diagrams
- Custom mermaid diagrams are preserved 
- Diagram content accurately reflects the relationships defined in the requirements
- All relationship types are correctly visualized with proper arrows and labels (verifies, trace, refines, contains, derives, satisfies)
- Special relationship types like "deriveReqT" are properly rendered

#### Relations
  * verify: [UserRequirements.md/Automate Diagram Generation](../UserRequirements.md#automate-diagram-generation)
  * satisfiedBy: [tests/test-diagram-generation/test.sh](../../tests/test-diagram-generation/test.sh)

---

### Visualize Model Relationships Verification

This test verifies that the system provides visual representations of relationships within the MBSE model in the generated diagrams.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should generate diagrams showing relationships between model elements
- Diagrams should clearly represent different relationship types
- Visual representation should aid in understanding dependencies between elements

##### Test Criteria
- Command exits with success (0) return code
- Generated diagrams contain all expected relationship types
- Relationships are visually differentiated based on their type
- Element dependencies are clearly displayed in the diagrams

#### Relations
  * verify: [UserRequirements.md/Automate Diagram Generation](../UserRequirements.md#automate-diagram-generation)
  * satisfiedBy: [tests/test-diagram-generation/test.sh](../../tests/test-diagram-generation/test.sh)

---

### Automated Diagram Generation on PR Merge Verification

This test verifies that the system automatically generates and updates diagrams when pull requests are merged to the main branch.

#### Metadata
  * type: demonstration-verification

#### Details

##### Acceptance Criteria
- System should have a GitHub workflow that automatically generates diagrams on PR merge
- The workflow should only be triggered when PRs are merged to the main branch
- Generated diagrams and traceability matrix SVG should be committed back to the main branch
- The commit message should clearly indicate the automated nature of the change

##### Test Criteria
- Workflow defined in the GitHub workflow configuration correctly
- Workflow triggers only on PR merge to main branch
- Workflow correctly checks out code, builds the tool, and generates diagrams
- Workflow generates a traceability matrix SVG file
- Workflow commits and pushes changes back to the main branch
- Commit message is informative and standardized

#### Relations
  * verify: [SystemRequirements/Requirements.md/Automated Diagram Generation on PR Merge](../SystemRequirements/Requirements.md#automated-diagram-generation-on-pr-merge)
  * satisfiedBy: [.github/workflows/generate_diagrams.yml](../../.github/workflows/generate_diagrams.yml)

---

### Diagram Relation Filtering Verification

This test verifies that the system correctly filters relations in diagram generation to render only forward relations while ensuring complete element hierarchy representation.

#### Metadata
  * type: test-verification

#### Details

##### Acceptance Criteria
- System should render only forward relations to prevent duplicate arrows for bidirectional relationships
- System should include parent elements in diagrams even when they belong to different sections
- System should apply direction-based rendering according to relation type registry
- Generated diagrams should not contain both forward and backward relations for the same logical relationship

##### Test Criteria
- Command exits with success (0) return code
- Diagrams contain only forward relations (e.g., `contain` but not `containedBy`)
- Bidirectional relationships appear as single arrows in their forward direction
- Parent elements are included when child elements are in the section
- No duplicate arrows exist for the same logical relationship
- Arrow directions follow the semantic direction defined in relation type registry

#### Relations
  * verify: [SystemRequirements/Requirements.md/Diagram Relation Filtering](../SystemRequirements/Requirements.md#diagram-relation-filtering)
  * satisfiedBy: [tests/test-diagram-filtering/test.sh](../../tests/test-diagram-filtering/test.sh)

---

### Remove Generated Diagrams Verification

This test verifies that the system can remove all generated mermaid diagrams while preserving custom user-created diagrams and respecting file exclusion patterns.

#### Metadata
  * type: test-verification

#### Details

##### Acceptance Criteria
- System should remove all auto-generated mermaid diagrams that appear immediately after section headers
- System should preserve custom mermaid diagrams that appear in other locations within the document
- System should respect exclusion patterns defined in reqvire.yaml configuration
- System should handle files with multiple diagrams correctly
- System should work on files without any diagrams
- System should maintain file structure and content except for diagram removal

##### Test Criteria
- Command exits with success (0) return code
- Generated diagrams are removed from files (diagrams immediately after ## Section headers)
- Custom diagrams are preserved (diagrams in other locations)
- Files matching exclusion patterns are not processed for diagram removal
- File structure remains intact except for diagram removal
- Element text and relationships are preserved
- Section headers and element headers are preserved

#### Relations
  * verify: [UserRequirements.md/Remove Generated Diagrams](../UserRequirements.md#remove-generated-diagrams)
  * verify: [SystemRequirements/Requirements.md/Diagram Removal](../SystemRequirements/Requirements.md#diagram-removal)
  * satisfiedBy: [tests/test-remove-diagrams/test.sh](../../tests/test-remove-diagrams/test.sh)

---