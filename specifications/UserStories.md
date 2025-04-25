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

  86153e8e478c5ae5["AI-Assisted MBSE Model Management"];
  click 86153e8e478c5ae5 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#ai-assisted-mbse-model-management";
  class 86153e8e478c5ae5 requirement;
  1902659ed14c4615["#Managing MBSE Models"];
  class 1902659ed14c4615 requirement;
  click 1902659ed14c4615 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#managing-mbse-models";
  86153e8e478c5ae5 -->|refines| 1902659ed14c4615;
  ab3c35050243abb3["Generate Diagrams"];
  click ab3c35050243abb3 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#generate-diagrams";
  class ab3c35050243abb3 requirement;
  bd8b46053261f1d0["MOEs.md/MOE_UA"];
  class bd8b46053261f1d0 requirement;
  click bd8b46053261f1d0 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/MOEs.md#moe_ua";
  ab3c35050243abb3 -.->|trace| bd8b46053261f1d0;
  dfb5ca2ca89e0152["Integrate with GitHub Workflows"];
  click dfb5ca2ca89e0152 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#integrate-with-github-workflows";
  class dfb5ca2ca89e0152 requirement;
  a7b946053261f1d0["MOEs.md/MOE_CE"];
  class a7b946053261f1d0 requirement;
  click a7b946053261f1d0 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/MOEs.md#moe_ce";
  dfb5ca2ca89e0152 -.->|trace| a7b946053261f1d0;
  e411816051b86f1c["Validating Structures"];
  click e411816051b86f1c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#validating-structures";
  class e411816051b86f1c requirement;
  e411816051b86f1c -.->|trace| bd8b46053261f1d0;
  c99ffeebd04e23f["Trace Changes in MBSE Model"];
  click c99ffeebd04e23f "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#trace-changes-in-mbse-model";
  class c99ffeebd04e23f requirement;
  c99ffeebd04e23f -.->|trace| bd8b46053261f1d0;
  3aa9c1e4906c1b45["Aligning Design with Code"];
  click 3aa9c1e4906c1b45 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#aligning-design-with-code";
  class 3aa9c1e4906c1b45 requirement;
  3aa9c1e4906c1b45 -.->|trace| bd8b46053261f1d0;
  eba099aa902aa2f9["Fostering Community Contributions"];
  click eba099aa902aa2f9 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#fostering-community-contributions";
  class eba099aa902aa2f9 requirement;
  eba099aa902aa2f9 -.->|trace| a7b946053261f1d0;
  69208127f6580b16["Provide Reports"];
  click 69208127f6580b16 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#provide-reports";
  class 69208127f6580b16 requirement;
  69208127f6580b16 -.->|trace| a7b946053261f1d0;
  53d8b873d743e94["Export Specifications"];
  click 53d8b873d743e94 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#export-specifications";
  class 53d8b873d743e94 requirement;
  53d8b873d743e94 -.->|trace| bd8b46053261f1d0;
  1902659ed14c4615 -.->|trace| bd8b46053261f1d0;
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