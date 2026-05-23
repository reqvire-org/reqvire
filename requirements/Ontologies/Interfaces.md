# Elements

# Interfaces

### Reqvire Interface Ontology

The Reqvire interface ontology defines the vocabulary for model access interfaces and their shared boundary expectations.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:InterfaceContract a owl:Class .
reqvire:CliInterfaceContract a owl:Class ;
  rdfs:subClassOf reqvire:InterfaceContract .
reqvire:WebInterfaceContract a owl:Class ;
  rdfs:subClassOf reqvire:InterfaceContract .
reqvire:McpInterfaceContract a owl:Class ;
  rdfs:subClassOf reqvire:InterfaceContract .
reqvire:McpStructuredPayloadContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract .
reqvire:McpToolCallContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract .
reqvire:McpToolEnvelopeContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract .
reqvire:McpToolSideEffectContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract .
reqvire:McpCompatibilityVersionContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract .
reqvire:McpResourceContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract .
reqvire:McpToolSideEffectClass a owl:Class .
reqvire:McpReadOnlyToolClass a owl:Class ;
  rdfs:subClassOf reqvire:McpToolSideEffectClass .
reqvire:McpConditionalMutationToolClass a owl:Class ;
  rdfs:subClassOf reqvire:McpToolSideEffectClass .
reqvire:McpMutationToolClass a owl:Class ;
  rdfs:subClassOf reqvire:McpToolSideEffectClass .

reqvire:mcpSideEffectClassName a owl:DatatypeProperty .
reqvire:mcpSideEffectMeaning a owl:DatatypeProperty .

reqvire:mcpReadOnlySideEffectClass a reqvire:McpReadOnlyToolClass ;
  reqvire:mcpSideEffectClassName "read_only" ;
  reqvire:mcpSideEffectMeaning "Tool class for operations with no model mutation side effects." .
reqvire:mcpConditionalMutationSideEffectClass a reqvire:McpConditionalMutationToolClass ;
  reqvire:mcpSideEffectClassName "conditional_mutation" ;
  reqvire:mcpSideEffectMeaning "Tool class for operations whose side effects depend on request intent." .
reqvire:mcpMutationSideEffectClass a reqvire:McpMutationToolClass ;
  reqvire:mcpSideEffectClassName "mutation" ;
  reqvire:mcpSideEffectMeaning "Tool class for operations that can mutate model or artifact state." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---
