# Requirements

This is a test requirements document for links to files in subfolders without fragments.

## Section 1

```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef satisfies fill:#fff2cc,stroke:#ffcc00,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef paragraph fill:#efefef,stroke:#999999,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

    REQ_001["REQ-001"];
    click REQ_001 "Requirements.md#req-001";
    class REQ_001 requirement;
    _Status_DSD__DesignSpecifications_Status_md_ -->|satisfies| REQ_001;
    _Status_DSD__DesignSpecifications_Status_md_["Status DSD"];
    click _Status_DSD__DesignSpecifications_Status_md_ "[Status DSD](DesignSpecifications/Status.md)";
    class _Status_DSD__DesignSpecifications_Status_md_ satisfies;
    REQ_001 -->|relates to| _Architecture_DSD__DesignSpecifications_Architecture_md_;
    _Architecture_DSD__DesignSpecifications_Architecture_md_["Architecture DSD"];
    click _Architecture_DSD__DesignSpecifications_Architecture_md_ "[Architecture DSD](DesignSpecifications/Architecture.md)";
    class _Architecture_DSD__DesignSpecifications_Architecture_md_ requirement;
    REQ_002["REQ-002"];
    click REQ_002 "Requirements.md#req-002";
    class REQ_002 requirement;
    _REQ_001___req_001_ -->|traces| REQ_002;
    _REQ_001___req_001_["REQ-001"];
    click _REQ_001___req_001_ "#req-001";
    class _REQ_001___req_001_ requirement;
```


### REQ-001

First test requirement referencing a file in a subfolder.

#### Relations
  * satisfiedBy: [Status DSD](DesignSpecifications/Status.md)
  * verifies: [Architecture DSD](DesignSpecifications/Architecture.md)

### REQ-002

Second test requirement with a local reference.

#### Relations
  * tracedFrom: [REQ-001](#req-001)