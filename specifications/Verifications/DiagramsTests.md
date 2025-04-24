# Diagram Tests

This document verifies the requirements for ReqFlow's diagram generation functionality.

## Diagram Generation Tests
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  f75fd64834a53a9["Export Diagrams Verification"];
  click f75fd64834a53a9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/DiagramsTests.md#export-diagrams-verification";
  class f75fd64834a53a9 verification;
  3e72f83cabd0bad8["UserRequirements.md/Export Diagrams in Standard Formats"];
  class 3e72f83cabd0bad8 requirement;
  click 3e72f83cabd0bad8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  f75fd64834a53a9 -.->|verifies| 3e72f83cabd0bad8;
  b678046eb95fb241["tests/test-diagram-generation/test.sh"];
  class b678046eb95fb241 default;
  click b678046eb95fb241 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/tests/test-diagram-generation/test.sh";
  f75fd64834a53a9 -.->|trace| b678046eb95fb241;
  ea26b60dd62761cc["Diagram Generation Test"];
  click ea26b60dd62761cc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/DiagramsTests.md#diagram-generation-test";
  class ea26b60dd62761cc verification;
  64be2a98bd80a653["UserRequirements.md/Automate Diagram Generation"];
  class 64be2a98bd80a653 requirement;
  click 64be2a98bd80a653 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#automate-diagram-generation";
  ea26b60dd62761cc -.->|verifies| 64be2a98bd80a653;
  ea26b60dd62761cc -.->|trace| b678046eb95fb241;
  9c5bd4294c6701c6["Highlight Changes in Diagrams Verification"];
  click 9c5bd4294c6701c6 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/DiagramsTests.md#highlight-changes-in-diagrams-verification";
  class 9c5bd4294c6701c6 verification;
  d3a1b6b68298a744["UserRequirements.md/Highlight Changes in Diagrams"];
  class d3a1b6b68298a744 requirement;
  click d3a1b6b68298a744 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  9c5bd4294c6701c6 -.->|verifies| d3a1b6b68298a744;
  9c5bd4294c6701c6 -.->|trace| b678046eb95fb241;
  f4a75b18d7e9ae3["Visualize Model Relationships Verification"];
  click f4a75b18d7e9ae3 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/DiagramsTests.md#visualize-model-relationships-verification";
  class f4a75b18d7e9ae3 verification;
  37611ee8059e0f03["UserRequirements.md/Visualize Model Relationships"];
  class 37611ee8059e0f03 requirement;
  click 37611ee8059e0f03 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#visualize-model-relationships";
  f4a75b18d7e9ae3 -.->|verifies| 37611ee8059e0f03;
  f4a75b18d7e9ae3 -.->|trace| b678046eb95fb241;
  4c6d245c9314fe1f["Filter Relationships by Type Verification"];
  click 4c6d245c9314fe1f "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/DiagramsTests.md#filter-relationships-by-type-verification";
  class 4c6d245c9314fe1f verification;
  aaa09eb94d160979["UserRequirements.md/Filter Relationships by Type"];
  class aaa09eb94d160979 requirement;
  click aaa09eb94d160979 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#filter-relationships-by-type";
  4c6d245c9314fe1f -.->|verifies| aaa09eb94d160979;
  4c6d245c9314fe1f -.->|trace| b678046eb95fb241;
  f7cceb82cc5597b5["Diagram Storage Verification"];
  click f7cceb82cc5597b5 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/DiagramsTests.md#diagram-storage-verification";
  class f7cceb82cc5597b5 verification;
  e5f8f9f127a22da["UserRequirements.md/Store Automated Diagrams in Designated Locations"];
  class e5f8f9f127a22da requirement;
  click e5f8f9f127a22da "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  f7cceb82cc5597b5 -.->|verifies| e5f8f9f127a22da;
  f7cceb82cc5597b5 -.->|trace| b678046eb95fb241;
  e8ef9e868c2b1da8["Automated Diagram Generation on PR Merge Verification"];
  click e8ef9e868c2b1da8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/Verifications/DiagramsTests.md#automated-diagram-generation-on-pr-merge-verification";
  class e8ef9e868c2b1da8 verification;
  4b9934c3f9197112["SystemRequirements/Requirements.md/Automated Diagram Generation on PR Merge"];
  class 4b9934c3f9197112 requirement;
  click 4b9934c3f9197112 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#automated-diagram-generation-on-pr-merge";
  e8ef9e868c2b1da8 -.->|verifies| 4b9934c3f9197112;
  aae2507df9855ad7[".github/workflows/generate_diagrams.yml"];
  class aae2507df9855ad7 default;
  click aae2507df9855ad7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/.github/workflows/generate_diagrams.yml";
  e8ef9e868c2b1da8 -.->|trace| aae2507df9855ad7;
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
3. Run ReqFlow with the `--generate-diagrams` flag on the test fixtures
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