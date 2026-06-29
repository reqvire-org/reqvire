# Elements

### Application Boundary Isolation

The system shall keep consumer-specific parsing, source mapping, diagnostics, runtime graph policy, store assembly policy, and response shaping outside `o-kernel`.

#### Details
Detailed boundary, adaptation, and consumer-ownership rules shall follow the associated specification.

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
Detailed public API, dependency-direction, and consumer-exclusion rules shall follow the associated specification.

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

#### Contract Bindings
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
Detailed term selection, support/annotation policy, depth policy, construction, metadata, and consumer-boundary rules shall follow the associated specification.

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
Detailed graph inputs, dependency profile, reference extraction, support context, annotation context, RDF list closure, bounded expansion, metadata, and consumer-boundary rules shall follow the associated specification.

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
Detailed SHACL, reserved-vocabulary, RDF utility, and construct-classification service rules shall follow the associated specification.

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
The classifier shall cover standards-based ontology construct families and return consumer-neutral construct records. Detailed construct families, ordering, namespace, provenance, and consumer-boundary rules shall follow the associated specification.

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
Detailed registry inputs, ontology-index contents, SHACL reference checks, value-constraint handling, and diagnostic rules shall follow the associated specification.

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
Detailed expanded-IRI matching, bundled standards graph, datatype/facet, semantic-position, and rejection rules shall follow the associated specification.

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
