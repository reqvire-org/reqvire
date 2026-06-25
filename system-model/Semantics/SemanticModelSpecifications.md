# Elements

### Built-In External Ontology Source Specification

The built-in external ontology source contract defines Reqvire-shipped RDF vocabulary dependencies that behave like external ontology sources without project-local source declarations.

#### Details
Ownership:
- Reqvire core owns the built-in external source registry because Reqvire core already owns external-source policy, diagnostics, semantic index assembly, `include_external` behavior, MCP payloads, and Explorer visibility.
- `o-kernel` remains generic RDF/OWL/SHACL infrastructure. It consumes parsed RDF quads, builds ontology indexes, parses SHACL, aligns SHACL against supplied ontology graphs, and does not decide which vocabularies Reqvire ships as built-in dependencies.

Built-in source registry:
- Each built-in source entry must define `id`, `prefix`, `namespace`, `resource`, `source`, `format`, and embedded RDF source content.
- Built-in source content must be pinned in the repository and loaded without network access.
- Built-in sources are external ontology sources, not RDF/OWL/SHACL language built-ins.

Initial source:

```text
id: skos
prefix: skos
namespace: http://www.w3.org/2004/02/skos/core#
resource: http://www.w3.org/2004/02/skos/core
source: builtin:skos.rdf
format: rdfxml
```

Behavior:
- Built-in source RDF is parsed through the same RDF parser path as local external ontology sources.
- Built-in source declarations are marked external and built-in.
- Built-in source terms are available for ontology Turtle validation, semantic-contract SHACL alignment, and native concept RDF generation.
- Authored ontology resources typed by a class declared in a built-in source are treated as authored named individuals for structural ontology validation, but they are not valid concept-reference targets unless they are generated from native `concept` elements.
- Authored Turtle must still explicitly declare `@prefix skos: <http://www.w3.org/2004/02/skos/core#> .`; the built-in source does not inject hidden Turtle prefixes.
- `reqvire semantic export --layer ontologies --layer external-used`, `reqvire semantic export --layer external-used`, and MCP `reqvire.semantic.export` with the `external-used` layer materialize only the used subset of built-in external source triples, following the same used-subset policy as local external ontology sources. MCP helper-tool `include_external: true` provides query-time visibility into the same used subset.
- Invalid SKOS terms that are not present in the pinned RDF source do not become valid merely because they share the SKOS namespace.

Conceptual/structural layer separation:
- Native `concept-scheme` and `concept` elements generate the SKOS conceptual layer: `skos:Concept` resources, labels, definitions, broader/narrower hierarchy, and related-concept links.
- Project-owned structural ontology terms may reference those generated conceptual anchors with authored properties instead of mixing conceptual labels and structural class/property commitments in one namespace.

#### Metadata
  * type: specification

#### Relations
  * define: [Built-In External Ontology Source Resolution](SemanticModelRequirements.md#built-in-external-ontology-source-resolution)
---

### External Vocabulary Description Construction Specification

The external vocabulary description construction contract defines how Reqvire applies o-kernel referenced graph subset construction to used external vocabulary.

#### Details
Reqvire adapter behavior:
- Pass ontology graphs of interest and internal raw external dependency graphs to the o-kernel referenced graph subset construction service.
- Treat reference extraction, support context, annotation context, RDF list closure, and bounded expansion as o-kernel standard external ontology dependency subset profile behavior.
- Preserve external source metadata and external declaration markers on constructed terms.
- Keep constructed subset triples out of authored Markdown ontology, semantic-contract, requirement, and contract blocks.
- Forward constructed used external vocabulary content to reporting exposure policy; this requirement does not define public visibility by itself.

#### Concept References
  * [External ontology subset construct query](../Thesaurus/Thesaurus.md#external-ontology-subset-construct-query)
  * [Raw external ontology graph](../Thesaurus/Thesaurus.md#raw-external-ontology-graph)

#### Metadata
  * type: specification

#### Relations
  * define: [External Vocabulary Description Construction](SemanticModelRequirements.md#external-vocabulary-description-construction)
---

### External Vocabulary Reference Resolution Specification

The external vocabulary reference resolution contract defines how Reqvire makes imported terms available without promoting them into authored vocabulary.

#### Details
Resolution rules:
- Imported terms are available for semantic-reference validation through the ontology element that declared the external source.
- Imported terms are inherited by ontology descendants of the declaring ontology element.
- Imported terms are visible to semantic contracts that use the declaring ontology context or an ontology descendant context.
- Built-in external ontology source terms are visible to ontology elements and semantic contracts without requiring a local `#### External Ontology` declaration. Non-ontology concept references resolve to generated native `concept` resources typed as `skos:Concept`.
- Imported terms remain marked as external declarations in semantic metadata.
- Imported terms do not count as authored project-owned term declarations for duplicate authored-term validation.
- Standard reserved RDF, RDFS, OWL, XSD, and SHACL vocabulary remains handled through the reserved vocabulary registry rather than local external source declarations.

#### Metadata
  * type: specification

#### Relations
  * define: [External Vocabulary Reference Resolution](SemanticModelRequirements.md#external-vocabulary-reference-resolution)
---

### Local External Ontology Source Specification

The local external ontology source contract defines how ontology elements declare local external vocabulary graphs.

#### Details
An ontology element may define one or more repeatable `#### External Ontology` sections:

Example section body, under an `External Ontology` subsection:

```markdown
  * prefix: ext
  * namespace: https://example.org/external#
  * resource: https://example.org/external
  * source: references/ontologies/external.ttl
  * format: turtle
```

Rules:
- Only ontology elements may define `#### External Ontology`.
- The section is repeatable.
- Each section requires `prefix`, `namespace`, `resource`, and `source`.
- `format` is optional and defaults to Turtle. Supported source formats are `turtle`, `ttl`, `rdf`, `rdfxml`, `rdf+xml`, and `jsonld`.
- Markdown subsection grammar, bullet parsing, required-field extraction, defaults, and source line numbers are parser-owned. Semantic model construction consumes parsed external-source records and must not duplicate markdown list parsing for this section.
- `source` must be a local path. `http://` and `https://` source paths are rejected; network ontology fetches are not part of validation or export.
- Source paths are resolved as model paths using the repository root, with file-relative resolution as a fallback for local fixture and authoring ergonomics.
- Turtle/TTL sources must explicitly declare the configured prefix/namespace pair.
- RDF/XML sources must use RDF/XML syntax; `format: rdf` is treated as RDF/XML for local `.rdf` ontology files.
- JSON-LD sources must define equivalent local context mappings or expanded IRIs so the parsed RDF graph mentions the configured namespace.
- The parsed source graph must contain `<resource> a owl:Ontology`.
- The parsed source graph must declare or reference at least one term in the configured namespace.
- The same prefix may not be bound to different namespaces.
- The same namespace may not be bound to different prefixes; aliases are rejected in this version.
- Full external ontology files are internal dependency inputs for validation and term resolution.

External sections are source declarations, not hidden Turtle injection. Authored ontology and SHACL blocks must still be complete Turtle with their own explicit prefixes and semantic statements.

#### Metadata
  * type: specification

#### Relations
  * define: [Local External Ontology Sources](SemanticModelRequirements.md#local-external-ontology-sources)
---

### Namespace-Scoped Ontology Export Specification

The namespace-scoped ontology export contract defines the clean semantic export filter used by runtime artifact generation and other namespace-specific ontology publishing workflows.

#### Details
Filter behavior:
- `reqvire semantic export --layer ontologies` and `reqvire semantic export` shall accept `--namespace-base <IRI>`.
- The value may be an ontology document base IRI, such as `https://www.reqvire.org/ontology`, or a term namespace IRI, such as `https://www.reqvire.org/ontology#`.
- Reqvire shall normalize the value to the term namespace before filtering.
- Generated ontology document declarations shall be limited to matching ontology documents.
- Generated `rdfs:isDefinedBy` term definition links shall be limited to authored terms in the matching term namespace.
- Authored `#### Ontology` blocks shall be included when they belong to a matching ontology document or declare a named subject in the matching term namespace.
- Authored `#### Shapes` blocks shall be included when they declare a shape subject in the matching term namespace.
- The filter shall deduplicate the resulting Turtle/JSON-LD output through the same serializer path used by unfiltered clean exports.
- Filtered Turtle output shall preserve the same deterministic prefix declaration and safe IRI compaction behavior as unfiltered prefixed Turtle semantic export.
- The filter shall reject combination with the `model` layer until a separate model/projection filter contract exists.

The filter is an export boundary only. It does not mutate authored ontology source, does not change validation scope, and does not turn unrelated ontology namespaces into runtime vocabulary.

#### Metadata
  * type: specification

#### Relations
  * define: [Namespace-Scoped Ontology Export](SemanticModelRequirements.md#namespace-scoped-ontology-export)
---

### OWL Reserved Vocabulary Recognition Specification

The OWL reserved vocabulary recognition contract defines how Reqvire consumes the o-kernel reserved vocabulary registry.

#### Details
Reqvire must use the o-kernel standards reserved vocabulary registry for RDF, RDFS, XSD, OWL, and SHACL IRIs with special treatment. RDF, RDFS, OWL, and SHACL term recognition is graph-backed by compile-time bundled standards vocabulary files; XSD datatype and facet recognition remains explicit kernel datatype policy.

Reqvire must not require local `#### External Ontology` source declarations for standard reserved prefixes or reserved vocabulary IRIs recognized by the kernel registry.

Semantic-contract shape reference validation must skip standard reserved vocabulary IRIs recognized by the kernel registry before enforcing authored/external ontology reachability. This includes RDF properties used as SHACL paths, such as `rdf:type`, and preserves normal reachability checks for Reqvire-owned and project-owned classes or properties.

A non-reserved IRI must resolve through authored ontology terms, local external ontology sources, or built-in external ontology sources when term existence validation applies.

OWL reserved vocabulary recognition is a semantic validation rule. It does not make standard namespaces authored ontology namespaces, does not synthesize external source triples, and does not require standard reserved prefixes to appear as External Sources in Explorer.

Core SHACL vocabulary used to parse and validate `#### Shapes` blocks is handled through the o-kernel SHACL services and Reqvire semantic-contract adapter. It is not modeled as a local or built-in external ontology source. External ontology sources are reserved for additional RDF vocabularies outside the Reqvire authored model, such as SKOS or project-specific exported vocabularies.

#### Metadata
  * type: specification

#### Relations
  * define: [OWL Reserved Vocabulary Recognition](SemanticModelRequirements.md#owl-reserved-vocabulary-recognition)
---

### Ontology Term Definition Link Materialization Specification

#### Details
Runtime materialization:
- Build generated ontology document declarations from resolved ontology metadata before serializing authored ontology and SHACL blocks.
- For each authored named subject collected from a `#### Ontology` block whose IRI is inside the generated ontology document term namespace, emit one generated triple: `<term> rdfs:isDefinedBy <ontology-document-iri>`.
- Resolve `<ontology-document-iri>` from the same `ontology_base` used for the generated `owl:Ontology` declaration.
- Exclude the ontology document IRI itself from generated term definition links.
- Include generated definition links in default Turtle output, JSON-LD output, default semantic export, MCP semantic ontology output, and any semantic store built from the same semantic index.
- Declare the prefixes used by the generated definition-link section itself, at minimum `rdfs:` and `owl:` when those terms appear in that serialized section.
- Expose the resolved ontology document as metadata that MCP vocabulary and Explorer graph projections can use for grouping, exact filtering, and modal evidence.
- Do not generate definition links for imported external ontology terms or named subjects outside the owning document term namespace.
- Do not mutate authored Markdown ontology blocks.

Deduplication and validation:
- If an authored ontology block already includes the matching `rdfs:isDefinedBy` triple for an authored named ontology resource, semantic output must emit the fact once.
- If an authored ontology block gives an authored named ontology resource an explicit `rdfs:isDefinedBy` object that differs from the generated ontology document IRI, semantic validation must fail.
- `rdfs:isDefinedBy` statements for non-authored external terms are outside this generation rule and, when present, must originate from external source parsing rather than Reqvire ownership materialization.
- Explorer ontology graph projection must not render `rdfs:isDefinedBy` as a canvas edge or render generated `owl:Ontology` document IRIs as primary graph nodes.

#### Metadata
  * type: specification

#### Relations
  * define: [Ontology Term Definition Link Materialization](SemanticModelRequirements.md#ontology-term-definition-link-materialization)
---

### Ontology and Shapes Collection Specification

The ontology and shapes collection contract defines the reusable semantic context that Reqvire core builds from authored ontology and semantic-contract elements.

#### Details
Semantic context construction:
- Collect parser-extracted ontology `#### Ontology` and semantic-contract `#### Shapes` fenced RDF blocks from the graph registry.
- Consume parser-extracted local `#### External Ontology` source records when adding external dependency graphs.
- Use the reusable semantic index built for ontology and semantic-contract validation so RDF parsing is performed once per block.
- Preserve source element identifier, source name, file path, section kind, and line number in the semantic index.
- Keep authored Markdown as the source of truth; semantic context construction must not mutate authored ontology or semantic-contract blocks.
- Semantic model construction must not rescan element Markdown to parse reserved subsection grammar; only the model parser owns that grammar.
- Include authored ontology RDF, semantic-contract SHACL RDF, generated ontology document declarations, generated ontology term definition links, and available source/provenance metadata.
- When full semantic context is requested by a consumer, include model triples, relation-family projection facts, concept-reference facts, ontology term declaration facts, semantic-contract shape-reference facts, and generated ontology projection facts.
- In full semantic context, emit concrete parsed Reqvire elements as `owl:NamedIndividual` instances of `reqvire:Element` and their more specific element class, such as `reqvire:Requirement`, `reqvire:Specification`, `reqvire:SemanticContract`, or concrete verification classes. Emit referenced files, evidence, and implementation targets as `owl:NamedIndividual` instances of `reqvire:Artifact` and any more specific artifact class such as `reqvire:File`. Do not add these model ABox individuals to clean authored ontology-only exports.

Reqvire core uses o-kernel contracts for RDF-native parsing/classification and SHACL services, but the Reqvire semantic model remains the owner of graph-registry source mapping, element provenance, semantic-contract reachability, and product-specific projection policy.

#### Metadata
  * type: specification

#### Relations
  * define: [Ontology and Shapes Collection](SemanticModelRequirements.md#ontology-and-shapes-collection)
---

### Prefixed Turtle Semantic Export Specification

The prefixed Turtle export contract defines how Reqvire serializes semantic RDF graphs as readable Turtle without changing graph identity.

#### Details
Serialization boundary:
- Prefix compaction shall be applied at RDF serialization time after semantic blocks, generated ontology document declarations, generated definition links, optional model triples, generated projection facts, and optional used external subset triples have been collected as RDF terms.
- The exporter shall not compact IRIs by string-rewriting already serialized Turtle.
- Internal semantic stores, validation, graph construction, and Oxigraph loading shall continue to use full RDF IRIs.
- The Turtle output shall remain parseable as ordinary RDF/Turtle and equivalent to the unprefixed RDF graph.
- Final Turtle artifacts shall emit one deterministic top-level prefix declaration block before serialized graph triples and source comments. Prefix declarations are syntax bindings only; they must not be interpreted as ontology document membership or as a replacement for `owl:Ontology` document triples.

Prefix map construction:
- The `reqvire` prefix shall always be available for generated Reqvire export/runtime vocabulary. In the Reqvire repository it is sourced from the authored Reqvire ontology element when that ontology declares the canonical namespace; in downstream repositories it is a built-in export namespace.
- Built-in standards prefixes shall include at least `rdf`, `rdfs`, `owl`, `xsd`, `sh`, and `skos` when those namespaces appear in the serialized graph.
- Authored ontology prefixes shall come from parsed ontology metadata such as `ontology_prefix` and `ontology_base`.
- Native concept-scheme prefixes shall come from parsed concept-scheme metadata such as `concept_prefix` and `concept_base`.
- Local and built-in external ontology source prefixes shall be included only when that external vocabulary is included in the selected output surface.
- Prefix ordering shall be deterministic: `reqvire` first, standards vocabulary prefixes next, authored ontology prefixes next, concept prefixes next, external prefixes last, with lexical ordering inside each group unless a group defines a canonical vocabulary order.
- Full semantic export projection facts shall model prefixed Turtle output as a `PrefixedTurtleExport` linked to a `TurtlePrefixMap`, and the map shall contain one `TurtlePrefixDeclaration` per emitted prefix binding.

Collision policy:
- The canonical `reqvire` prefix and built-in standards prefixes are reserved and must not be redefined by authored ontology, concept-scheme, or external-source metadata. An authored Reqvire ontology declaration may own the canonical `reqvire` prefix only when it binds to the canonical Reqvire namespace.
- The same prefix token bound to multiple namespace IRIs shall be a validation or export error.
- The same namespace bound to multiple prefix tokens shall prefer the canonical authored Reqvire prefix when one exists; otherwise aliases shall be rejected unless a later requirement defines alias export.
- The serializer shall not silently invent random prefix aliases to work around collisions.

Compaction policy:
- The serializer may compact a named IRI only when the IRI starts with a registered namespace base and the suffix is a valid Turtle local name.
- Named IRIs with invalid or empty local names shall remain serialized as full `<IRI>` terms.
- Multiple `owl:Ontology` document subjects may appear in one export and shall be preserved as RDF graph facts.
- `owl:Ontology`, `owl:imports`, generated ontology document declarations, and generated `rdfs:isDefinedBy` facts shall be preserved as RDF triples. Exact duplicate RDF triples may be emitted once.
- Authored ontology document metadata such as `rdfs:label`, `owl:imports`, version annotations, and comments shall remain attached to the ontology document IRI subject in the exported graph.

Format separation:
- Turtle output shall use `@prefix` declarations.
- JSON-LD output shall remain a separate RDF serialization mode and shall use JSON-LD context handling rather than Turtle `@prefix` declarations.

#### Metadata
  * type: specification

#### Relations
  * define: [Prefixed Turtle Semantic Export](SemanticModelRequirements.md#prefixed-turtle-semantic-export)
---

### Runtime Reqvire Ontology Artifact Specification

The runtime Reqvire ontology artifact contract defines the generated ontology-vocabulary Turtle snapshot embedded by Reqvire core.

#### Details
Artifact contract:
- The artifact path is `crates/reqvire-core/src/runtime_ontology/reqvire.ttl`.
- The Rust access point is `crates/reqvire-core/src/runtime_ontology.rs`.
- The embedded constant `REQVIRE_ONTOLOGY_TTL` exposes the generated ontology Turtle content without requiring runtime access to `system-model/Ontologies`.
- The artifact is generated from the authored Reqvire model with `reqvire semantic export --layer ontologies --layer shapes --namespace-base https://www.reqvire.org/ontology#`.
- The artifact must include generated ontology document declarations, generated term definition links, and authored runtime Reqvire ontology RDF whose declared subjects are in the runtime Reqvire term namespace.
- Intermediate generated Turtle sections may be self-contained for the prefixes they use before final artifact assembly.
- The final runtime Turtle artifact shall follow the shared prefixed Turtle export contract with one deterministic top-level prefix declaration block. The committed artifact must not depend on repeated in-section prefix declarations.
- The artifact must not include model-layer triples, generated ontology projection facts, raw external source dumps, or used external subset triples unless a future requirement changes the runtime bootstrap contract.
- The artifact must not include semantic-contract SHACL shape blocks or ontology blocks whose declared subjects are outside the runtime Reqvire term namespace.
- If the authored runtime ontology maps structural terms to standalone native concepts, the checked-in runtime artifact must be curated after namespace export so runtime bootstrap excludes concept-scheme `owl:imports`, the `reqvire:mapsToConcept` property declaration, generated definition links for `reqvire:mapsToConcept`, and authored `reqvire:mapsToConcept` bridge usage triples. Concept bridges are concept-layer evidence and are not runtime bootstrap facts.
- The artifact is derived implementation, not authored ontology source. Changes to vocabulary terms shall be made in `system-model/Ontologies` first, then propagated by regenerating the artifact.

#### Metadata
  * type: specification

#### Relations
  * define: [Runtime Reqvire Ontology Artifact](SemanticModelRequirements.md#runtime-reqvire-ontology-artifact)
---

### Runtime Reqvire Ontology Synchronization Specification

The synchronization contract defines how Reqvire prevents embedded runtime semantic artifacts from drifting from the authored ontology model.

#### Details
Synchronization behavior:
- A dedicated e2e test shall regenerate the runtime namespace-scoped ontology export from the current workspace using `reqvire semantic export --layer ontologies --layer shapes --namespace-base https://www.reqvire.org/ontology# --output <temporary-file>`.
- If runtime-artifact curation rules are active, the test shall apply those rules to the regenerated export before comparison.
- The test shall split regenerated output into `reqvire.ttl` ontology vocabulary and `reqvire-shacl.ttl` SHACL rules using semantic block kind metadata.
- The regenerated, curated, and split temporary outputs shall be compared with `crates/reqvire-core/src/runtime_ontology/reqvire.ttl` and `crates/reqvire-core/src/runtime_ontology/reqvire-shacl.ttl` after deterministic blank-node label normalization.
- Blank-node labels are serializer-local and do not represent semantic drift; all non-blank-node textual differences remain test failures.
- A mismatch shall fail the test suite with guidance to regenerate the artifact.
- The comparison shall run against the real repository root, not a copied e2e fixture workspace.
- The comparison is intentionally strict so authored ontology and semantic-contract changes propagate to the runtime artifact through normal change impact and test failure.

Regeneration command:

```bash
cargo run -q -p reqvire-cli -- semantic export --layer ontologies --layer shapes --namespace-base https://www.reqvire.org/ontology# --output /tmp/reqvire-runtime-semantic-export.ttl
```

If the authored model includes concept-layer imports or bridge usages that are not runtime bootstrap facts, maintainers must apply the documented runtime curation before committing `reqvire.ttl` and `reqvire-shacl.ttl`. The curation and split are part of the runtime artifact contract, not an ad hoc edit.

#### Metadata
  * type: specification

#### Relations
  * define: [Runtime Reqvire Ontology Synchronization](SemanticModelRequirements.md#runtime-reqvire-ontology-synchronization)
---

### Runtime Reqvire SHACL Artifact Specification

The runtime Reqvire SHACL artifact contract defines the generated shape-rule Turtle snapshot embedded by Reqvire core.

#### Details
Artifact contract:
- The artifact path is `crates/reqvire-core/src/runtime_ontology/reqvire-shacl.ttl`.
- The Rust access point is `crates/reqvire-core/src/runtime_ontology.rs`.
- The embedded constant `REQVIRE_SHACL_TTL` exposes generated SHACL Turtle content without requiring runtime access to `system-model/Ontologies`.
- The artifact is generated from the authored Reqvire model with `reqvire semantic export --layer ontologies --layer shapes --namespace-base https://www.reqvire.org/ontology#`, then split from the ontology artifact using semantic block kind metadata.
- The artifact must include authored runtime semantic-contract SHACL RDF whose declared subjects are in the runtime Reqvire term namespace.
- The artifact must not include generated ontology document declarations, generated term definition links, authored ontology vocabulary blocks, model-layer triples, generated ontology projection facts, raw external source dumps, or used external subset triples unless a future requirement changes the runtime bootstrap contract.
- Runtime code may load `REQVIRE_ONTOLOGY_TTL` and `REQVIRE_SHACL_TTL` into one RDF store when it needs vocabulary plus validation rules, but the checked-in compile-time source files remain separate.

#### Metadata
  * type: specification

#### Relations
  * define: [Runtime Reqvire SHACL Artifact](SemanticModelRequirements.md#runtime-reqvire-shacl-artifact)
---

### Used External Vocabulary Selection Specification

The used external vocabulary selection contract defines how Reqvire identifies external terms referenced by model-owned semantic content.

#### Details
Selection boundary:
- Raw external ontology graphs are internal dependency inputs for validation and term resolution.
- The used term set is derived from declared external namespaces and references found in authored semantic content, model context, and generated semantic projection facts.
- Selection identifies the used external term set only; description construction and public exposure policy are separate contracts.

Seed query:
```sparql
PREFIX reqvire: <https://www.reqvire.org/ontology#>

SELECT DISTINCT ?term
WHERE {
  ?source a reqvire:ExternalOntologySource ;
    reqvire:externalOntologyNamespace ?namespace .

  {
    ?block reqvire:referencesTerm ?term .
  }
  UNION {
    ?block reqvire:declaresTerm ?term .
  }
  UNION {
    ?projection reqvire:conceptReference ?term .
  }
  UNION {
    ?projection reqvire:constructSubject|reqvire:constructPredicate|reqvire:constructObject|reqvire:constructProperty ?term .
  }

  FILTER(isIRI(?term))
  FILTER(STRSTARTS(STR(?term), STR(?namespace)))
}
```

Implementation contract:
- Current Rust code implements equivalent selection directly until the generic subset construction service exists.
- The seed query is a SELECT query because it identifies the used external term set.

#### Concept References
  * [Used external ontology subset](../Thesaurus/Thesaurus.md#used-external-ontology-subset)

#### Metadata
  * type: specification

#### Relations
  * define: [Used External Vocabulary Selection](SemanticModelRequirements.md#used-external-vocabulary-selection)
---
