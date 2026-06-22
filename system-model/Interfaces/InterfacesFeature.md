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

#### Relations
  * derive: [Command-Line Interface](#command-line-interface)
  * derive: [MCP Tool Interface](#mcp-tool-interface)
  * derive: [Public Documentation Website Interface](#public-documentation-website-interface)
  * derive: [Web Explorer Interface](#web-explorer-interface)
---

### Command-Line Interface

As a **System Engineer**, I want Reqvire to expose command-line operations, so that model management, validation, reporting, and automation can be performed from scripts and local terminals.

#### Details
Command-line interface is the capability for CLI commands, arguments, workspace selection, output modes, and command error behavior.

CLI requirements define concrete command behavior. Shared interface vocabulary is bound by explicit concept references on the elements that use those terms.

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
  * derive: [MCP Semantic Query Interface](#mcp-semantic-query-interface)
  * derivedFrom: [System Model Interfaces](#system-model-interfaces)
  * specifiedBy: [MCP Interface](InterfacesRequirements.md#mcp-interface)
---

### MCP Semantic Query Interface

As an **AI Tool Integrator**, I want Reqvire MCP to run SPARQL over model-owned semantic RDF evidence, so that agents can ask precise ontology and traceability questions without exporting and reloading the graph themselves.

#### Details
MCP semantic query interface is the capability for read-only SPARQL query execution over authored ontology, SHACL, authored model facts, normalized relation-family facts, and generated ontology projection facts through MCP.

#### Metadata
  * type: capability

#### Relations
  * derive: [MCP Semantic Prefix Registry Interface](#mcp-semantic-prefix-registry-interface)
  * derivedFrom: [MCP Tool Interface](#mcp-tool-interface)
  * specifiedBy: [MCP Semantic Query Tools](MCP/Tools.md#mcp-semantic-query-tools)
---

### MCP Semantic Prefix Registry Interface

As an **AI Tool Integrator**, I want Reqvire MCP to list ontology-defined prefixes and namespaces with source element context, so that agents can construct correct SPARQL queries without guessing or rebuilding semantic stores.

#### Details
MCP semantic prefix registry interface is the capability for exposing ontology element prefix metadata, namespace IRIs, source provenance, source prose content, and reusable SPARQL prefix blocks through MCP.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [MCP Semantic Query Interface](#mcp-semantic-query-interface)
  * specifiedBy: [MCP Semantic Prefix Registry Tools](MCP/Tools.md#mcp-semantic-prefix-registry-tools)
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

### Public Documentation Website Interface

As a **Reqvire Evaluator**, I want Reqvire to publish a public documentation website, so that project concepts, workflows, interface capabilities, and AI-assistant integration guidance can be understood without running the CLI first.

#### Details
Public documentation website is the capability for the externally published `www.reqvire.org` documentation surface. It explains the same system model vocabulary, relation semantics, ontology workflow, verification model, implementation coverage, and assistant-facing workflows that Reqvire validates in the repository model.

Website requirements define page-level documentation context and link each source page to the model concepts it presents so change impact can route wording and documentation updates to concrete website implementation files.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [System Model Interfaces](#system-model-interfaces)
  * specifiedBy: [Public Documentation Website](Website/WebsiteRequirements.md#public-documentation-website)
---
