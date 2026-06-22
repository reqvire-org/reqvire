---
name: reqvire-concept-authoring
description: Expert workflow for authoring Reqvire native concept schemes and concepts as SKOS thesauri. Use when creating or revising `concept-scheme` or `concept` elements, designing controlled vocabularies, concept taxonomies, broader/narrower/related links, labels, definitions, scope notes, examples, mappings, concept scheme namespaces, or concept references to generated SKOS concepts. Use instead of ontology-authoring when the task is terminology/thesaurus/concept curation rather than structural OWL/Turtle vocabulary or SHACL contracts.
---

# Reqvire Concept Authoring

Author curated terminology as native Reqvire Markdown elements, not as SKOS Turtle inside ontology elements. Reqvire generates `skos:ConceptScheme` and `skos:Concept` RDF from `concept-scheme` and `concept` elements.

Use this skill for thesaurus work: stakeholder terminology, controlled vocabularies, synonyms, definitions, broader/narrower taxonomy, related concepts, concept mappings, and concept references from capabilities, requirements, contracts, or verifications.

Use `reqvire-ontology-authoring` instead when the work is structural OWL/RDFS vocabulary, classes, properties, individuals, axioms, SHACL targets, or semantic contracts.

## Native Concept Model

| Reqvire element | Generated SKOS | Purpose |
|---|---|---|
| `concept-scheme` | `skos:ConceptScheme` | Thesaurus or vocabulary root that owns `concept_base` and `concept_prefix` for generated concept IRIs. |
| `concept` | `skos:Concept` | One curated term, idea, engineering concern, stakeholder concept, or controlled vocabulary entry. |

Do not add `concept_id`, `concept_kind`, `pref_label`, or `language` metadata. Reqvire derives identity from the element identifier, `skos:prefLabel` from the element name, and `skos:definition` from the main element body. Language policy is intentionally deferred.

## Authoring Workflow

1. Create one `concept-scheme` root with `concept_base` and `concept_prefix`.
2. Keep concept schemes as standalone thesaurus roots, not ontology children.
3. Add child `concept` elements under the scheme or under broader concept groupings.
4. Put the definition in the main body before reserved subsections.
5. Add synonyms and stakeholder wording in `#### Labels`.
6. Add usage boundaries in `#### Scope Note`.
7. Add practical examples in `#### Examples`.
8. Add cross-vocabulary mappings in `#### Mappings` only when SKOS mapping semantics are intentional.
9. Use concept relations for taxonomy and association: `broader`, `narrower`, `related`, `exactMatch`, `closeMatch`.
10. Use `#### Concept References` from non-ontology, non-semantic-contract elements to bind prose to generated concept IRIs.

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
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" concepts export --include-mappings
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" model --filter-type=concept-scheme
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
```

For detailed examples, read `references/ConceptAuthoring.md`.
