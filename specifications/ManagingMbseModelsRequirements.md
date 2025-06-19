# MBSE Models Requirements

## Managing MBSE Models
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  c9cc6878a73bb951["Default Requirement Type Assignment"];
  class c9cc6878a73bb951 requirement;
  click c9cc6878a73bb951 "ManagingMbseModelsRequirements.md#default-requirement-type-assignment";
  d9354ef2eca0f2d0["Configurable User Requirements Root Folder"];
  class d9354ef2eca0f2d0 requirement;
  click d9354ef2eca0f2d0 "ManagingMbseModelsRequirements.md#configurable-user-requirements-root-folder";
  c9cc6878a73bb951 -->|refines| d9354ef2eca0f2d0;
  16b4b380c917deb1["Project Configuration with YAML"];
  class 16b4b380c917deb1 requirement;
  click 16b4b380c917deb1 "ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  d9354ef2eca0f2d0 -.->|deriveReqT| 16b4b380c917deb1;
  d193d11c43776bec["Efficient Processing"];
  class d193d11c43776bec requirement;
  click d193d11c43776bec "ManagingMbseModelsRequirements.md#efficient-processing";
  e61b7c1baa89bfc6["UserStories.md/Managing MBSE Models"];
  class e61b7c1baa89bfc6 requirement;
  click e61b7c1baa89bfc6 "UserStories.md#managing-mbse-models";
  d193d11c43776bec -.->|deriveReqT| e61b7c1baa89bfc6;
  f0d721424636370e["Coexistence of Structured and Unstructured Documents"];
  class f0d721424636370e requirement;
  click f0d721424636370e "ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  f0d721424636370e -.->|deriveReqT| e61b7c1baa89bfc6;
  649d72765b13e14f["Git Repository as Project Root"];
  class 649d72765b13e14f requirement;
  click 649d72765b13e14f "ManagingMbseModelsRequirements.md#git-repository-as-project-root";
  649d72765b13e14f -.->|deriveReqT| e61b7c1baa89bfc6;
  16b4b380c917deb1 -.->|deriveReqT| e61b7c1baa89bfc6;
  16b4b380c917deb1 -->|refines| 649d72765b13e14f;
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