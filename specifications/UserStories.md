# User stories

## Personas
1. System Engineer: Focused on managing system models, ensuring alignment with project requirements, and validating structures.  
2. SOI Developer: Implements features and makes system changes based on MBSE models, ensuring consistency between design and code.  
3. Contributor: An external community member contributing to Reqvire by improving models, creating features, or providing feedback.  
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

  91a0adb0b4d959c4["Provide Reports"];
  class 91a0adb0b4d959c4 requirement;
  click 91a0adb0b4d959c4 "UserStories.md#provide-reports";
  e9ad540a6411a0fc["MOEs.md/MOE_CE"];
  class e9ad540a6411a0fc requirement;
  click e9ad540a6411a0fc "MOEs.md#moe_ce";
  91a0adb0b4d959c4 -.->|trace| e9ad540a6411a0fc;
  5ef9c8ae19a9f55a["Trace Changes in MBSE Model"];
  class 5ef9c8ae19a9f55a requirement;
  click 5ef9c8ae19a9f55a "UserStories.md#trace-changes-in-mbse-model";
  2c5f30f14e792200["MOEs.md/MOE_UA"];
  class 2c5f30f14e792200 requirement;
  click 2c5f30f14e792200 "MOEs.md#moe_ua";
  5ef9c8ae19a9f55a -.->|trace| 2c5f30f14e792200;
  e61b7c1baa89bfc6["Managing MBSE Models"];
  class e61b7c1baa89bfc6 requirement;
  click e61b7c1baa89bfc6 "UserStories.md#managing-mbse-models";
  e61b7c1baa89bfc6 -.->|trace| 2c5f30f14e792200;
  cd2d5dab49985ca2["AI-Assisted MBSE Model Management"];
  class cd2d5dab49985ca2 requirement;
  click cd2d5dab49985ca2 "UserStories.md#ai-assisted-mbse-model-management";
  cd2d5dab49985ca2 -->|refines| e61b7c1baa89bfc6;
  1c32d488dd358c54["Fostering Community Contributions"];
  class 1c32d488dd358c54 requirement;
  click 1c32d488dd358c54 "UserStories.md#fostering-community-contributions";
  1c32d488dd358c54 -.->|trace| e9ad540a6411a0fc;
  855a1b3061c7bcdd["Aligning Design with Code"];
  class 855a1b3061c7bcdd requirement;
  click 855a1b3061c7bcdd "UserStories.md#aligning-design-with-code";
  855a1b3061c7bcdd -.->|trace| 2c5f30f14e792200;
  b3b899678f557ee9["Export Specifications"];
  class b3b899678f557ee9 requirement;
  click b3b899678f557ee9 "UserStories.md#export-specifications";
  b3b899678f557ee9 -.->|trace| 2c5f30f14e792200;
  113748a94885138d["Validating Structures"];
  class 113748a94885138d requirement;
  click 113748a94885138d "UserStories.md#validating-structures";
  113748a94885138d -.->|trace| 2c5f30f14e792200;
  37a5b8e199a838f["Generate Diagrams"];
  class 37a5b8e199a838f requirement;
  click 37a5b8e199a838f "UserStories.md#generate-diagrams";
  37a5b8e199a838f -.->|trace| 2c5f30f14e792200;
  98eaeddc27f99e11["Integrate with GitHub Workflows"];
  class 98eaeddc27f99e11 requirement;
  click 98eaeddc27f99e11 "UserStories.md#integrate-with-github-workflows";
  98eaeddc27f99e11 -.->|trace| e9ad540a6411a0fc;
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
As a **Contributor**, I want Reqvire to integrate seamlessly with GitHub workflows, so that I can collaborate on updates and manage contributions effectively.

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
As a **Contributor**, I want Reqvire tools to be intuitive and well-documented, so that I can contribute effectively to the open-source project.

#### Relations
  * trace: [MOEs.md/MOE_CE](MOEs.md#moe_ce)

---