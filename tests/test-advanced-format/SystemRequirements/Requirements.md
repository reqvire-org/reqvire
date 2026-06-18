# Elements

### Requirements Processing
The system shall parse the files in all folders and subfolders from the root of git repository.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: ../UserStories.md#managing-mbse-models
  * satisfiedBy: ../core/src/model.rs

### Validation Framework

The system shall implement a two-pass validation architecture for comprehensive error detection.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #requirements-processing
  * satisfiedBy: ../core/src/graph_registry.rs

---

### Path Resolution Testing

This requirement tests various absolute path conversions from a subfolder.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [../UserStories.md#managing-mbse-models](/UserStories.md#managing-mbse-models)
  * verifiedBy: [../Verifications/Tests.md#format-test](/Verifications/Tests.md#format-test)
  * satisfiedBy: /core/src/parser.rs
