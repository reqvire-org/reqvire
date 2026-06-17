# Elements

### MCP Tool Side-Effect Shape

Defines SHACL constraints for MCP tool side-effect class vocabulary tokens.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:McpToolSideEffectClassShape
  a sh:NodeShape ;
  sh:targetClass reqvire:McpReadOnlyToolClass, reqvire:McpConditionalMutationToolClass, reqvire:McpMutationToolClass ;
  sh:property [
    sh:path reqvire:mcpSideEffectClassName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("read_only" "conditional_mutation" "mutation") ;
    sh:message "MCP side-effect classes must use a supported tool discovery side-effect token." ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [MCP Tool Side Effect Classification](../Interfaces/MCP/Tools.md#mcp-tool-side-effect-classification)
  * use: [Reqvire Interface Ontology](#reqvire-interface-ontology)
---

### Reqvire Interface Ontology

The Reqvire interface ontology defines the vocabulary for model access interfaces and their shared boundary expectations.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:InterfaceContract a owl:Class ;
  rdfs:comment "Shared vocabulary for human and machine interfaces that expose Reqvire model semantics." .
reqvire:CliInterfaceContract a owl:Class ;
  rdfs:subClassOf reqvire:InterfaceContract ;
  rdfs:comment "Interface contract vocabulary for command-line Reqvire operations." .
reqvire:WebInterfaceContract a owl:Class ;
  rdfs:subClassOf reqvire:InterfaceContract ;
  rdfs:comment "Interface contract vocabulary for generated web documentation and browser-oriented model exploration." .
reqvire:BrowserLocalProjectStore a owl:Class ;
  rdfs:subClassOf reqvire:WebInterfaceContract ;
  rdfs:comment "Browser-local generated store snapshot hosted by the Reqvire Explorer shell for all routed Explorer views." .
reqvire:ExplorerShell a owl:Class ;
  rdfs:subClassOf reqvire:WebInterfaceContract ;
  rdfs:comment "Single-page browser shell that hosts routed Reqvire Explorer views and initializes the Project Store." .
reqvire:ExplorerRoute a owl:Class ;
  rdfs:subClassOf reqvire:WebInterfaceContract ;
  rdfs:comment "Hash-route contract for navigating generated Reqvire Explorer views from index.html." .
reqvire:ElementDetailModal a owl:Class ;
  rdfs:subClassOf reqvire:WebInterfaceContract ;
  rdfs:comment "Scrollable in-shell Explorer dialog for inspecting a Project Store element record without navigating away from the active view." .
reqvire:FileContainer a owl:Class ;
  rdfs:subClassOf reqvire:WebInterfaceContract ;
  rdfs:comment "Served filesystem or source-document container used for containment, source links, file navigation, and breadcrumbs." .
reqvire:ModeledResource a owl:Class ;
  rdfs:subClassOf reqvire:WebInterfaceContract ;
  rdfs:comment "Modeled resource, implementation target, evidence file, local asset, or external URL referenced by Reqvire facts." .
reqvire:StoreProjection a owl:Class ;
  rdfs:subClassOf reqvire:BrowserLocalProjectStore ;
  rdfs:comment "View-neutral normalized store section consumed by one or more Explorer routes." .
reqvire:McpInterfaceContract a owl:Class ;
  rdfs:subClassOf reqvire:InterfaceContract ;
  rdfs:comment "Interface contract vocabulary for MCP tool, resource, payload, and safety behavior." .
reqvire:McpStructuredPayloadContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract ;
  rdfs:comment "Contract vocabulary for structured MCP request and response payloads." .
reqvire:McpToolCallContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract ;
  rdfs:comment "Contract vocabulary for MCP tool invocation semantics." .
reqvire:McpToolEnvelopeContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract ;
  rdfs:comment "Contract vocabulary for MCP tool discovery envelopes, schemas, and annotations." .
reqvire:McpToolSideEffectContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract ;
  rdfs:comment "Contract vocabulary for classifying MCP tools by mutation side-effect behavior." .
reqvire:McpCompatibilityVersionContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract ;
  rdfs:comment "Contract vocabulary for versioned MCP compatibility and Reqvire-specific protocol metadata." .
reqvire:McpResourceContract a owl:Class ;
  rdfs:subClassOf reqvire:McpInterfaceContract ;
  rdfs:comment "Contract vocabulary for MCP resource exposure." .
reqvire:McpToolSideEffectClass a owl:Class ;
  rdfs:subClassOf reqvire:McpToolSideEffectContract ;
  rdfs:comment "Controlled vocabulary class for MCP tool side-effect categories." .
reqvire:McpReadOnlyToolClass a owl:Class ;
  rdfs:subClassOf reqvire:McpToolSideEffectClass ;
  rdfs:label "read-only MCP tool class" ;
  rdfs:comment "MCP tool side-effect category for operations with no model mutation side effects." ;
  owl:disjointWith reqvire:McpConditionalMutationToolClass, reqvire:McpMutationToolClass .
reqvire:McpConditionalMutationToolClass a owl:Class ;
  rdfs:subClassOf reqvire:McpToolSideEffectClass ;
  rdfs:label "conditional-mutation MCP tool class" ;
  rdfs:comment "MCP tool side-effect category for operations whose mutation behavior depends on request intent or arguments." ;
  owl:disjointWith reqvire:McpMutationToolClass .
reqvire:McpMutationToolClass a owl:Class ;
  rdfs:subClassOf reqvire:McpToolSideEffectClass ;
  rdfs:label "mutation MCP tool class" ;
  rdfs:comment "MCP tool side-effect category for operations that can mutate model or artifact state." .

reqvire:mcpSideEffectClassName a owl:DatatypeProperty ;
  rdfs:domain reqvire:McpToolSideEffectClass ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical MCP side-effect class token consumed by interface contracts and MCP tool discovery expectations." .
reqvire:storeSectionName a owl:DatatypeProperty ;
  rdfs:domain reqvire:StoreProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical top-level Project Store section name consumed by Explorer views." .
reqvire:routePattern a owl:DatatypeProperty ;
  rdfs:domain reqvire:ExplorerRoute ;
  rdfs:range xsd:string ;
  rdfs:comment "Hash-route pattern supported by the index.html Explorer shell." .
reqvire:resourceReferenceKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:ModeledResource ;
  rdfs:range xsd:string ;
  rdfs:comment "Resource reference classification such as implementation, evidence, local-file, external-url, or asset." .

reqvire:mcpReadOnlySideEffectClass a reqvire:McpReadOnlyToolClass ;
  reqvire:mcpSideEffectClassName "read_only" ;
  rdfs:comment "Tool class for operations with no model mutation side effects." .
reqvire:mcpConditionalMutationSideEffectClass a reqvire:McpConditionalMutationToolClass ;
  reqvire:mcpSideEffectClassName "conditional_mutation" ;
  rdfs:comment "Tool class for operations whose side effects depend on request intent." .
reqvire:mcpMutationSideEffectClass a reqvire:McpMutationToolClass ;
  reqvire:mcpSideEffectClassName "mutation" ;
  rdfs:comment "Tool class for operations that can mutate model or artifact state." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

