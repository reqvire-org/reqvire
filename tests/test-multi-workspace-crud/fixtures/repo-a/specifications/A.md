# Elements

### Repo A CRUD Capability

Capability in repo A used by cross-repo CRUD tests.

#### Metadata
  * type: capability

---

### Repo A Movable Requirement

The system shall move this requirement from repo A to repo B.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Repo A CRUD Capability](#repo-a-crud-capability)

---

### Repo A Merge Source Requirement

The system shall merge this source requirement from repo A into a repo B target.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Repo B CRUD Capability](/repo-b/specifications/B.md#repo-b-crud-capability)

---

### Repo A Removable Requirement

The system shall remove this requirement from repo A.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Repo A CRUD Capability](#repo-a-crud-capability)

---
