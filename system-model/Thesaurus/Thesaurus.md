# Elements

### Reqvire Concept Scheme

Curated conceptual vocabulary for Reqvire system-model authoring, search, explanation, and ontology-to-human terminology alignment.

#### Metadata
  * type: concept-scheme
  * concept_base: https://www.reqvire.org/concepts
  * concept_prefix: concept
---
### Model Structure

Conceptual area covering Reqvire elements, hierarchy, capability structure, requirements, contracts, and authored model organization.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
---
### Semantic Model

Conceptual area covering ontology, semantic contracts, RDF export, SHACL shape context, and semantic graph services.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
---
### Verification

Conceptual area covering verification objectives, concrete verification methods, evidence, and verification coverage.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
---
### Traceability

Conceptual area covering links between intent, implementation, verification, evidence, ontology context, and change impact.

#### Labels
  * altLabel: Trace links
#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * related: [Semantic Model](#semantic-model)
  * related: [Verification](#verification)
---
### Governance

Conceptual area covering ownership, status, priority, risk, review readiness, and model stewardship metadata.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
---
### Integration

Conceptual area covering interfaces, source markers, external systems, MCP, CLI, Explorer, and other integration surfaces.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
---
### Element

Addressable Reqvire model item with metadata, prose, sections, and optional relations.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Model Structure](#model-structure)
---
### Capability

Coherent ability the system can provide or support.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Model Structure](#model-structure)
---
### Requirement

Obligation the system must satisfy.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Model Structure](#model-structure)
  * related: [Verification Coverage](#verification-coverage)
---
### Requirement Contract

Precise terms that define a requirement, including source basis, specifications, constraints, behaviors, states, and input/output details.

#### Labels
  * altLabel: Contract
#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Model Structure](#model-structure)
  * related: [Semantic Contract](#semantic-contract)
---
### Ontology Element

Reqvire model element that authors reusable RDF/OWL vocabulary.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
---
### Semantic Contract

Reusable SHACL profile that uses ontology vocabulary and constrains requirements.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
  * related: [Ontology Projection](#ontology-projection)
---
### Concept Reference

Human-readable binding from non-ontology model prose to curated SKOS concepts.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
  * related: [Traceability](#traceability)
  * related: [Ontology Projection](#ontology-projection)
---
### Runtime Ontology Artifact

Generated Reqvire ontology snapshot embedded by runtime code as bootstrap vocabulary.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
  * related: [Ontology Source Of Truth](#ontology-source-of-truth)
  * related: [Semantic Export](#semantic-export)
---
### Namespace Scoped Ontology Export

Clean semantic export filtered to one ontology base or term namespace.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
  * related: [Semantic Export](#semantic-export)
---
### Runtime Ontology Namespace

Runtime vocabulary namespace selected for Reqvire bootstrap ontology generation.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
---
### Ontology Source Of Truth

Authored ontology model that governs generated semantic and runtime ontology artifacts.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
---
### Ontology Document

Ontology document identity that groups contributed semantic terms under an ontology base IRI.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
---
### External Ontology Source

Local or built-in ontology dependency used for term resolution and validation without becoming authored Reqvire vocabulary.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
  * related: [Semantic Export](#semantic-export)
  * related: [Ontology Projection](#ontology-projection)
---
### Built In External Ontology Source

Reqvire-shipped external ontology dependency available without project-local source declaration.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [External Ontology Source](#external-ontology-source)
---
### Used External Ontology Subset

Materialized subset of an external ontology source containing terms and support facts used by authored semantic content.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [External Ontology Source](#external-ontology-source)
---
### Raw External Ontology Graph

Parsed dependency graph loaded from an external ontology source before used-subset construction.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [External Ontology Source](#external-ontology-source)
---
### External Ontology Subset Construct Query

Construct-query pattern that selects and describes the used subset of an external ontology dependency graph.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Ontology Projection](#ontology-projection)
---
### External Ontology Prefix

Prefix label assigned to an external ontology source namespace.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [External Ontology Source](#external-ontology-source)
---
### External Ontology Namespace

Namespace IRI provided by an external ontology source.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [External Ontology Source](#external-ontology-source)
---
### External Ontology Resource

Ontology document IRI or provenance resource associated with an external ontology source.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [External Ontology Source](#external-ontology-source)
---
### External Ontology Source Path

Repository-local path to an external ontology source file.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [External Ontology Source](#external-ontology-source)
---
### External Ontology Format

RDF serialization format used by an external ontology source.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [External Ontology Source](#external-ontology-source)
---
### Semantic Export

RDF, JSON-LD, or graph-oriented output generated from authored ontology, SHACL, and optional model context.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
  * related: [Ontology Projection](#ontology-projection)
  * related: [Runtime Ontology Artifact](#runtime-ontology-artifact)
---
### Ontology Projection

Generated semantic facts that classify authored OWL/RDFS/SHACL constructs for graph exploration and tooling.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
  * related: [Relation Family Construct Query](#relation-family-construct-query)
---
### Owl Reserved Vocabulary Registry

Kernel registry of RDF, RDFS, OWL, XSD, and SHACL reserved vocabulary recognized without project-local external sources.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
---
### Owl Reserved Vocabulary Term

Reserved vocabulary IRI recognized by the semantic kernel in valid vocabulary positions.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Owl Reserved Vocabulary Registry](#owl-reserved-vocabulary-registry)
---
### Owl Built In Datatype

Reserved datatype IRI accepted by ontology and SHACL validation without project-local declaration.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Owl Reserved Vocabulary Term](#owl-reserved-vocabulary-term)
---
### Report

Generated view or output that presents selected model structure, semantic context, coverage, traceability, resources, or validation evidence.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
---
### Query

Structured question over model, ontology, or semantic graph context.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Semantic Model](#semantic-model)
---
### Submodel

Scoped model region rooted in capability, ontology, or another supported model boundary.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Model Structure](#model-structure)
---
### Interface

Human or machine surface that exposes Reqvire model semantics or operations.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Integration](#integration)
---
### Command Line Interface

Terminal-oriented interface for invoking Reqvire operations and reports.

#### Labels
  * altLabel: CLI
#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Interface](#interface)
---
### Model Context Protocol Interface

MCP tool, resource, prompt, and payload interface used by AI assistants to access Reqvire model context.

#### Labels
  * altLabel: MCP interface
#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Interface](#interface)
---
### Explorer Interface

Browser-oriented interface for navigating Reqvire model structure, ontology graphs, coverage, traces, resources, and details.

#### Labels
  * altLabel: Web Explorer
#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Interface](#interface)
---
### Structured Payload

Machine-readable request or response data exchanged through a Reqvire interface.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Interface](#interface)
---
### Verification Objective

Planning concept that groups verification intent before concrete verification evidence is authored.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Verification](#verification)
---
### Concrete Verification

Executable, inspectable, demonstrable, analytical, or formal verification method.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Verification](#verification)
---
### Verification Coverage

Computed state describing whether requirement obligations are verified and capability coverage is sufficient.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Verification](#verification)
  * related: [Change Impact](#change-impact)
  * related: [Concrete Verification](#concrete-verification)
---
### Relation Model

Conceptual model for authored and normalized relation families connecting Reqvire elements.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Traceability](#traceability)
  * related: [Change Impact](#change-impact)
---
### Relation Family

Stable semantic grouping of authored relation names and inverse pairs.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Relation Model](#relation-model)
---
### Relation Predicate Layer

Conceptual distinction between source-authored relation predicates and generated normalized relation predicates.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Relation Model](#relation-model)
---
### Authored Relation Predicate

Source relation predicate that preserves the relation token authored in Reqvire Markdown.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Relation Predicate Layer](#relation-predicate-layer)
---
### Normalized Relation Predicate

Generated relation-family predicate intended for semantic traversal, graph projection, rollups, reports, and query APIs.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Relation Predicate Layer](#relation-predicate-layer)
  * related: [Relation Family Construct Query](#relation-family-construct-query)
---
### Relation Rule

Controlled rule for relation name, endpoint compatibility, direction, ownership, and impact behavior.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Relation Model](#relation-model)
---
### Relation Semantic Pattern

Semantic pattern describing relation behavior such as hierarchy, bridge, ownership, dependency, verification, or satisfaction.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Relation Model](#relation-model)
---
### Model Relation

Projection record for one authored Reqvire relation edge.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Relation Model](#relation-model)
---
### Relation Family Construct Query

Construct-query pattern that materializes normalized relation-family facts for semantic search.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Relation Model](#relation-model)
  * related: [Semantic Export](#semantic-export)
---
### Change Impact

Analysis of model elements, artifacts, verifications, and semantic context affected by a change.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Reqvire Concept Scheme](#reqvire-concept-scheme)
  * broader: [Traceability](#traceability)
  * related: [Semantic Export](#semantic-export)
  * related: [Verification Coverage](#verification-coverage)
---
