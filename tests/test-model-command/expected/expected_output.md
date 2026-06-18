**Total Elements**: 2
**Total Relations**: 17
**Direction**: Forward

## [Model Command Ontology](specifications/Ontologies.md#model-command-ontology)

**Type**: ontology
**File**: [specifications/Ontologies.md](specifications/Ontologies.md)

## [Test Capability Test Model Command Specifications Capabilities Md](specifications/Capabilities.md#test-capability-test-model-command-specifications-capabilities-md)

**Type**: capability
**File**: [specifications/Capabilities.md](specifications/Capabilities.md)

```mermaid
graph TD
  classDef capability fill:#BBDEFB,stroke:#1976D2,stroke-width:2.5px;
  classDef systemRequirement fill:#E1D8EE,stroke:#673AB7,stroke-width:1.5px;
  classDef ontology fill:#F4E3A1,stroke:#B08A00,stroke-width:2px;
  classDef verification fill:#DCEDC8,stroke:#4CAF50,stroke-width:2px;
  classDef default fill:#F5F5F5,stroke:#424242,stroke-width:1.5px;

  classDef folder fill:#FAFAFA,stroke:#9E9E9E,stroke-width:3px;
  classDef file fill:#FFF8E1,stroke:#FFCA28,stroke-width:2px;

  subgraph 9d68b769fcbc79d0["📁 specifications"]
    subgraph afe26fd45621e6a["📄 Capabilities.md"]
      458b8fd2c32e6014["Model Structure Exploration"];
      class 458b8fd2c32e6014 systemRequirement;
      click 458b8fd2c32e6014 "specifications/Capabilities.md#model-structure-exploration";
      8c7466173ee6b05d["Test Capability Test Model Command Specifications Capabilities Md"];
      class 8c7466173ee6b05d capability;
      click 8c7466173ee6b05d "specifications/Capabilities.md#test-capability-test-model-command-specifications-capabilities-md";
    end
    subgraph 77808752d543f615["📄 SystemRequirements.md"]
      f352ca56d3ce0fdd["Default Model Roots"];
      class f352ca56d3ce0fdd systemRequirement;
      click f352ca56d3ce0fdd "specifications/SystemRequirements.md#default-model-roots";
      f7eb2f9d9cd7bb11["Forward Relation Traversal"];
      class f7eb2f9d9cd7bb11 systemRequirement;
      click f7eb2f9d9cd7bb11 "specifications/SystemRequirements.md#forward-relation-traversal";
      5abb4a3caae293d9["JSON Output Format"];
      class 5abb4a3caae293d9 systemRequirement;
      click 5abb4a3caae293d9 "specifications/SystemRequirements.md#json-output-format";
      25879bdc5e196bec["Markdown Output Format"];
      class 25879bdc5e196bec systemRequirement;
      click 25879bdc5e196bec "specifications/SystemRequirements.md#markdown-output-format";
      b7ec4bb3813f1dea["Model Diagram Generation"];
      class b7ec4bb3813f1dea systemRequirement;
      click b7ec4bb3813f1dea "specifications/SystemRequirements.md#model-diagram-generation";
      836c732a54d7f48f["Model Filtering Capability"];
      class 836c732a54d7f48f systemRequirement;
      click 836c732a54d7f48f "specifications/SystemRequirements.md#model-filtering-capability";
      28d69f5dca868721["Pure Mermaid Output Format"];
      class 28d69f5dca868721 systemRequirement;
      click 28d69f5dca868721 "specifications/SystemRequirements.md#pure-mermaid-output-format";
    end
  end
  subgraph 9f0649e759bd2822["📁 specifications/Verifications"]
    subgraph 7d9dffd79ed1d5b8["📄 Tests.md"]
      a2368d1b4b67f0d7["Default Filtering Test"];
      class a2368d1b4b67f0d7 verification;
      click a2368d1b4b67f0d7 "specifications/Verifications/Tests.md#default-filtering-test";
      bbd610799ac8e00f["Filter Type Test"];
      class bbd610799ac8e00f verification;
      click bbd610799ac8e00f "specifications/Verifications/Tests.md#filter-type-test";
      293200814c46cd0d["From Flag Filtering Test"];
      class 293200814c46cd0d verification;
      click 293200814c46cd0d "specifications/Verifications/Tests.md#from-flag-filtering-test";
      6e0e2613c4bfcfcb["Model Generation Test"];
      class 6e0e2613c4bfcfcb verification;
      click 6e0e2613c4bfcfcb "specifications/Verifications/Tests.md#model-generation-test";
      2f7b3b3deb29891d["Output Format Test"];
      class 2f7b3b3deb29891d verification;
      click 2f7b3b3deb29891d "specifications/Verifications/Tests.md#output-format-test";
      fb891ab92c9824aa["Reverse Traversal Test"];
      class fb891ab92c9824aa verification;
      click fb891ab92c9824aa "specifications/Verifications/Tests.md#reverse-traversal-test";
    end
  end
  8c7466173ee6b05d -->|specifiedBy| 458b8fd2c32e6014;
  458b8fd2c32e6014 -->|derive| 25879bdc5e196bec;
  25879bdc5e196bec -->|verifiedBy| 2f7b3b3deb29891d;
  458b8fd2c32e6014 -->|derive| b7ec4bb3813f1dea;
  b7ec4bb3813f1dea -->|derive| 5abb4a3caae293d9;
  b7ec4bb3813f1dea -->|derive| 836c732a54d7f48f;
  836c732a54d7f48f -->|derive| f352ca56d3ce0fdd;
  f352ca56d3ce0fdd -->|verifiedBy| a2368d1b4b67f0d7;
  836c732a54d7f48f -->|derive| f7eb2f9d9cd7bb11;
  f7eb2f9d9cd7bb11 -->|verifiedBy| 293200814c46cd0d;
  f7eb2f9d9cd7bb11 -->|verifiedBy| fb891ab92c9824aa;
  836c732a54d7f48f -->|verifiedBy| bbd610799ac8e00f;
  b7ec4bb3813f1dea -->|verifiedBy| 6e0e2613c4bfcfcb;
  458b8fd2c32e6014 -->|derive| 28d69f5dca868721;
```
