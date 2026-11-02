# Elements

### Manifest-Defined Workspace Scope

When the shared workspace service supplies a declared composition, the system SHALL construct one workspace-relative model scope from its owning repository and pinned components, retaining file ownership and excluding undeclared repositories whether or not the owner contains model elements.

#### Concept References
  * [Workspace Composition](../Thesaurus/Workspaces.md#workspace-composition)
  * [Workspace Manifest](../Thesaurus/Workspaces.md#workspace-manifest)

#### Metadata
  * type: requirement

#### Contract References
  * [Native Workspace Manifest Specification](../Workspaces/Specifications.md#native-workspace-manifest-specification)
  * [Workspace Management API Specification](../Workspaces/Specifications.md#workspace-management-api-specification)

#### Relations
  * derivedFrom: [Workspace Root Path Authority](ModelManagement.md#workspace-root-path-authority)
  * satisfiedBy: [config.rs](../../crates/reqvire-core/src/config.rs)
  * satisfiedBy: [workspace.rs](../../crates/reqvire-core/src/workspace.rs)
---

### Manifest-Defined Workspace Scope Specification

#### Details
Consume the authoritative effective directory, owning repository, declared component inventory, exact revisions and file ownership supplied by the workspace service. Model processing does not parse another manifest, infer checkouts, update pins or implement repository preparation.

Construct one graph with globally consistent workspace-relative identifiers and cross-repository relations. The owner may contribute only its manifest/configuration while components supply all model elements; require neither a root model document nor a synthetic model element.

Workspace exclusion rules apply globally and each participating repository's Git ignores apply within its ownership boundary. Root ignore entries for component checkout directories do not hide declared component content. Exclude undeclared nested repositories, administration, derived caches and copied build/dependency models.

Use the same immutable resolved inventory throughout one validation pass, including file-reference checks. New passes observe through the caller's normal admission policy. An accepted model retains its captured composition and input provenance when live repository state drifts.

Repository registration, checkout, source selection, locks and pinning remain workspace-service responsibilities. This contract owns only model membership, ownership-aware file eligibility and path projection.

#### Metadata
  * type: specification

#### Relations
  * define: [Manifest-Defined Workspace Scope](#manifest-defined-workspace-scope)
---
