**Total Elements**: 1
**Total Relations**: 9
**Filtered From**: Model Diagram Generation

## Model Diagram Generation

**Type**: requirement
**File**: [specifications/SystemRequirements.md](specifications/SystemRequirements.md)

```mermaid
graph LR
  classDef userRequirement fill:#dbeafe,stroke:#2563EB,stroke-width:2px;
  classDef systemRequirement fill:#dbeafe,stroke:#2563EB,stroke-width:1px;
  classDef verification fill:#d1fae5,stroke:#059669,stroke-width:2px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  4f998d84bbf8f547["Model Diagram Generation"];
  class 4f998d84bbf8f547 systemRequirement;
  click 4f998d84bbf8f547 "specifications/SystemRequirements.md#model-diagram-generation";
  f27d93928246808["JSON Output Format"];
  class f27d93928246808 systemRequirement;
  click f27d93928246808 "specifications/SystemRequirements.md#json-output-format";
  4f998d84bbf8f547 -->|derive| f27d93928246808;
  f27d93928246808["JSON Output Format"];
  class f27d93928246808 systemRequirement;
  click f27d93928246808 "specifications/SystemRequirements.md#json-output-format";
  8cc3b7ebaf3ea9b["Output Format Test"];
  class 8cc3b7ebaf3ea9b verification;
  click 8cc3b7ebaf3ea9b "specifications/Verifications/Tests.md#output-format-test";
  f27d93928246808 -->|verifiedBy| 8cc3b7ebaf3ea9b;
  8cc3b7ebaf3ea9b["Output Format Test"];
  class 8cc3b7ebaf3ea9b verification;
  click 8cc3b7ebaf3ea9b "specifications/Verifications/Tests.md#output-format-test";
  1ca5a7c02ab1c5f4["Model Filtering Capability"];
  class 1ca5a7c02ab1c5f4 systemRequirement;
  click 1ca5a7c02ab1c5f4 "specifications/SystemRequirements.md#model-filtering-capability";
  4f998d84bbf8f547 -->|derive| 1ca5a7c02ab1c5f4;
  1ca5a7c02ab1c5f4["Model Filtering Capability"];
  class 1ca5a7c02ab1c5f4 systemRequirement;
  click 1ca5a7c02ab1c5f4 "specifications/SystemRequirements.md#model-filtering-capability";
  5b114dac21ad2026["Default Root Filtering"];
  class 5b114dac21ad2026 systemRequirement;
  click 5b114dac21ad2026 "specifications/SystemRequirements.md#default-root-filtering";
  1ca5a7c02ab1c5f4 -->|derive| 5b114dac21ad2026;
  5b114dac21ad2026["Default Root Filtering"];
  class 5b114dac21ad2026 systemRequirement;
  click 5b114dac21ad2026 "specifications/SystemRequirements.md#default-root-filtering";
  c28803f3ded267fb["Default Filtering Test"];
  class c28803f3ded267fb verification;
  click c28803f3ded267fb "specifications/Verifications/Tests.md#default-filtering-test";
  5b114dac21ad2026 -->|verifiedBy| c28803f3ded267fb;
  c28803f3ded267fb["Default Filtering Test"];
  class c28803f3ded267fb verification;
  click c28803f3ded267fb "specifications/Verifications/Tests.md#default-filtering-test";
  46fbef5d552a5c01["Forward Relation Traversal"];
  class 46fbef5d552a5c01 systemRequirement;
  click 46fbef5d552a5c01 "specifications/SystemRequirements.md#forward-relation-traversal";
  1ca5a7c02ab1c5f4 -->|derive| 46fbef5d552a5c01;
  46fbef5d552a5c01["Forward Relation Traversal"];
  class 46fbef5d552a5c01 systemRequirement;
  click 46fbef5d552a5c01 "specifications/SystemRequirements.md#forward-relation-traversal";
  15d847f8ebf34901["From Flag Filtering Test"];
  class 15d847f8ebf34901 verification;
  click 15d847f8ebf34901 "specifications/Verifications/Tests.md#from-flag-filtering-test";
  46fbef5d552a5c01 -->|verifiedBy| 15d847f8ebf34901;
  15d847f8ebf34901["From Flag Filtering Test"];
  class 15d847f8ebf34901 verification;
  click 15d847f8ebf34901 "specifications/Verifications/Tests.md#from-flag-filtering-test";
  e80d22d575e02537["Model Generation Test"];
  class e80d22d575e02537 verification;
  click e80d22d575e02537 "specifications/Verifications/Tests.md#model-generation-test";
  4f998d84bbf8f547 -->|verifiedBy| e80d22d575e02537;
  e80d22d575e02537["Model Generation Test"];
  class e80d22d575e02537 verification;
  click e80d22d575e02537 "specifications/Verifications/Tests.md#model-generation-test";
```
