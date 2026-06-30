# Elements

### Ontology Kernel Architecture Verification Objective

This objective groups verification that the o-kernel crate boundary remains independent, RDF-native, and covered by focused Rust-level verification when implemented.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [O-Kernel Ontology Classification Unit Test Verification](#o-kernel-ontology-classification-unit-test-verification)
  * derive: [O-Kernel Physical Module Unit Test Verification](#o-kernel-physical-module-unit-test-verification)
  * derive: [O-Kernel RDF Term Description Unit Test Verification](#o-kernel-rdf-term-description-unit-test-verification)
  * derive: [O-Kernel Referenced Graph Subset Unit Test Verification](#o-kernel-referenced-graph-subset-unit-test-verification)
  * derive: [O-Kernel Reserved Vocabulary Unit Test Verification](#o-kernel-reserved-vocabulary-unit-test-verification)
  * derive: [O-Kernel SHACL Services Unit Test Verification](#o-kernel-shacl-services-unit-test-verification)
  * derive: [Ontology Kernel Boundary Analysis](#ontology-kernel-boundary-analysis)
---

### O-Kernel Ontology Classification Unit Test Verification

This unit-test verification defines the Rust test evidence required for ontology construct classification after the o-kernel code refactor.

#### Details
Required Rust test coverage:

- Verify direct-authored RDF/RDFS/OWL/SHACL quads classify property domain/range, subclass inclusion, class membership, disjointness, equivalence, inverse properties, property chains, property characteristics, restrictions, class expressions, and SHACL shape overlays.
- Verify RDF list member order is preserved for property chains and class expressions.
- Verify classification does not perform OWL reasoning, SHACL-AF rule execution, or inferred materialization.
- Verify returned construct records are generic and require consumers to add source provenance and runtime graph placement outside o-kernel.
- Verify default construct and construct-projection identifiers use the neutral `urn:o-kernel` namespace.
- Verify caller-provided classifier options control the construct and construct-projection ID namespace without introducing Reqvire-specific strings into o-kernel output.

The o-kernel unit tests in `crates/o-kernel/src/constructs/classify.rs` cover ontology construct classification, default neutral ID namespaces, and caller-provided ID namespace selection.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [classify.rs](../../../crates/o-kernel/src/constructs/classify.rs)
  * verify: [Ontology Construct Classification](../../Architecture/OntologyKernelRequirements.md#ontology-construct-classification)
---

### O-Kernel Physical Module Unit Test Verification

This unit-test verification defines the Rust test evidence required for o-kernel physical module architecture after the o-kernel code refactor.

#### Details
Required Rust test coverage:

- Verify the crate exposes `vocab`, `rdf`, `shacl`, `ontology`, `constructs`, `describe`, `subset`, `diagnostics`, and `prelude` public modules.
- Verify module APIs accept or return RDF-native data types for public RDF inputs and outputs.
- Verify diagnostics remain generic and carry no application file path, source-location, element identifier, graph-layer, or protocol payload dependency.
- Verify SHACL parser tests are colocated with SHACL target, path, constraint, registry, and alignment modules or submodules.
- Verify construct-classification tests are colocated with RDF/RDFS, OWL expression, OWL property axiom, restriction, and SHACL overlay modules or submodules.
- Verify referenced graph subset tests are colocated with seed, reference, closure, and construct modules or submodules.
- Verify `prelude` re-exports stable public types without exposing private module internals.

This verification remains unsatisfied until the code refactor creates and links the o-kernel Rust test target.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [O-Kernel Physical Module Architecture](../../Architecture/OntologyKernelRequirements.md#o-kernel-physical-module-architecture)
---

### O-Kernel RDF Term Description Unit Test Verification

This unit-test verification defines the Rust test evidence required for RDF term description construction after the o-kernel code refactor.

#### Details
Required Rust test coverage:

- Verify selected RDF terms produce direct subject-description triples from supplied RDF graph data.
- Verify configured support predicates include one-hop support resources without including unrelated graph content.
- Verify configured annotation predicates include labels, comments, preferred labels, definitions, and descriptions for selected and support terms.
- Verify construction returns generic direct/support/annotation classification metadata without source-location, output-surface, or application external-source assumptions.

This verification remains unsatisfied until the code refactor creates and links the o-kernel Rust test target.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [RDF Term Description Construction](../../Architecture/OntologyKernelRequirements.md#rdf-term-description-construction)
---

### O-Kernel Referenced Graph Subset Unit Test Verification

This analysis defines the Rust unit-test evidence required for referenced graph subset construction after the o-kernel code refactor.

#### Details
Required Rust unit-test coverage:

- Verify ontology graphs of interest seed dependency terms from subjects, predicates, objects, and RDF list members using the standard external ontology dependency subset profile.
- Verify selected dependency terms produce direct description triples from dependency RDF graphs.
- Verify the standard support context adds bounded support triples and reports depth-boundary terms when traversal reaches the profile expansion bound.
- Verify the standard annotation context adds annotation triples for selected and support terms.
- Verify RDF list closure preserves selected list heads, list cells, list order, and terminal `rdf:nil`.
- Verify output metadata distinguishes seed, directly referenced, support, annotation, list-closure, and depth-boundary triples.
- Verify the service returns generic RDF-native subset data without application source locations, element identifiers, graph-layer names, or presentation payloads.

This verification remains unsatisfied until the code refactor creates and links the o-kernel Rust test target.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Referenced Graph Subset Construction](../../Architecture/OntologyKernelRequirements.md#referenced-graph-subset-construction)
---

### O-Kernel Reserved Vocabulary Unit Test Verification

This unit-test verification defines the Rust test evidence required for standards reserved vocabulary recognition after the o-kernel code refactor.

#### Details
Required Rust test coverage:

- Verify reserved vocabulary recognition is based on expanded IRIs, not Turtle prefix names.
- Verify RDF, RDFS, OWL, and SHACL standards vocabulary graphs are bundled, parsed, and used for reserved vocabulary term recognition.
- Verify OWL built-in datatype IRIs and SHACL-supported datatype-position IRIs are classified separately.
- Verify datatype facets are recognized as facets and not accepted as datatypes.
- Verify arbitrary custom IRIs under standard namespaces are not treated as valid reserved vocabulary unless they appear in the bundled standards graph or explicit XSD datatype/facet policy.

#### Evidence
The o-kernel reserved vocabulary unit tests in `crates/o-kernel/src/vocab/reserved.rs` cover standards graph parsing and classification behavior.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [reserved.rs](../../../crates/o-kernel/src/vocab/reserved.rs)
  * verify: [Standards Reserved Vocabulary Recognition](../../Architecture/OntologyKernelRequirements.md#standards-reserved-vocabulary-recognition)
---

### O-Kernel SHACL Services Unit Test Verification

This unit-test verification defines the Rust test evidence required for SHACL parsing and ontology alignment after the o-kernel code refactor.

#### Details
Required Rust test coverage:

- Verify SHACL parser discovery of node shapes, property shapes, target-only shapes, nested property shapes, recursive paths, raw constraints, and typed constraints from RDF quads.
- Verify SHACL ontology alignment accepts declared classes, properties, datatypes, and target nodes from a supplied ontology index.
- Verify SHACL ontology alignment reports generic undeclared class, property, datatype, target-node, and invalid inverse-path diagnostics without application source-document assumptions.
- Verify `sh:hasValue` and `sh:in` values are preserved without requiring every listed value IRI to be declared as an ontology term.

This verification remains unsatisfied until the code refactor creates and links the o-kernel Rust test target.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [SHACL Ontology Alignment](../../Architecture/OntologyKernelRequirements.md#shacl-ontology-alignment)
  * verify: [SHACL Structural Parser Registry](../../Architecture/OntologyKernelRequirements.md#shacl-structural-parser-registry)
---

### Ontology Kernel Boundary Analysis

This analysis verifies the planned o-kernel crate boundary before implementation.

#### Details
Expected checks:

- Confirm `o-kernel` has no dependency on consumer application crates.
- Confirm consumers depend on `o-kernel`, not the reverse.
- Confirm SHACL parsing, ontology construct classification, RDF term description construction, and generic reserved vocabulary recognition are owned by `o-kernel`.
- Confirm consumer-specific parsing, element registries, semantic indexes, source maps, diagnostics, graph-layer visibility policy, runtime store assembly policy, and response DTOs remain outside `o-kernel`.
- Confirm the implementation does not introduce a custom RDF store, triple/quad model, or graph-layer abstraction that duplicates the selected RDF implementation.
- Confirm kernel diagnostics are generic and do not require source-document assumptions.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Application Boundary Isolation](../../Architecture/OntologyKernelRequirements.md#application-boundary-isolation)
  * verify: [Ontology Kernel Public Contract](../../Architecture/OntologyKernelRequirements.md#ontology-kernel-public-contract)
  * verify: [Ontology Kernel RDF Native Boundary](../../Architecture/OntologyKernelRequirements.md#ontology-kernel-rdf-native-boundary)
---
