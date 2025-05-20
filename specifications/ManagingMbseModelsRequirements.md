# MBSE Models Requirements

## Managing MBSE Models
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  da5831ca85881025["Coexistence of Structured and Unstructured Documents"];
  click da5831ca85881025 "ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  class da5831ca85881025 requirement;
  c75ac8fa29479ca5["UserStories.md/Managing MBSE Models"];
  class c75ac8fa29479ca5 requirement;
  click c75ac8fa29479ca5 "UserStories.md#managing-mbse-models";
  da5831ca85881025 -.->|deriveReqT| c75ac8fa29479ca5;
  82d6413dc779c791["Configurable User Requirements Root Folder"];
  click 82d6413dc779c791 "ManagingMbseModelsRequirements.md#configurable-user-requirements-root-folder";
  class 82d6413dc779c791 requirement;
  b3ef0fb91572bcf0["Project Configuration with YAML"];
  class b3ef0fb91572bcf0 requirement;
  click b3ef0fb91572bcf0 "ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  82d6413dc779c791 -.->|deriveReqT| b3ef0fb91572bcf0;
  103e5403ec1d2f3c["Default Requirement Type Assignment"];
  click 103e5403ec1d2f3c "ManagingMbseModelsRequirements.md#default-requirement-type-assignment";
  class 103e5403ec1d2f3c requirement;
  103e5403ec1d2f3c -->|refines| 82d6413dc779c791;
  b3ef0fb91572bcf0 -.->|deriveReqT| c75ac8fa29479ca5;
  b166738857fe340f["Git Repository as Project Root"];
  class b166738857fe340f requirement;
  click b166738857fe340f "ManagingMbseModelsRequirements.md#git-repository-as-project-root";
  b3ef0fb91572bcf0 -->|refines| b166738857fe340f;
  386d7b145d008870["Efficient Processing"];
  click 386d7b145d008870 "ManagingMbseModelsRequirements.md#efficient-processing";
  class 386d7b145d008870 requirement;
  386d7b145d008870 -.->|deriveReqT| c75ac8fa29479ca5;
  b166738857fe340f -.->|deriveReqT| c75ac8fa29479ca5;
```
---

### Coexistence of Structured and Unstructured Documents

The system shall allow structured markdown and unstructured. (eg., markdown, PDFs, DOCX, raw text) documents to coexist within the same MBSE model.

#### Relations
  * derivedFrom: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---

### Efficient Processing

The system shall process structured documents and relations to extract model-relevant information efficiently.

#### Relations
  * derivedFrom: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---

### Git Repository as Project Root

The system shall treat the **root directory of the Git repository as the project's base** for all file and folder references, streamlining configuration and promoting a self-contained project structure.

#### Details

All paths specified in the Reqvire configuration will be resolved relative to the Git repository root.

#### Relations
  * derivedFrom: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---

### Project Configuration with YAML

The system shall support a **YAML-based configuration file** that defines folder names and structures to be used by the tool when processing model artifacts, **relative to the Git repository root**.

#### Relations
  * derivedFrom: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)
   * refine: [Git Repository as Project Root](#git-repository-as-project-root)

---

### Configurable User Requirements Root Folder

The system shall allow users to configure a specific **root folder within the Git repository**, designated exclusively for **user requirements**.

#### Details

This folder will be specified in the `reqvire.yaml` configuration.

#### Relations
  * derivedFrom: [Project Configuration with YAML](#project-configuration-with-yaml)

---

### Default Requirement Type Assignment

The system shall automatically assign a **default type to requirements** if not explicitly specified in their `metadata`, based on their location:
* Requirements found directly within the designated "Configurable User Requirements Root Folder" (and not in its sub-folders) shall be assigned `user-requirements` type.
* Requirements found in any other folder or sub-folder within the Git repository (including sub-folders of the user requirements root) shall be assigned `system-requirements` type.

#### Relations
  * refine: [Configurable User Requirements Root Folder](#configurable-user-requirements-root-folder)

---