# MBSE Models Requirements

## Managing MBSE Models
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  aac74edfeb367501["Efficient Processing"];
  click aac74edfeb367501 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  class aac74edfeb367501 requirement;
  a469d82e490f4e4["UserStories.md/Managing MBSE Models"];
  class a469d82e490f4e4 requirement;
  click a469d82e490f4e4 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#managing-mbse-models";
  aac74edfeb367501 -->|refines| a469d82e490f4e4;
  afba6897082c7c6a["Project Configuration with YAML"];
  click afba6897082c7c6a "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  class afba6897082c7c6a requirement;
  afba6897082c7c6a -->|refines| a469d82e490f4e4;
  ecc3a81573b50a5c["Configurable External Folders"];
  click ecc3a81573b50a5c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#configurable-external-folders";
  class ecc3a81573b50a5c requirement;
  fe433967471bc04f["Support for Distributed Requirements"];
  class fe433967471bc04f requirement;
  click fe433967471bc04f "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  ecc3a81573b50a5c -->|refines| fe433967471bc04f;
  2a1b2c862241398["Configurable Specifications Folder"];
  click 2a1b2c862241398 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#configurable-specifications-folder";
  class 2a1b2c862241398 requirement;
  2a1b2c862241398 -.->|deriveReqT| afba6897082c7c6a;
  fe433967471bc04f -.->|deriveReqT| afba6897082c7c6a;
  ab0a6601a5fb42c8["Coexistence of Structured and Unstructured Documents"];
  click ab0a6601a5fb42c8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  class ab0a6601a5fb42c8 requirement;
  ab0a6601a5fb42c8 -->|refines| a469d82e490f4e4;
  20b88aa91dd68c24["Bootstrap Model Structure"];
  click 20b88aa91dd68c24 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#bootstrap-model-structure";
  class 20b88aa91dd68c24 requirement;
  20b88aa91dd68c24 -->|refines| a469d82e490f4e4;
```

---

### Coexistence of Structured and Unstructured Documents
The system shall allow structured markdown and unstructured. (eg., markdown, PDFs, DOCX, raw text) documents to coexist within the same MBSE model.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---

### Efficient Processing
The system shall process structured documents and relations to extract model-relevant information efficiently.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---

### Project Configuration with YAML
The system shall support a YAML-based configuration file that defines folder names and structures to be used by the ReqFlow tool when processing model artifacts.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---

### Configurable Specifications Folder

The system shall allow users to configure the main specification folder through the configuration file, supporting flexible project organization.

#### Relations
  * derivedFrom: [Project Configuration with YAML](#project-configuration-with-yaml)

---

### Support for Distributed Requirements
The system shall support referencing folders that may exist in different repositories through configuration, allowing for distributed requirements management across multiple repositories.

#### Relations
  * derivedFrom: [Project Configuration with YAML](#project-configuration-with-yaml)

---

### Configurable External Folders
The system shall allow users to configure the External folders through the configuration file.

#### Relations
  * refine: [Support for Distributed Requirements](#support-for-distributed-requirements)

---

### Bootstrap Model Structure
The system shall provide a command to bootstrap a basic model structure with sample templates for common element types.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---