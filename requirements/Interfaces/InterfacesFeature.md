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
  * derive: [Web Explorer Interface](#web-explorer-interface)
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
  * specifiedBy: [CLI interface](InterfacesRequirements.md#cli-interface)
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
  * specifiedBy: [MCP Interface](InterfacesRequirements.md#mcp-interface)
---

### Web Explorer Interface

As a **System Engineer**, I want Reqvire to expose a browsable Explorer interface, so that model structure, reports, diagrams, traces, resources, and ontology artifacts can be inspected in a web interface.

#### Details
Web Explorer interface is the capability for the local Explorer server, navigation, visual styling, embedded browser assets, and browser-oriented model exploration.

Web requirements define concrete Explorer views, local server behavior, browser assets, and visual rendering contracts.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [System Model Interfaces](#system-model-interfaces)
  * specifiedBy: [Web Interface](InterfacesRequirements.md#web-interface)
---

