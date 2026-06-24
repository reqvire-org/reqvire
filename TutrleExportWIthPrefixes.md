# Turtle Export With Prefixes Plan

## Goal

Make Reqvire Turtle ontology exports readable by emitting stable `@prefix`
declarations and compact prefixed names where legal, instead of expanding every
IRI as a full `<...>` term.

The change belongs at the RDF serialization boundary. Internal semantic storage,
validation, Oxigraph terms, and graph construction should continue to use full
IRIs.

## Current Problem

Reqvire ontology exports are semantically correct but noisy because Turtle output
uses expanded IRIs for most subjects, predicates, and objects.

This makes exported ontology artifacts harder to inspect in:

- text diffs
- WebOWL and ontology viewers
- generated runtime ontology artifacts
- CLI/MCP ontology export output
- semantic-contract debugging

## Desired Behavior

Turtle export should emit prefixes such as:

```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix concept: <https://www.reqvire.org/concepts#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .
```

Then serialize compact names where valid:

```turtle
reqvire:Capability a owl:Class ;
  rdfs:label "Capability" ;
  reqvire:mapsToConcept concept:Capability .
```

instead of:

```turtle
<https://www.reqvire.org/ontology#Capability> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .
```

Prefix declarations should be emitted once at the top of the final Turtle
artifact. Source comments and grouped block output may remain below the prefix
block, but individual blocks should not require repeated local `@prefix`
declarations in the final export.

## Ontology Document Preservation

Prefixes are Turtle syntax. They are not ontology document declarations and must
not replace `owl:Ontology` triples.

The export must preserve ontology document facts as RDF triples:

```turtle
<https://example.test/ontology> a owl:Ontology ;
  owl:imports <https://example.test/shared> ;
  rdfs:label "Main ontology" .

<https://example.test/shared> a owl:Ontology ;
  rdfs:label "Shared ontology" .
```

Multiple `owl:Ontology` subjects can appear in one exported RDF graph. They
remain separate ontology document resources because their subject IRIs are
distinct. A top-level prefix block does not collapse those documents.

The exporter may deduplicate exact duplicate RDF triples, including duplicate
`owl:imports` statements, because RDF graph semantics do not preserve repeated
identical statements.

## Scope

In scope:

- Turtle ontology export.
- Runtime Reqvire ontology artifact generation.
- CLI ontology export output.
- MCP ontology export/read tools if they expose Turtle.
- Full graph export when output format is Turtle.
- Semantic-contract Turtle serialization if it shares the same serializer.

Out of scope:

- Changing internal RDF graph identity.
- Changing validation semantics.
- Post-processing arbitrary Turtle strings after serialization.
- Making JSON-LD use Turtle prefixes.

## JSON-LD Behavior

JSON-LD must use `@context`, not Turtle `@prefix` declarations.

The same prefix collection policy can feed JSON-LD context generation later, but
Turtle prefix output and JSON-LD context output should remain separate formatter
concerns.

## Prefix Sources

The exporter should build one deterministic prefix map before serializing.

Prefix sources:

- Reqvire built-ins:
  - `reqvire`
  - `rdf`
  - `rdfs`
  - `owl`
  - `xsd`
  - `sh`
  - `skos`
- Authored ontology namespace metadata.
- Native concept scheme namespace metadata:
  - `concept_prefix`
  - `concept_base`
- Built-in external ontology source prefixes.
- Local external ontology source prefixes only when the exported graph uses terms
  from that source.

## Collision Policy

Prefix output must be deterministic and validation-friendly.

Rules:

- Built-in prefixes are reserved and must not be redefined by authored metadata.
- A prefix mapped to multiple namespace IRIs is an error.
- A namespace IRI mapped by multiple prefixes should prefer the canonical
  Reqvire-authored prefix.
- Export should not silently invent aliases for collisions.
- Prefix ordering should be stable:
  - built-ins first
  - authored ontology prefixes next
  - concept prefixes next
  - external source prefixes last
  - lexical order inside each group

## Compact IRI Rules

The serializer may compact an IRI only when:

- the IRI starts with a registered namespace base
- the remaining local name is non-empty
- the local name is valid Turtle `PN_LOCAL`
- compaction does not change meaning

Otherwise, the term must remain a full IRI:

```turtle
<https://example.org/terms/not valid local> rdfs:label "..." .
```

## Implementation Plan

1. Locate the Turtle serialization boundary.

Likely candidates:

- `serialize_triples_turtle`
- ontology export command serialization
- runtime ontology artifact generation
- semantic contract export serialization

2. Introduce a prefix map type.

Example shape:

```rust
struct TurtlePrefixMap {
    entries: BTreeMap<String, String>,
}
```

3. Collect prefixes from registry/export context.

The prefix collector should not inspect raw Turtle text. It should use parsed
metadata and known built-in namespace declarations.

4. Configure or replace Turtle serialization.

Preferred:

- use the RDF/Turtle serializer API with namespace bindings if available.

Fallback:

- keep current triple ordering but render named nodes through a safe compact IRI
  function before writing Turtle.

5. Preserve deterministic output.

Stable output matters because many tests compare expected artifacts.

Requirements:

- deterministic prefix order
- deterministic triple order
- deterministic blank node rendering
- no random generated prefix names

6. Add/adjust tests.

Expected affected tests:

- ontology export command fixtures
- runtime ontology artifact guard
- MCP ontology export/read fixtures
- semantic contract export fixtures
- full graph Turtle export fixtures

7. Update documentation.

Document:

- built-in reserved prefixes
- authored namespace prefix collision behavior
- Turtle vs JSON-LD formatting behavior
- ontology/SHACL model coverage for prefix-map records and prefix declaration
  records
- full semantic export projection vocabulary for `PrefixedTurtleExport` records
  linked to `TurtlePrefixMap` records containing `TurtlePrefixDeclaration`
  records

## Risks

- Snapshot churn will be large because many expected Turtle fixtures will change.
- Prefix collision policy must be strict enough to avoid ambiguous output.
- Turtle local-name validation must be correct; invalid prefixed names would make
  exports syntactically invalid.
- JSON-LD output must not accidentally receive Turtle-specific formatting.

## Acceptance Criteria

- Turtle ontology exports include stable `@prefix` declarations.
- Known Reqvire, RDF/RDFS/OWL/XSD/SHACL/SKOS terms serialize compactly.
- Authored ontology and concept scheme terms serialize compactly when their
  prefixes are declared and local names are Turtle-safe.
- Invalid local names remain expanded IRIs.
- JSON-LD export behavior is unchanged unless separately updated through
  `@context`.
- Existing validation behavior is unchanged.
- CLI, MCP, runtime ontology artifact, and relevant e2e fixtures pass after
  expected output updates.
- The semantic export ontology defines first-class prefix-map and prefix
  declaration vocabulary, and the semantic export SHACL shape constrains the
  expected prefix record fields.
- Full semantic export projection facts can represent the selected Turtle
  prefix map without treating prefix syntax as ontology document identity.
