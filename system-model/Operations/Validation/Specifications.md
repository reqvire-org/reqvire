# Elements

### Contract Bindings Scope Validation Contract Specification

#### Details
When validating contract_bindings, the system is expected to enforce contract_bindings scope constraints for contract-element identifier targets and report errors with clear messages indicating the bindContract element, the contract_bindings target, and the reason for the violation.

Contract Bindings scope validation is expected to enforce:
- Hierarchical independence from the contract's defining hierarchy
- Upstream propagation within a hierarchy branch
- One-direction contract_bindings flow between capability-root subgraphs

#### Metadata
  * type: specification
---

### Contract Bindings Target Validation Contract Specification

#### Details
Contract Bindings targets support model element identifier references with family-specific compatibility rules.

**Identifier Targets:**
- Requirement contract_bindings must point to reusable requirement-owned non-semantic-contract element types only (`source`, `constraint`, `behavior`, `specification`, `state`, `input-output`)
- Contract Bindings to `ontology` is invalid; SKOS concept bindings use `#### Concept References` on non-ontology, non-semantic-contract elements, while semantic-contract ontology dependencies use `use`/`usedBy`
- Normalized like relation targets (resolved to full identifier path)
- Validation is expected to reject identifiers pointing to non-reusable element types
- Validation is expected to reject unresolved identifiers
- Provides clear error message indicating the expected element type

This validation ensures that contract_bindings reference reusable requirement-owned contracts and do not carry ontology semantics.

#### Metadata
  * type: specification
---

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

### Structured Validation Diagnostic Contract Specification

Validation failures are expected to retain machine-readable diagnostic context until the final interface rendering boundary.

#### Details
Diagnostic data contract:
- `code`: stable diagnostic token when the validator can classify the failure.
- `message`: human-readable summary suitable for existing terminal output.
- `context.file`: git-root-relative file path when the failure is tied to a source file.
- `context.line` and `context.column`: one-based source location values when known.
- `context.element_id`: full Reqvire element identifier when the failure is tied to a model element.

Rendering rules:
- Text output may preserve the existing message format while sourcing fields from structured diagnostics.
- JSON and MCP output should expose the structured fields directly.
- Missing context fields must be omitted or null rather than replaced with misleading placeholder values.
- Diagnostic codes should be additive and stable once exposed to machine consumers.

#### Metadata
  * type: specification

#### Relations
  * define: [Structured Validation Diagnostics](ValidationRequirements.md#structured-validation-diagnostics)
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

### Native Concept Taxonomy Scheme Boundary Validation Specification

The native concept taxonomy scheme boundary contract defines the hard validation rule for SKOS taxonomy authored through Reqvire native concept elements.

#### Details
Validation rules:
- For every native `concept` element, resolve its source concept scheme using the same nearest `concept-scheme` ancestry used for generated SKOS concept IRIs.
- For each authored `broader` or `narrower` relation, resolve the target to a native `concept` element and then resolve that target concept's scheme.
- Validation shall fail when the source and target scheme identifiers differ.
- Validation shall not reject cross-scheme `related`, `exactMatch`, or `closeMatch` relations. Those are the intended cross-scheme association and mapping channels.
- The diagnostic shall name the source concept, relation kind, target concept, source scheme, target scheme, and repair guidance.
- This rule runs during normal model validation before semantic concept export and before any consumers rely on materialized inverse concept relations.

#### Metadata
  * type: specification

#### Relations
  * define: [Native Concept Taxonomy Scheme Boundary Validation](ValidationRequirements.md#native-concept-taxonomy-scheme-boundary-validation)
---

### Relation Element Type Validator Contract Specification

#### Details
The validator enforces the Reqvire relation ontology together with the canonical element type vocabulary.

Validation must check:
- relation endpoint families and inverse relation compatibility from the relation ontology
- ontology, capability, requirement, and contract compatibility from the ontology, capability, requirement, and semantic-contracts
- evidence-backed verification compatibility from the verification contracts
- canonical semantic relations are rejected for custom `other` and `other-TYPENAME` element types
- contract restrictions: contract elements use only `define` relations and cannot have Contract Bindings subsections
- `definedBy` targets resolve to element identifiers, not plain file paths or `# Element` file links without element fragments

This validation occurs:
- During model parsing and validation (model.rs, parser.rs)
- During link operations at CRUD time (`graph_registry/crud_ops.rs`)

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

### Reqvire SHACL Context Adapter Specification

Technical specification for adapting Reqvire semantic-contract context to generic SHACL ontology alignment.

#### Details
The Reqvire SHACL context adapter must:
- Ask the semantic index for parsed RDF quads from the ontology subset reachable through the semantic contract's explicit `use` relations and ontology ancestors.
- Include parsed local external ontology source quads reachable through that ontology subset.
- Derive the generic SHACL domain ontology index from those supplied RDF quads rather than manually rebuilding declaration buckets in the validation adapter.
- Treat built-in RDF, RDFS, OWL, XSD, and SHACL vocabulary through the o-kernel reserved vocabulary registry where supported positions allow it.
- Pass the compiled SHACL registry and the reachable domain ontology index into the generic SHACL ontology aligner.
- Reject hidden ontology dependencies by passing only the explicit reachable ontology subset to the generic aligner; a model-owned SHACL reference outside that subset must fail as an undeclared alignment reference from the perspective of the semantic contract.
- Convert generic SHACL parser and alignment diagnostics into Reqvire validation errors that include semantic-contract identifiers, reference kind, referenced IRI, declaring ontology context when available, and fix guidance.
- Keep full SHACL data validation/execution out of scope unless a separate verification requirement introduces a SHACL execution engine.

#### Metadata
  * type: specification

#### Relations
  * define: [Reqvire SHACL Context Adapter](ValidationRequirements.md#reqvire-shacl-context-adapter)
---

### Semantic Contract Shape Validation Specification

Technical specification for validating semantic-contract `#### Shapes` documents as SHACL/RDF model artifacts.

#### Details
Semantic-contract shape validation behavior:
- Parse each semantic-contract `#### Shapes` fenced Turtle block and pass the resulting SHACL RDF subgraph into the o-kernel SHACL parser.
- The o-kernel SHACL parser must be independent of Reqvire element types, semantic-contract relations, filenames, source identifiers, and validation error wording. It consumes RDF terms/quads and produces a compiled SHACL registry/AST plus parser diagnostics.
- The compiled SHACL registry must identify `sh:NodeShape`, `sh:PropertyShape`, `sh:Shape`, targeted shape nodes, property-shape nodes, target definitions, property paths, nested property shapes, syntax constraints, logical constraints, qualified value shapes, enumerations, and SPARQL constraints.
- Reject invalid Turtle syntax, duplicate or missing `#### Shapes` sections, and shape documents that do not contain any discoverable SHACL shape node.
- Allow SHACL target mechanisms supported by the SHACL RDF vocabulary, including `sh:targetClass`, `sh:targetNode`, `sh:targetSubjectsOf`, `sh:targetObjectsOf`, and `sh:target`, instead of requiring every node shape to be class-targeted.
- Check supported SHACL constraint structure needed by Reqvire, including property paths, class constraints, datatype constraints, target-node references, node kind, cardinality, value ranges, string constraints, relational property constraints, logical constraints, qualified value shapes, allowed-value lists, constants, and SPARQL query constraints.
- Build a Reqvire-specific ontology alignment input outside the SHACL parser by taking the ontology subset reachable from the semantic contract's explicit `use` relations and ontology ancestors.
- Resolve model-owned SHACL references by running the generic SHACL registry against a domain ontology index derived from the semantic index's reachable parsed RDF context for the contract. Standard RDF, RDFS, OWL, XSD, and SHACL vocabulary must be resolved through the o-kernel reserved vocabulary registry only at the Reqvire adapter/alignment layer.
- Keep full SHACL data validation/execution out of scope unless a separate verification requirement introduces a SHACL execution engine.

The SHACL structural parser and ontology aligner must operate as a validation-time design/schema alignment phase:

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
   - Preserve raw RDF predicate/object pairs for SHACL constraints beside typed constraints so later validation phases can inspect unsupported or not-yet-specialized SHACL predicates without reparsing Turtle.
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
