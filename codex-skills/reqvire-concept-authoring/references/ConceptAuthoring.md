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

## Taxonomy And Association Rules

Concept definitions explain terminology. Do not write requirements, obligations, implementation rules, or formal schema constraints as concept definitions.

Use this decision rule for concept relations:

- Use `broader` when the current concept is a narrower thesaurus term under the target concept.
- Use `narrower` only as the inverse authoring direction for the same kind of taxonomy edge.
- Use `related` when concepts are associated but not true browse-tree parent/child terms.
- Use `exactMatch` or `closeMatch` for intentional concept-to-concept mapping across vocabularies.
- Use ontology classes/properties when the intended meaning is formal structure, such as `Payment hasStatus Pending`.

Good taxonomy examples:

- `StripePaymentProvider broader PaymentProvider`
- `CorrectionInvoice broader Invoice`
- `User broader Actor`
- `PendingPayment broader Payment` when pending payment is intended as a narrower payment term in the thesaurus.

Association examples that should usually use `related`:

- `PendingPayment related Invoice`
- `DataProtectionPolicy related ComplianceObligation`
- `NotificationChannel related Notification`
- `ActivitySource related ActivityEvent`

Avoid:

- Generic top concepts such as `PlatformConcept` that every other concept hangs under.
- Copying every `rdfs:subClassOf` assertion into `broader` without checking thesaurus/navigation meaning.
- Modeling states, methods, policies, attempts, channels, or sources as taxonomy children when a formal ontology property or a `related` link is clearer.

## Concept References

Use concept references on non-ontology, non-semantic-contract elements when prose should bind readable text to a native concept element:

```markdown
#### Concept References
  * [Verification evidence](../Thesaurus/Thesaurus.md#verification-evidence)
```

The Markdown target must resolve to a native `concept` element. Reqvire derives the generated `skos:Concept` IRI from that target for RDF export, Explorer store data, collect context, and MCP output. Do not author concept-reference IRIs or CURIEs directly; run `reqvire migrate --fix` to rewrite legacy `Label: IRI` entries when the IRI resolves to one generated native concept.

## Quality Checks

- Every concept scheme needs unique `concept_base` and `concept_prefix` metadata; duplicate generated namespaces are invalid.
- Every concept needs a nearest concept-scheme context.
- Every concept should have a clear main-body definition before `#### Metadata` or other reserved subsections.
- Avoid duplicating top-concept lists; Reqvire derives top concepts. Keep top concepts meaningful instead of using a generic catch-all parent.
- Do not put `#### Details`, ontology, or SHACL blocks on concept elements.
- Do not use language metadata until Reqvire has a language policy.
