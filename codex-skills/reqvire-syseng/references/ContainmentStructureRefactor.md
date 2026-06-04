# Reqvire Containment Structure Refactor

Use this reference when creating, reorganizing, or reviewing the physical containment structure of a Reqvire engineering knowledge graph.

Containment is the folder/file layout that makes ownership and review boundaries obvious. It must support the graph; it must not replace graph relations.

## Core Principle

Use folders and files to make ownership obvious, but use Reqvire relations to define model meaning.

Physical containment answers:

- Where should this model content live?
- Which capability or ontology area owns this content?
- What files should a reviewer inspect together?

Graph relations answer:

- Which capability does this requirement specify?
- Which requirement or capability owns this refinement?
- Which ontology gives this subgraph meaning?
- Which verification proves this capability or requirement?
- Which evidence satisfies this requirement or verification?

## Recommended Root Layout

In this reference, `./` means the chosen Reqvire model root relative to the workspace or repository root where Reqvire is run. That model root may be the workspace root, `requirements/`, `specs/system-model/`, or another project-selected folder. Do not assume the whole repository root is the model root.

Recommended top-level model planes:

```text
./
  Capabilities/
  Ontologies/
  Verifications/
```

Optional project-level files such as `UseCases.md`, `Stakeholders.md`, or `Glossary.md` may remain at `./` when they intentionally sit outside one capability folder. If content becomes independently traceable, verifiable, or capability-owned, move it into the appropriate capability subgraph.

## Model Planes

### Capabilities

`Capabilities/` contains capability-rooted subgraphs.

A capability represents a coherent operational, product, business, regulatory, or system ability. Capability folders group the requirements, refinements, architecture notes, and local contracts that belong to that ability.

### Ontologies

`Ontologies/` contains first-class `ontology` elements.

Ontologies define reusable semantic vocabulary: domain concepts, relationships, categories, and stable model meaning. Capabilities attach ontology elements to make vocabulary reachable for the capability and the requirements that specify it.

### Verifications

`Verifications/` contains verification elements grouped by verification domain.

Verification elements prove capabilities or requirements through `verify` / `verifiedBy`. Evidence-backed verification types may use `satisfiedBy` to point at test results, proof reports, fixtures, or other evidence artifacts.

## Capability Folder Pattern

Recommended pattern:

```text
Capabilities/<Area>/<CapabilityName>/
  <CapabilityName>.md
  <CapabilityName>Requirements.md
  <Topic>Requirements.md
  <Topic>Specifications.md
  <Topic>Behaviors.md
  <Topic>Constraints.md
  <Topic>States.md
  <Topic>InputOutput.md
  <Topic>SemanticContracts.md
  Architecture/
    <ArchitectureOrServiceSpecifications>.md
```

Area names should match the system being modeled. Common examples:

- `Product/` for customer-facing or product workflow capabilities.
- `Platform/` for shared infrastructure, identity, API, security, reliability, or observability capabilities.
- `Integration/` for external clients, protocols, agents, APIs, partner systems, or tool ecosystems.
- `Operations/` for diagnosis, remediation, operator workflows, automated support, or production operations.

Additional project-specific areas such as `Safety/`, `Compliance/`, `Mission/`, `Data/`, `Infrastructure/`, or `DeveloperExperience/` are fine when they carry stable meaning.

Historical file names such as `*Feature.md` are acceptable when they are stable paths, but element metadata and prose must use capability vocabulary:

```text
#### Metadata
  * type: capability
```

File names do not define Reqvire element semantics. Element metadata does.

## Capability Subgraph Ownership

Each capability folder should encapsulate one independent capability-rooted subgraph whenever practical.

A capability subgraph may own:

- the root `type: capability` element
- child capabilities when decomposition is useful
- requirements that specify the capability
- capability-owned refinements
- requirement-owned refinements
- semantic contracts for specific capability or requirement obligations
- local architecture or design specifications that refine elements in that subgraph
- verification links to external verification elements

Use child capabilities when concerns differ in verification, ownership, lifecycle, architecture impact, operational semantics, or requirement clusters.

Avoid creating child capabilities only to share vocabulary. Shared vocabulary belongs in `Ontologies/` and is attached by capabilities.

## Ontology Plane Rules

Put content in `Ontologies/` when it says:

- X is a Y.
- X has property Z.
- X relates to Y.
- This domain term means this.
- These terms form a reusable semantic vocabulary.

Keep content in capabilities, requirements, or refinements when it says:

- The system shall...
- The system must reject, write, emit, show, route, or store...
- This endpoint, path, command, field, or report must behave this way.
- This workflow step or exact output must occur.
- This code, service, or architecture component must exist.

Ontology hierarchy uses `derivedFrom` / `derive` between ontology elements. Capabilities consume ontology by attachments.

## Requirement and Refinement Files

Requirement files usually use names like:

```text
<CapabilityName>Requirements.md
<Topic>Requirements.md
```

Refinement files usually use names like:

```text
<Topic>Specifications.md
<Topic>Behaviors.md
<Topic>Constraints.md
<Topic>States.md
<Topic>InputOutput.md
<Topic>SemanticContracts.md
```

Refinements are owned by exactly one compatible capability or requirement through `refine` / `refinedBy`. Use refinements when content is too detailed for a requirement statement but still needs traceability.

Use semantic contracts when one capability or requirement needs a machine-readable SHACL profile over reachable ontology terms. Semantic contracts should refine the most specific capability or requirement that owns that semantic rule.

## Architecture Folders

`Architecture/` folders may be kept inside the capability subgraph they refine.

Use architecture folders for service-level design specifications, interface implementation architecture, service responsibility boundaries, deployment or runtime architecture, and architectural details that refine the owning capability or requirements.

Do not put shared ontology in architecture folders. Shared vocabulary belongs in `Ontologies/` and is attached by capabilities.

If architecture content becomes shared across many independent capability roots, extract reusable meaning into ontology or create an explicit reusable refinement contract and attach it where needed.

## Verification Plane

Verification files live in `Verifications/` and are grouped by verification domain, for example:

```text
Verifications/
  API/
  Auth/
  Billing/
  Platform/
  Services/
  UI/
```

Capabilities may be directly verified. Requirements remain the primary implementation coverage anchors.

## Relation Expectations

Main graph relations:

```text
capability --derivedFrom/derive--> capability
requirement --derivedFrom/derive--> requirement
ontology --derivedFrom/derive--> ontology
requirement --specify--> capability
capability --specifiedBy--> requirement
capability/requirement --refinedBy--> refinement
refinement --refine--> capability/requirement
capability/requirement --verifiedBy--> verification
verification --verify--> capability/requirement
requirement --satisfiedBy--> implementation/evidence
test-verification/formal-proof-verification --satisfiedBy--> evidence
```

Attachments are separate from normal relations:

```text
capability --Attachment--> ontology
requirement --Attachment--> compatible requirement-owned refinement contract
```

Attachments are the approved way to reuse ontology or contracts across otherwise independent subgraphs.

## Submodel Boundary Rule

Capability folders are physical containers for independent capability-rooted subgraphs.

Hierarchy should stay inside one logical subgraph:

- capability-to-capability hierarchy stays inside the same capability family
- requirement-to-requirement hierarchy stays inside the owning capability subgraph
- ontology-to-ontology hierarchy stays inside the ontology plane

Cross-subgraph reuse should use attachments, not hierarchy.

Avoid cross-submodel requirement hierarchy. If a requirement in one capability needs context from another capability, attach reusable ontology or a compatible requirement-owned refinement contract instead of creating a parent/child requirement relation across subgraphs.

## File Placement Heuristics

When adding or moving model content:

1. If it defines reusable meaning, put it in `Ontologies/`.
2. If it defines a system ability, put it as a `capability` under `Capabilities/<Area>/<CapabilityName>/`.
3. If it states an obligation, put it as a `requirement` in the owning capability folder.
4. If it elaborates an obligation or capability, put it as a refinement in the same capability folder.
5. If it is service or architecture detail for one capability, put it under that capability's `Architecture/` folder.
6. If it proves behavior, put it under `Verifications/<Domain>/`.
7. If another subgraph needs vocabulary, attach ontology from `Ontologies/`.
8. If another subgraph needs a reusable requirement-owned contract, attach that contract explicitly.

## Refactor Workflow

1. Inspect current physical containment:
   - `reqvire containment --json`
   - `reqvire search --json`
2. Inspect logical roots and boundaries:
   - `reqvire submodels --json`
   - `reqvire search --filter-type=capability --short`
   - `reqvire search --filter-type=ontology --short`
3. Classify misplaced content using the file placement heuristics.
4. Produce a move plan before editing:
   - element/file moves
   - relation rewrites
   - attachment substitutions
   - validation risks
5. Confirm high-risk boundary decisions with the user before bulk moves.
6. Apply changes in slices with `mv`, `mv-file`, `link`, `unlink`, or `relink`.
7. Validate after each slice:
   - `reqvire validate`
   - `reqvire lint`
   - `reqvire submodels`
   - `reqvire containment`
   - `reqvire coverage`

## What Not To Automate Blindly

- Do not force every repo into the example area names.
- Do not move content only to satisfy a folder pattern when the current graph is valid and understandable.
- Do not replace graph relations with path naming conventions.
- Do not remove cross-subgraph relations without preserving required context through attachments.
- Do not create a broad capability root only to share ontology.

The goal is clearer navigation and maintainability with graph semantics preserved.
