# Requirements

This is a requirements document specifically created for testing diagram generation.

### Custom Root Requirement

This is the root requirement for testing purposes.

#### Metadata
  * type: user-requirement

#### Relations
  * satisfiedBy: [root_implementation.py](root_implementation.py)
---

```mermaid
graph TD;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  d94b2c1859["Custom Element 2"];
  click d94b2c1859 "Requirements-with-custom-mermaid.md#custom-element-2";
  class d94b2c1859 requirement;
  2ac7edcf81["Custom Element 1"];
  click 2ac7edcf81 "Requirements-with-custom-mermaid.md#custom-element-1";
  class 2ac7edcf81 requirement;
  87ce6a6132["Custom Element 3"];
  class 87ce6a6132 requirement;
  click 87ce6a6132 "Requirements-with-custom-mermaid.md#custom-element-3";
  2ac7edcf81 -->|verifies| 87ce6a6132;
```


### Custom Element 1

This is a test verification element with relations.

#### Metadata
  * type: verification

#### Relations
  * satisfiedBy: [test_implementation.py](test_implementation.py)
  * verify: [Custom Element 3](#custom-element-3)
---

### Custom Element 2

This is another test element with relations.

#### Relations
  * derivedFrom: [Custom Root Requirement](#custom-root-requirement)
  * verifiedBy: [Custom Element 1](#custom-element-1)
  * satisfiedBy: [implementation.py](implementation.py)
  * derive: [Custom Element 3](#custom-element-3)
  * derive: [Custom Element 4](#custom-element-4)
---

This paragraph contains a custom mermaid diagram - should not be removed:

```mermaid
flowchart TD
    A[Start] --> B{Is it?}
    B -- Yes --> C[OK]
    C --> D[Rethink]
    D --> B
    B -- No --> E[End]
```

This section contains BOTH a custom mermaid diagram AND elements - custom diagram should be preserved:

```mermaid
flowchart LR
    X[Custom Start] --> Y[Custom End]
```

### Custom Diagram Element

This element is in a section with a custom diagram.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Custom Root Requirement](#custom-root-requirement)
---

```mermaid
flowchart TB
    P[Custom Process] --> Q[Custom Output]
```

This section has custom diagram immediately after header - it should be preserved.

### Header Diagram Element

This element is in a section with custom diagram right after header.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Custom Root Requirement](#custom-root-requirement)
---


### Custom Element 3

This is a third test element.

#### Relations
  * derivedFrom: [Custom Element 2](#custom-element-2)
  * verifiedBy: [Custom Element 1](#custom-element-1)
  * trace: [Custom Element 4](#custom-element-4)
---

### Custom Element 4

This is a fourth test element with relations.

#### Relations
  * derivedFrom: [Custom Element 2](#custom-element-2)
  * trace: [Custom Element 1](#custom-element-1)
---
