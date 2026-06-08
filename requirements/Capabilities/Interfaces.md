# Elements

### System Model Interfaces

As a **System Engineer**, I want Reqvire to expose the system model through human and machine interfaces, so that users and tools can choose the interface appropriate to their workflow.

#### Details
System model interfaces is the capability for command-line access, generated web documentation, and MCP tool access over the same Reqvire model semantics.

Interface requirements define the concrete commands, generated views, transports, payloads, safety boundaries, and protocol behavior.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: medium
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Operation Ontology](../Ontologies/BehaviorValidationOperations.md#reqvire-operation-ontology)
  * [Reqvire Report Ontology](../Ontologies/ReportsAndQuery.md#reqvire-report-ontology)
  * [Reqvire Interface Ontology](../Ontologies/Interfaces.md#reqvire-interface-ontology)

#### Relations
  * derive: [Command-Line Interface](#command-line-interface)
  * derive: [MCP Tool Interface](#mcp-tool-interface)
  * derive: [Web Documentation Interface](#web-documentation-interface)
---

### Command-Line Interface

As a **System Engineer**, I want Reqvire to expose command-line operations, so that model management, validation, reporting, and automation can be performed from scripts and local terminals.

#### Details
Command-line interface is the capability for CLI commands, arguments, workspace selection, output modes, and command error behavior.

CLI requirements define concrete command behavior. Shared interface vocabulary is inherited from the parent capability and attached interface ontology.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [System Model Interfaces](#system-model-interfaces)
  * specifiedBy: [CLI interface](../Interfaces/Interfaces.md#cli-interface)
---

### MCP Tool Interface

As an **AI Tool Integrator**, I want Reqvire to expose model evidence and controlled mutations through MCP, so that external tools and agents can interact with the model through typed protocol operations.

#### Details
MCP tool interface is the capability for MCP transport, tool schemas, resource exposure, side-effect classification, mutation safety, and session behavior.

MCP requirements define concrete protocol behavior while reusing the same model semantics as CLI and web interfaces.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [System Model Interfaces](#system-model-interfaces)
  * specifiedBy: [MCP Interface](../Interfaces/Interfaces.md#mcp-interface)
---

### Reqvire MCP Side-Effect Ontology Shape Profile

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
  * refine: [MCP Tool Side-Effect Classification](../Interfaces/MCP/Tools.md#mcp-tool-side-effect-classification)
---

### Web Documentation Interface

As a **System Engineer**, I want Reqvire to expose browsable generated documentation, so that model structure, reports, diagrams, traces, resources, and ontology exports can be inspected in a web interface.

#### Details
Web documentation interface is the capability for HTML export, local serving, navigation, visual styling, generated pages, and browser-oriented model exploration.

Web requirements define concrete generated views, local server behavior, HTML assets, and visual rendering contracts.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [System Model Interfaces](#system-model-interfaces)
  * specifiedBy: [Web Interface](../Interfaces/Interfaces.md#web-interface)
---
