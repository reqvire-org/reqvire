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

  63454899a7184acc["AI-Assisted MBSE Model Management"];
  click 63454899a7184acc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#ai-assisted-mbse-model-management";
  class 63454899a7184acc requirement;
  a469d82e490f4e4["#Managing MBSE Models"];
  class a469d82e490f4e4 requirement;
  click a469d82e490f4e4 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#managing-mbse-models";
  63454899a7184acc -->|refines| a469d82e490f4e4;
  725d90f6f42e9407["Validating Structures"];
  click 725d90f6f42e9407 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#validating-structures";
  class 725d90f6f42e9407 requirement;
  5d0b88381f707008["MOEs.md/MOE_UA"];
  class 5d0b88381f707008 requirement;
  click 5d0b88381f707008 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/MOEs.md#moe_ua";
  725d90f6f42e9407 -.->|trace| 5d0b88381f707008;
  8bcbfc64008114c9["Aligning Design with Code"];
  click 8bcbfc64008114c9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#aligning-design-with-code";
  class 8bcbfc64008114c9 requirement;
  8bcbfc64008114c9 -.->|trace| 5d0b88381f707008;
  a8ca33c2b9a1e9de["Provide Reports"];
  click a8ca33c2b9a1e9de "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#provide-reports";
  class a8ca33c2b9a1e9de requirement;
  34ccc009ec6ea573["MOEs.md/MOE_CE"];
  class 34ccc009ec6ea573 requirement;
  click 34ccc009ec6ea573 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/MOEs.md#moe_ce";
  a8ca33c2b9a1e9de -.->|trace| 34ccc009ec6ea573;
  a469d82e490f4e4 -.->|trace| 5d0b88381f707008;
  8edc9736fd751490["Integrate with GitHub Workflows"];
  click 8edc9736fd751490 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#integrate-with-github-workflows";
  class 8edc9736fd751490 requirement;
  8edc9736fd751490 -.->|trace| 34ccc009ec6ea573;
  95e53eb18f860c5a["Fostering Community Contributions"];
  click 95e53eb18f860c5a "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#fostering-community-contributions";
  class 95e53eb18f860c5a requirement;
  95e53eb18f860c5a -.->|trace| 34ccc009ec6ea573;
  625ef9aa59da5635["Export Specifications"];
  click 625ef9aa59da5635 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#export-specifications";
  class 625ef9aa59da5635 requirement;
  625ef9aa59da5635 -.->|trace| 5d0b88381f707008;
  3ca57dc5dc6a2dae["Generate Diagrams"];
  click 3ca57dc5dc6a2dae "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#generate-diagrams";
  class 3ca57dc5dc6a2dae requirement;
  3ca57dc5dc6a2dae -.->|trace| 5d0b88381f707008;
  1e57ec134f31779c["Trace Changes in MBSE Model"];
  click 1e57ec134f31779c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#trace-changes-in-mbse-model";
  class 1e57ec134f31779c requirement;
  1e57ec134f31779c -.->|trace| 5d0b88381f707008;
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