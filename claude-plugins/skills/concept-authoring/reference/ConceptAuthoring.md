# Reqvire Concept Authoring Reference

Use native Reqvire concept elements for thesaurus work. Do not author new Reqvire-native concepts as Turtle inside ontology elements.

## Placement

- Put shared concept schemes under `system-model/Thesaurus` or the project's semantic vocabulary folder.
- Put stable concept IRI base/prefix metadata directly on each `concept-scheme`.
- Use one or more `concept-scheme` roots for distinct vocabularies.
- Use child `concept` elements for terms.

## Concept Scheme Pattern

````markdown
### Reqvire Concepts

Curated concept scheme for Reqvire system modeling terms.

#### Metadata
  * type: concept-scheme
  * concept_base: https://www.reqvire.org/concepts
  * concept_prefix: concept
````

## Concept Pattern

````markdown
### Verification Evidence

Evidence that supports a verification result.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concepts](#reqvire-concepts)
  * related: [Traceability](#traceability)

#### Labels
  * altLabel: Verification artifact

#### Scope Note
Use for evidence tied to verification outcomes, not for arbitrary project files.

#### Examples
A test report satisfies a test-verification element.
````

## Relation Semantics

- `derivedFrom` gives concept-scheme context or concept grouping context.
- `broader` / `narrower` represent SKOS taxonomy, not Reqvire containment and not OWL subclassing.
- `related` represents non-hierarchical concept association.
- `exactMatch` means concepts can be used interchangeably across vocabularies.
- `closeMatch` means concepts are sufficiently similar for many uses but not interchangeable.

Use `derivedFrom` for authoring context; use SKOS relations for thesaurus semantics.

## Concept References

Use concept references on non-ontology, non-semantic-contract elements when prose should bind to a generated concept:

```markdown
#### Concept References
  * Verification evidence: https://www.reqvire.org/concepts#VerificationEvidence
```

The target must resolve to a generated native `skos:Concept` resource. Prefer absolute IRIs unless the prefix is available through reachable namespace context.

## Quality Checks

- Every concept scheme needs `concept_base` and `concept_prefix` metadata.
- Every concept needs a nearest concept-scheme context.
- Every concept should have a clear main-body definition before `#### Metadata` or other reserved subsections.
- Avoid duplicating top-concept lists; Reqvire derives top concepts.
- Do not put `#### Details`, ontology, or SHACL blocks on concept elements.
- Do not use language metadata until Reqvire has a language policy.
