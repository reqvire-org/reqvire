# User stories

## Personas

1. System Engineer: Focused on managing system models, ensuring alignment with project requirements, and validating structures.  
2. SOI Developer: Implements features and makes system changes based on MBSE models, ensuring consistency between design and code.  
3. Contributor: An external community member contributing to ReqFlow by improving models, creating features, or providing feedback.  
4. Manager: Oversees the MBSE processes, tracks progress, ensures alignment with objectives, and generates reports for decision-making.  

## User Stories
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  cdc9a9c908["Fostering Community Contributions"];
  click cdc9a9c908 "UserStories.md#fostering-community-contributions";
  class cdc9a9c908 requirement;
  b3f57e49f7["MOEs.md/MOE_CE"];
  class b3f57e49f7 requirement;
  click b3f57e49f7 "MOEs.md#moe_ce";
  cdc9a9c908 -->|traces| b3f57e49f7;
  100197ce81["Export Specifications"];
  click 100197ce81 "UserStories.md#export-specifications";
  class 100197ce81 requirement;
  f6237e49f7["MOEs.md/MOE_UA"];
  class f6237e49f7 requirement;
  click f6237e49f7 "MOEs.md#moe_ua";
  100197ce81 -->|traces| f6237e49f7;
  fe32882ee2["Provide Reports"];
  click fe32882ee2 "UserStories.md#provide-reports";
  class fe32882ee2 requirement;
  fe32882ee2 -->|traces| b3f57e49f7;
  e4eb5bf7e5["Generate Traceability Matrix"];
  click e4eb5bf7e5 "UserStories.md#generate-traceability-matrix";
  class e4eb5bf7e5 requirement;
  e4eb5bf7e5 -->|traces| f6237e49f7;
  a60d88b6e2["Validating Structures"];
  click a60d88b6e2 "UserStories.md#validating-structures";
  class a60d88b6e2 requirement;
  a60d88b6e2 -->|traces| f6237e49f7;
  a4657fca5a["AI-Driven Code Suggestions"];
  click a4657fca5a "UserStories.md#ai-driven-code-suggestions";
  class a4657fca5a requirement;
  a4657fca5a -->|traces| f6237e49f7;
  cf0026c2fe["Generate Diagrams"];
  click cf0026c2fe "UserStories.md#generate-diagrams";
  class cf0026c2fe requirement;
  cf0026c2fe -->|traces| f6237e49f7;
  852dea6cfe["Managing MBSE Models"];
  click 852dea6cfe "UserStories.md#managing-mbse-models";
  class 852dea6cfe requirement;
  852dea6cfe -->|traces| f6237e49f7;
  36d8b2eb16["Trace Changes in MBSE Model"];
  click 36d8b2eb16 "UserStories.md#trace-changes-in-mbse-model";
  class 36d8b2eb16 requirement;
  36d8b2eb16 -->|traces| f6237e49f7;
  ba120b7caf["Integrate with GitHub Workflows"];
  click ba120b7caf "UserStories.md#integrate-with-github-workflows";
  class ba120b7caf requirement;
  ba120b7caf -->|traces| b3f57e49f7;
  6912d98267["AI-Driven Model Suggestions"];
  click 6912d98267 "UserStories.md#ai-driven-model-suggestions";
  class 6912d98267 requirement;
  6912d98267 -->|traces| f6237e49f7;
  de2d3516cd["Aligning Design with Code"];
  click de2d3516cd "UserStories.md#aligning-design-with-code";
  class de2d3516cd requirement;
  de2d3516cd -->|traces| f6237e49f7;
```
































### Managing MBSE Models

As an **System Engineer**, I want to manage MBSE models effectively, so that I can ensure they align with project requirements and deliverable goals.

#### Relations
  * trace: [MOEs.md/MOE_UA](MOEs.md#moe_ua)

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

### AI-Driven Code Suggestions

As a **Developer**, I want AI agents to provide actionable suggestions for code improvements, so that I can accelerate development tasks.

#### Relations
  * trace: [MOEs.md/MOE_UA](MOEs.md#moe_ua)

---

### AI-Driven Model Suggestions
		
As a **System Engineer**, I want AI agents to provide actionable suggestions for model improvements, so that I can refine the system design and maintain alignment with project requirements.

#### Relations
  * trace: [MOEs.md/MOE_UA](MOEs.md#moe_ua)
 
 

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

### Generate Traceability Matrix

As a **System Engineer**, I want ReqFlow to generate traceability matrices, so that I can maintain alignment between requirements and related system elements.

#### Relations
  * trace: [MOEs.md/MOE_UA](MOEs.md#moe_ua)

---

### Fostering Community Contributions

As a **Contributor**, I want ReqFlow tools to be intuitive and well-documented, so that I can contribute effectively to the open-source project.

#### Relations
  * trace: [MOEs.md/MOE_CE](MOEs.md#moe_ce)