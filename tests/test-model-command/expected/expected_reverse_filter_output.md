**Total Elements**: 6
**Total Relations**: 14
**Direction**: Reverse
**Type Filter**: test-verification

## [Default Filtering Test](specifications/Verifications/Tests.md#default-filtering-test)

**Type**: test-verification
**File**: [specifications/Verifications/Tests.md](specifications/Verifications/Tests.md)

```mermaid
graph LR
  classDef userRequirement fill:#D1C4E9,stroke:#7E57C2,stroke-width:2px;
  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;
  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;
  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;

  classDef folder fill:#FAFAFA,stroke:#9E9E9E,stroke-width:3px;
  classDef file fill:#FFF8E1,stroke:#FFCA28,stroke-width:2px;

  subgraph b4c308bac4df6d65["📁 specifications"]
    subgraph d76ffb0335b04f31["📄 SystemRequirements.md"]
      5b114dac21ad2026["Default Root Filtering"];
      class 5b114dac21ad2026 systemRequirement;
      click 5b114dac21ad2026 "specifications/SystemRequirements.md#default-root-filtering";
      4f998d84bbf8f547["Model Diagram Generation"];
      class 4f998d84bbf8f547 systemRequirement;
      click 4f998d84bbf8f547 "specifications/SystemRequirements.md#model-diagram-generation";
      1ca5a7c02ab1c5f4["Model Filtering Capability"];
      class 1ca5a7c02ab1c5f4 systemRequirement;
      click 1ca5a7c02ab1c5f4 "specifications/SystemRequirements.md#model-filtering-capability";
    end
    subgraph c057e38f409f215b["📄 UserRequirements.md"]
      32fb52886d6166a1["Model Structure Exploration"];
      class 32fb52886d6166a1 userRequirement;
      click 32fb52886d6166a1 "specifications/UserRequirements.md#model-structure-exploration";
    end
  end
  subgraph 186822cd467e0417["📁 specifications/Verifications"]
    subgraph b64358bba6ee017f["📄 Tests.md"]
      c28803f3ded267fb["Default Filtering Test"];
      class c28803f3ded267fb verification;
      click c28803f3ded267fb "specifications/Verifications/Tests.md#default-filtering-test";
    end
  end
  c28803f3ded267fb -->|verify| 5b114dac21ad2026;
  5b114dac21ad2026 -->|derivedFrom| 1ca5a7c02ab1c5f4;
  1ca5a7c02ab1c5f4 -->|derivedFrom| 4f998d84bbf8f547;
  4f998d84bbf8f547 -->|derivedFrom| 32fb52886d6166a1;
```

## [Filter Type Test](specifications/Verifications/Tests.md#filter-type-test)

**Type**: test-verification
**File**: [specifications/Verifications/Tests.md](specifications/Verifications/Tests.md)

```mermaid
graph LR
  classDef userRequirement fill:#D1C4E9,stroke:#7E57C2,stroke-width:2px;
  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;
  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;
  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;

  classDef folder fill:#FAFAFA,stroke:#9E9E9E,stroke-width:3px;
  classDef file fill:#FFF8E1,stroke:#FFCA28,stroke-width:2px;

  subgraph b4c308bac4df6d65["📁 specifications"]
    subgraph d76ffb0335b04f31["📄 SystemRequirements.md"]
      4f998d84bbf8f547["Model Diagram Generation"];
      class 4f998d84bbf8f547 systemRequirement;
      click 4f998d84bbf8f547 "specifications/SystemRequirements.md#model-diagram-generation";
      1ca5a7c02ab1c5f4["Model Filtering Capability"];
      class 1ca5a7c02ab1c5f4 systemRequirement;
      click 1ca5a7c02ab1c5f4 "specifications/SystemRequirements.md#model-filtering-capability";
    end
    subgraph c057e38f409f215b["📄 UserRequirements.md"]
      32fb52886d6166a1["Model Structure Exploration"];
      class 32fb52886d6166a1 userRequirement;
      click 32fb52886d6166a1 "specifications/UserRequirements.md#model-structure-exploration";
    end
  end
  subgraph 186822cd467e0417["📁 specifications/Verifications"]
    subgraph b64358bba6ee017f["📄 Tests.md"]
      e712dc05f32bec2c["Filter Type Test"];
      class e712dc05f32bec2c verification;
      click e712dc05f32bec2c "specifications/Verifications/Tests.md#filter-type-test";
    end
  end
  e712dc05f32bec2c -->|verify| 1ca5a7c02ab1c5f4;
  1ca5a7c02ab1c5f4 -->|derivedFrom| 4f998d84bbf8f547;
  4f998d84bbf8f547 -->|derivedFrom| 32fb52886d6166a1;
```

## [From Flag Filtering Test](specifications/Verifications/Tests.md#from-flag-filtering-test)

**Type**: test-verification
**File**: [specifications/Verifications/Tests.md](specifications/Verifications/Tests.md)

```mermaid
graph LR
  classDef userRequirement fill:#D1C4E9,stroke:#7E57C2,stroke-width:2px;
  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;
  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;
  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;

  classDef folder fill:#FAFAFA,stroke:#9E9E9E,stroke-width:3px;
  classDef file fill:#FFF8E1,stroke:#FFCA28,stroke-width:2px;

  subgraph b4c308bac4df6d65["📁 specifications"]
    subgraph d76ffb0335b04f31["📄 SystemRequirements.md"]
      46fbef5d552a5c01["Forward Relation Traversal"];
      class 46fbef5d552a5c01 systemRequirement;
      click 46fbef5d552a5c01 "specifications/SystemRequirements.md#forward-relation-traversal";
      4f998d84bbf8f547["Model Diagram Generation"];
      class 4f998d84bbf8f547 systemRequirement;
      click 4f998d84bbf8f547 "specifications/SystemRequirements.md#model-diagram-generation";
      1ca5a7c02ab1c5f4["Model Filtering Capability"];
      class 1ca5a7c02ab1c5f4 systemRequirement;
      click 1ca5a7c02ab1c5f4 "specifications/SystemRequirements.md#model-filtering-capability";
    end
    subgraph c057e38f409f215b["📄 UserRequirements.md"]
      32fb52886d6166a1["Model Structure Exploration"];
      class 32fb52886d6166a1 userRequirement;
      click 32fb52886d6166a1 "specifications/UserRequirements.md#model-structure-exploration";
    end
  end
  subgraph 186822cd467e0417["📁 specifications/Verifications"]
    subgraph b64358bba6ee017f["📄 Tests.md"]
      15d847f8ebf34901["From Flag Filtering Test"];
      class 15d847f8ebf34901 verification;
      click 15d847f8ebf34901 "specifications/Verifications/Tests.md#from-flag-filtering-test";
    end
  end
  15d847f8ebf34901 -->|verify| 46fbef5d552a5c01;
  46fbef5d552a5c01 -->|derivedFrom| 1ca5a7c02ab1c5f4;
  1ca5a7c02ab1c5f4 -->|derivedFrom| 4f998d84bbf8f547;
  4f998d84bbf8f547 -->|derivedFrom| 32fb52886d6166a1;
```

## [Model Generation Test](specifications/Verifications/Tests.md#model-generation-test)

**Type**: test-verification
**File**: [specifications/Verifications/Tests.md](specifications/Verifications/Tests.md)

```mermaid
graph LR
  classDef userRequirement fill:#D1C4E9,stroke:#7E57C2,stroke-width:2px;
  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;
  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;
  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;

  classDef folder fill:#FAFAFA,stroke:#9E9E9E,stroke-width:3px;
  classDef file fill:#FFF8E1,stroke:#FFCA28,stroke-width:2px;

  subgraph b4c308bac4df6d65["📁 specifications"]
    subgraph d76ffb0335b04f31["📄 SystemRequirements.md"]
      4f998d84bbf8f547["Model Diagram Generation"];
      class 4f998d84bbf8f547 systemRequirement;
      click 4f998d84bbf8f547 "specifications/SystemRequirements.md#model-diagram-generation";
    end
    subgraph c057e38f409f215b["📄 UserRequirements.md"]
      32fb52886d6166a1["Model Structure Exploration"];
      class 32fb52886d6166a1 userRequirement;
      click 32fb52886d6166a1 "specifications/UserRequirements.md#model-structure-exploration";
    end
  end
  subgraph 186822cd467e0417["📁 specifications/Verifications"]
    subgraph b64358bba6ee017f["📄 Tests.md"]
      e80d22d575e02537["Model Generation Test"];
      class e80d22d575e02537 verification;
      click e80d22d575e02537 "specifications/Verifications/Tests.md#model-generation-test";
    end
  end
  e80d22d575e02537 -->|verify| 4f998d84bbf8f547;
  4f998d84bbf8f547 -->|derivedFrom| 32fb52886d6166a1;
```

## [Output Format Test](specifications/Verifications/Tests.md#output-format-test)

**Type**: test-verification
**File**: [specifications/Verifications/Tests.md](specifications/Verifications/Tests.md)

```mermaid
graph LR
  classDef userRequirement fill:#D1C4E9,stroke:#7E57C2,stroke-width:2px;
  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;
  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;
  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;

  classDef folder fill:#FAFAFA,stroke:#9E9E9E,stroke-width:3px;
  classDef file fill:#FFF8E1,stroke:#FFCA28,stroke-width:2px;

  subgraph b4c308bac4df6d65["📁 specifications"]
    subgraph d76ffb0335b04f31["📄 SystemRequirements.md"]
      f27d93928246808["JSON Output Format"];
      class f27d93928246808 systemRequirement;
      click f27d93928246808 "specifications/SystemRequirements.md#json-output-format";
      8accb4e2c9363546["Markdown Output Format"];
      class 8accb4e2c9363546 systemRequirement;
      click 8accb4e2c9363546 "specifications/SystemRequirements.md#markdown-output-format";
      4f998d84bbf8f547["Model Diagram Generation"];
      class 4f998d84bbf8f547 systemRequirement;
      click 4f998d84bbf8f547 "specifications/SystemRequirements.md#model-diagram-generation";
    end
    subgraph c057e38f409f215b["📄 UserRequirements.md"]
      32fb52886d6166a1["Model Structure Exploration"];
      class 32fb52886d6166a1 userRequirement;
      click 32fb52886d6166a1 "specifications/UserRequirements.md#model-structure-exploration";
    end
  end
  subgraph 186822cd467e0417["📁 specifications/Verifications"]
    subgraph b64358bba6ee017f["📄 Tests.md"]
      8cc3b7ebaf3ea9b["Output Format Test"];
      class 8cc3b7ebaf3ea9b verification;
      click 8cc3b7ebaf3ea9b "specifications/Verifications/Tests.md#output-format-test";
    end
  end
  8cc3b7ebaf3ea9b -->|verify| f27d93928246808;
  f27d93928246808 -->|derivedFrom| 4f998d84bbf8f547;
  4f998d84bbf8f547 -->|derivedFrom| 32fb52886d6166a1;
  8cc3b7ebaf3ea9b -->|verify| 8accb4e2c9363546;
```

## [Reverse Traversal Test](specifications/Verifications/Tests.md#reverse-traversal-test)

**Type**: test-verification
**File**: [specifications/Verifications/Tests.md](specifications/Verifications/Tests.md)

```mermaid
graph LR
  classDef userRequirement fill:#D1C4E9,stroke:#7E57C2,stroke-width:2px;
  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;
  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;
  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;

  classDef folder fill:#FAFAFA,stroke:#9E9E9E,stroke-width:3px;
  classDef file fill:#FFF8E1,stroke:#FFCA28,stroke-width:2px;

  subgraph b4c308bac4df6d65["📁 specifications"]
    subgraph d76ffb0335b04f31["📄 SystemRequirements.md"]
      46fbef5d552a5c01["Forward Relation Traversal"];
      class 46fbef5d552a5c01 systemRequirement;
      click 46fbef5d552a5c01 "specifications/SystemRequirements.md#forward-relation-traversal";
      4f998d84bbf8f547["Model Diagram Generation"];
      class 4f998d84bbf8f547 systemRequirement;
      click 4f998d84bbf8f547 "specifications/SystemRequirements.md#model-diagram-generation";
      1ca5a7c02ab1c5f4["Model Filtering Capability"];
      class 1ca5a7c02ab1c5f4 systemRequirement;
      click 1ca5a7c02ab1c5f4 "specifications/SystemRequirements.md#model-filtering-capability";
    end
    subgraph c057e38f409f215b["📄 UserRequirements.md"]
      32fb52886d6166a1["Model Structure Exploration"];
      class 32fb52886d6166a1 userRequirement;
      click 32fb52886d6166a1 "specifications/UserRequirements.md#model-structure-exploration";
    end
  end
  subgraph 186822cd467e0417["📁 specifications/Verifications"]
    subgraph b64358bba6ee017f["📄 Tests.md"]
      a418de0940661a72["Reverse Traversal Test"];
      class a418de0940661a72 verification;
      click a418de0940661a72 "specifications/Verifications/Tests.md#reverse-traversal-test";
    end
  end
  a418de0940661a72 -->|verify| 46fbef5d552a5c01;
  46fbef5d552a5c01 -->|derivedFrom| 1ca5a7c02ab1c5f4;
  1ca5a7c02ab1c5f4 -->|derivedFrom| 4f998d84bbf8f547;
  4f998d84bbf8f547 -->|derivedFrom| 32fb52886d6166a1;
```
