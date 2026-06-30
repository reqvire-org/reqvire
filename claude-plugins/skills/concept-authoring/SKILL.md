---
name: reqvire-concept-authoring
description: Expert workflow for authoring Reqvire native concept schemes and concepts as SKOS thesauri. Use when creating or revising `concept-scheme` or `concept` elements, designing controlled vocabularies, concept taxonomies, broader/narrower/related links, labels, definitions, scope notes, examples, mappings, concept scheme namespaces, or concept references to native concept elements. Use instead of ontology-authoring when the task is terminology/thesaurus/concept curation rather than structural OWL/Turtle vocabulary or SHACL contracts.
---

# Reqvire Concept Authoring

Author curated terminology as native Reqvire Markdown elements, not as SKOS Turtle inside ontology elements. Reqvire generates `skos:ConceptScheme` and `skos:Concept` RDF from `concept-scheme` and `concept` elements.

Use this skill for thesaurus work: stakeholder terminology, controlled vocabularies, synonyms, definitions, broader/narrower taxonomy, related concepts, concept mappings, and concept references from capabilities, requirements, contracts, or verifications.

Use `reqvire-ontology-authoring` instead when the work is structural OWL/RDFS vocabulary, classes, properties, individuals, axioms, SHACL targets, or semantic contracts.

Use `reqvire-syseng` instead when the main change is to capabilities,
requirements, specifications, verification elements, evidence, coverage,
change-impact workflows, or broader model refactoring. This skill may update
concept references on those elements only as part of concept-authoring work.

## Reqvire Operations for Concept Authoring

Use Reqvire as the source-of-truth runtime when authoring or changing native
concept schemes and concepts.

- Work from the intended effective workspace root unless the user gives a different workspace; the workspace must contain at least one eligible Git worktree.
- Default CLI form: `npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" <command>`.
- Inside the Reqvire source repository, `cargo run -- <command>` is also acceptable when the local binary is the intended target.
- Inspect authored concepts with `search --filter-type concept-scheme --json`, `search --filter-type concept --json`, `model --filter-type concept-scheme --json`, `semantic export --layer concepts`, `concepts export`, and `concepts validate`.
- Use `semantic export --layer ontologies --layer concepts` when checking authored structural `reqvire:mapsToConcept` bridge triples together with generated native SKOS concepts. Use `semantic export --layer model` when checking downstream graph-store model facts.
- Prefer Reqvire CLI mutation commands for broad structural edits when available, such as `add`, `link`, `relink`, `mv`, `mv-file`, `mv-folder`, `rm`, and `rename-element`.
- Manual Markdown edits are valid for focused authoring. Preserve `# Elements`, `### Element Name`, `#### Metadata`, and Reqvire relation list syntax.
- Place concept-scheme and concept elements under `system-model/Thesaurus` unless the existing project uses a different `system-model/` content structure.
- Author concept-scheme elements with `type: concept-scheme`, `concept_base`, and `concept_prefix`; this element is the namespace and SKOS `ConceptScheme` holder.
- Author concept elements with `type: concept`; the main body is the SKOS definition-like human explanation. Do not add `#### Details`, `#### Ontology`, `#### Shapes`, `#### Definition`, or `#### Top Concepts` to concept elements.
- Use `#### Labels`, `#### Scope Note`, `#### Examples`, and `#### Mappings` for SKOS-specific concept authoring fields when needed.
- Use `#### Relations` for Reqvire-authored links: `derivedFrom` for scheme/parent containment, `broader` / `narrower` for taxonomy, `related` for associative links, and `exactMatch` / `closeMatch` for intentional concept-to-concept mappings.
- Relation syntax is `  * relationName: [Target Element](path.md#target-element)`.
- Keep ontology-to-concept bridges in ontology Turtle with `reqvire:mapsToConcept`; do not model those bridges as concept Markdown fallback data.
- When changing concept identity, check dependent ontology bridges, concept references, Explorer thesaurus data, MCP concept tools, and any tests that assert generated SKOS IRIs.
- When validation is part of the task, run focused checks such as `validate`, `semantic export --layer concepts`, `concepts validate`, and any affected fixture tests before finishing.

## Native Concept Model

| Reqvire element | Generated SKOS | Purpose |
|---|---|---|
| `concept-scheme` | `skos:ConceptScheme` | Thesaurus or vocabulary root that owns `concept_base` and `concept_prefix` for generated concept IRIs. |
| `concept` | `skos:Concept` | One curated term, idea, engineering concern, stakeholder concept, or controlled vocabulary entry. |

Do not add `concept_id`, `concept_kind`, `pref_label`, or `language` metadata. Reqvire derives identity from the element identifier, `skos:prefLabel` from the element name, and `skos:definition` from the main element body. Language policy is intentionally deferred.

## Concept and Element Naming

Prefer pure domain names for native `concept` elements. A concept should own the clean thesaurus term because generated SKOS concepts are terminology anchors.

Recommended precedence when an element name would otherwise collide:

1. `concept` gets the pure name: `Payment`, `Invoice`, `Traceability`, `Verification Coverage`.
2. `capability` becomes broader ability wording: `Payment Processing`, `Invoice Management`, `Traceability Management`, `API Operations`, `Audit Evidence Management`, `Fault Recovery`, `Regulatory Compliance Management`. Do not force every capability to be a `Feature`; use `Feature` only when the ability is genuinely product-feature shaped.
3. `requirement` gets obligation wording, usually with `Requirement`: `Payment Settlement Requirement`, `Invoice Export Requirement`, `Traceability Link Resolution Requirement`.
4. `specification` and other requirement-owned contracts get role-specific names: `Payment State Model Specification`, `Invoice Export Payload Specification`, `Retry Limit Constraint`, `Checkout Submission Behavior`, `Payment Lifecycle State`, `Webhook Event Input Output`.
5. Verification elements get validation wording: `Payment Settlement Validation Test`, `Invoice Export Contract Verification`, `Traceability Rollup Analysis`, `API Compatibility Inspection`.
6. `ontology` elements get structural/context holder names while Turtle can still define the pure term: element `Payment Ontology`, Turtle term `ex:Payment`.

Do not solve naming conflicts by making concept names artificial or by relying on `(type, name)` tuple identity. Reqvire element names stay globally unique; concepts get the pure terminology names, and other element families carry their role in the name.

## Concept Modeling Guardrails

Concepts are curated terminology. They are not ontology classes, requirements, obligations, specifications, lifecycle states, or implementation behavior. A concept definition should explain what the term means to people and tools; it should not say what the system shall do.

Use `broader` / `narrower` only when the relation passes the thesaurus "is-a kind of" test. Read `Child broader Parent` as "Child is a narrower term or kind of Parent." Good examples include `StripePaymentProvider broader PaymentProvider`, `CorrectionInvoice broader Invoice`, and `User broader Actor` when those are useful browse terms. Bad examples include generic catch-alls such as `PlatformConcept broader everything`, policy/obligation confusion such as `DataProtectionPolicy broader ComplianceObligation`, or status modeling when a property would be more precise.

Use `related` when concepts are associated but not a good browse-tree parent/child pair. Examples: `PendingPayment related Invoice`, `DataProtectionPolicy related ComplianceObligation`, `NotificationChannel related Notification`, and `ActivitySource related ActivityEvent`.

Do not blindly copy ontology `rdfs:subClassOf` into concept `broader`. OWL hierarchy is structural; SKOS hierarchy is thesaurus/navigation meaning. They can overlap, but they are not the same model. If the intended semantics are formal, such as `Payment hasStatus Pending`, model that with ontology classes/properties instead of forcing concept taxonomy.

In SKOS terms, `PendingPayment broader Payment` can be valid when "pending payment" is deliberately treated as a narrower payment term for browsing/search. In OWL/domain modeling, pending may instead be a lifecycle status or state property of a payment. Choose the layer based on the intended use.

Each concept scheme needs meaningful top concepts. Avoid making a scheme hang under a generic parent such as `PlatformConcept`; it creates weak diagrams and misleading modals. Each `concept-scheme` must also have a unique `concept_base` and `concept_prefix`; Reqvire rejects duplicate generated concept namespaces.

## Authoring Workflow

1. Create one `concept-scheme` root with `concept_base` and `concept_prefix`.
2. Keep concept schemes as standalone thesaurus roots under `system-model/Thesaurus`, not ontology children.
3. Add child `concept` elements under the scheme or under broader concept groupings.
4. Put the definition in the main body before reserved subsections.
5. Add synonyms and stakeholder wording in `#### Labels`.
6. Add usage boundaries in `#### Scope Note`.
7. Add practical examples in `#### Examples`.
8. Add cross-vocabulary mappings in `#### Mappings` only when SKOS mapping semantics are intentional.
9. Use concept relations for taxonomy and association: `broader`, `narrower`, `related`, `exactMatch`, `closeMatch`.
10. Use `#### Concept References` from non-ontology, non-semantic-contract elements to bind prose to native concept elements with Markdown links; Reqvire derives generated SKOS IRIs from those targets.

## Canonical Sections

Native `concept-scheme` and `concept` elements use:

- `#### Metadata`
- `#### Relations`
- `#### Labels`
- `#### Scope Note`
- `#### Examples`
- `#### Mappings`

Do not use these sections on native concept elements:

- `#### Ontology`
- `#### Shapes`
- `#### Concepts`
- `#### Details`
- `#### Definition`
- `#### Top Concepts`

Top concepts are derived from direct scheme child concepts that do not author `broader`; do not maintain a separate top-concepts list in Markdown.

## SKOS Subset

| Priority | SKOS terms | Reqvire authoring source |
|---|---|---|
| Core | `skos:Concept` | Native `concept` element. |
| Core | `skos:ConceptScheme` | Native `concept-scheme` element. |
| Core | `skos:inScheme` | Generated from nearest scheme ancestry. |
| Core | `skos:prefLabel` | Generated from element name. |
| Core | `skos:definition` | Generated from the main element body. |
| Core | `skos:broader` / `skos:narrower` | Authored concept relations. |
| Core | `skos:related` | Authored concept relation. |
| High | `skos:topConceptOf` / `skos:hasTopConcept` | Derived from direct scheme child concepts without `broader`. |
| High | `skos:altLabel` | `#### Labels` with `altLabel`. |
| High | `skos:scopeNote` | `#### Scope Note`. |
| High | `skos:example` | `#### Examples`. |
| High | `skos:exactMatch` / `skos:closeMatch` | `#### Mappings` or concept relations when intentional. |

## Example

````markdown
### Engineering Concepts

Curated engineering terminology for system-model authoring.

#### Metadata
  * type: concept-scheme
  * concept_base: https://example.org/concepts
  * concept_prefix: concept

---

### Traceability

The conceptual practice of connecting intent, implementation, verification, and evidence.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Engineering Concepts](#engineering-concepts)

#### Labels
  * altLabel: Trace link analysis

#### Scope Note
Use for engineering artifact traceability, not runtime distributed tracing.

#### Examples
A requirement traces to implementation and verification evidence.
````

## Structural Bridge

Keep structural ontology and curated concepts separate. If an OWL class or property needs an explicit concept anchor, use `reqvire:mapsToConcept` in ontology Turtle:

```turtle
@prefix concept: <https://example.org/concepts#> .
@prefix ex: <https://example.org/ontology/platform#> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:TraceLink a owl:Class ;
  reqvire:mapsToConcept concept:Traceability .
```

`reqvire:mapsToConcept` is an annotation bridge. It does not make the structural term a SKOS concept and does not imply OWL equivalence or SKOS mapping semantics.

## Commands

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type=concept-scheme --json
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-type=concept --json
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" concepts validate
# Export generated SKOS plus ontology-to-concept bridges when needed:
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" semantic export --layer ontologies --layer concepts
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" model --filter-type=concept-scheme
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
```

For detailed examples, read `references/ConceptAuthoring.md`.
