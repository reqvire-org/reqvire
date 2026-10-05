# Elements

### Semantic Query Authoring Specification

Contract for semantic query authoring.

#### Details
A `semantic-query` element MUST contain exactly one `#### Query` subsection with exactly one nonempty fenced `sparql` block. The heading supplies its name; introductory prose supplies optional purpose. The structured source MUST retain the query document, Produces entries, source locations, and diagnostics. Generic prose storage alone is insufficient.

The generated IRI MUST be `urn:reqvire:semantic-query:<element-id>` and follow the existing element identity lifecycle. Query form MUST be derived by the SPARQL parser as SELECT, ASK, CONSTRUCT, or DESCRIBE. Metadata MUST declare `type: semantic-query`; query form and IRI MUST be generated.

An optional single `#### Produces` subsection MUST accept repeatable `property: CURIE-or-<absoluteIRI>` and `family: CURIE-or-<absoluteIRI>` list entries for CONSTRUCT queries. A present section MUST contain at least one entry. Properties MUST resolve to RDF properties, including object, datatype, and annotation properties. Families MUST resolve to RelationFamily individuals. Equivalent entries MUST resolve to one exported fact. Output declarations describe intended results and need not enumerate or prove every possible output predicate.

Query content and Produces MUST participate in serialization, collect/read payloads, source hashing, change impact, and supported element mutations. Vocabulary rebases MUST preserve executable SPARQL prefix syntax, literals, and comments while rewriting dependent vocabulary references and Produces entries. Rejected mutations MUST follow the existing atomic model validation contract.

#### Metadata
  * type: specification

#### Relations
  * define: [Semantic Query Authoring](SemanticModelRequirements.md#semantic-query-authoring)
---

### Semantic Query Context Validation Specification

Contract for semantic query context validation.

#### Details
Each semantic-query MUST use one or more ontology elements through `use` or inverse `usedBy`. Context MUST include those elements, their ontology ancestors, applicable local external sources, and built-in sources. Context resolution MUST use the existing ontology inheritance/import rules. Selecting a parent MUST retain the existing ancestor-context semantics.

The query MUST be parsed using the SPARQL parser without an implicit workspace base or injected prefixes. Prefixed names MUST have explicit PREFIX declarations. Relative IRIs MUST have an explicit absolute BASE. SPARQL Update MUST be rejected.

Validation MUST traverse parsed graph patterns, subqueries, property paths, CONSTRUCT templates, expressions, inline values, and literal datatypes. Explicit class terms (including objects of rdf:type), predicates, and literal datatypes MUST resolve with the correct role in the used context, including existing reserved language vocabulary. Data-instance, GRAPH, dataset, and DESCRIBE-target IRIs MUST remain data identifiers. Variable schema positions MUST be accepted. A term used in a schema role MUST be checked even when it also appears in a data role.

Syntax-valid SERVICE, SERVICE SILENT, FROM, FROM NAMED, and custom function calls MUST be preserved. Validation and artifact operations MUST NOT execute query content or require endpoints, datasets, or functions to be available.

Produces CURIEs MUST resolve against the used context and its applicable prefix registry. Unknown or ambiguous prefixes, wrong-role terms, undeclared families, and out-of-context declarations MUST fail validation. Produces MUST NOT inject prefixes into SPARQL.

Resolved query schema terms and Produces terms MUST participate in semantic term-reference projection and used-external-vocabulary selection.

Diagnostics MUST identify the source element, file, location, and violated rule. Validate-all MUST retain malformed native candidates and report their diagnostics alongside other query candidates. The ordinary model validation gate MUST continue to govern every persisted mutation and artifact export.

#### Metadata
  * type: specification

#### Relations
  * define: [Semantic Query Context Validation](SemanticModelRequirements.md#semantic-query-context-validation)
---

### Semantic Query Discovery Specification

Contract for semantic query discovery.

#### Details
The managed query set MUST consist of native authored semantic-query elements in the effective workspace. Discovery MUST use native provenance in the existing semantic index. RDF-only query descriptions from imported or built-in sources MUST remain semantic data. A same-name imported description MUST NOT make native selection ambiguous.

Records MUST expose generated IRI, name, optional purpose, parsed form, resolved Produces sets, ontology context, and source locations, sorted by IRI then name. Name and IRI selectors MUST be exact. A supplied selector MUST resolve exactly one record; unknown and ambiguous selectors MUST fail.

Namespace filtering MUST match namespaces of used ontologies, including inherited context, with the existing normalization between document bases and term namespaces. Matching any used namespace MUST include a query once. Namespace selection MUST be shared by CLI, MCP, and RDF query export.

The `queries` RDF layer MUST expose query resources using reqvire:SemanticQuery, queryName, queryText, queryPurpose, queryForm, queryMaterializesProperty, and queryMaterializesFamily. Query records MUST include queryElement source linkage, used ontology dependencies through use, and filePath/lineNumber provenance. Omitted or empty public layer selections MUST include queries. Namespace-filtered query exports MUST retain the complete selected record even though its generated subject is a URN. Ontology exports MUST contain the generic vocabulary; query resources belong to the queries layer.

CLI and MCP MUST reuse the shared resolver, validator, renderer, and index. Query content inclusion MUST expose the exact artifact text and digest. Query operations MUST preserve the existing model validation gate.

#### Metadata
  * type: specification

#### Relations
  * define: [Semantic Query Discovery](SemanticModelRequirements.md#semantic-query-discovery)
---

### Semantic Query Artifact Export Specification

Contract for semantic query artifact export.

#### Details
Rendering MUST emit UTF-8, remove leading and trailing blank lines outside the query document, and end with exactly one LF. It MUST preserve internal characters, comments, prefix order, whitespace, and CR/LF sequences inside multiline literals. Raw artifacts MUST contain only SPARQL. SHA-256 MUST cover the exact emitted bytes.

Export MUST require exactly one exact name or IRI selector. With neither output nor JSON requested, it MUST emit raw SPARQL to stdout. JSON output MUST contain metadata, content, and sha256. File output MUST use an atomic sibling temporary file and rename after successful validation, with only a short stderr diagnostic. JSON and file output MUST be mutually exclusive. A failed selection, validation, or write MUST preserve the existing artifact.

Metadata-only edits, including Produces changes, MUST change the appropriate model revision and impact facts while preserving artifact content and digest when query text is unchanged.

#### Metadata
  * type: specification

#### Relations
  * define: [Semantic Query Artifact Export](SemanticModelRequirements.md#semantic-query-artifact-export)
---

### Semantic Query Artifact Drift Check Specification

Contract for semantic query artifact drift check.

#### Details
Check MUST render expected bytes in memory and compare the complete artifact without modifying it. Exact equality MUST report matching and succeed. Stale content MUST report stale, expected SHA-256, actual SHA-256, and a failure exit status. A missing artifact MUST report missing, expected SHA-256, an absent actual digest, and failure. Other read failures MUST be reported as I/O errors. Selection and query validation MUST use the shared artifact contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [Semantic Query Artifact Drift Check](SemanticModelRequirements.md#semantic-query-artifact-drift-check)
---
