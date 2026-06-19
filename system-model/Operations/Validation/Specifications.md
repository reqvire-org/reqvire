# Elements

### Excluded File Relation Validation Contract Specification

#### Details
Excluded-file relation validation behavior:
1. Registers files matching exclusion patterns in registry context for relation-target validation.
2. Skips internal element parsing/validation for excluded files.
3. Preserves ability to validate references that point to excluded file paths.

#### Metadata
  * type: specification

#### Relations
  * define: [Excluded File Relation Validation](ValidationRequirements.md#excluded-file-relation-validation)
---

### Integrated Validation Contract Specification

#### Details
Integrated validation execution behavior:
- Commands are split into model-dependent commands and raw-file commands.
- Model-dependent commands invoke two-pass validation before execution and stop on validation failures.
- Raw-file commands skip model validation when their behavior operates directly on file content.
- Validation gating ensures commands needing graph consistency do not run with invalid model state.

#### Metadata
  * type: specification

#### Relations
  * define: [Integrated Validation](ValidationRequirements.md#integrated-validation)
---

### Internal Consistency Validator Contract Specification

#### Details
The consistency validator is expected to verify:
- **Global Element Name Uniqueness**: Element names are globally unique across all files in the model
- **Duplicate Detection**: Detect and report when multiple elements in different files share the same name
- **Location Reporting**: Report both file locations where duplicate element names occur
- **Clear Error Messages**: Error messages clearly indicate that element names must be globally unique
- **Circular Dependencies**: Detect and report circular dependency chains in requirements
- **Orphaned Elements**: Identify elements without proper traceability connections
- **Inconsistent Patterns**: Detect relationship patterns that violate model constraints

Rationale: Element names serve as stable IDs for element identity, independent of file location. Global uniqueness is essential for proper element identification and change tracking across the model.

#### Metadata
  * type: specification
---

### Relation Element Type Validator Contract Specification

#### Details
The validator enforces the Reqvire relation ontology together with the canonical element type vocabulary.

Validation shall check:
- relation endpoint families and inverse relation compatibility from the relation ontology
- ontology, capability, requirement, and contract compatibility from the ontology, capability, requirement, and semantic-contracts
- evidence-backed verification compatibility from the verification contracts
- canonical semantic relations are rejected for custom `other` and `other-TYPENAME` element types
- contract restrictions: contract elements use only `define` relations and cannot have Reused Contract Context subsections
- `definedBy` targets resolve to element identifiers, not plain file paths or `# Element` file links without element fragments

This validation occurs:
- During model parsing and validation (model.rs, parser.rs)
- During link operations at CRUD time (graph_registry.rs)

#### Metadata
  * type: specification
---

### Requirements Processing Specification

Specification for how requirements files are discovered and processed.

#### Details
**File Discovery:**
- Parse all .md files from git repository root
- Apply .gitignore and .reqvireignore exclusions

**Processing Pipeline:**
- Pass 1: Element collection and local validation
- Pass 2: Graph construction and relation validation
- GraphRegistry built from ElementRegistry after Pass 1

#### Metadata
  * type: specification
---

### Reused Contract Context Scope Validation Contract Specification

#### Details
When validating reused_contract_context, the system is expected to enforce reused_contract_context scope constraints for contract-element identifier targets and report errors with clear messages indicating the reusesContract element, the reused_contract_context target, and the reason for the violation.

Reused Contract Context scope validation is expected to enforce:
- Hierarchical independence from the contract's defining hierarchy
- Upstream propagation within a hierarchy branch
- One-direction reused_contract_context flow between capability-root subgraphs

#### Metadata
  * type: specification
---

### Reused Contract Context Target Validation Contract Specification

#### Details
Reused Contract Context targets support model element identifier references with family-specific compatibility rules.

**Identifier Targets:**
- Requirement reused_contract_context must point to reusable requirement-owned non-semantic-contract element types only (`source`, `constraint`, `behavior`, `specification`, `state`, `input-output`)
- Reused Contract Context to `ontology` is invalid; ontology vocabulary bindings use `#### Concept References` on non-ontology, non-semantic-contract elements or `use`/`usedBy` on semantic contracts
- Normalized like relation targets (resolved to full identifier path)
- Validation is expected to reject identifiers pointing to non-reusable element types
- Validation is expected to reject unresolved identifiers
- Provides clear error message indicating the expected element type

This validation ensures that reused_contract_context reference reusable requirement-owned contracts and do not carry ontology semantics.

#### Metadata
  * type: specification
---

### Semantic Contract Shape Validation Specification

Technical specification for validating semantic-contract `#### Shapes` documents as SHACL/RDF model artifacts.

#### Details
Semantic-contract shape validation behavior:
- Parse each semantic-contract `#### Shapes` fenced Turtle block with Oxigraph and pass the resulting SHACL RDF subgraph into the reusable SHACL parser.
- The reusable SHACL parser must be independent of Reqvire element types, semantic-contract relations, filenames, source identifiers, and validation error wording. It consumes Oxigraph RDF terms/quads and produces a compiled Oxigraph-backed `ShaclRegistry`/AST plus parser diagnostics.
- The compiled SHACL registry must identify `sh:NodeShape`, `sh:PropertyShape`, `sh:Shape`, targeted shape nodes, property-shape nodes, target definitions, property paths, nested property shapes, syntax constraints, logical constraints, qualified value shapes, enumerations, and SPARQL constraints.
- Reject invalid Turtle syntax, duplicate or missing `#### Shapes` sections, and shape documents that do not contain any discoverable SHACL shape node.
- Allow SHACL target mechanisms supported by the SHACL RDF vocabulary, including `sh:targetClass`, `sh:targetNode`, `sh:targetSubjectsOf`, `sh:targetObjectsOf`, and `sh:target`, instead of requiring every node shape to be class-targeted.
- Check supported SHACL constraint structure needed by Reqvire, including property paths, class constraints, datatype constraints, target-node references, node kind, cardinality, value ranges, string constraints, relational property constraints, logical constraints, qualified value shapes, allowed-value lists, constants, and SPARQL query constraints.
- Build a Reqvire-specific ontology alignment input outside the SHACL parser by taking the ontology subset reachable from the semantic contract's explicit `use` relations and ontology ancestors.
- Resolve model-owned SHACL references by running the generic SHACL registry against a domain ontology index derived from the semantic index's reachable parsed RDF context for the contract. Standard RDF, RDFS, OWL, XSD, and SHACL vocabulary shall be resolved through Reqvire's built-in reserved vocabulary handling only at the Reqvire adapter/alignment layer.
- Keep full SHACL data validation/execution out of scope unless a separate verification requirement introduces a SHACL execution engine.

The SHACL structural parser and ontology aligner shall operate as a compile-time design/schema alignment phase:

1. Shape node discovery:
   - Identify structural graph nodes containing explicit shape indicators: `sh:NodeShape`, `sh:PropertyShape`, and `sh:Shape`.
   - Identify structural graph nodes containing explicit target definitions: `sh:targetClass`, `sh:targetNode`, `sh:targetSubjectsOf`, `sh:targetObjectsOf`, and `sh:target`.
   - Isolate a deduplicated pipeline of all discovered shape node candidates.
2. Structural parsing and scoping:
   - Determine whether each shape candidate defines a node layout or a specific property path.
   - Extract target class, node, or predicate pointers from shape definitions.
   - Deconstruct complex property path terms into recursive path nodes for IRI paths, sequence paths, choices, inverse paths, and repetition modifiers.
   - Organize structural parent-child configurations when nested property shapes are detected.
3. Constraint syntax mapping:
   - Map literal validation syntax bounds: `sh:datatype`, `sh:nodeKind`, `sh:minLength`, `sh:maxLength`, `sh:pattern`, `sh:languageIn`, and `sh:uniqueLang`.
   - Map value range syntax thresholds: `sh:minCount`, `sh:maxCount`, `sh:minExclusive`, `sh:minInclusive`, `sh:maxExclusive`, and `sh:maxInclusive`.
   - Map structural logic references: `sh:and`, `sh:or`, `sh:not`, `sh:xone`, `sh:node`, `sh:in`, `sh:hasValue`, and `sh:qualifiedValueShape`.
   - Map relational path mapping properties: `sh:class`, `sh:equals`, `sh:disjoint`, `sh:lessThan`, and `sh:lessThanOrEquals`.
   - Map extension rules referencing raw custom string scripts through `sh:sparql`.
   - Preserve raw Oxigraph predicate/object pairs for SHACL constraints beside typed constraints so later validation phases can inspect unsupported or not-yet-specialized SHACL predicates without reparsing Turtle.
4. Static ontology alignment verification:
   - Cross-reference extracted target identifiers to verify class, predicate, and target-node references exist inside the domain ontology subset supplied by Reqvire.
   - Cross-reference parsed recursive property paths to verify every referenced schema predicate exists in the supplied domain ontology subset.
   - Cross-reference class parameters (`sh:class`) and datatype restrictions (`sh:datatype`) against valid ontology definitions or built-in datatype vocabulary.
   - Preserve value constraints such as `sh:hasValue` and `sh:in` without requiring every value IRI to be a declared ontology term.
   - Deliver and store a compiled static SHACL AST layout safely inside the semantic validation registry for reuse by diagnostics, ontology projection, export, and Explorer rendering.

#### Metadata
  * type: specification

#### Relations
  * define: [Semantic Contract Shape Validation](ValidationRequirements.md#semantic-contract-shape-validation)
---

### SHACL Structural Parser Registry Specification

Technical specification for the reusable SHACL structural parser and registry.

#### Details
The SHACL parser registry shall:
- Accept Oxigraph RDF terms and quads as input.
- Avoid dependencies on Reqvire element types, semantic-contract relations, graph registry internals, filenames, source identifiers, or Reqvire validation wording.
- Discover shape node candidates from explicit shape indicators (`sh:NodeShape`, `sh:PropertyShape`, `sh:Shape`), target predicates (`sh:targetClass`, `sh:targetNode`, `sh:targetSubjectsOf`, `sh:targetObjectsOf`, `sh:target`), property shape references, and `sh:path`.
- Deduplicate shape node candidates before structural parsing.
- Classify each shape as a node shape or property shape based on SHACL type and path structure.
- Extract target identifiers as typed SHACL target variants.
- Deconstruct property paths into recursive AST path nodes for IRI paths, inverse paths, sequence paths, alternative paths, and repetition modifiers while preserving Oxigraph `NamedNode`, `NamedOrBlankNode`, and `Term` values directly.
- Preserve nested property-shape parent-child relationships.
- Map supported constraint syntax into typed AST constraints for datatype, class, node kind, cardinality, value range, string, language, relational property, logical, qualified value, enumeration, constant, and SPARQL constraints.
- Preserve raw SHACL constraint facts as Oxigraph predicate/object pairs alongside typed constraints.
- Return parser diagnostics for malformed SHACL structures without converting those diagnostics into Reqvire-specific errors.
- Store compiled shapes in a reusable registry keyed by Oxigraph shape identifiers.

#### Metadata
  * type: specification

#### Relations
  * define: [SHACL Structural Parser Registry](ValidationRequirements.md#shacl-structural-parser-registry)
---

### SHACL Ontology Alignment Specification

Technical specification for aligning a compiled SHACL registry with a supplied domain ontology index.

#### Details
The generic SHACL ontology aligner shall:
- Accept a compiled SHACL registry and a domain ontology index as input.
- Provide a domain-index constructor from supplied Oxigraph RDF quads so callers can pass a reachable authored/external ontology context without hand-populating class/property/datatype buckets.
- Avoid dependencies on Reqvire element types, semantic-contract relations, graph registry internals, source identifiers, and Reqvire validation wording.
- Cross-reference SHACL target classes against declared ontology classes.
- Cross-reference named `sh:targetNode` references against resolvable named nodes from the supplied ontology index.
- Cross-reference `sh:targetSubjectsOf`, `sh:targetObjectsOf`, parsed property paths, inverse paths, and relational property constraints against declared ontology properties.
- Cross-reference `sh:class` constraints against declared ontology classes.
- Cross-reference `sh:datatype` constraints against declared ontology datatypes or accepted built-in datatype vocabulary.
- Preserve `sh:hasValue` and `sh:in` values as parsed constraint facts without treating every listed IRI as an ontology term-existence requirement.
- Return generic alignment errors such as undeclared class, undeclared property, undeclared datatype, undeclared target node, and invalid inverse path, preserving the SHACL predicate that caused the reference.
- Keep full SHACL data validation/execution out of scope unless a separate validation engine is introduced.

#### Metadata
  * type: specification

#### Relations
  * define: [SHACL Ontology Alignment](ValidationRequirements.md#shacl-ontology-alignment)
---

### Reqvire SHACL Context Adapter Specification

Technical specification for adapting Reqvire semantic-contract context to generic SHACL ontology alignment.

#### Details
The Reqvire SHACL context adapter shall:
- Ask the semantic index for parsed RDF quads from the ontology subset reachable through the semantic contract's explicit `use` relations and ontology ancestors.
- Include parsed local external ontology source quads reachable through that ontology subset.
- Derive the generic SHACL domain ontology index from those supplied RDF quads rather than manually rebuilding declaration buckets in the validation adapter.
- Treat built-in RDF, RDFS, OWL, XSD, and SHACL vocabulary through Reqvire's reserved vocabulary registry where supported positions allow it.
- Pass the compiled SHACL registry and the reachable domain ontology index into the generic SHACL ontology aligner.
- Reject hidden ontology dependencies by passing only the explicit reachable ontology subset to the generic aligner; a model-owned SHACL reference outside that subset shall fail as an undeclared alignment reference from the perspective of the semantic contract.
- Convert generic SHACL parser and alignment diagnostics into Reqvire validation errors that include semantic-contract identifiers, reference kind, referenced IRI, declaring ontology context when available, and fix guidance.
- Keep full SHACL data validation/execution out of scope unless a separate verification requirement introduces a SHACL execution engine.

#### Metadata
  * type: specification

#### Relations
  * define: [Reqvire SHACL Context Adapter](ValidationRequirements.md#reqvire-shacl-context-adapter)
---
