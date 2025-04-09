# Change Impact Detection System Requirements

This document defines the detailed requirements for the change impact detection algorithm and visualization in ReqFlow. It provides the technical specifications for implementing the concepts described in the Design Specification Document for Requirements Change Impact Propagation.

## Change Impact Detection Components
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  503dda29a8["Change Impact Detection Algorithm"];
  click 503dda29a8 "ChangeImpactRequirements.md#change-impact-detection-algorithm";
  class 503dda29a8 requirement;
  33e6fb86b3["DesignSpecifications/RequirementsChangePropagation.md"];
  class 33e6fb86b3 default;
  click 33e6fb86b3 "../DesignSpecifications/RequirementsChangePropagation.md";
  503dda29a8 -.->|deriveReqT| 33e6fb86b3;
  cc976e6bcd["../../core/src/change_impact.rs"];
  class cc976e6bcd default;
  click cc976e6bcd "../../core/src/change_impact.rs";
  cc976e6bcd -->|satisfies| 503dda29a8;
  401764f61b["Change Impact Relations Test"];
  class 401764f61b requirement;
  click 401764f61b "../Verifications/ChangeImpactTests.md#change-impact-relations-test";
  401764f61b -->|verifies| 503dda29a8;
  9102eb42eb["Change Impact Detection Test"];
  class 9102eb42eb requirement;
  click 9102eb42eb "../Verifications/ChangeImpactTests.md#change-impact-detection-test";
  9102eb42eb -->|verifies| 503dda29a8;
  9d6f79f601["Change Impact Command Line Interface"];
  click 9d6f79f601 "ChangeImpactRequirements.md#change-impact-command-line-interface";
  class 9d6f79f601 requirement;
  665b7456c9["CLI Change Impact Report Flag"];
  class 665b7456c9 requirement;
  click 665b7456c9 "Requirements.md#cli-change-impact-report-flag";
  9d6f79f601 -.->|deriveReqT| 665b7456c9;
  11ffc4632a["../../cli/src/cli.rs"];
  class 11ffc4632a default;
  click 11ffc4632a "../../cli/src/cli.rs";
  11ffc4632a -->|satisfies| 9d6f79f601;
  401764f61b -->|verifies| 9d6f79f601;
  9102eb42eb -->|verifies| 9d6f79f601;
  9e4af55877["Change Impact Visualization"];
  click 9e4af55877 "ChangeImpactRequirements.md#change-impact-visualization";
  class 9e4af55877 requirement;
  9e4af55877 -.->|deriveReqT| 33e6fb86b3;
  cc976e6bcd -->|satisfies| 9e4af55877;
```

---

### Change Impact Detection Algorithm

The system shall implement a requirement change detection algorithm that identifies changes between versions of the model and determines their impact through relationship traversal.

#### Details

The algorithm shall consist of the following steps:

1. **Diff Analysis**:
   - Compare the current state of a requirement with its previous state
   - Identify structural changes (additions, deletions, modifications)
   - Generate a ChangeSet representing all detected changes
   - Associate changes with specific elements in the model

2. **Impact Determination**:
   - For each changed element, identify all relations from the element
   - Apply relation-specific propagation rules as defined in RelationTypesRegistry.md
   - Consider the relation direction and change impact direction for each relation
   - Build an impact tree representing the propagation of changes

3. **Recursive Traversal**:
   - Perform a depth-first traversal of relationships
   - Create a directed acyclic graph (DAG) of change impact
   - Handle circular dependencies by preventing infinite recursion
   - Track visited nodes to prevent duplicate processing

4. **Impact Classification**:
   - Assign impact severity levels based on relation types
   - Classify changes as:
     - Direct: Changes to the element itself
     - Indirect: Changes propagated from related elements
     - Potential: Changes that might affect an element based on semantic analysis
   - Calculate aggregated impact scores for each affected element

5. **Performance Optimization**:
   - Implement caching of traversal results
   - Use parallel processing for independent branches of the impact tree
   - Apply pruning techniques to limit traversal depth when appropriate
   - Support incremental impact analysis for large models

#### Relations
  * derivedFrom: [DesignSpecifications/RequirementsChangePropagation.md](../../specifications/DesignSpecifications/RequirementsChangePropagation.md)
  * satisfiedBy: [../../core/src/change_impact.rs](../../core/src/change_impact.rs)

---

### Change Impact Visualization

The system shall provide visual representations of change impact to help users understand the scope and implications of changes.

#### Details

The visualization shall include:

1. **Tree View**:
   - Display a hierarchical tree of affected elements
   - Group elements by impact type (direct, indirect, potential)
   - Show relation types between elements
   - Support collapsing/expanding nodes for better navigation

2. **Color Coding**:
   - Use consistent color scheme for impact types:
     - Direct impacts: Red
     - Indirect impacts: Yellow
     - Potential impacts: Blue
   - Indicate relation types with different line styles
   - Highlight newly introduced or removed relationships

3. **Interactive Elements**:
   - Allow clicking on elements to focus the view
   - Provide filtering options for relation types
   - Support search functionality within impact results
   - Enable toggling between different visualization modes

4. **Summary Statistics**:
   - Display counts of affected elements by type
   - Show metrics for impact breadth and depth
   - Calculate change propagation fan-out metrics
   - Generate overall change impact assessment

#### Relations
  * derivedFrom: [DesignSpecifications/RequirementsChangePropagation.md](../../specifications/DesignSpecifications/RequirementsChangePropagation.md)
  * satisfiedBy: [../../core/src/change_impact.rs](../../core/src/change_impact.rs)

---

### Change Impact Command Line Interface

The system shall provide a command-line interface for initiating change impact analysis and controlling output formats.

#### Details

The CLI shall support the following functionality:

1. **Analysis Invocation**:
   - Support analyzing changes between git commits
   - Enable specifying elements to analyze by ID or pattern
   - Allow limiting analysis to specific relation types
   - Support depth limitations for large models

2. **Output Formats**:
   - Generate formatted text reports
   - Produce JSON-structured impact data
   - Create Mermaid diagrams of impact trees
   - Integrate with HTML report generation

3. **Filtering Options**:
   - Filter by element types (requirement, verification, etc.)
   - Filter by relation types
   - Filter by impact severity
   - Support inclusion/exclusion patterns

4. **Integration Points**:
   - Support integration with CI/CD pipelines
   - Enable calling from external systems via API
   - Support webhook triggers for automated analysis
   - Allow scripting of analysis operations

#### Relations
  * derivedFrom: [CLI Change Impact Report Flag](../SystemRequirements/Requirements.md#cli-change-impact-report-flag)
  * satisfiedBy: [../../cli/src/cli.rs](../../cli/src/cli.rs)

---

## Change Impact Test Requirements
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  9257bdbe19["End-to-End Change Impact Tests"];
  click 9257bdbe19 "ChangeImpactRequirements.md#end-to-end-change-impact-tests";
  class 9257bdbe19 requirement;
  33e6fb86b3["DesignSpecifications/RequirementsChangePropagation.md"];
  class 33e6fb86b3 default;
  click 33e6fb86b3 "../DesignSpecifications/RequirementsChangePropagation.md";
  9257bdbe19 -.->|deriveReqT| 33e6fb86b3;
  524acc7470["tests/test-change-impact-detection/test.sh"];
  class 524acc7470 default;
  click 524acc7470 "../../tests/test-change-impact-detection/test.sh";
  524acc7470 -->|verifies| 9257bdbe19;
  a4d4b4ced7["Relation Type Impact Tests"];
  click a4d4b4ced7 "ChangeImpactRequirements.md#relation-type-impact-tests";
  class a4d4b4ced7 requirement;
  a4d4b4ced7 -.->|deriveReqT| 33e6fb86b3;
  524acc7470 -->|verifies| a4d4b4ced7;
```

---