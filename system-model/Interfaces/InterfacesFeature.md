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

As a **System Engineer**, I want command-line model inspection, quality evidence, exports, and source upgrades, so that terminal workflows and CI can inspect and migrate models independently of an MCP client.

#### Details
Command-line access covers retained read/query/report commands, generated artifacts, workspace and output controls, server launch, and known deterministic source migrations. It does not expose model authoring, lint repairs, or formatting. Migration is the explicit source-rewrite exception required before an incompatible model can pass normal validation.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [System Model Interfaces](#system-model-interfaces)
  * specifiedBy: [CLI interface](InterfacesRequirements.md#cli-interface)
---

### MCP Tool Interface

As an **AI Tool Integrator**, I want Reqvire to expose model evidence and the supported authoring and maintenance operations through typed MCP tools, so that agents and other clients evolve models under controlled permissions and shared core semantics.

#### Details
MCP tool access covers validated evidence, semantic queries, authoring, maintenance, worktree isolation, and change publication. Mutation enablement governs authoring and applying maintenance changes. Formatting preview and lint inspection remain read-only operations. Normal model validation is required before protocol service starts; source migration remains in CLI.

#### Metadata
  * type: capability

#### Relations
  * derive: [MCP Change Publication](#mcp-change-publication)
  * derive: [MCP Semantic Query Interface](#mcp-semantic-query-interface)
  * derive: [MCP Worktree Model Management](#mcp-worktree-model-management)
  * derivedFrom: [System Model Interfaces](#system-model-interfaces)
  * specifiedBy: [MCP Interface](InterfacesRequirements.md#mcp-interface)
---

### MCP Change Publication

As a **System Engineer**, I want an MCP client to commit accepted model changes and publish them to a branch and pull request in the same repository, so that model work can be reviewed through the existing GitHub workflow.

#### Concept References
  * [Model Change Publication](../Thesaurus/Thesaurus.md#model-change-publication)

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [MCP Tool Interface](#mcp-tool-interface)
  * specifiedBy: [MCP Model Change Publication](MCP/Tools.md#mcp-model-change-publication)
---

### MCP Model Authoring

As a **System Engineer**, I want one controlled protocol interface for evolving authored model content and structure, so that authoring preserves model integrity, traceability, and accepted workspace state.

#### Details
Model authoring covers element content, relations, contract dependencies, terminology bindings, and model and evidence relocation. Typed MCP tools expose the shared core operations; command-line inspection and source migration have separate roles.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [MCP Tool Interface](#mcp-tool-interface)
---

### MCP Model Maintenance

As a **System Engineer**, I want controlled model formatting and automatic lint repairs through the protocol interface, so that maintenance remains reviewable and follows the same permissions and persistence guarantees as authoring.

#### Details
Maintenance includes formatting preview and application and safe lint repairs. Read-only quality inspection remains available from both interfaces. Source migration remains a command-line upgrade operation.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [MCP Tool Interface](#mcp-tool-interface)
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

### MCP Worktree Model Management

As an **AI Tool Integrator**, I want to manage independent model branches through Git worktrees behind one MCP endpoint, so that concurrent model work stays isolated without switching the original checkout.

#### Details
Mixed model and implementation workflows can adopt eligible external commits on an owned branch through checked HEAD refresh and normal validated model loading while retaining context identity.

#### Concept References
  * [MCP Worktree Context](../Thesaurus/Thesaurus.md#mcp-worktree-context)

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [MCP Tool Interface](#mcp-tool-interface)
  * specifiedBy: [MCP Worktree Model Sessions](MCP/Tools.md#mcp-worktree-model-sessions)
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

### Web Explorer Interface

As a **System Engineer**, I want Reqvire to expose a browsable Explorer interface, so that model structure, reports, diagrams, traces, resources, and ontology artifacts can be inspected in a web interface.

#### Details
Web Explorer interface is the capability for the local Explorer server, navigation, visual styling, embedded browser assets, and browser-oriented model exploration.

Web requirements define concrete Explorer views, local server behavior, browser assets, and visual rendering contracts.

#### Metadata
  * type: capability

#### Relations
  * derive: [Explorer Selection Sharing](#explorer-selection-sharing)
  * derive: [Explorer Worktree Browsing](#explorer-worktree-browsing)
  * derivedFrom: [System Model Interfaces](#system-model-interfaces)
  * specifiedBy: [Web Interface](InterfacesRequirements.md#web-interface)
---

### Explorer Directional Model Browsing

As a **System Engineer**, I want to inspect either the downstream realization or upstream context of a selected model scope, so that I can understand its trace chains without mixing the two horizons.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Web Explorer Interface](#web-explorer-interface)
---

### Explorer Selection Sharing

As a **System Engineer**, I want to share a selected model context and item across Explorer views, so that collaborators can open the same selection and browser navigation can restore it.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Web Explorer Interface](#web-explorer-interface)
---

### Explorer Worktree Browsing

As a **System Engineer**, I want to choose any local branch in Explorer and inspect its model independently in each browser tab, so that I can compare concurrent work without switching an existing Git checkout or another client's context. Worktrees provide the isolated model workspaces behind this single branch-browsing experience.

#### Details
Branch selection defaults to HEAD mode. Owned contexts follow eligible local branch-tip changes through the shared check-and-refresh operation, with an explicit Refresh control and clear retained-model provenance when adoption is blocked.

#### Concept References
  * [MCP Worktree Context](../Thesaurus/Thesaurus.md#mcp-worktree-context)

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Web Explorer Interface](#web-explorer-interface)
  * specifiedBy: [Explorer Worktree Runtime Isolation](WebExplorer/Capabilities.md#explorer-worktree-runtime-isolation)
  * specifiedBy: [Explorer Worktree Selection](WebExplorer/Capabilities.md#explorer-worktree-selection)
---
