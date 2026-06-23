# Elements

### Semantic Model Core

As a **system engineer**, I want Reqvire to manage ontology, semantic-contract, and external vocabulary context as a core semantic model capability, so that validation, export, Explorer, MCP, and assistant workflows all depend on one authoritative semantic source layer.

#### Details
Semantic Model Core is the root capability for Reqvire-owned semantic model behavior. It owns ontology source resolution, semantic-contract vocabulary context, local and built-in external ontology source handling, used external vocabulary selection, and source provenance before any reporting or presentation surface consumes those facts.

Reports, Explorer views, MCP tools, website docs, and assistant skills consume this capability through requirements and contracts. They do not own the semantic source policy.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Relations
  * specifiedBy: [Built-In External Ontology Source Resolution](SemanticModelRequirements.md#built-in-external-ontology-source-resolution)
  * specifiedBy: [External Vocabulary Description Construction](SemanticModelRequirements.md#external-vocabulary-description-construction)
  * specifiedBy: [External Vocabulary Reference Resolution](SemanticModelRequirements.md#external-vocabulary-reference-resolution)
  * specifiedBy: [Local External Ontology Sources](SemanticModelRequirements.md#local-external-ontology-sources)
  * specifiedBy: [Namespace-Scoped Ontology Export](SemanticModelRequirements.md#namespace-scoped-ontology-export)
  * specifiedBy: [Ontology and Shapes Collection](SemanticModelRequirements.md#ontology-and-shapes-collection)
  * specifiedBy: [Ontology Term Definition Link Materialization](SemanticModelRequirements.md#ontology-term-definition-link-materialization)
  * specifiedBy: [OWL Reserved Vocabulary Recognition](SemanticModelRequirements.md#owl-reserved-vocabulary-recognition)
  * specifiedBy: [Runtime Reqvire Ontology Artifact](SemanticModelRequirements.md#runtime-reqvire-ontology-artifact)
  * specifiedBy: [Runtime Reqvire Ontology Synchronization](SemanticModelRequirements.md#runtime-reqvire-ontology-synchronization)
  * specifiedBy: [Used External Vocabulary Selection](SemanticModelRequirements.md#used-external-vocabulary-selection)
---

### External Ontology Source Management

As a **semantic model maintainer**, I want Reqvire to treat local and built-in external ontology sources consistently, so that imported vocabulary can support ontology and semantic-contract validation without becoming authored Reqvire vocabulary.

#### Details
External ontology source management covers source declaration, parsing, prefix/namespace availability, provenance, and used-subset construction for RDF vocabulary dependencies that are not authored by the Reqvire model.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Relations
  * derivedFrom: [Semantic Model Core](#semantic-model-core)
  * specifiedBy: [External Vocabulary Description Construction](SemanticModelRequirements.md#external-vocabulary-description-construction)
  * specifiedBy: [External Vocabulary Reference Resolution](SemanticModelRequirements.md#external-vocabulary-reference-resolution)
  * specifiedBy: [Local External Ontology Sources](SemanticModelRequirements.md#local-external-ontology-sources)
  * specifiedBy: [Used External Vocabulary Selection](SemanticModelRequirements.md#used-external-vocabulary-selection)
---

### Built-In External Ontology Sources

As a **system engineer**, I want Reqvire to ship selected external ontology vocabularies as built-in dependency sources, so that projects can use common conceptual vocabularies such as SKOS without vendoring source files into every model.

#### Details
Built-in external ontology sources support conceptual-layer modeling alongside structural ontology modeling. SKOS is the first built-in source because it provides `skos:Concept`, concept schemes, preferred/alternative labels, definitions, and broader/related links that let a model separate conceptual anchors from project-owned structural classes and properties.

This capability is still external-source behavior. Built-in sources are parsed RDF dependency inputs owned by Reqvire core policy, not RDF/OWL/SHACL language built-ins and not authored Reqvire ontology terms.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: medium
  * status: approved

#### Relations
  * derivedFrom: [External Ontology Source Management](#external-ontology-source-management)
  * specifiedBy: [Built-In External Ontology Source Resolution](SemanticModelRequirements.md#built-in-external-ontology-source-resolution)
---

### Runtime Reqvire Ontology Vocabulary

As a **Reqvire runtime maintainer**, I want Reqvire to ship generated runtime semantic artifacts derived from its authored ontology model, so that bootstrap and runtime semantic services can consume the Reqvire vocabulary and SHACL rules without making the authored system model a hidden runtime dependency.

#### Details
Runtime Reqvire Ontology Vocabulary covers the chicken-and-egg boundary between Reqvire's authored ontology model and the runtime code that needs the Reqvire vocabulary and SHACL rules to parse, validate, export, or expose semantic facts.

The authored source of truth remains `system-model/Ontologies`. The generated `reqvire.ttl` and `reqvire-shacl.ttl` artifacts are implementation evidence derived from that authored source and checked for reproducibility.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Relations
  * derivedFrom: [Semantic Model Core](#semantic-model-core)
  * specifiedBy: [Namespace-Scoped Ontology Export](SemanticModelRequirements.md#namespace-scoped-ontology-export)
  * specifiedBy: [Runtime Reqvire Ontology Artifact](SemanticModelRequirements.md#runtime-reqvire-ontology-artifact)
  * specifiedBy: [Runtime Reqvire Ontology Synchronization](SemanticModelRequirements.md#runtime-reqvire-ontology-synchronization)
  * specifiedBy: [Runtime Reqvire SHACL Artifact](SemanticModelRequirements.md#runtime-reqvire-shacl-artifact)
---
