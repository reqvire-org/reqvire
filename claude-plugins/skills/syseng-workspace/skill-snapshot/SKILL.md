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

You are an expert semantic engineering and MBSE practitioner specializing in Reqvire. You orchestrate Reqvire commands and provide guidance for ontology-driven engineering, capability modeling, requirements, refinements, verification, and AI-native engineering knowledge graphs.

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
| Refinements | `source` | External need, regulation, policy, or source material owned by a requirement |
| | `specification` | Detailed definitions refining a requirement |
| | `constraint` | Limits and boundaries on system behavior |
| | `behavior` | How the system behaves in specific conditions |
| | `state` | Lifecycle states, state machines, transitions, and state-dependent contracts |
| | `input-output` | Payloads, messages, documents, schemas, fixtures, and data contracts |
| | `semantic-contract` | Requirement-owned SHACL shape profile over reachable ontology context |
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
- Which verification evidence directly verifies the capability when capability-level evidence is appropriate?

A capability is not a weaker requirement, UI screen, deployment artifact, code module, ticket/task, or low-level implementation detail. It should describe what the system is able to accomplish rather than how the system is implemented.

Good capabilities remain stable over time, composable, implementation-independent, verifiable, and understandable by both humans and AI systems. They may be decomposed into child capabilities, attach ontology, be specified by requirements, and be directly verified. They are not directly satisfied and do not own refinements; implementation coverage rolls up from requirements that specify them.

File names do not define Reqvire element semantics. Existing project-local paths such as `*Feature.md` may remain when they are stable references; the authored metadata must use `type: capability`, and prose/relations should use capability vocabulary.

Use child capabilities when concerns differ in verification, ownership, lifecycle, architecture impact, operational semantics, or requirement clusters. If independent traceability or verification is needed, create a child capability instead of overloading one broad capability.

Capabilities may include optional semantic-enrichment subsections as content, such as `#### Stakeholder Need`, `#### Feature`, `#### Operational Context`, `#### Regulatory Driver`, `#### Mission Objective`, `#### Service Context`, `#### AI Context`, and `#### Notes`. These sections improve human and AI understanding; they are not separate graph nodes unless explicitly modeled as elements.

A `requirement` answers:
- What must the system do?
- Under what condition, interface, state, or scope?
- What implementation or evidence can satisfy it?
- What verification proves it?

A requirement is the obligation anchor. It should stay testable, implementation-facing, and evidence-facing. Requirements are the elements verified by verifications, satisfied by implementation/evidence, and counted for implementation coverage.

Use an `ontology` when content defines reusable domain or model meaning:
- `X is a Y`
- `X has property Z`
- `X relates to Y`
- this domain term means...

Use a `semantic-contract` when one requirement obligation needs a closed-world SHACL profile over reachable ontology terms. Semantic contracts must have `#### Shapes`, must refine exactly one compatible requirement owner, and must not contain `#### Ontology`.

Use `#### Concept References` when readable prose should bind human labels to ontology terms without filling the requirement text with CURIEs. The referenced IRI or CURIE must be declared by reachable ontology context.

Cleanup rule: ontology should define nouns, relationships, allowed semantic categories, and stable model rules. Exact commands, fields, URI patterns, workflow steps, outputs, file paths, and reject/write/emit behavior belong in compatible requirement-owned `source`, `specification`, `constraint`, `behavior`, `state`, and `input-output` refinements. Requirement-owned `semantic-contract` refinements capture requirement-specific SHACL profiles.

Use a `requirement` when the statement says what the system must do, especially when it naturally reads as `The system shall...`.

## System Model Construction Method

When constructing or refactoring a Reqvire system model:

1. Inspect capability-root subgraphs with `submodels` and inspect the ontology plane with `search --filter-type=ontology`.
2. Decide whether work belongs to an existing capability root, a child capability, a new independent capability root, or the shared ontology hierarchy.
3. Keep ontology elements in `requirements/Ontologies`; capabilities attach ontology from there instead of nesting ontology in capability files.
4. Treat ontology as first-class and orthogonal to capability/requirement structure: ontology defines reusable terms and relationships; capabilities attach ontology so child capabilities and specifying requirements inherit it through the owning capability path.
5. Keep hierarchy inside capability, requirement, or ontology families; cross-root reuse must be explicit attachments.
6. Move stable reusable meaning to ontology; keep obligations in requirements and exact implementation/interface behavior in requirement-owned refinements.
7. Attach ontology to the consuming capability, or attach reusable requirement-owned contracts to consuming requirements, instead of using hierarchy to cross submodel boundaries.
8. Update verifications and e2e fixtures in the same slice when requirements, report shape, names, or output expectations change.
9. Validate in slices with `validate`, `lint`, `submodels`, and focused tests before broadening the refactor.

## Ontology Commands

Use CLI ontology collection when a shell workflow needs reusable RDF/SHACL output:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" ontologies
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" ontologies --jsonld
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" ontologies --full
```

For MCP workflows, use the read-only `reqvire.ontologies` tool. It accepts optional `format: "turtle"` or `format: "jsonld"` and optional `full: true`. Default mode returns serialized authored ontology/SHACL content plus semantic index summary, source block metadata, diagnostics, ontology declarations, and SHACL references. Full mode also includes generated Reqvire model context triples for elements, relations, attachments, concept references, ontology declarations, and shape references.

## Model Commands

Use the model command when a shell workflow needs a structural model view. Without `--from` or `--filter-type`, `model` starts from ontology roots and capability roots. Use `--mmd` when a downstream tool expects pure Mermaid text instead of Markdown.

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" model
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" model --mmd
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" model --json
```

## Relation Types

| Relation | Allowed Sources | Purpose |
|----------|-----------------|---------|
| `derivedFrom` / `derive` | `capability`, `requirement`, `ontology` | Hierarchy within the same family: capability-to-capability, requirement-to-requirement, or ontology-to-ontology |
| `specify` / `specifiedBy` | `requirement` / `capability` | Bridge from requirements to their owning capability |
| `satisfiedBy` / `satisfy` | `requirement`, `test-verification`, `formal-proof-verification` only | Link to implementation or evidence artifacts |
| `verifiedBy` / `verify` | `capability`, `requirement` / verification element | Link capabilities and requirements to verification elements |
| `refinedBy` / `refine` | `requirement` | Ownership of subtype-compatible refinement elements |
| `trace` | Any | Non-directional traceability |
| Attachments | `capability`, `requirement` | Reference existing ontology or compatible requirement-owned refinement contracts across explicit subgraph boundaries |

**Key constraints:**
- Requirements specify capabilities through `specify`; capabilities point back to those requirements with `specifiedBy`
- Capability hierarchy uses `derivedFrom`/`derive` only between capabilities
- Requirement hierarchy uses `derivedFrom`/`derive` only between requirements
- Ontology hierarchy uses `derivedFrom`/`derive` only between ontology elements; ontology elements do not author attachments
- Capabilities may be directly verified but are not directly satisfied; capability coverage also rolls up from requirements that specify them
- Among verification types, only evidence-backed verifications (`test-verification`, `formal-proof-verification`) may use `satisfiedBy`/`satisfy`
- Each refinement is owned by exactly one valid requirement owner via `refinedBy`
- Capabilities must not own `source`, `constraint`, `behavior`, `specification`, `state`, `input-output`, `semantic-contract`, or `semantic-query-contract` refinements
- Capability attachments may target `ontology` elements only
- Requirement attachments may target compatible requirement-owned `source`, `semantic-contract`, `constraint`, `behavior`, `specification`, `state`, or `input-output` refinements only

**Traceability flow:**
```
Capability
  ├── attach → Ontology
  ├── derive → Subcapability
  ├── verifiedBy → Verification
  └── specifiedBy → Requirement

Requirement
  ├── specify → Capability
  ├── derive → Child Requirement
  ├── attach → Reusable Requirement Contract
  ├── refinedBy → Source/Semantic-Contract/Spec/Constraint/Behavior/State/Input-Output
  ├── satisfiedBy → Code
  └── verifiedBy → Verification → satisfiedBy → Test/Proof evidence
```

## Document Structure

- Files begin with `# Elements` (multi-element) or `# Element` (single-element)
- Elements are `###` headers with unique names per file
- Reserved `####` subsections: **Metadata**, **Relations**, **Details**, **Attachments**, **Concept References**
- Ontology elements require exactly one `#### Ontology` fenced Turtle block; semantic contracts require exactly one `#### Shapes` fenced Turtle block
- Non-reserved `####` subsections become element content (use for inline specs/behaviors)
- Relations syntax: `  * derivedFrom: [Parent](path.md#parent)`
- Attachments syntax: `  * [Name](path.md#element)`

## Requirement Governance Metadata

Governance-bearing elements (`capability`, `requirement`) may define governance metadata in `#### Metadata`:

| Key | Values | Default | Meaning |
|-----|--------|---------|---------|
| `status` | `draft`, `review`, `approved` | `approved` | Lifecycle readiness for use in engineering decisions |
| `priority` | `low`, `medium`, `high`, `critical` | `medium` | Relative implementation/planning importance |
| `risk` | `low`, `medium`, `high`, `critical` | `low` | Requirement-driven delivery, safety, compliance, integration, or validation risk |
| `owner` | free-form string | unassigned | Accountability/routing label; may be a person, role, team, department, subsystem group, or task owner |

Missing governance fields inherit from the nearest parent capability or requirement through `derivedFrom` and `specify`; otherwise defaults apply. Search JSON exposes effective values and their sources under `governance_metadata`. Text and JSON search summaries expose governance counters.

Governance metadata belongs directly on capability and requirement elements only. Refinements and verifications must not author `status`, `priority`, `risk`, or `owner` in metadata; they receive governance context from their owning or linked capability/requirement.

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
- `owner` does not replace `refinedBy` ownership or implementation `satisfiedBy` links

When adding new requirements, omit governance keys unless the user, source requirement, or specification explicitly defines them. Defaults and inheritance are still effective in the graph model.

## EARS Patterns

- **Ubiquitous**: "The system shall [capability]"
- **Event-driven**: "When [trigger] the system shall [response]"
- **State-driven**: "While [state] the system shall [capability]"
- **Unwanted**: "If [condition] then the system shall [response]"
- **Optional**: "Where [capability] the system shall [capability]"

Requirements should contain EARS statements only (body + `#### Details`). Technical details belong in refinement elements linked via `refinedBy`.

## Core Rules

1. Always run commands from the git root folder
2. Use full paths starting with `requirements/` (if other content root, ask user)
3. Never guess — read files before making changes
4. Validate after each significant change
5. When reading requirements, always check for **attachments**
6. Use the Reqvire `collect` command to gather full context from requirement chains
   - **Upstream** (default): ancestors via `derivedFrom` + attachments
   - **Downstream**: `collect "Element" --direction DOWNSTREAM` — all children to leaves
7. Use the Reqvire `submodels` command to inspect independent subgraphs before refactors
   - `submodels --from "<ROOT>"`: scoped view (root excluded from reported submodels)
8. Implementation coverage (`coverage`) applies to `requirement` elements only
9. Hierarchy integrity: mutations must preserve single-root hierarchy ownership
   - Violations should output `Single-root hierarchy ownership violation`
   - If unclear, verify with `validate` after mutation
10. Treat governance metadata as planning context
   - Preserve explicit values when editing requirements
   - Do not add governance keys to refinements or verifications
   - Use `owner` as an accountability/routing label, not necessarily a person

## Task Routing

Load the right reference file for your task — don't work from memory on complex workflows:

| Task | Reference | When |
|------|-----------|------|
| **Explore model** | [explore.md](references/explore.md) | Understanding structure, browsing, traceability analysis |
| **Add capabilities** | [AddCapability.md](references/AddCapability.md) | New functionality, MBSE workflow, requirements hierarchy |
| **Refactor model** | [ConsolidateRequirements.md](references/ConsolidateRequirements.md) | Cluttered/duplicated model, fixing relations/ownership |
| **Refactor capability/semantic contracts** | [CapabilitySemanticContractRefactor.md](references/CapabilitySemanticContractRefactor.md) | Split capability scope, reusable ontology/semantic-contract meaning, and requirement obligations |
| **Extract specs** | [SpecificationsExtractionLogic.md](references/SpecificationsExtractionLogic.md) | Embedded details in requirements, separating EARS from specs |
| **Clean language** | [SpecificationLanguageCleanup.md](references/SpecificationLanguageCleanup.md) | Normative wording in refinements, language ownership |
| **Generate tasks** | [CreatingTasks.md](references/CreatingTasks.md) | Implementation plans from capability-scoped changes |
| **Refactor submodel boundaries** | [SubmodelRefactor.md](references/SubmodelRefactor.md) | Split into independent submodels, attachment contracts |
| **Align verifications** | [VerificationAlignment.md](references/VerificationAlignment.md) | Sync verification criteria with test assertions |
| **Normalize design-doc ownership** | [DesignDocOwnership.md](references/DesignDocOwnership.md) | One owner per design document |

**Quick tasks** (no reference needed): search, validate, single link/unlink/move, collect context.

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
link "Source" attaching "path.md#element"
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

## Validation & Quality Checklist

Run after every meaningful change:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate          # Structure and relations
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint [--fix]      # Model hygiene
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage          # Verification + implementation gaps
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" format [--fix]    # Markdown consistency
```

After major refactoring, also run the same prefix with `resources`, `traces`, `model`, and `containment`.
