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

  f8849dfe948d04fa["Automated Diagram Generation on PR Merge Verification"];
  class f8849dfe948d04fa verification;
  click f8849dfe948d04fa "DiagramsTests.md#automated-diagram-generation-on-pr-merge-verification";
  3e3df7ad427a88fa["SystemRequirements/Requirements.md/Automated Diagram Generation on PR Merge"];
  class 3e3df7ad427a88fa requirement;
  click 3e3df7ad427a88fa "../SystemRequirements/Requirements.md#automated-diagram-generation-on-pr-merge";
  f8849dfe948d04fa -.->|verifies| 3e3df7ad427a88fa;
  98af8a1cf9c86822[".github/workflows/generate_diagrams.yml"];
  class 98af8a1cf9c86822 default;
  click 98af8a1cf9c86822 "../../.github/workflows/generate_diagrams.yml";
  98af8a1cf9c86822 -->|satisfies| f8849dfe948d04fa;
  2d2cf67bb8070da8["Diagram Generation Test"];
  class 2d2cf67bb8070da8 verification;
  click 2d2cf67bb8070da8 "DiagramsTests.md#diagram-generation-test";
  98a581084d5542fa["UserRequirements.md/Automate Diagram Generation"];
  class 98a581084d5542fa requirement;
  click 98a581084d5542fa "../UserRequirements.md#automate-diagram-generation";
  2d2cf67bb8070da8 -.->|verifies| 98a581084d5542fa;
  3df49fd1a91c3db7["tests/test-diagram-generation/test.sh"];
  class 3df49fd1a91c3db7 default;
  click 3df49fd1a91c3db7 "../../tests/test-diagram-generation/test.sh";
  3df49fd1a91c3db7 -->|satisfies| 2d2cf67bb8070da8;
  de8ab093f22a5cd7["Visualize Model Relationships Verification"];
  class de8ab093f22a5cd7 verification;
  click de8ab093f22a5cd7 "DiagramsTests.md#visualize-model-relationships-verification";
  de8ab093f22a5cd7 -.->|verifies| 98a581084d5542fa;
  3df49fd1a91c3db7["tests/test-diagram-generation/test.sh"];
  class 3df49fd1a91c3db7 default;
  click 3df49fd1a91c3db7 "../../tests/test-diagram-generation/test.sh";
  3df49fd1a91c3db7 -->|satisfies| de8ab093f22a5cd7;
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