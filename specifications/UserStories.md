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

  1563f8454019c887["Integrate with GitHub Workflows"];
  click 1563f8454019c887 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserStories.md#integrate-with-github-workflows";
  class 1563f8454019c887 requirement;
  ac59f565725d5e41["MOEs.md/MOE_CE"];
  class ac59f565725d5e41 requirement;
  click ac59f565725d5e41 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/MOEs.md#moe_ce";
  1563f8454019c887 -.->|trace| ac59f565725d5e41;
  694c4bb24b10f0c7["Generate Diagrams"];
  click 694c4bb24b10f0c7 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserStories.md#generate-diagrams";
  class 694c4bb24b10f0c7 requirement;
  6c33f565725d5e41["MOEs.md/MOE_UA"];
  class 6c33f565725d5e41 requirement;
  click 6c33f565725d5e41 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/MOEs.md#moe_ua";
  694c4bb24b10f0c7 -.->|trace| 6c33f565725d5e41;
  2591b90e1fb90daa["Trace Changes in MBSE Model"];
  click 2591b90e1fb90daa "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserStories.md#trace-changes-in-mbse-model";
  class 2591b90e1fb90daa requirement;
  2591b90e1fb90daa -.->|trace| 6c33f565725d5e41;
  c75ac8fa29479ca5["Managing MBSE Models"];
  click c75ac8fa29479ca5 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserStories.md#managing-mbse-models";
  class c75ac8fa29479ca5 requirement;
  c75ac8fa29479ca5 -.->|trace| 6c33f565725d5e41;
  3bb2096c4648e3ff["Export Specifications"];
  click 3bb2096c4648e3ff "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserStories.md#export-specifications";
  class 3bb2096c4648e3ff requirement;
  3bb2096c4648e3ff -.->|trace| 6c33f565725d5e41;
  a9bc74070fc71e6d["Aligning Design with Code"];
  click a9bc74070fc71e6d "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserStories.md#aligning-design-with-code";
  class a9bc74070fc71e6d requirement;
  a9bc74070fc71e6d -.->|trace| 6c33f565725d5e41;
  8fc677b7413bb247["AI-Assisted MBSE Model Management"];
  click 8fc677b7413bb247 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserStories.md#ai-assisted-mbse-model-management";
  class 8fc677b7413bb247 requirement;
  8fc677b7413bb247 -->|refines| c75ac8fa29479ca5;
  a0f9571f6563d9d3["Validating Structures"];
  click a0f9571f6563d9d3 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserStories.md#validating-structures";
  class a0f9571f6563d9d3 requirement;
  a0f9571f6563d9d3 -.->|trace| 6c33f565725d5e41;
  b7a255ea304a325e["Fostering Community Contributions"];
  click b7a255ea304a325e "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserStories.md#fostering-community-contributions";
  class b7a255ea304a325e requirement;
  b7a255ea304a325e -.->|trace| ac59f565725d5e41;
  9cb6618c17d19a11["Provide Reports"];
  click 9cb6618c17d19a11 "https://github.com/Reqvire/reqvire/blob/92e82ef235559b4819fd85d700024b79b452d8e3/specifications/UserStories.md#provide-reports";
  class 9cb6618c17d19a11 requirement;
  9cb6618c17d19a11 -.->|trace| ac59f565725d5e41;
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