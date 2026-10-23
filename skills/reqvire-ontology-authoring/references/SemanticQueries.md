# Managed SPARQL query artifacts

Use `semantic-query` for reusable SPARQL documents. Keep `ontology` vocabulary in Ontology and semantic-contract SHACL in Shapes.

Author one `#### Query` section containing exactly one fenced `sparql` block. The element heading supplies its name; introductory prose supplies optional purpose. Metadata contains `type: semantic-query`. Query form (SELECT, ASK, CONSTRUCT, DESCRIBE) and `urn:reqvire:semantic-query:<element-id>` are generated.

Use one or more `use` relations to ontology elements (or ontology `usedBy`). Context includes the selected ontology and ancestors, applicable external sources, and built-ins. Declare SPARQL PREFIX bindings explicitly; relative IRIs need explicit absolute BASE. Ordinary data, graph, and dataset IRIs are not schema declarations. Explicit classes, predicates, paths, and literal datatypes must resolve with correct roles in context.

CONSTRUCT may include one optional `#### Produces` section with repeatable `property: prefix:Term` and `family: prefix:Family` entries. Values may also be `<absoluteIRI>`. Properties include object, datatype, and annotation properties; families are RelationFamily individuals. Declarations describe output, not an exhaustive static proof. A present section needs at least one entry.

SERVICE/SILENT, FROM/FROM NAMED, and custom functions are portable downstream runtime features. Reqvire validates but does not execute them during artifact operations.

Commands:
- `semantic query list --json [--namespace-base IRI]`: native provenance only; filter by used ontology namespaces.
- `semantic query validate --json [--name NAME|--iri IRI]`: all candidates or one exact selector, including diagnostics.
- `semantic query export --name NAME [--output FILE|--json]`: raw SPARQL by default; JSON includes content and SHA-256; output is atomic.
- `semantic query check --name NAME --artifact FILE [--json]`: exact-byte drift check, preserving the file.
- `semantic export --layer queries`: RDF descriptions; full public exports include this layer.

MCP uses `reqvire.semantic.queries` with optional `name`, `iri`, `namespace_base`, and `include_content`, and `reqvire.semantic.queries.validate` with optional selector. Reuse the core contracts. Imported RDF-only descriptions do not become managed artifacts. Validate after edits; normal mutation validation prevents invalid persistence.
