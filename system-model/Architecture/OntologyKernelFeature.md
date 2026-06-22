# Elements

### Ontology Kernel Crate

As a **semantic infrastructure maintainer**, I want a foundational o-kernel crate for RDF, OWL, and SHACL algorithms, so that ontology processing starts from an independent standards-based kernel before any product-specific model semantics are applied.

#### Details
Ontology Kernel Crate is the capability root for reusable ontology computation over RDF graph data. It defines the o-kernel contract for RDF vocabulary constants, SHACL syntax services, ontology construct classification, and SPARQL-compatible RDF processing utilities.

The kernel is intentionally independent from consumer model structures, source-document formats, runtime graph policy, source maps, diagnostics, and presentation surfaces. Consumers use the kernel contract; the kernel does not know its consumers.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * derive: [Standards Vocabulary Support](#standards-vocabulary-support)
  * specifiedBy: [Ontology Kernel Public Contract](OntologyKernelRequirements.md#ontology-kernel-public-contract)
  * specifiedBy: [Application Boundary Isolation](OntologyKernelRequirements.md#application-boundary-isolation)
  * specifiedBy: [O-Kernel Physical Module Architecture](OntologyKernelRequirements.md#o-kernel-physical-module-architecture)
  * specifiedBy: [Ontology Kernel RDF Native Boundary](OntologyKernelRequirements.md#ontology-kernel-rdf-native-boundary)
  * specifiedBy: [Referenced Graph Subset Construction](OntologyKernelRequirements.md#referenced-graph-subset-construction)
  * specifiedBy: [SHACL and Ontology Algorithm Services](OntologyKernelRequirements.md#shacl-and-ontology-algorithm-services)
---

### Standards Vocabulary Support

As a **semantic infrastructure maintainer**, I want o-kernel to provide standards vocabulary support for RDF, RDFS, OWL, XSD, and SHACL, so that consumers can validate and classify standards terms without project-local ontology source declarations.

#### Details
Standards Vocabulary Support covers the kernel-owned vocabulary layer for semantic processing. It provides namespace constants, expanded-IRI recognition, bundled standards vocabulary graphs for RDF, RDFS, OWL, and SHACL, and explicit XSD datatype/facet policy.

This capability does not model SKOS or project vocabularies. Those are external ontology sources owned by consuming systems such as Reqvire core.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * derivedFrom: [Ontology Kernel Crate](#ontology-kernel-crate)
  * specifiedBy: [Standards Reserved Vocabulary Recognition](OntologyKernelRequirements.md#standards-reserved-vocabulary-recognition)
---
