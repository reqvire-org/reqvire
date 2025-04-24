# User stories

## Personas
1. System Engineer: Focused on managing system models, ensuring alignment with project requirements, and validating structures.  
2. SOI Developer: Implements features and makes system changes based on MBSE models, ensuring consistency between design and code.  
3. Contributor: An external community member contributing to ReqFlow by improving models, creating features, or providing feedback.  
4. Manager: Oversees the MBSE processes, tracks progress, ensures alignment with objectives, and generates reports for decision-making.  

---

## User Stories
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  cf0026c2feeb1e0f["Generate Diagrams"];
  click cf0026c2feeb1e0f "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#generate-diagrams";
  class cf0026c2feeb1e0f requirement;
  f6237e49f7494c0e["MOEs.md/MOE_UA"];
  class f6237e49f7494c0e requirement;
  click f6237e49f7494c0e "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/MOEs.md#moe_ua";
  cf0026c2feeb1e0f -.->|trace| f6237e49f7494c0e;
  100197ce818804["Export Specifications"];
  click 100197ce818804 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#export-specifications";
  class 100197ce818804 requirement;
  100197ce818804 -.->|trace| f6237e49f7494c0e;
  cdc9a9c9087e707f["Fostering Community Contributions"];
  click cdc9a9c9087e707f "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#fostering-community-contributions";
  class cdc9a9c9087e707f requirement;
  b3f57e49f7494c0e["MOEs.md/MOE_CE"];
  class b3f57e49f7494c0e requirement;
  click b3f57e49f7494c0e "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/MOEs.md#moe_ce";
  cdc9a9c9087e707f -.->|trace| b3f57e49f7494c0e;
  ba120b7cafc5435f["Integrate with GitHub Workflows"];
  click ba120b7cafc5435f "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#integrate-with-github-workflows";
  class ba120b7cafc5435f requirement;
  ba120b7cafc5435f -.->|trace| b3f57e49f7494c0e;
  2b35b7b37d52d4e6["AI-Assisted MBSE Model Management"];
  click 2b35b7b37d52d4e6 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#ai-assisted-mbse-model-management";
  class 2b35b7b37d52d4e6 requirement;
  852dea6cfecb47f5["#Managing MBSE Models"];
  class 852dea6cfecb47f5 requirement;
  click 852dea6cfecb47f5 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#managing-mbse-models";
  2b35b7b37d52d4e6 -->|refines| 852dea6cfecb47f5;
  de2d3516cd5ef91d["Aligning Design with Code"];
  click de2d3516cd5ef91d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#aligning-design-with-code";
  class de2d3516cd5ef91d requirement;
  de2d3516cd5ef91d -.->|trace| f6237e49f7494c0e;
  852dea6cfecb47f5 -.->|trace| f6237e49f7494c0e;
  a60d88b6e2cb3842["Validating Structures"];
  click a60d88b6e2cb3842 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#validating-structures";
  class a60d88b6e2cb3842 requirement;
  a60d88b6e2cb3842 -.->|trace| f6237e49f7494c0e;
  36d8b2eb16113a7f["Trace Changes in MBSE Model"];
  click 36d8b2eb16113a7f "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#trace-changes-in-mbse-model";
  class 36d8b2eb16113a7f requirement;
  36d8b2eb16113a7f -.->|trace| f6237e49f7494c0e;
  fe32882ee273e24d["Provide Reports"];
  click fe32882ee273e24d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#provide-reports";
  class fe32882ee273e24d requirement;
  fe32882ee273e24d -.->|trace| b3f57e49f7494c0e;
```

---

### Managing MBSE Models

As an **System Engineer**, I want to manage MBSE models effectively, so that I can ensure they align with project requirements and deliverable goals.

#### Relations
  * trace: [MOEs.md/MOE_UA](MOEs.md#moe_ua)

---

### AI-Assisted MBSE Model Management

As a **Systems Engineer**, I want to efficiently manage MBSE models using AI-powered tools and LLM-based assistants.

#### Relations
  * refine: [#Managing MBSE Models](#managing-mbse-models)

---

### Generate Diagrams
As a **System Engineer**, I want to generate diagrams for different system viewpoints, so that I can communicate system architecture effectively and understand dependencies and impacts across the system.

#### Relations
  * trace: [MOEs.md/MOE_UA](MOEs.md#moe_ua)

---

### Export Specifications
As a **Manager**, I want to export specifications into diferent formats including HTML, so that I can communicate system architecture effectively.

#### Relations
  * trace: [MOEs.md/MOE_UA](MOEs.md#moe_ua)

---

### Aligning Design with Code
As a **Developer**, I want to align code with MBSE models, so that implementation remains consistent with design specifications.

#### Relations
  * trace: [MOEs.md/MOE_UA](MOEs.md#moe_ua)

---

### Validating Structures
As an **System Engineer**, I want to validate the structure of MBSE models, so that I can ensure compliance with organizational and project standards.

#### Relations
  * trace: [MOEs.md/MOE_UA](MOEs.md#moe_ua)

---

### Integrate with GitHub Workflows
As a **Contributor**, I want ReqFlow to integrate seamlessly with GitHub workflows, so that I can collaborate on updates and manage contributions effectively.

#### Relations
  * trace: [MOEs.md/MOE_CE](MOEs.md#moe_ce)

---

### Provide Reports
As a **Manager**, I want to generate structured reports based on the MBSE model, so that I can track progress and ensure alignment with organizational objectives.

#### Relations
  * trace: [MOEs.md/MOE_CE](MOEs.md#moe_ce)

---

### Trace Changes in MBSE Model
As a **System Engineer**, I want to trace changes in the MBSE model to identify affected components and ensure all updates are consistent across the system.

#### Relations
  * trace: [MOEs.md/MOE_UA](MOEs.md#moe_ua)

---

### Fostering Community Contributions
As a **Contributor**, I want ReqFlow tools to be intuitive and well-documented, so that I can contribute effectively to the open-source project.

#### Relations
  * trace: [MOEs.md/MOE_CE](MOEs.md#moe_ce)

---