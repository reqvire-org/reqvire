# Elements

### Application Boundary Isolation

The system shall keep consumer-specific parsing, source mapping, diagnostics, runtime graph policy, store assembly policy, and response shaping outside `o-kernel`.

#### Details
The o-kernel shall not depend on consumer element models, model registries, source-document formats, source-location metadata, runtime graph visibility rules, or presentation DTOs.

Consumers shall adapt their own data into RDF quads and shall map kernel results back to source locations outside the kernel boundary.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * definedBy: [Application Boundary Isolation Specification](OntologyKernelSpecifications.md#application-boundary-isolation-specification)
  * specify: [Ontology Kernel Crate](OntologyKernelFeature.md#ontology-kernel-crate)
  * verifiedBy: [Ontology Kernel Boundary Analysis](../Verifications/Architecture/OntologyKernelVerifications.md#ontology-kernel-boundary-analysis)
---

### Ontology Kernel Public Contract

The system shall provide an `o-kernel` crate that exposes a generic contract for standards-based RDF, RDFS, OWL, XSD, SHACL, and SPARQL-compatible ontology processing.

#### Details
The kernel contract shall define reusable algorithm services over RDF data without assuming any consumer-specific model, document language, source map, graph-layer policy, or presentation format.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * definedBy: [Ontology Kernel Public Contract Specification](OntologyKernelSpecifications.md#ontology-kernel-public-contract-specification)
  * derive: [RDF Term Description Construction](#rdf-term-description-construction)
  * specify: [Ontology Kernel Crate](OntologyKernelFeature.md#ontology-kernel-crate)
  * verifiedBy: [Ontology Kernel Boundary Analysis](../Verifications/Architecture/OntologyKernelVerifications.md#ontology-kernel-boundary-analysis)
---

### O-Kernel Physical Module Architecture

The system shall organize the `o-kernel` crate into focused modules for vocabulary, RDF utilities, SHACL structure, ontology indexing, construct classification, term description construction, referenced graph subset construction, and generic diagnostics.

#### Details
The o-kernel shall expose separate vocabulary, RDF utility, SHACL, ontology index, construct classification, term description, referenced graph subset, and diagnostic modules. Each module shall own one standards-based responsibility and shall publish APIs that use RDF-native data types at the boundary.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Reused Contract Context
  * [Ontology Kernel RDF Native Boundary Specification](OntologyKernelSpecifications.md#ontology-kernel-rdf-native-boundary-specification)

#### Relations
  * definedBy: [O-Kernel Physical Module Architecture Specification](OntologyKernelSpecifications.md#o-kernel-physical-module-architecture-specification)
  * derivedFrom: [Ontology Kernel Public Contract](#ontology-kernel-public-contract)
  * specify: [Ontology Kernel Crate](OntologyKernelFeature.md#ontology-kernel-crate)
  * verifiedBy: [O-Kernel Physical Module Unit Test Verification](../Verifications/Architecture/OntologyKernelVerifications.md#o-kernel-physical-module-unit-test-verification)
---

### RDF Term Description Construction

The system shall provide an o-kernel service that constructs description triples for selected RDF terms from supplied RDF graph data.

#### Details
The service shall accept selected RDF terms, supplied RDF graph data, support predicate policy, annotation predicate policy, and support depth policy. It shall construct direct description triples, selected support triples, and selected annotation triples without assuming why the terms were selected or how the constructed triples will be exposed.

The service shall not depend on consumer source declarations, external dependency policy, output visibility rules, source maps, diagnostics, or presentation payloads.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * definedBy: [RDF Term Description Construction Specification](OntologyKernelSpecifications.md#rdf-term-description-construction-specification)
  * derivedFrom: [Ontology Kernel Public Contract](#ontology-kernel-public-contract)
  * specify: [Ontology Kernel Crate](OntologyKernelFeature.md#ontology-kernel-crate)
  * verifiedBy: [O-Kernel RDF Term Description Unit Test Verification](../Verifications/Architecture/OntologyKernelVerifications.md#o-kernel-rdf-term-description-unit-test-verification)
---

### Referenced Graph Subset Construction

The system shall provide an o-kernel service that constructs a bounded external ontology dependency subset graph from dependency RDF graphs referenced by ontology graphs of interest.

#### Details
The service shall accept ontology graphs of interest and external dependency RDF graphs. It shall own the standard ontology dependency subset profile for reference extraction, support context, annotation context, RDF list closure, and bounded expansion. It shall discover dependency terms referenced by the ontology graphs of interest, construct a bounded subset graph for those dependency terms, and return generic construction metadata. Consumer-specific graph role assignment, source provenance, output visibility, and presentation mapping shall remain outside the service.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * definedBy: [Referenced Graph Subset Construction Specification](OntologyKernelSpecifications.md#referenced-graph-subset-construction-specification)
  * derivedFrom: [RDF Term Description Construction](#rdf-term-description-construction)
  * specify: [Ontology Kernel Crate](OntologyKernelFeature.md#ontology-kernel-crate)
  * verifiedBy: [O-Kernel Referenced Graph Subset Unit Test Verification](../Verifications/Architecture/OntologyKernelVerifications.md#o-kernel-referenced-graph-subset-unit-test-verification)
---

### Ontology Kernel RDF Native Boundary

The system shall make `o-kernel` public RDF processing APIs operate directly over RDF terms, triples, quads, datasets, parser/serializer inputs, and query results.

#### Details
The kernel shall not introduce a custom RDF store, triple model, quad model, query result model, graph-layer framework, or source-block abstraction when the selected RDF implementation already provides the needed behavior.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * definedBy: [Ontology Kernel RDF Native Boundary Specification](OntologyKernelSpecifications.md#ontology-kernel-rdf-native-boundary-specification)
  * specify: [Ontology Kernel Crate](OntologyKernelFeature.md#ontology-kernel-crate)
  * verifiedBy: [Ontology Kernel Boundary Analysis](../Verifications/Architecture/OntologyKernelVerifications.md#ontology-kernel-boundary-analysis)
---

### SHACL and Ontology Algorithm Services

The system shall provide standards-based SHACL syntax services, reserved vocabulary recognition, and ontology construct classification as o-kernel services.

#### Details
The kernel services shall include SHACL target, path, constraint, and syntax sanity parsing; generic RDF/RDFS/OWL/XSD/SHACL vocabulary constants; reserved vocabulary recognition; and generic ontology construct classification over RDF quads.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * definedBy: [SHACL and Ontology Algorithm Services Specification](OntologyKernelSpecifications.md#shacl-and-ontology-algorithm-services-specification)
  * derive: [Ontology Construct Classification](#ontology-construct-classification)
  * derive: [SHACL Ontology Alignment](#shacl-ontology-alignment)
  * derive: [SHACL Structural Parser Registry](#shacl-structural-parser-registry)
  * derive: [Standards Reserved Vocabulary Recognition](#standards-reserved-vocabulary-recognition)
  * specify: [Ontology Kernel Crate](OntologyKernelFeature.md#ontology-kernel-crate)
---

### Ontology Construct Classification

The system shall classify direct-authored OWL, RDFS, RDF, and SHACL constructs from RDF quads without applying reasoning or consumer-specific layer placement.

#### Details
The classifier shall identify construct families such as property domain/range, subclass inclusion, class membership, disjointness, equivalence, inverse properties, property chains, property characteristics, restrictions, class expressions, and SHACL shape overlays. Classification shall preserve RDF list member order for ordered RDF constructs and shall return generic construct records. Consumer-specific source provenance and runtime graph placement shall remain outside the classifier.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * definedBy: [Ontology Construct Classification Specification](OntologyKernelSpecifications.md#ontology-construct-classification-specification)
  * derivedFrom: [SHACL and Ontology Algorithm Services](#shacl-and-ontology-algorithm-services)
  * specify: [Ontology Kernel Crate](OntologyKernelFeature.md#ontology-kernel-crate)
  * verifiedBy: [O-Kernel Ontology Classification Unit Test Verification](../Verifications/Architecture/OntologyKernelVerifications.md#o-kernel-ontology-classification-unit-test-verification)
---

### SHACL Ontology Alignment

The system shall align compiled SHACL shape registries against a supplied domain ontology index.

#### Details
The aligner shall accept a compiled shape registry and a domain ontology index derived from supplied RDF quads. The index shall expose declared classes, properties, datatypes, and available named terms from that supplied graph. The aligner shall cross-check SHACL targets, paths, class constraints, datatype constraints, target-node references, and relational property constraints against that supplied ontology index, then return generic alignment errors.

Value constraints such as `sh:hasValue` and `sh:in` shall be preserved in the registry without requiring every value IRI to be a declared ontology term.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * definedBy: [SHACL Ontology Alignment Specification](OntologyKernelSpecifications.md#shacl-ontology-alignment-specification)
  * derivedFrom: [SHACL and Ontology Algorithm Services](#shacl-and-ontology-algorithm-services)
  * specify: [Ontology Kernel Crate](OntologyKernelFeature.md#ontology-kernel-crate)
  * verifiedBy: [O-Kernel SHACL Services Unit Test Verification](../Verifications/Architecture/OntologyKernelVerifications.md#o-kernel-shacl-services-unit-test-verification)
---

### SHACL Structural Parser Registry

The system shall compile SHACL RDF graphs into a reusable structural registry independent of consumer model elements, source documents, and graph-layer policies.

#### Details
The parser registry shall consume RDF terms and quads, discover SHACL shape nodes, parse targets, recursive property paths, nested shape structure, and supported constraint syntax, then expose a typed AST, raw RDF-backed constraint facts, and generic diagnostics.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * definedBy: [SHACL Structural Parser Registry Specification](OntologyKernelSpecifications.md#shacl-structural-parser-registry-specification)
  * derivedFrom: [SHACL and Ontology Algorithm Services](#shacl-and-ontology-algorithm-services)
  * specify: [Ontology Kernel Crate](OntologyKernelFeature.md#ontology-kernel-crate)
  * verifiedBy: [O-Kernel SHACL Services Unit Test Verification](../Verifications/Architecture/OntologyKernelVerifications.md#o-kernel-shacl-services-unit-test-verification)
---

### Standards Reserved Vocabulary Recognition

The system shall recognize RDF, RDFS, OWL, XSD, and SHACL reserved vocabulary IRIs by expanded IRI and semantic position.

#### Details
Reserved vocabulary recognition shall be based on expanded IRIs, not prefix-name matching. RDF, RDFS, OWL, and SHACL vocabulary terms shall be derived from compile-time bundled standards RDF/Turtle graphs owned by o-kernel. XSD datatype and facet recognition shall remain an explicit kernel datatype policy because XML Schema datatypes are language built-ins rather than a project external ontology source.

The registry shall classify reserved IRIs by semantic position, including built-in datatypes, datatype facets, annotation vocabulary, reserved classes, reserved object properties, reserved data properties, and SHACL syntax vocabulary.

The registry shall not treat arbitrary custom IRIs as reserved vocabulary merely because an IRI starts with a standard namespace.

#### Metadata
  * type: requirement
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * definedBy: [Standards Reserved Vocabulary Recognition Specification](OntologyKernelSpecifications.md#standards-reserved-vocabulary-recognition-specification)
  * derivedFrom: [SHACL and Ontology Algorithm Services](#shacl-and-ontology-algorithm-services)
  * specify: [Standards Vocabulary Support](OntologyKernelFeature.md#standards-vocabulary-support)
  * verifiedBy: [O-Kernel Reserved Vocabulary Unit Test Verification](../Verifications/Architecture/OntologyKernelVerifications.md#o-kernel-reserved-vocabulary-unit-test-verification)
---
