# User stories

**Personas**
1. System Engineer: Focused on managing system models, ensuring alignment with project requirements, and validating structures.  
2. SOI Developer: Implements features and makes system changes based on MBSE models, ensuring consistency between design and code.  
3. Contributor: An external community member contributing to Reqvire by improving models, creating features, or providing feedback.  
4. Manager: Oversees the MBSE processes, tracks progress, ensures alignment with objectives, and generates reports for decision-making.

## User Stories

### Managing MBSE Models

As an **System Engineer**, I want to manage MBSE models effectively, so that I can ensure they align with project requirements and deliverable goals.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [MOE_UA](MOEs.md#moe_ua)
---

### AI-Assisted MBSE Model Management

As a **Systems Engineer**, I want to efficiently manage MBSE models using AI-powered tools and LLM-based assistants.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing MBSE Models](#managing-mbse-models)
---

### Generate Diagrams

As a **System Engineer**, I want to generate diagrams for different system viewpoints, so that I can communicate system architecture effectively and understand dependencies and impacts across the system.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [MOE_UA](MOEs.md#moe_ua)
---

### Export Specifications

As a **Manager**, I want to export specifications into diferent formats including HTML, so that I can communicate system architecture effectively.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [MOE_UA](MOEs.md#moe_ua)
---

### Aligning Design with Code

As a **Developer**, I want to align code with MBSE models, so that implementation remains consistent with design specifications.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [MOE_UA](MOEs.md#moe_ua)
---

### Validating Structures

As an **System Engineer**, I want to validate the structure of MBSE models, so that I can ensure compliance with organizational and project standards.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [MOE_UA](MOEs.md#moe_ua)
---

### Integrate with GitHub Workflows

As a **Contributor**, I want Reqvire to integrate seamlessly with GitHub workflows, so that I can collaborate on updates and manage contributions effectively.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Promote Automation and Efficiency](MissionRequirements.md#promote-automation-and-efficiency)
  * trace: [MOE_CE](MOEs.md#moe_ce)
---

### Provide Reports

As a **Manager**, I want to generate structured reports based on the MBSE model, so that I can track progress and ensure alignment with organizational objectives.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [MOE_CE](MOEs.md#moe_ce)
---

### Trace Changes in MBSE Model

As a **System Engineer**, I want to trace changes in the MBSE model to identify affected components and ensure all updates are consistent across the system.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [MOE_UA](MOEs.md#moe_ua)
---

### Fostering Community Contributions

As a **Contributor**, I want Reqvire tools to be intuitive and well-documented, so that I can contribute effectively to the open-source project.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [MOE_CE](MOEs.md#moe_ce)
---
