# Element

## Metadata
  * type: specification

## Relations
  * define: [Reserved Subsections Support](StructureAndParsing.md#reserved-subsections-support)

## ReservedSubsections

# Reserved Subsections Specification

Reserved subsection vocabulary is defined by the Reqvire core element ontology. This specification defines parser-facing syntax, validation, and serialization behavior for those ontology-defined subsection concepts.

## Relations Subsection

Must be defined with a level 4 header: `#### Relations`.

The Relations subsection defines relationships between elements using markdown bullet list syntax.

**Parsing Rules:**
- Relation entries are bullet points starting with `*`
- Format: `* relationType: [Element Name](path#element-id)`
- Duplicate relation entries within the same `#### Relations` subsection are not allowed

**Examples:**
```markdown
#### Relations
* derivedFrom: [Parent Requirement](../path/file.md#parent)
* satisfiedBy: [Implementation](../../src/module.rs)
* verify: [Target Requirement](file.md#target)
```

## Details Subsection

Must be defined with a level 4 header: `#### Details`.

The Details subsection provides supporting requirement context and intent-level clarification.

**Parsing Rules:**
- When parsing `#### Details` subsections, any markdown headers or elements within `<details>...</details>` tags are skipped
- Content within the Details subsection is part of the requirement narrative context
- Details subsection does **not** replace contract relations
- Technical specifications, constraints, and behaviors that define implementation contracts should be captured in explicit contract elements linked via `definedBy`
- Any normative statements in the Details subsection are interpreted as requirement-level statements unless moved to dedicated contract elements

**Examples:**
```markdown
### My Requirement

The system shall perform action X.

#### Details
Implementation details shall follow associated contract specifications.
Additional context about action X:
- Operational assumption 1
- Clarification 2
```

## Metadata Subsection

Must be defined with a level 4 header: `#### Metadata`.

The Metadata subsection stores element properties including type, requirement governance metadata where valid, and custom attributes.

**Parsing Rules:**
- Contains properties in list format: `* property_name: property_value`
- Property entries are listed as bullet points (`*`), with **two spaces** (`  *`) of indentation followed by property_name + ': ' + property_value
- May include custom properties, but reserved property names have the constraints defined by their owning specifications

**Reserved Properties:**

The following properties have special meaning:

- `type`: Defines the element type (supported types are defined in [Supported Element Types Specification](../Specifications.md#supported-element-types-specification))
- `status`, `priority`, `risk`, `owner`: Define governance metadata for capability and requirement elements only (defined in [Requirement Governance Metadata Specification](../Specifications.md#requirement-governance-metadata-specification))
- Additional reserved properties may be defined in future releases

**Examples:**
```markdown
### My Element

This is a verification element.

#### Metadata
  * type: verification
  * domain: safety
  * review_method: inspection

#### Relations
* verify: [Some Requirement](#some-requirement)
```

```markdown
### My Element

This is a verification element.

#### Details
Some details.

#### Metadata
  * type: verification
  * domain: safety
  * review_method: inspection

#### Relations
  * verify: [Some Requirement](#some-requirement)
```

## Contract Bindings Subsection

Must be defined with a level 4 header: `#### Contract Bindings`.

The Contract Bindings subsection links a requirement to explicit reusable contract context from another subgraph. Reused contract context does not provide ontology context. SKOS concept bindings belong in `#### Concept References`; semantic-contract ontology dependencies belong in `use`/`usedBy` relations.

### Element Identifier Contract Bindings

Element identifier contract_bindings link to model elements that are reusable under the Reqvire relation and contract_bindings compatibility model.

**Parsing Rules:**
- Support markdown link syntax with fragment identifiers: `* [Element Name](path#element-id)`
- Same-file references: `* [Element Name](#element-id)`
- Cross-file references: `* [Element Name](relative/path/file.md#element-id)`
- Identifiers are normalized using the same rules as relation targets
- Link text contains the element name

**Validation Rules:**
- Target element must exist in the model
- Requirement contract_bindings targets must be reusable requirement-owned `source`, `constraint`, `behavior`, `specification`, `state`, or `input-output` contract elements. Semantic-contract dependencies use `constrainedBy`/`constrain` and ontology `use` relations.
- Capability, verification, contract, and requirement semantic vocabulary references use `#### Concept References`, not contract_bindings.
- Requirement-owned contract targets must have exactly one compatible `define` relation before they are reusable.
- Non-reusable element identifiers are rejected with a validation error.
- Redundant same-hierarchy contract_bindings and invalid cross-subgraph contract_bindings flow are rejected by contract_bindings validation.

**Examples:**
```markdown
### API Requirement

The system shall expose an API contract.

#### Contract Bindings
* [Reusable Payload Shape](Contracts.md#reusable-payload-shape)
```

## Concept References Subsection

Must be defined with a level 4 header: `#### Concept References`.

Concept references bind readable element prose to native `concept` elements; Reqvire derives generated native SKOS concept IRIs from those targets. They may be authored on capability, requirement, contract, verification-objective, and concrete verification elements. They must not target arbitrary OWL classes, properties, individuals, or SKOS resources authored directly in ontology Turtle; structural ontology terms should point back to curated native concepts with `reqvire:mapsToConcept` when that bridge is useful. They must not be authored on ontology elements, because ontology elements declare structural terms in `#### Ontology`. They must not be authored on semantic-contract elements, because semantic contracts are semantic graph artifacts that use ontology through `use`/`usedBy` relations and SHACL `#### Shapes`.

**Parsing Rules:**
- Entries are bullet points starting with `*`.
- Format: `* [Label](concept-element-link)`.
- The link target must resolve with normal Reqvire Markdown link rules to an existing native `concept` element.
- The parser records the authored label, normalized concept-element target, and source line number.
- Semantic validation fails when the target is missing, does not have `type: concept`, or cannot derive a generated SKOS concept IRI from a `concept-scheme` namespace.
- Semantic validation derives the generated SKOS concept IRI from the target concept element and scheme; authors do not write concept IRIs or CURIEs in this subsection.
- `format --fix` normalizes already-valid concept-reference Markdown links to canonical source-relative targets. `migrate --fix` rewrites legacy `* Label: IRI` entries only when the IRI resolves to exactly one generated native concept element.

**Example:**
```markdown
  * [External ontology source](../Thesaurus/Thesaurus.md#external-ontology-source)
```

## Ontology Subsection

Must be defined with a level 4 header: `#### Ontology`.

Ontology subsections contain fenced RDF/Turtle content owned by ontology elements.

**Parsing Rules:**
- The parser extracts fenced code blocks under `#### Ontology`.
- Ontology elements must contain exactly one valid ontology block.
- The parser preserves fenced block language, content, and source line number.
- RDF parsing and ontology validation are semantic responsibilities over the extracted block, not markdown grammar parsing responsibilities.

## External Ontology Subsection

Must be defined with a level 4 header: `#### External Ontology`.

External ontology subsections declare local external vocabulary source files for ontology elements.

**Parsing Rules:**
- The subsection is repeatable.
- Entries are bullet points starting with `*` or `-`.
- Format: `* key: value`.
- Required keys: `prefix`, `namespace`, `resource`, and `source`.
- Optional key: `format`; defaults to `turtle`.
- The parser records the parsed source declaration and subsection line number.
- Semantic model construction consumes parsed declarations and must not reparse this markdown grammar from raw element content.

**Example:**
```markdown
  * prefix: skos
  * namespace: http://www.w3.org/2004/02/skos/core#
  * resource: http://www.w3.org/2004/02/skos/core
  * source: references/ontologies/skos.rdf
  * format: rdfxml
```

## Shapes Subsection

Must be defined with a level 4 header: `#### Shapes`.

Shapes subsections contain fenced SHACL/RDF content owned by semantic-contract elements.

**Parsing Rules:**
- The parser extracts fenced code blocks under `#### Shapes`.
- Semantic-contract elements must contain exactly one valid shapes block.
- The parser preserves fenced block language, content, and source line number.
- SHACL parsing and ontology alignment run over the extracted RDF block; they must not parse the markdown subsection grammar directly.

## Query Subsection

Must be defined with a level 4 header: `#### Query`.

Query subsections contain fenced SPARQL or query content when a semantic contract owns query text.

**Parsing Rules:**
- The parser extracts fenced code blocks under `#### Query`.
- The parser preserves fenced block language, content, and source line number.
- Query validation or execution behavior is owned by semantic model requirements over the parsed block.
