# Elements

### Commit Pin

An exact Git commit identifier recording which committed revision of a dependent repository belongs to a workspace composition.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](Thesaurus.md#reqvire-concept-scheme)
  * related: [Workspace Composition](#workspace-composition)
---

### Repositories Directory

The configurable parent directory used to place the dependent checkouts of one declared workspace, interpreted relative to the workspace manifest directory.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](Thesaurus.md#reqvire-concept-scheme)
  * related: [Workspace Manifest](#workspace-manifest)
---

### Tracking Ref

A configured Git reference identifying the source from which an explicit workspace update resolves a component revision. It is independent of the component's current working branch and its exact recorded commit pin.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](Thesaurus.md#reqvire-concept-scheme)
  * related: [Workspace Manifest](#workspace-manifest)
---

### Workspace Composition

The owning repository revision together with its declared component repositories and selected commits. A composition may contain only the owning repository; its model content may reside in the owner, its components, or both.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](Thesaurus.md#reqvire-concept-scheme)
  * related: [Workspace Manifest](#workspace-manifest)
---

### Workspace Freeze

The recording of current committed component revisions as the selected pins of a workspace composition.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](Thesaurus.md#reqvire-concept-scheme)
  * related: [Commit Pin](#commit-pin)
---

### Workspace Manifest

A version-controlled description of the dependent repositories, checkout locations and exact component commits selected by an owning repository branch. Its source is tracked with that branch's other composition changes.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](Thesaurus.md#reqvire-concept-scheme)
---

### Workspace Migration

The explicit conversion of an existing repository-composition description into Reqvire's native workspace representation while preserving the selected repositories, checkout paths and committed content.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](Thesaurus.md#reqvire-concept-scheme)
  * related: [Workspace Manifest](#workspace-manifest)
---
