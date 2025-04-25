# MBSE Models Requirements

## Managing MBSE Models
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  1355222227e76540["Support for Distributed Requirements"];
  click 1355222227e76540 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  class 1355222227e76540 requirement;
  fbe7e0d3b82611ef["Project Configuration with YAML"];
  class fbe7e0d3b82611ef requirement;
  click fbe7e0d3b82611ef "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  1355222227e76540 -.->|deriveReqT| fbe7e0d3b82611ef;
  1902659ed14c4615["UserStories.md/Managing MBSE Models"];
  class 1902659ed14c4615 requirement;
  click 1902659ed14c4615 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#managing-mbse-models";
  fbe7e0d3b82611ef -->|refines| 1902659ed14c4615;
  f645de527a1eb977["Configurable External Folders"];
  click f645de527a1eb977 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#configurable-external-folders";
  class f645de527a1eb977 requirement;
  f645de527a1eb977 -->|refines| 1355222227e76540;
  a67f325ec6416b1d["Coexistence of Structured and Unstructured Documents"];
  click a67f325ec6416b1d "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  class a67f325ec6416b1d requirement;
  a67f325ec6416b1d -->|refines| 1902659ed14c4615;
  cfe4bd1a448c3930["Configurable Specifications Folder"];
  click cfe4bd1a448c3930 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#configurable-specifications-folder";
  class cfe4bd1a448c3930 requirement;
  cfe4bd1a448c3930 -.->|deriveReqT| fbe7e0d3b82611ef;
  6c00ff29cf036776["Bootstrap Model Structure"];
  click 6c00ff29cf036776 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#bootstrap-model-structure";
  class 6c00ff29cf036776 requirement;
  6c00ff29cf036776 -->|refines| 1902659ed14c4615;
  b6c8faf2d138c14a["Efficient Processing"];
  click b6c8faf2d138c14a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  class b6c8faf2d138c14a requirement;
  b6c8faf2d138c14a -->|refines| 1902659ed14c4615;
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