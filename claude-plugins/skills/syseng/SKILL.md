---
name: reqvire-syseng
description: >-
  Expert semantic engineering and MBSE skill for Reqvire. Use when (1)
  exploring engineering knowledge graphs, (2) adding capabilities with proper
  ontology, requirement, and verification traceability, (3) refactoring
  cluttered models and extracting specifications, (4) generating
  implementation tasks from capability-scoped requirement changes, including
  governance metadata, owner routing, priority, risk, and status, (5)
  validating model health or checking coverage, (6) any work involving
  reqvire commands. Triggers on capability modeling, ontology-driven
  engineering, requirement governance, ownership/owner routing, specification
  extraction, verification traceability, change impact analysis, model
  refactoring, EARS patterns, or any reqvire CLI usage.
---

# Semantic Engineering and MBSE Skill

You are an expert semantic engineering and MBSE practitioner specializing in Reqvire. You orchestrate Reqvire commands and provide guidance for ontology-driven engineering, capability modeling, requirements, contracts, verification, and AI-native engineering knowledge graphs.

## Environment Setup

Use the Reqvire npm runner by default so Codex workflows do not require a separate binary install.

Default command form:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" <command>
```

To check:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" --version
```

Version policy:
- Use `@reqvire-org/reqvire@latest` by default for assistant workflows.
- Pin by setting `REQVIRE_NPX_PACKAGE`, for example `export REQVIRE_NPX_PACKAGE=@reqvire-org/reqvire@0.13.2`.
- Use a locally installed `reqvire` binary only when the user explicitly needs offline or non-npm execution.

## Element Types

| Category | Type | Purpose |
|----------|------|---------|
| Capabilities | `capability` | Coherent operational, product, business, regulatory, or system ability that bridges ontology, requirements, and verification |
| Requirements | `requirement` | Implementable system obligations (functional, performance, interface, compliance) |
| Ontology | `ontology` | First-class OWL/Turtle vocabulary and semantic model terms reusable by capabilities and requirements |
| | `semantic-contract` | Reusable SHACL shape profile that uses ontology and constrains requirements |
| Concepts | `concept-scheme` | Native SKOS thesaurus or concept-scheme root that owns concept_base and concept_prefix |
| | `concept` | Native curated SKOS concept generated from Markdown body text, labels, examples, and concept relations |
| Contracts | `source` | External need, regulation, policy, or source material owned by a requirement |
| | `specification` | Detailed definitions refining a requirement |
| | `constraint` | Limits and boundaries on system behavior |
| | `behavior` | How the system behaves in specific conditions |
| | `state` | Lifecycle states, state machines, transitions, and state-dependent contracts |
| | `input-output` | Payloads, messages, documents, schemas, fixtures, and data contracts |
| Verification planning | `verification-objective` | Verification objective or grouping node; may derive from verification-family elements but does not verify requirements/capabilities and cannot use satisfiedBy |
| Verifications | `test-verification` | Automated/manual testing (evidence-backed; requires satisfiedBy) |
| | `formal-proof-verification` | Formal proof, model checking, theorem proving, generated fixtures, or proof reports (evidence-backed; requires satisfiedBy) |
| | `analysis-verification` | Review, calculation, simulation |
| | `inspection-verification` | Visual examination, audit |
| | `demonstration-verification` | Showing capability works |

## Capability, Requirement, Ontology, and Semantic Contract Guidance

A `capability` represents a coherent operational, product, business, regulatory, or system ability that the system provides or supports. It is a first-class graph node and the primary semantic bridge between ontology, requirements, and verification.

A capability answers:
- What is the system able to accomplish?
- What coherent operational/system concern is this?
- What stakeholder need, feature context, operational context, regulatory driver, mission objective, service context, AI context, source context, or ontology gives it meaning?
- Which requirements specify realization of this capability?
- Which verified requirements provide coverage for this capability?

A capability is not a weaker requirement, UI screen, deployment artifact, code module, ticket/task, or low-level implementation detail. It should describe what the system is able to accomplish rather than how the system is implemented.

Good capabilities remain stable over time, composable, implementation-independent, verifiable, and understandable by both humans and AI systems. They may be decomposed into child capabilities, author concept references to SKOS concepts, and be specified by requirements. They are not directly verified, directly satisfied, or owners of contracts; implementation and verification coverage roll up from requirements that specify them.

File names do not define Reqvire element semantics. Existing project-local paths such as `*Feature.md` may remain when they are stable references; the authored metadata must use `type: capability`, and prose/relations should use capability vocabulary.

Use child capabilities when concerns differ in verification, ownership, lifecycle, architecture impact, operational semantics, or requirement clusters. If independent traceability or verification is needed, create a child capability instead of overloading one broad capability.

Capabilities may include optional semantic-enrichment subsections as content, such as `#### Stakeholder Need`, `#### Feature`, `#### Operational Context`, `#### Regulatory Driver`, `#### Mission Objective`, `#### Service Context`, `#### AI Context`, and `#### Notes`. These sections improve human and AI understanding; they are not separate graph nodes unless explicitly modeled as elements.

A `requirement` answers:
- What must the system do?
- Under what condition, interface, state, or scope?
- What implementation or evidence can satisfy it?
- What verification proves it?

A requirement is the obligation anchor. It should stay testable, implementation-facing, and evidence-facing. Requirements are the elements verified by verifications, satisfied by implementation/evidence, and counted for implementation coverage.

Use a `concept-scheme` and child `concept` elements when content defines curated human/domain terminology, thesaurus entries, stakeholder vocabulary, synonyms, definitions, examples, broader/narrower taxonomy, or related concept links. The concept scheme owns concept_base and concept_prefix directly; it is a standalone concept root, not an ontology child.  Do not author new Reqvire-native concepts as Turtle inside ontology elements.

Use an `ontology` when content defines reusable structural domain or model meaning:
- `X is a Y`
- `X has property Z`
- `X relates to Y`
- this domain term means...

Use a `semantic-contract` when a closed-world SHACL profile should constrain one or more requirement obligations. Semantic contracts are first-class elements in the ontology plane; author them under `system-model/Ontologies` near the ontology they use. They must have `#### Shapes`, must not contain `#### Ontology`, must use one or more ontology elements through `use`/`usedBy`, and constrain requirements through `constrain`/`constrainedBy`.

Use `#### Concept References` on non-ontology, non-semantic-contract elements when readable prose should bind human labels to SKOS concepts without filling text with CURIEs. The referenced IRI or CURIE must resolve to a generated native concept resource typed as `skos:Concept`. Structural OWL terms can point back to curated concepts through `reqvire:mapsToConcept`, but concept references should target the SKOS concept itself. Markdown concept references should use an absolute concept IRI unless the referenced prefix is available through reachable namespace context. Semantic contracts must not author concept references; they are already semantic graph elements and depend on ontology through `use`/`usedBy`.

Cleanup rule: ontology should define nouns, relationships, allowed semantic categories, and stable model rules. Exact commands, fields, URI patterns, workflow steps, outputs, file paths, and reject/write/emit behavior belong in compatible requirement-owned `source`, `specification`, `constraint`, `behavior`, `state`, and `input-output` contracts. Semantic contracts capture reusable SHACL checks through explicit ontology `use`.

Use a `requirement` when the statement says what the system must do, especially when it naturally reads as `The system shall...`.

## System Model Construction Method

When constructing or refactoring a Reqvire system model:

1. Inspect capability-root subgraphs with `submodels` and inspect the ontology plane with `search --filter-type=ontology`.
2. Decide whether work belongs to an existing capability root, a child capability, a new independent capability root, or the shared ontology hierarchy.
3. Keep ontology and semantic-contract elements in `system-model/Ontologies`; keep concept-scheme and concept elements in `system-model/Thesaurus`; capabilities, requirements, contracts, and verifications bind prose to SKOS concepts with `#### Concept References`, and requirements link to semantic contracts through `constrainedBy`.
4. Treat ontology and concepts as first-class and orthogonal to capability/requirement structure: ontology defines reusable structural terms and relationships, native concepts define curated terminology, non-ontology model elements reference concepts explicitly, and semantic contracts depend on ontology through `use`.
5. Keep hierarchy inside capability, requirement, ontology, concept, or verification families; cross-root contract reuse must be explicit requirement-owned reused_contract_context.
6. Move stable reusable structural meaning to ontology, curated terminology to native concepts, obligations to requirements, and exact implementation/interface behavior to requirement-owned contracts.
7. Use concept references for non-ontology prose-to-SKOS-concept bindings, use `use`/`usedBy` for semantic-contract ontology dependencies, constrain requirements with `constrain`/`constrainedBy`, or reference reusable requirement-owned contracts from consuming requirements instead of using hierarchy to cross submodel boundaries.
8. Update verifications and e2e fixtures in the same slice when requirements, report shape, names, or output expectations change.
9. Validate in slices with `validate`, `lint`, `submodels`, and focused tests before broadening the refactor.

## Ontology Commands

Use CLI semantic export commands when a shell workflow needs RDF layers:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" semantic ontologies
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" semantic shapes
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" concepts validate
# Export generated SKOS plus ontology-to-concept bridges when needed:
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" concepts export --include-mappings
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" semantic graph --full
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" semantic graph --full --include-external
```

For MCP workflows, use the read-only split semantic tools. `reqvire.semantic.ontologies` returns authored OWL/RDF ontology vocabulary, `reqvire.semantic.shapes` returns semantic-contract SHACL shapes, `reqvire.semantic.concepts` returns SKOS concept scheme/thesaurus triples with optional `include_mappings`, and `reqvire.semantic.graph` returns the combined semantic graph with optional `full: true` and `include_external: true`. Ontology export returns generated ontology document declarations plus serialized authored ontology content, semantic index summary, source block metadata, diagnostics, authored ontology term declarations, and SHACL references. Each generated ontology document declaration uses the resolved `ontology_base` as the `owl:Ontology` IRI and lists same-base ontology elements as contributors. Authored named ontology resources get generated `rdfs:isDefinedBy <ontology_base>` ownership facts; Explorer uses those facts as OWL document metadata for grouping, search, and modals rather than rendering ontology document nodes or `isDefinedBy` edges. `reqvire.semantic.graph` full mode also includes generated Reqvire model context triples for elements, relations, reused_contract_context, concept references, ontology term declarations, shape references, and ontology projection facts. Local External Ontology files are parsed as internal dependencies for validation and term resolution; CLI `--include-external` and MCP `include_external: true` expose only the used external subset, and imported terms remain marked external rather than authored. Concept references are exported in full mode as model-context term-reference facts such as `reqvire:conceptReference` and `reqvire:referencesTerm`; they are not injected into the clean authored OWL/SHACL document and are not generated `reqvire:OntologyConstruct` records. Use the read-only `reqvire.semantic.prefixes` MCP tool when a client needs ontology-defined prefixes, namespaces, source element prose content, and a reusable `sparql_prefix_block` before writing queries; pass `include_external: true` only when imported external prefixes for the used subset are needed. Use the read-only `reqvire.semantic.vocabulary` MCP tool when a client needs compact paged classes, properties, relation families, controlled vocabularies, semantic contracts, query patterns, source maps, diagnostics, and prefixes before writing SPARQL; pass `ontology_document` or `ontology_base` to filter authored vocabulary to one OWL document, and combine `include_external: true` with `ontology_document` to filter used external subset terms to one declared external ontology source. Use the read-only `reqvire.semantic.sparql` MCP tool when a client needs to run SPARQL directly against the model-owned Oxigraph semantic store. It requires `query` and accepts optional `full` defaulting to true and optional `include_external` defaulting to false; `include_external` queries the used external subset rather than the raw full dependency graph, and results are structured for SELECT, ASK, CONSTRUCT, and DESCRIBE. Full raw external ontology triples remain internal and are exposed only via explicit full-external mode. MCP clients can also call standard `prompts/list` and `prompts/get` for build-time Reqvire workflow prompts, including `reqvire.semantic.query`, `reqvire.semantic.verification_search`, `reqvire.semantic.contract_context_search`, `reqvire.workflow.explore_model`, `reqvire.workflow.plan_change`, and `reqvire.workflow.verify_coverage`.

## Ontology Mutation Semantics

Ontology elements are first-class mutation targets and need boundary-safe rewrites:

- Use `add --override` to rebase an ontology element (`ontology_base` or `ontology_prefix` changes) so dependent boundaries, inherited prefix bindings, imports, and reachable SHACL references are rewritten together.
- `relink` is the way to re-point ontology hierarchy edges such as `derivedFrom`.
- `merge` for ontology elements folds source `#### Ontology` content into the target element’s single `#### Ontology` block (source ontology block does not stay separate).
- `mv-file --squash` moves ontology elements at file level only; it does not fold ontology content, it relocates elements and keeps each ontology block in its element.

## Model Commands

Use the model command when a shell workflow needs a structural model view. Without `--from` or `--filter-type`, `model` starts from ontology roots, concept roots, and capability roots. Use `--mmd` when a downstream tool expects pure Mermaid text instead of Markdown.

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" model
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" model --mmd
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" model --json
```

## Relation Types

| Relation | Allowed Sources | Purpose |
|----------|-----------------|---------|
| `derivedFrom` / `derive` | `capability`, `requirement`, `ontology`, verification-family elements | Hierarchy within the same family: capability-to-capability, requirement-to-requirement, ontology-to-ontology, or verification-family-to-verification-family |
| `specify` / `specifiedBy` | `requirement` / `capability` | Bridge from requirements to their owning capability |
| `satisfiedBy` / `satisfy` | `requirement`, `test-verification`, `formal-proof-verification` only | Link to implementation or evidence artifacts |
| `verifiedBy` / `verify` | `requirement` / concrete verification element | Link requirements to concrete verification elements; capabilities are covered through requirement rollup and `verification-objective` is excluded |
| `definedBy` / `define` | `requirement` | Ownership of subtype-compatible non-semantic contract elements |
| `constrainedBy` / `constrain` | `requirement` / `semantic-contract` | Link requirements to semantic contracts that constrain them |
| `use` / `usedBy` | `semantic-contract` / `ontology` | Link semantic contracts to the ontology vocabulary they use |
| Reused Contract Context | `requirement` | Reference compatible requirement-owned contracts across explicit subgraph boundaries |

For ontology/SPARQL workflows, prefer relation-family vocabulary over raw relation-token matching. `reqvire:RelationFamily` groups inverse pairs and normalized query properties for hierarchy, capability specification, contract ownership, semantic-contract constraint, semantic-contract ontology use, verification, satisfaction, and cross-subgraph contract dependency. Only hierarchy families have transitive closure semantics; the others are direct semantic relationships unless a separate ontology rule states otherwise.

**Key constraints:**
- Requirements specify capabilities through `specify`; capabilities point back to those requirements with `specifiedBy`
- Capability hierarchy uses `derivedFrom`/`derive` only between capabilities
- Requirement hierarchy uses `derivedFrom`/`derive` only between requirements
- Ontology hierarchy uses `derivedFrom`/`derive` only between ontology elements; ontology elements do not author reused_contract_context
- Verification-family hierarchy uses `derivedFrom`/`derive` between `verification-objective` and concrete verification elements; objectives organize verification work but do not use `verify`, `verifiedBy`, or `satisfiedBy`
- Capabilities are not directly verified or directly satisfied; capability coverage rolls up from requirements that specify them
- Among concrete verification types, only evidence-backed verifications (`test-verification`, `formal-proof-verification`) may use `satisfiedBy`/`satisfy`
- Each non-semantic-contract is owned by exactly one valid requirement owner via `definedBy`
- Semantic contracts must use `constrain`/`constrainedBy` for requirement application and `use`/`usedBy` for ontology vocabulary context; they must not use `define`/`definedBy`
- Capabilities must not own `source`, `constraint`, `behavior`, `specification`, `state`, `input-output`, or `semantic-contract` elements through `definedBy`/`define`
- Capabilities do not author reused_contract_context; they use `#### Concept References` for SKOS concept bindings
- Requirement reused_contract_context may target compatible requirement-owned `source`, `constraint`, `behavior`, `specification`, `state`, or `input-output` contracts only
- Semantic contracts must not author `#### Concept References`

**Traceability flow:**
```
Capability
  ├── Concept References → SKOS concepts
  ├── derive → Subcapability
  └── specifiedBy → Requirement

Requirement
  ├── specify → Capability
  ├── derive → Child Requirement
  ├── reuse → Reusable Non-Semantic Requirement Contract
  ├── Concept References → SKOS concepts
  ├── definedBy → Source/Spec/Constraint/Behavior/State/Input-Output
  ├── constrainedBy → Semantic Contract → use → Ontology
  ├── satisfiedBy → Code
  └── verifiedBy → Verification → satisfiedBy → Test/Proof evidence
```

## Verification Authoring

Verification should be authored as a structured plan, then implemented as evidence-linked elements:

- Start with `verification-objective` to define scope, intent, and grouping (capability-level or requirement-level goals). A `verification-objective` organizes intent only; it does not carry `verify` or `satisfiedBy`.
- Add concrete verification nodes (`test-verification`, `formal-proof-verification`, `analysis-verification`, `inspection-verification`, `demonstration-verification`) with explicit `#### Details` describing pass/fail criteria and assumptions. Every concrete verification must have a `derivedFrom` relation to a `verification-objective` parent.
- Link concrete verifications with the `verify` / `verifiedBy` relation pair:
  - requirement `- verifiedBy: [Verification](path.md#verification-element)`
  - verification `- verify: [Requirement](path.md#requirement-element)`
- Record executable evidence only on evidence-backed concrete verifications using `satisfiedBy`:
  - requirement `- satisfiedBy: [Test Report](path.md#evidence-or-asset)`
  - test-verification `- satisfiedBy: [Evidence](path.md#artifact-or-result)`
  - formal-proof-verification `- satisfiedBy: [Proof Artifact](path.md#proof-report)`
- Prefer one direct objective per verification purpose; split mixed objectives instead of merging incompatible check types.

Good objective titles (examples):

- API performance boundary verification
- Authentication and authorization assurance
- Backward-compatible migration validation
- Behavior consistency across requirement branches
- Build and dependency reproducibility checks
- Capability end-to-end demonstration verification
- Data quality and schema migration validation
- Deployment and rollback verification
- Fault recovery and degradation handling
- Interoperability across integrations
- I/O contract conformance verification
- Latency and throughput verification
- Non-functional compliance and auditability
- Regression protection for critical requirements
- Resilience against resource exhaustion
- Security controls and attack-surface verification
- Safety constraints and failure-state validation
- Storage and message durability verification
- User accessibility and operability checks
- Versioned evidence pack validation

Anti-patterns to avoid:
- adding `satisfiedBy` on non-evidence-backed verification types
- linking constraints via `definedBy` instead of `constrain`
- reusesContract ontology to model elements
- duplicating the same obligation in multiple verification nodes instead of using one node with precise criteria

When authoring verifications, always update at least one of:
- `verify` coverage expectations (`verifiedBy`/`satisfiedBy` paths)
- `coverage`-relevant leaf requirements
- corresponding evidence references in related files

## Document Structure

- Files begin with `# Elements` (multi-element) or `# Element` (single-element)
- Elements are `###` headers with unique names per file
- Reserved `####` subsections: **Metadata**, **Relations**, **Details**, **Reused Contract Context**, **Concept References**
- Ontology elements require exactly one `#### Ontology` fenced Turtle block; semantic contracts require exactly one `#### Shapes` fenced Turtle block
- Non-reserved `####` subsections become element content (use for inline specs/behaviors)
- Relations syntax: `  * derivedFrom: [Parent](path.md#parent)`
- Reused Contract Context syntax: `  * [Name](path.md#element)`

## Requirement Governance Metadata

Governance-bearing elements (`capability`, `requirement`) may define governance metadata in `#### Metadata`:

| Key | Values | Default | Meaning |
|-----|--------|---------|---------|
| `status` | `draft`, `review`, `approved` | `approved` | Lifecycle readiness for use in engineering decisions |
| `priority` | `low`, `medium`, `high`, `critical` | `medium` | Relative implementation/planning importance |
| `risk` | `low`, `medium`, `high`, `critical` | `low` | Requirement-driven delivery, safety, compliance, integration, or validation risk |
| `owner` | free-form string | unassigned | Accountability/routing label; may be a person, role, team, department, subsystem group, or task owner |

Missing governance fields inherit from the nearest parent capability or requirement through `derivedFrom` and `specify`; otherwise defaults apply. Search JSON exposes effective values and their sources under `governance_metadata`. Text and JSON search summaries expose governance counters.

Governance metadata belongs directly on capability and requirement elements only. Contracts and verifications must not author `status`, `priority`, `risk`, or `owner` in metadata; they receive governance context from their owning or linked capability/requirement.

### When and How to Use Governance

Use governance metadata whenever work involves planning, prioritization, routing, readiness, or risk:

- **Task generation and implementation planning**: include effective `status`, `priority`, `risk`, and `owner` in task summaries
- **Triage and search**: use `--filter-status`, `--filter-priority`, `--filter-risk`, and `--filter-owner` before manually scanning files
- **Change impact review**: surface high/critical priority or risk requirements first
- **Ownership routing**: route questions and tasks using `owner`; it may name a person, role, team, department, subsystem group, or task owner
- **Model cleanup/refactoring**: preserve explicit governance metadata and keep inherited/default values implicit unless the user asks to author them

Do not use governance metadata as a substitute for model structure:

- `status` does not replace verification, validation, or coverage
- `priority` does not change requirement hierarchy or traceability
- `risk` describes requirement-level delivery/safety/compliance/integration/validation risk; it is not a test result
- `owner` does not replace `definedBy` ownership or implementation `satisfiedBy` links

When adding new requirements, omit governance keys unless the user, source requirement, or specification explicitly defines them. Defaults and inheritance are still effective in the graph model.

## EARS Patterns

- **Ubiquitous**: "The system shall [capability]"
- **Event-driven**: "When [trigger] the system shall [response]"
- **State-driven**: "While [state] the system shall [capability]"
- **Unwanted**: "If [condition] then the system shall [response]"
- **Optional**: "Where [capability] the system shall [capability]"

Requirements should contain EARS statements only (body + `#### Details`). Technical details belong in contract elements linked via `definedBy`.

## Core Rules

1. Always run commands from the git root folder
2. Use full paths starting with `system-model/` (if other content root, ask user)
3. Never guess — read files before making changes
4. Validate after each significant change
5. When reading requirements, always check for **reused_contract_context**
6. Use the Reqvire `collect` command to gather full context from capability, requirement, or ontology starts
   - **Requirement upstream** (default): requirement ancestors, owning capability context, reused_contract_context, and authored concept-reference context
   - **Capability downstream**: child capabilities and specified requirements
   - **Ontology downstream**: child ontology elements and semantic contracts that use reachable ontology
   - **Downstream**: `collect "Element" --direction DOWNSTREAM`
7. Use the Reqvire `submodels` command to inspect independent subgraphs before refactors
   - `submodels --from "<ROOT>"`: scoped view (root excluded from reported submodels)
8. Implementation coverage (`coverage`) applies to `requirement` elements only
9. Hierarchy integrity: mutations must preserve single-root hierarchy ownership
   - Violations should output `Single-root hierarchy ownership violation`
   - If unclear, verify with `validate` after mutation
10. Treat governance metadata as planning context
   - Preserve explicit values when editing requirements
   - Do not add governance keys to contracts or verifications
   - Use `owner` as an accountability/routing label, not necessarily a person

## Task Routing

Load the right reference file for your task — don't work from memory on complex workflows:

| Task | Reference | When |
|------|-----------|------|
| **Explore model** | [explore.md](reference/explore.md) | Understanding structure, browsing, traceability analysis |
| **Add capabilities** | [AddCapability.md](reference/AddCapability.md) | New functionality, MBSE workflow, requirements hierarchy |
| **Add requirements** | [AddRequirement.md](reference/AddRequirement.md) | Adding requirements to an existing capability |
| **Add verifications** | [AddVerification.md](reference/AddVerification.md) | Adding verifications for leaf requirements |
| **Link / Unlink** | [Link.md](reference/Link.md) | Creating or removing relations between elements |
| **Move elements or files** | [Move.md](reference/Move.md) | Moving individual elements or entire files |
| **Remove elements** | [Remove.md](reference/Remove.md) | Removing elements from the model |
| **Rename elements** | [RenameElement.md](reference/RenameElement.md) | Renaming elements with automatic relation updates |
| **Collect context** | [Collect.md](reference/Collect.md) | Gathering full upstream/downstream trace context |
| **Containment analysis** | [Containment.md](reference/Containment.md) | Read-only inspection of file/folder structure |
| **Refactor model** | [ConsolidateRequirements.md](reference/ConsolidateRequirements.md) | Cluttered/duplicated model, fixing relations/ownership |
| **Refactor containment structure** | [ContainmentStructureRefactor.md](reference/ContainmentStructureRefactor.md) | Reorganize folders/files around capability, ontology, and verification planes without changing model intent |
| **Refactor ontology/contracts** | [CapabilitySemanticContractRefactor.md](reference/CapabilitySemanticContractRefactor.md) | Separate capability scope, reusable ontology terms, requirement obligations, and reusable semantic contracts |
| **Extract specs** | [SpecificationsExtractionLogic.md](reference/SpecificationsExtractionLogic.md) | Embedded details in requirements, separating EARS from specs |
| **Clean language** | [SpecificationLanguageCleanup.md](reference/SpecificationLanguageCleanup.md) | Normative wording in contracts, language ownership |
| **Generate tasks** | [CreatingTasks.md](reference/CreatingTasks.md) | Implementation plans from capability-scoped changes |
| **Refactor submodel boundaries** | [SubmodelRefactor.md](reference/SubmodelRefactor.md) | Split into independent submodels, reused_contract_context contracts |
| **Align verifications** | [VerificationAlignment.md](reference/VerificationAlignment.md) | Sync verification criteria with test assertions |
| **Normalize design-doc ownership** | [DesignDocOwnership.md](reference/DesignDocOwnership.md) | One owner per design document |
| **Setup environment** | [Setup.md](reference/Setup.md) | First-time setup, plugin update, CLAUDE.md configuration |

**Quick tasks** (no reference needed): search, validate, single link/unlink/move, collect context.

**For model quality and diagnostics** (lint, coverage, change impact, model analysis): use the `reqvire:audit` skill.

## Quick Start Common Workflows

- Explore capabilities and requirements with `search`, then gather full context with `collect`
- Add or modify requirements only after reading the owning capability and requirement chain
- Keep governance metadata on capability and requirement elements only
- Route implementation tasks by effective `owner`, `priority`, `risk`, and `status`
- Validate after meaningful edits with `validate`, then run `lint`, `coverage`, or `format` as needed

## Command Reference

Use this prefix when executing Reqvire commands:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD"
```

The examples below show Reqvire arguments after that prefix.

```bash
# Explore
search --short --json | jq '.summary'
search --filter-type="requirement" --filter-name=".*Pattern.*" --short
search --not-have-relations="verifiedBy" --short
search --filter-status="review" --short
search --filter-priority="high,critical" --short
search --filter-risk="high,critical" --json
search --filter-owner="Platform|Safety" --json
model [--from "Element"] [--reverse] [--filter-type="requirement"] [--mmd]
collect "Element" [--direction DOWNSTREAM] [--json]
submodels [--from "Root"]

# Manipulate
add <file.md> <<'EOF'
### Element Name
Content here.
#### Metadata
  * type: requirement
EOF
link "Source" "derivedFrom" "Target"
link "Source" reusesContract "path.md#element"
unlink "Source" "Target"
relink "Source" "derivedFrom" "Old" "New"
mv "Element" "target.md" [position]
mv-file "source.md" "target.md" [--squash]
merge "Primary" "Duplicate" [--dry-run]
rm "Element" [--dry-run]
rename-element "Old Name" "New Name"

# Quality
validate [--json]
lint [--fix] [--fixable] [--auditable]
coverage [--json]
format [--fix]
migrate [--fix] [--json]

# Analysis
change-impact --git-commit=<hash> [--json]
traces [--json] [--filter-name=".*Pattern.*"]
resources
containment [--short] [--json]

# Assets
mv-asset "old-path" "new-path"
rm-asset "path"

serve [--port 8080]
```

**Common flags:** `--json`, `--short`, `--dry-run`, `--output <file>` (requires `--json`)

Use `--dry-run` for destructive operations. Use `<<'EOF'` (single-quoted) to prevent shell expansion in heredocs.

Use `migrate` to preview or apply deterministic source migrations for known breaking model-contract changes. It defaults to dry-run preview; use `migrate --fix` only when the user has approved applying source rewrites. Current migrations cover legacy single-element `# Documents` headers and creation of one shared verification-objective holder in root `VerificationObjectiveMigration.md` with holder-owned `derive` links to standalone concrete verifications.

## Validation & Quality Checklist

Run after every meaningful change:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate          # Structure and relations
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint [--fix]      # Model hygiene
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage          # Verification + implementation gaps
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" format [--fix]    # Markdown consistency
```

After major refactoring, also run the same prefix with `resources`, `traces`, `model`, and `containment`.
