# Test Requirements

```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef satisfies fill:#fff2cc,stroke:#ffcc00,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef paragraph fill:#efefef,stroke:#999999,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  subgraph para1["Section 1"];
    Element_1["Element 1"];
    click Element_1 "Requirements.md#element-1";
    class Element_1 requirement;
    Element_1 -->|relates to| _Element_2___element_2_;
  _Element_2___element_2_["[Element 2](#element-2)"];
  class _Element_2___element_2_ externalLink;
    Element_1 -->|relates to| _Element_3___element_3_;
  _Element_3___element_3_["[Element 3](#element-3)"];
  class _Element_3___element_3_ externalLink;
    Element_2["Element 2"];
    click Element_2 "Requirements.md#element-2";
    class Element_2 requirement;
    Element_2 -->|traces from| _Element_1___element_1_;
  _Element_1___element_1_["[Element 1](#element-1)"];
  class _Element_1___element_1_ externalLink;
  end;
  class para1 paragraph;
  subgraph para2["Section 2"];
    Element_3["Element 3"];
    click Element_3 "Requirements.md#element-3";
    class Element_3 requirement;
    Element_3 -->|verifies| _Element_1___element_1_;
    Element_4["Element 4"];
    click Element_4 "Requirements.md#element-4";
    class Element_4 requirement;
  end;
  class para2
<!-- ReqFlow-generated diagram - do not modify manually -->
 paragraph;
```



This is a requirements document specifically created for testing diagram generation.

## Section 1

### Element 1

This is a test element with relations.

#### Relations
  * satisfies: [Element 2](#element-2)
  * verifies: [Element 3](#element-3)

### Element 2

This is another test element with relations.

#### Relations
  * tracedFrom: [Element 1](#element-1)

## Section 2

### Element 3

This is a third test element.

#### Relations
  * verifiedBy: [Element 1](#element-1)

### Element 4

This is a fourth test element with no relations.