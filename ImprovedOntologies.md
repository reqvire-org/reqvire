# Improved Ontologies Plan

## Goal

Make Reqvire ontology authoring less repetitive while preserving explicit,
portable OWL/Turtle semantics.

Reqvire currently validates each `#### Ontology` Turtle block independently.
That makes every ontology element repeat shared prefix declarations such as:

```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
```

The intended model is that the root ontology element owns the ontology document
context. Descendant ontology elements should inherit that context instead of
redeclaring the same prefixes in every block.

## Root Ontology Element Convention

The ontology root element is the Reqvire element at the top of the connected
ontology hierarchy. Today Reqvire already requires ontology elements to belong
to one connected ontology graph through `derive` / `derivedFrom` relations.

The root ontology element should:

- Declare shared Turtle `@prefix` and `@base` declarations.
- Declare root-level `owl:imports` statements when the ontology depends on
  external ontology documents.
- Contain ordinary ontology terms that belong to the root vocabulary.
- Provide prose that can become the exported ontology document comment.

Example root element:

````markdown
# Element

### Reqvire Core Element Ontology

Reqvire's root ontology document context. Shared prefixes declared here are
inherited by descendant ontology elements.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:Element a owl:Class ;
  rdfs:label "Element" ;
  rdfs:comment "A Reqvire model element." .

reqvire:Capability a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:label "Capability" .
```
````

## Generated `owl:Ontology`

Reqvire should generate one `owl:Ontology` subject for the root ontology element
during ontology export.

The generated subject should use:

- IRI: `urn:reqvire:ontology:<root-element-id>`
- `rdf:type`: `owl:Ontology`
- `rdfs:label`: root element name
- `rdfs:comment`: root element prose, excluding reserved subsections
- `owl:imports`: root-authored ontology import targets

Example generated Turtle:

```turtle
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

<urn:reqvire:ontology:reqvire-core-element-ontology>
  a owl:Ontology ;
  rdfs:label "Reqvire Core Element Ontology" ;
  rdfs:comment "Reqvire's root ontology document context. Shared prefixes declared here are inherited by descendant ontology elements." .
```

If the root block authors `owl:imports`, export should lift those imports onto
the generated `owl:Ontology` subject:

```turtle
<urn:reqvire:ontology:reqvire-core-element-ontology>
  a owl:Ontology ;
  rdfs:label "Reqvire Core Element Ontology" ;
  rdfs:comment "Reqvire's root ontology document context." ;
  owl:imports <https://example.org/external-domain.ttl> .
```

The generated `owl:Ontology` subject is ontology document metadata. It should
not replace normal authored class, property, restriction, or individual triples.

## Prefix Inheritance

Reqvire should extract only root `@prefix` and `@base` declarations into a root
Turtle prelude.

When parsing descendant ontology elements, Reqvire should parse:

```text
root_prefix_and_base_prelude + descendant_authored_turtle
```

Reqvire must not parse descendants as:

```text
root_full_turtle_block + descendant_authored_turtle
```

Using the full root block would duplicate root classes and properties into every
descendant block, corrupting ownership, provenance, diagnostics, and declaration
checks.

Descendant ontology elements must not declare `@prefix`, `@base`, or
`owl:imports`. They are term modules inside the same ontology document and must
consume the root ontology context.

## Imports

`@prefix` declarations and `owl:imports` are related authoring concerns but have
different RDF meanings.

- `@prefix` is Turtle parser syntax. It defines CURIE expansion inside Turtle.
- `owl:imports` is ontology metadata. It declares that one ontology document
  depends on another ontology document.

Reqvire should treat root-authored `owl:imports` as real ontology dependencies,
not only exported metadata. All ontology dependencies used for validation,
semantic contracts, semantic queries, and ontology projection must be checked in
locally and imported from the root ontology element.

Reqvire should resolve import IRIs using the same path concepts as Reqvire
identifier targets, with imports restricted to local workspace files:

- External URL schemes are forbidden for `owl:imports`.
- Same-file fragments are not valid ontology import targets.
- Relative local paths resolve from the root ontology element file's parent
  directory, matching authored Markdown relation and attachment links.
- Leading `/` local paths are treated as git-root-relative paths, matching
  Reqvire's internal path normalization behavior.
- Resolved local imports are normalized to git-root-relative paths for storage,
  diagnostics, resource reporting, and deterministic export metadata.

Reqvire must load imported local ontology documents into the semantic index as
external dependency graphs. Imported terms are referenceable and queryable, but
they are not owned by Reqvire ontology elements.

Terms used by Reqvire validation and semantic contracts must be declared either
in reachable Reqvire-authored ontology blocks or in locally imported ontology
dependency files. Reqvire should not rely on hard-coded built-in understanding of
OWL, RDFS, RDF, XSD, SHACL, Schema.org, PROV, SKOS, or other external
vocabularies for declaration validation.

Parser syntax remains parser syntax. For example, Turtle shorthand `a` still
parses as `rdf:type`, but validation of `rdf:type`, `owl:Class`, `sh:NodeShape`,
`xsd:string`, and similar vocabulary terms must come from local/imported
ontology dependency graphs rather than from hard-coded semantic assumptions.

Child ontology elements must not author `owl:imports`. Ontology-document
dependencies belong on the root ontology element only.

External vocabularies such as OWL, RDF, RDFS, XSD, SHACL, Schema.org, SKOS, or
PROV may still appear as prefix namespaces, but if Reqvire validates against
their definitions, the corresponding dependency document must be vendored inside
the workspace and imported through root `owl:imports`.

## Semantic Contracts

Semantic contracts currently repeat prefixes too. They should inherit the root
ontology context during validation instead of declaring Turtle/SPARQL prefixes
locally.

Required direction:

- Ontology descendants inherit root ontology prefixes.
- Ontology descendants must not author `@prefix`, `@base`, or `owl:imports`.
- Requirement-owned `semantic-contract` blocks inherit the root ontology prelude
  before SHACL validation.
- Requirement-owned `semantic-query-contract` blocks inherit the root ontology
  prelude before SPARQL validation.
- `semantic-contract` and `semantic-query-contract` blocks must not author
  `@prefix`, `@base`, `PREFIX`, or `BASE` declarations.
- SHACL and SPARQL vocabulary prefixes such as `sh:`, `rdf:`, `rdfs:`, `owl:`,
  and `xsd:` must come from the inherited root ontology context, not from each
  contract block.
- SHACL, SPARQL, RDF, RDFS, OWL, and XSD vocabulary definitions used by
  validation must come from locally imported ontology dependency files, not from
  hard-coded built-in vocabulary knowledge.

This should be implemented carefully so diagnostics still point to authored
contract lines rather than generated prelude lines.

## Export Behavior

Default `reqvire ontologies` and exported `ontologies.ttl` should include:

1. Generated root `owl:Ontology` metadata.
2. Root `owl:imports` metadata for local dependency files.
3. Authored ontology and SHACL blocks.
4. No generated ontology projection facts unless `--full` is requested.

The default export may continue preserving authored blocks as source sections.
A later cleanup can normalize repeated shared prefixes into a single top-level
prefix prelude in the exported Turtle, but that is separate from validation
prefix inheritance.

JSON-LD export should include the same generated root `owl:Ontology` metadata as
Turtle export so both formats describe the same ontology document.

Default export should preserve imported dependency identity. A separate option
may later choose whether to inline imported dependency graphs, emit them as
separate copied files, or include only import references.

## Validation and Lint

Validation should:

- Accept descendant ontology blocks that use root-defined prefixes.
- Reject authored prefix/base declarations inside non-root ontology elements.
- Reject authored `owl:imports` inside non-root ontology elements.
- Accept semantic-contract and semantic-query-contract blocks that use
  root-defined prefixes.
- Reject authored prefix/base declarations inside semantic-contract and
  semantic-query-contract blocks.
- Preserve declaration ownership for only the descendant's authored triples.
- Extract `owl:imports` from the root-authored block only.
- Load local imported ontology dependency files into the semantic index.
- Keep validating duplicate declarations and reachable ontology context against
  Reqvire-authored ontology content plus imported dependency graphs.
- Treat imported dependency declarations as external. They satisfy references but
  do not create Reqvire element ownership.
- Reject validation paths that depend on hard-coded external vocabulary
  declarations instead of local/imported ontology dependency graphs.

Lint should eventually:

- Report descendant ontology elements that still author `@prefix`, `@base`, or
  `owl:imports` during migration tooling.
- Report semantic contracts that still author local prefix/base declarations
  during migration tooling.

## Implementation Notes

Implementation should update the semantic index pipeline rather than performing
string-only export rewrites.

Expected code changes:

- Locate the ontology root in the connected ontology graph.
- Extract root `@prefix` and `@base` declarations into a reusable parse prelude.
- Resolve root `owl:imports` local paths with the same file-relative and
  git-root-relative normalization used by Reqvire identifiers.
- Reject root `owl:imports` targets with external URL schemes or fragments.
- Parse imported local ontology dependency files into external semantic graphs.
- Parse descendant ontology blocks with that prelude.
- Add validation that forbids local prefix/base declarations and `owl:imports`
  in non-root ontology elements.
- Parse semantic-contract and semantic-query-contract blocks with that prelude.
- Add validation that forbids local prefix/base declarations in semantic
  contracts and semantic query contracts.
- Keep stored `SemanticBlock.content` as authored content, not prelude-expanded
  content.
- Ensure quads produced only by the inherited prelude are not treated as
  descendant-authored ontology declarations.
- Extract root-authored `owl:imports` triples.
- Replace hard-coded external vocabulary declaration assumptions with terms
  loaded from local/imported ontology dependency graphs.
- Generate root `owl:Ontology` metadata in default Turtle and JSON-LD export.
- Include the generated metadata before authored blocks in `ontologies.ttl`.
- Update CLI/export specs and focused ontology command tests.

Diagnostics need line-number care. If parsing uses a generated prelude, parser
errors may report shifted lines. The implementation should remap parse errors to
authored block line numbers where practical, or avoid prefix inheritance in error
paths that cannot report useful locations.
