# Elements

### Alpha Root

Ability to provide the alpha services.

#### Metadata
  * type: capability
---

### Alpha Left

Ability to provide the left alpha services.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Alpha Root](#alpha-root)
---

### Alpha Right

Ability to provide the right alpha services.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Alpha Root](#alpha-root)
---

### Shared Branch

Ability to provide services shared by both alpha branches.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Alpha Left](#alpha-left)
  * derivedFrom: [Alpha Right](#alpha-right)
---

### Empty Branch

Ability reserved for future alpha services.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Alpha Root](#alpha-root)
---

### Beta Root

Ability to provide the independent beta services.

#### Metadata
  * type: capability
---
