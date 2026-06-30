# Elements

### Repo B CRUD Capability

Capability in repo B used by cross-repo CRUD tests.

#### Metadata
  * type: capability

---

### Repo B Link Source Requirement

The system shall receive and then remove a cross-repo evidence relation.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Repo B CRUD Capability](#repo-b-crud-capability)

---

### Repo B Relink Requirement

The system shall relink evidence from repo B to repo A.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Repo B CRUD Capability](#repo-b-crud-capability)
  * satisfiedBy: [Old Evidence](/repo-b/docs/old-evidence.txt)

---

### Repo B Rename Requirement

The system shall be renamed while preserving cross-file references.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Repo B CRUD Capability](#repo-b-crud-capability)

---

### Repo B Rename Child Requirement

The system shall keep this relation aligned after its parent is renamed.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Repo B Rename Requirement](#repo-b-rename-requirement)

---

### Repo B Merge Target Requirement

The system shall receive merged content from a repo A requirement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Repo B CRUD Capability](#repo-b-crud-capability)

---

### Repo B Asset Requirement

The system shall keep this cross-repo asset relation aligned when the asset moves.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Repo B CRUD Capability](#repo-b-crud-capability)
  * satisfiedBy: [Asset To Move](/repo-a/docs/asset-to-move.txt)

---

### Repo B Remove Asset Requirement

The system shall lose this asset relation when the asset is removed.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Repo B CRUD Capability](#repo-b-crud-capability)
  * satisfiedBy: [Asset To Remove](/repo-b/docs/asset-remove.txt)

---
