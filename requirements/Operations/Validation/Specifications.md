# Elements

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

### Semantic Contract Reference Context Validation Specification

Technical specification for validating semantic-contract SHACL references against ontology context declared by explicit `use` relations.

#### Details
Semantic-contract SHACL references must resolve through the semantic contract's explicit ontology-use graph. Semantic reference validation issue kinds are defined by the Reqvire validation ontology. Missing references and references declared outside reachable ontology context are validation errors, not lint issues.

The validation rule is scoped through the semantic-contract ontology-use graph:
- A semantic-contract must use at least one ontology element through `use`/`usedBy`.
- The reachable ontology context includes explicit `use` targets and ontology ancestors reached through `derivedFrom`/`derive`.
- A semantic-contract may constrain zero, one, or many requirements through `constrain`/`constrainedBy`.
- Ontology elements outside the contract's explicit use graph are not considered reachable.

The validation rule inspects semantic-contract `#### Shapes` sections and checks these SHACL IRI references:
- `sh:targetClass`
- `sh:path`
- `sh:class`

For each referenced IRI, validation determines whether the IRI is declared by an ontology element and reachable from the semantic-contract ontology-use context:
- If the IRI is not declared anywhere in Reqvire ontology elements, validation reports a missing semantic declaration and CRUD operations that would create that condition are blocked.
- If the IRI is declared by an ontology element outside the reachable use context, validation reports an outside-context semantic reference and CRUD operations that would create that condition are blocked.
- Outside-context errors include the declaring ontology identifier and guidance to add a `use` relation when that dependency is intentional.

The rule is intentionally strict:
- It does not infer or create reused_contract_context.
- It does not rewrite Turtle.
- It enforces explicit `use` relations and ontology hierarchy as the valid semantic dependency path for semantic contracts.
- It prevents model changes that would bypass change-impact traceability.

#### Metadata
  * type: specification

#### Relations
  * define: [Semantic Contract Reference Context Validation](ValidationRequirements.md#semantic-contract-reference-context-validation)
---
