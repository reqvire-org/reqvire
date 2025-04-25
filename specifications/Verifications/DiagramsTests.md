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

  e680d84fe87126e9["Export Diagrams Verification"];
  click e680d84fe87126e9 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/DiagramsTests.md#export-diagrams-verification";
  class e680d84fe87126e9 verification;
  ff7932724ee600f1["UserRequirements.md/Export Diagrams in Standard Formats"];
  class ff7932724ee600f1 requirement;
  click ff7932724ee600f1 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  e680d84fe87126e9 -.->|verifies| ff7932724ee600f1;
  b296432506bb8be["tests/test-diagram-generation/test.sh"];
  class b296432506bb8be default;
  click b296432506bb8be "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-diagram-generation/test.sh";
  e680d84fe87126e9 -.->|trace| b296432506bb8be;
  f2df5cda6c7588de["Filter Relationships by Type Verification"];
  click f2df5cda6c7588de "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/DiagramsTests.md#filter-relationships-by-type-verification";
  class f2df5cda6c7588de verification;
  f3450185979ff229["UserRequirements.md/Filter Relationships by Type"];
  class f3450185979ff229 requirement;
  click f3450185979ff229 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#filter-relationships-by-type";
  f2df5cda6c7588de -.->|verifies| f3450185979ff229;
  f2df5cda6c7588de -.->|trace| b296432506bb8be;
  bc88dca7d6d0492a["Diagram Storage Verification"];
  click bc88dca7d6d0492a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/DiagramsTests.md#diagram-storage-verification";
  class bc88dca7d6d0492a verification;
  9276544d5ee17790["UserRequirements.md/Store Automated Diagrams in Designated Locations"];
  class 9276544d5ee17790 requirement;
  click 9276544d5ee17790 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  bc88dca7d6d0492a -.->|verifies| 9276544d5ee17790;
  bc88dca7d6d0492a -.->|trace| b296432506bb8be;
  6723e1748fa33d08["Automated Diagram Generation on PR Merge Verification"];
  click 6723e1748fa33d08 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/DiagramsTests.md#automated-diagram-generation-on-pr-merge-verification";
  class 6723e1748fa33d08 verification;
  273cce972e603178["SystemRequirements/Requirements.md/Automated Diagram Generation on PR Merge"];
  class 273cce972e603178 requirement;
  click 273cce972e603178 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#automated-diagram-generation-on-pr-merge";
  6723e1748fa33d08 -.->|verifies| 273cce972e603178;
  6699836c9fa45af6[".github/workflows/generate_diagrams.yml"];
  class 6699836c9fa45af6 default;
  click 6699836c9fa45af6 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/.github/workflows/generate_diagrams.yml";
  6723e1748fa33d08 -.->|trace| 6699836c9fa45af6;
  a8e581a16079b6a8["Diagram Generation Test"];
  click a8e581a16079b6a8 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/DiagramsTests.md#diagram-generation-test";
  class a8e581a16079b6a8 verification;
  4d4dad9ce307fade["UserRequirements.md/Automate Diagram Generation"];
  class 4d4dad9ce307fade requirement;
  click 4d4dad9ce307fade "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#automate-diagram-generation";
  a8e581a16079b6a8 -.->|verifies| 4d4dad9ce307fade;
  a8e581a16079b6a8 -.->|trace| b296432506bb8be;
  d11becf56f36d005["Visualize Model Relationships Verification"];
  click d11becf56f36d005 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/DiagramsTests.md#visualize-model-relationships-verification";
  class d11becf56f36d005 verification;
  e98d18ae3a41815a["UserRequirements.md/Visualize Model Relationships"];
  class e98d18ae3a41815a requirement;
  click e98d18ae3a41815a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#visualize-model-relationships";
  d11becf56f36d005 -.->|verifies| e98d18ae3a41815a;
  d11becf56f36d005 -.->|trace| b296432506bb8be;
  327e94cd7ded2a7a["Highlight Changes in Diagrams Verification"];
  click 327e94cd7ded2a7a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/DiagramsTests.md#highlight-changes-in-diagrams-verification";
  class 327e94cd7ded2a7a verification;
  efa8bb8c8484bb40["UserRequirements.md/Highlight Changes in Diagrams"];
  class efa8bb8c8484bb40 requirement;
  click efa8bb8c8484bb40 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  327e94cd7ded2a7a -.->|verifies| efa8bb8c8484bb40;
  327e94cd7ded2a7a -.->|trace| b296432506bb8be;
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