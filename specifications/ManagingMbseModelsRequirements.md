# MBSE Models Requirements

## Managing MBSE Models
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  87e38ccb52188638["Configurable Specifications Folder"];
  click 87e38ccb52188638 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/ManagingMbseModelsRequirements.md#configurable-specifications-folder";
  class 87e38ccb52188638 requirement;
  b3ef0fb91572bcf0["Project Configuration with YAML"];
  class b3ef0fb91572bcf0 requirement;
  click b3ef0fb91572bcf0 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  87e38ccb52188638 -.->|deriveReqT| b3ef0fb91572bcf0;
  c4d5865187c53ce6["Support for Distributed Requirements"];
  click c4d5865187c53ce6 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  class c4d5865187c53ce6 requirement;
  c4d5865187c53ce6 -.->|deriveReqT| b3ef0fb91572bcf0;
  da5831ca85881025["Coexistence of Structured and Unstructured Documents"];
  click da5831ca85881025 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  class da5831ca85881025 requirement;
  c75ac8fa29479ca5["UserStories.md/Managing MBSE Models"];
  class c75ac8fa29479ca5 requirement;
  click c75ac8fa29479ca5 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/UserStories.md#managing-mbse-models";
  c75ac8fa29479ca5 -->|refines| da5831ca85881025;
  b9c9d1bf19e76fc["Configurable External Folders"];
  click b9c9d1bf19e76fc "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/ManagingMbseModelsRequirements.md#configurable-external-folders";
  class b9c9d1bf19e76fc requirement;
  c4d5865187c53ce6 -->|refines| b9c9d1bf19e76fc;
  fa4727bd969c48f8["Bootstrap Model Structure"];
  click fa4727bd969c48f8 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/ManagingMbseModelsRequirements.md#bootstrap-model-structure";
  class fa4727bd969c48f8 requirement;
  c75ac8fa29479ca5 -->|refines| fa4727bd969c48f8;
  c75ac8fa29479ca5 -->|refines| b3ef0fb91572bcf0;
  386d7b145d008870["Efficient Processing"];
  click 386d7b145d008870 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  class 386d7b145d008870 requirement;
  c75ac8fa29479ca5 -->|refines| 386d7b145d008870;
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
The system shall support a YAML-based configuration file that defines folder names and structures to be used by the Reqvire tool when processing model artifacts.

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