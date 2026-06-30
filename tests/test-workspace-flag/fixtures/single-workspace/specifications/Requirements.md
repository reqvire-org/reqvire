# Elements

### Workspace Flag Capability

Capability root for workspace flag tests.

#### Metadata
  * type: capability

---

### Workspace Flag Root

Root requirement for explicit workspace selection.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Workspace Flag Child](#workspace-flag-child)
  * derive: [Workspace Flag Sibling](#workspace-flag-sibling)
  * specify: [Workspace Flag Capability](#workspace-flag-capability)

---

### Workspace Flag Child

Requirement used to verify explicit workspace selection.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Workspace Flag Root](#workspace-flag-root)

---

### Workspace Flag Sibling

Requirement used to verify workspace-selected file moves.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Workspace Flag Root](#workspace-flag-root)

---
