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

  fff74265828e19b3["Export Diagrams Verification"];
  click fff74265828e19b3 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/Verifications/DiagramsTests.md#export-diagrams-verification";
  class fff74265828e19b3 verification;
  a0274ca0625d8493["UserRequirements.md/Export Diagrams in Standard Formats"];
  class a0274ca0625d8493 requirement;
  click a0274ca0625d8493 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  fff74265828e19b3 -.->|verifies| a0274ca0625d8493;
  b73eaac7c0353fcb["tests/test-diagram-generation/test.sh"];
  class b73eaac7c0353fcb default;
  click b73eaac7c0353fcb "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/tests/test-diagram-generation/test.sh";
  fff74265828e19b3 -.->|trace| b73eaac7c0353fcb;
  44dc4128c7ea74d2["Automated Diagram Generation on PR Merge Verification"];
  click 44dc4128c7ea74d2 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/Verifications/DiagramsTests.md#automated-diagram-generation-on-pr-merge-verification";
  class 44dc4128c7ea74d2 verification;
  793154acc336992c["SystemRequirements/Requirements.md/Automated Diagram Generation on PR Merge"];
  class 793154acc336992c requirement;
  click 793154acc336992c "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/SystemRequirements/Requirements.md#automated-diagram-generation-on-pr-merge";
  44dc4128c7ea74d2 -.->|verifies| 793154acc336992c;
  fdd29f919065644d[".github/workflows/generate_diagrams.yml"];
  class fdd29f919065644d default;
  click fdd29f919065644d "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/.github/workflows/generate_diagrams.yml";
  44dc4128c7ea74d2 -.->|trace| fdd29f919065644d;
  494a61d80d3cb287["Highlight Changes in Diagrams Verification"];
  click 494a61d80d3cb287 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/Verifications/DiagramsTests.md#highlight-changes-in-diagrams-verification";
  class 494a61d80d3cb287 verification;
  ac914f743d73674e["UserRequirements.md/Highlight Changes in Diagrams"];
  class ac914f743d73674e requirement;
  click ac914f743d73674e "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  494a61d80d3cb287 -.->|verifies| ac914f743d73674e;
  494a61d80d3cb287 -.->|trace| b73eaac7c0353fcb;
  2c4c9bdcf9ede0fd["Visualize Model Relationships Verification"];
  click 2c4c9bdcf9ede0fd "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/Verifications/DiagramsTests.md#visualize-model-relationships-verification";
  class 2c4c9bdcf9ede0fd verification;
  eed0b020b6ddeae1["UserRequirements.md/Visualize Model Relationships"];
  class eed0b020b6ddeae1 requirement;
  click eed0b020b6ddeae1 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserRequirements.md#visualize-model-relationships";
  2c4c9bdcf9ede0fd -.->|verifies| eed0b020b6ddeae1;
  2c4c9bdcf9ede0fd -.->|trace| b73eaac7c0353fcb;
  e4a47f6f403fd4ff["Diagram Generation Test"];
  click e4a47f6f403fd4ff "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/Verifications/DiagramsTests.md#diagram-generation-test";
  class e4a47f6f403fd4ff verification;
  c522cf4c404bdc24["UserRequirements.md/Automate Diagram Generation"];
  class c522cf4c404bdc24 requirement;
  click c522cf4c404bdc24 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserRequirements.md#automate-diagram-generation";
  e4a47f6f403fd4ff -.->|verifies| c522cf4c404bdc24;
  e4a47f6f403fd4ff -.->|trace| b73eaac7c0353fcb;
  35f25ed1c49ab87a["Diagram Storage Verification"];
  click 35f25ed1c49ab87a "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/Verifications/DiagramsTests.md#diagram-storage-verification";
  class 35f25ed1c49ab87a verification;
  89097c1311055b72["UserRequirements.md/Store Automated Diagrams in Designated Locations"];
  class 89097c1311055b72 requirement;
  click 89097c1311055b72 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  35f25ed1c49ab87a -.->|verifies| 89097c1311055b72;
  35f25ed1c49ab87a -.->|trace| b73eaac7c0353fcb;
  7fd405bfa560fe5c["Filter Relationships by Type Verification"];
  click 7fd405bfa560fe5c "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/Verifications/DiagramsTests.md#filter-relationships-by-type-verification";
  class 7fd405bfa560fe5c verification;
  66e9d8186acafd13["UserRequirements.md/Filter Relationships by Type"];
  class 66e9d8186acafd13 requirement;
  click 66e9d8186acafd13 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserRequirements.md#filter-relationships-by-type";
  7fd405bfa560fe5c -.->|verifies| 66e9d8186acafd13;
  7fd405bfa560fe5c -.->|trace| b73eaac7c0353fcb;
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

##### Test Procedure
1. Create test fixtures in `/tests/test-diagram-generation/` with requirements containing various elements and relationships
2. Create test fixtures that include custom mermaid diagrams to test preservation
3. Run Reqvire with the `--generate-diagrams` flag on the test fixtures
4. Verify that mermaid diagrams are generated at the beginning of each file
5. Verify that existing custom mermaid diagrams are preserved
6. Verify diagram content accurately shows elements and relationships

#### Relations
  * verify: [UserRequirements.md/Automate Diagram Generation](../UserRequirements.md#automate-diagram-generation)
  * trace: [tests/test-diagram-generation/test.sh](../../tests/test-diagram-generation/test.sh)

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

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Visualize Model Relationships](../UserRequirements.md#visualize-model-relationships)
  * trace: [tests/test-diagram-generation/test.sh](../../tests/test-diagram-generation/test.sh)

---

### Filter Relationships by Type Verification

This test verifies that the system allows users to filter relationships in diagrams by type.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should allow filtering of relationships by type when generating diagrams
- Filtering should support different relationship types (dependency, refinement, verification, etc.)
- Filtered diagrams should only show the selected relationship types

##### Test Criteria
- Command exits with success (0) return code
- Filtering options are correctly applied to diagram generation
- Filtered diagrams contain only the specified relationship types

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Filter Relationships by Type](../UserRequirements.md#filter-relationships-by-type)
  * trace: [tests/test-diagram-generation/test.sh](../../tests/test-diagram-generation/test.sh)

---

### Diagram Storage Verification

This test verifies that the system properly stores automatically generated diagrams in pre-configured locations.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should store generated diagrams in pre-configured locations
- Storage paths should be configurable
- Diagrams should be accessible after generation

##### Test Criteria
- Command exits with success (0) return code
- Diagrams are saved to the expected pre-configured locations
- Diagram files are properly formatted and accessible

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Store Automated Diagrams in Designated Locations](../UserRequirements.md#store-automated-diagrams-in-designated-locations)
  * trace: [tests/test-diagram-generation/test.sh](../../tests/test-diagram-generation/test.sh)

---

### Export Diagrams Verification

This test verifies that the system allows users to export generated diagrams in standard formats.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should export diagrams in standard formats (PNG, SVG, PDF)
- Export functionality should be user-accessible
- Exported diagrams should maintain visual quality and content

##### Test Criteria
- Command exits with success (0) return code
- Diagrams are exported in the requested format
- Exported files contain the expected diagram content

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Export Diagrams in Standard Formats](../UserRequirements.md#export-diagrams-in-standard-formats)
  * trace: [tests/test-diagram-generation/test.sh](../../tests/test-diagram-generation/test.sh)

---

### Highlight Changes in Diagrams Verification

This test verifies that the system provides an option to highlight changes made to the model in generated diagrams.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should highlight changes in the model when generating diagrams
- Highlighting should visually differentiate added, modified, and removed elements
- Change highlighting should be optional and user-configurable

##### Test Criteria
- Command exits with success (0) return code
- Changes in the model are visually highlighted in the diagrams
- Different types of changes have distinct visual indicators

##### Test Procedure
TODO: write test procedure

#### Relations
  * verify: [UserRequirements.md/Highlight Changes in Diagrams](../UserRequirements.md#highlight-changes-in-diagrams)
  * trace: [tests/test-diagram-generation/test.sh](../../tests/test-diagram-generation/test.sh)

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

##### Test Procedure
1. Create a test branch with modifications to requirements
2. Create a PR from the test branch to main
3. Merge the PR to trigger the workflow
4. Verify the workflow runs successfully
5. Verify diagrams are updated and committed to the main branch
6. Verify the traceability matrix SVG is generated and committed
7. Verify the commit message follows the expected format

#### Relations
  * verify: [SystemRequirements/Requirements.md/Automated Diagram Generation on PR Merge](../SystemRequirements/Requirements.md#automated-diagram-generation-on-pr-merge)
  * trace: [.github/workflows/generate_diagrams.yml](../../.github/workflows/generate_diagrams.yml)

---