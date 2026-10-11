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
- Disk-loaded models are accepted only after element collection and shared rule evaluation succeed against the complete current registry. Reports, semantic exports, and consumers of materialized inverse relations must not publish an invalid model as authoritative.
- Honor the command's declared validation mode and reuse a matching validated snapshot when permitted by the model-build contract. Diagnostic query operations may retain malformed query candidates for their explicitly defined diagnostic response; they must not present those candidates as a successfully validated model or bypass structural validation.
- Invoke the central validators according to the owning requirement's Contract References. This integration owns execution timing; each shared contract owns its rule, permitted inputs, diagnostic, and repair semantics.
- Evaluate forward prose links only after all eligible elements have been collected, independently of file and declaration order. Apply the same shared identifier normalization and structural rules as mutation validation.
- Preserve aggregated diagnostics, their structured guidance, and non-zero failure status. A failed build must not publish an accepted model snapshot or modify authored files.
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
- **Circular Dependencies**: Detect and report circular dependency chains in requirements, including the combined fulfillment graph defined by the Relation Semantics Specification bound by the owning requirement. Diagnostics MUST identify a closed requirement path and distinguish hierarchy contributions from contract-consumer contributions. Cycle rejection MUST be independent of source declaration order and direct implementation evidence.
- **Orphaned Elements**: Identify elements without proper traceability connections
- **Inconsistent Patterns**: Detect relationship patterns that violate model constraints

Rationale: Element names serve as stable IDs for element identity, independent of file location. Global uniqueness is essential for proper element identification and change tracking across the model.

#### Metadata
  * type: specification
---

### Model Element Prose Link Validation Specification

Model dependencies belong in explicit structural declarations. Authored prose uses readable words and concepts, with Concept References providing terminology bindings where applicable.

#### Details

##### Authored Content Scope
- Inspect authored element prose in both `# Elements` and `# Element` documents using the parser's element-content boundaries. Include the main body, Details, non-reserved prose subsections, and prose in lists, tables, and blockquotes.
- Inspect inline Markdown links and reference-style Markdown links after resolving their link definitions. A different display label or link style does not change the target classification.
- Literal examples in fenced or indented code blocks, inline code, and escaped Markdown that does not form a hyperlink are not prose hyperlinks.
- Links declared in Relations, Contract Bindings, Contract References, and Concept References retain their own syntax and validation rules. Structured Metadata, Ontology, External Ontology, Shapes, and Query payloads are not prose for this check.
- Apply this rule to authored model-element content. Unstructured planning documents and other documents outside parsed model elements are outside its scope. Explorer links generated from valid Concept References are presentation output and do not become authored prose violations.

##### Target Resolution and Rejection
- Resolve link destinations using the existing workspace-root, source-relative path, same-file fragment, and fragment-normalization rules. Resolve against the complete current model registry, including eligible Git worktrees in the effective workspace.
- Report a validation error when the destination resolves to any registered model element, regardless of its type, display label, file location, or whether the author also declared a valid structural dependency to that element. Self-links are included.
- A file-only link to a registered `# Element` document also identifies that document's sole element for this check; omitting the fragment must not bypass the rule.
- External URLs, implementation or evidence assets, unstructured documents, and local document anchors that do not resolve to model elements do not produce this error. Recognize targets by model resolution rather than by a `system-model/` path prefix or a `.md` extension alone.
- Validation must not infer a relation or a contract dependency from a rejected prose hyperlink or silently rewrite authored content.

##### Validation Snapshot
- Resolve against the complete supplied model snapshot's current content, identifiers, and element membership. Results must be independent of file and declaration order, including forward links to later declarations.
- Classify every offending occurrence, including a previously unresolved destination that now resolves to an element in the supplied snapshot. Evaluation must not substitute a different snapshot or infer dependency semantics from the prose.

##### Diagnostic and Repair Guidance
- Emit the stable diagnostic code `model-element-link-in-prose` with the source element identifier, source file and link occurrence line, authored destination, and resolved target element identifier. Preserve structured source context according to the existing diagnostic contract.
- Include source and target element types and the offending link text. State why a prose hyperlink does not declare a valid model dependency; merely renaming its visible label or adding a second structural declaration does not repair it.
- Every prose-link diagnostic must carry a non-empty structured `guidance` list. Text and machine-readable renderers must preserve its repair steps and examples rather than reducing the response to the error code or a generic failure message.
- Explain that authored prose cannot link to a model element. Guide the author to use plain wording, valid Concept References for terminology, and the applicable typed relation for model structure. For a requirement's dependency on a requirement-owned contract, distinguish Contract References for review dependencies from Contract Bindings for implementation obligations.
- Repairs must respect element-type, ownership, hierarchy, and dependency constraints. Do not suggest adding contract dependency sections to capabilities, contracts, or verifications, and do not guess the dependency kind from hyperlink wording.
- The first repair step must identify the exact hyperlink to remove and explain how to express its intent as readable domain wording. If the wording denotes terminology, suggest a valid Concept Reference only on a source type that permits it and only to a native concept target. Do not propose copying the target specification into prose or duplicating its owning requirement's obligation.
- For a requirement linking to a requirement-owned contract, identify the target's owner and show source-relative Markdown templates for both dependency kinds, explaining when each applies. Show them as alternatives; retain the one-section constraint and advise meaningful requirement decomposition when obligations need different dependency kinds.
- For capability, contract, or verification prose linking to a contract, identify the appropriate responsible or owning requirement as the place to declare that dependency. Do not suggest a `definedBy` ownership transfer, a hierarchy edge to the contract, or a Contract References subsection on the invalid source type.
- For other source and target families, suggest only type-compatible structural relations or permitted Concept References. Ontology and semantic-contract guidance must respect their Turtle, ontology-use, and constraint channels; it must not propose unsupported concept-reference sections.
- Examples must use the actual resolved target name and a valid source-relative destination when known. If intent cannot be inferred, explain the valid choices without inventing an obligation, owner, relation, or automatic rewrite.

Example guidance for a requirement linking to a reusable specification:
```text
error[model-element-link-in-prose]: Authored prose links to a model element.
source: <source file>:<line> (<source requirement>, type: requirement)
target: <target identifier> (type: specification; owner: <target requirement>)
repair:
  1. Replace the offending hyperlink with readable domain wording in the prose.
  2. For a content dependency that needs review when the specification changes,
     add the target under Contract References on the source requirement.
  3. If the source requirement independently implements a shared obligation,
     use Contract Bindings instead. Do not author both sections on one requirement.
```

The diagnostic includes the applicable Markdown template as literal guidance:
```markdown
#### Contract References
  * [<actual target name>](<source-relative target identifier>)
```

For a capability or specification source, the response must instead direct the dependency to the responsible requirement and must not present this subsection as an edit to that source element.

#### Concept References
  * [Concept Reference](../../Thesaurus/Thesaurus.md#concept-reference)
  * [Contract Dependency](../../Thesaurus/Thesaurus.md#contract-dependency)
  * [Element](../../Thesaurus/Thesaurus.md#element)

#### Metadata
  * type: specification

#### Relations
  * define: [Model Element Prose Link Validation](ValidationRequirements.md#model-element-prose-link-validation)
---

### Native Concept Taxonomy Cycle Validation Specification

Native concept taxonomy must be acyclic independently of concept ancestry and requirement dependencies.

#### Details
- Resolve native concept `broader` and `narrower` targets, then normalize taxonomy to child-to-parent edges: `A broader B` and `B narrower A` are one edge. Consistent reciprocal declarations must not create a false cycle.
- Reject self-loops, contradictory two-concept taxonomy, and longer cycles, including cycles mixing the two authored aliases. Ignore `related`, `exactMatch`, and `closeMatch` when detecting taxonomy cycles; symmetric associations remain valid.
- Preserve acyclic multiple-parent and reconvergent taxonomy. Build taxonomy adjacency once per validation and traverse it without recursive stack growth or enumerating all possible paths.
- Report a deterministic closed concept-identifier path in canonical `broader` direction. Element and relation input order must not change the diagnostic.
- Evaluate the supplied model snapshot's current canonical relations rather than stale derived concept payloads or relations from another snapshot.
- Existing missing-target and scheme-boundary checks remain in force; cycle detection does not weaken them.

#### Metadata
  * type: specification

#### Relations
  * define: [Native Concept Taxonomy Cycle Validation](ValidationRequirements.md#native-concept-taxonomy-cycle-validation)
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
- Resolve scheme membership and taxonomy from the supplied model snapshot; generated inverse relations must not change the result.

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

#### Metadata
  * type: specification
---

### Requirements Processing Specification

Specification for how requirements files are discovered and processed.

#### Details
**File Discovery:**
- Parse all eligible `.md` files from the effective workspace root
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
- Reuse the SHACL registry compiled during the same semantic-index build, including for in-memory edits and declaration-removal validation, without recompiling its Shapes block for alignment.
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
- For an unresolved model-owned schema reference, distinguish a term absent from all declarations from a term declared outside the contract's explicit ontology-use context. Report the semantic-contract identifier, reference kind, referenced IRI, and declaring ontology identifier when one exists.
- Repair guidance for a missing declaration identifies valid choices to restore a declaration or update/remove the schema reference. For an out-of-context declaration, guidance identifies the explicit `use` relation, declaration placement, or reference change needed to restore valid ontology context. Preserve the existing built-in, external-vocabulary, datatype, and data-value rules when classifying the reference.
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
   - Deliver and store a compiled static SHACL AST layout safely inside the semantic validation registry for reuse by diagnostics, ontology projection, export, and downstream rendering data.

#### Metadata
  * type: specification

#### Relations
  * define: [Semantic Contract Shape Validation](ValidationRequirements.md#semantic-contract-shape-validation)
---

### Structured Validation Diagnostic Contract Specification

Validation failures are expected to retain machine-readable diagnostic context until the final interface rendering boundary.

#### Details
Diagnostic data contract:
- `code`: stable diagnostic token when the validator can classify the failure.
- `message`: human-readable summary suitable for existing terminal output.
- `context.file`: workspace-root-relative file path when the failure is tied to a source file.
- `context.line` and `context.column`: one-based source location values when known.
- `context.element_id`: full Reqvire element identifier when the failure is tied to a model element.
- `context.element_type`: source element type when repair choices depend on it.
- `target.element_id` and `target.element_type`: resolved target context when the rule identifies an offending model-element target.
- `guidance`: an ordered list of actionable repair entries when the rule defines them. Each entry contains a human-readable `message` and may contain a `section` and literal Markdown `example` appropriate to the source type. Required rule-specific guidance must not be omitted.

Rendering rules:
- Text output may preserve the existing message format while sourcing fields from structured diagnostics.
- Structured machine-readable outputs should expose the structured fields directly.
- Text, JSON, CLI, and MCP renderers must retain rule-provided guidance and examples. The message and guidance must let a model author or agent identify the mistake and choose a valid repair without guessing unsupported relation syntax or parsing terminal text.
- Missing context fields must be omitted or null rather than replaced with misleading placeholder values.
- Diagnostic codes should be additive and stable once exposed to machine consumers.

#### Metadata
  * type: specification

#### Relations
  * define: [Structured Validation Diagnostics](ValidationRequirements.md#structured-validation-diagnostics)
---
