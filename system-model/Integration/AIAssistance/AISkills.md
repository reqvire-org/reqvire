# Elements

### AI Assistant Skill-Guided Reqvire Modeling

The system shall define a dedicated AI-assistant skills submodel that provides instruction contracts for Reqvire modeling workflows and traceable implementation artifacts.

#### Metadata
  * type: requirement

#### Contract References
  * [Contract Specification](../../ModelStructure/Specifications.md#contract-specification)
  * [Relation Semantics Specification](../../ModelStructure/Specifications.md#relation-semantics-specification)
  * [Supported Element Types Specification](../../ModelStructure/Specifications.md#supported-element-types-specification)
  * [Traceability Reporting Specification](../../Reports/ModelReports/Specifications.md#traceability-reporting-specification)

#### Relations
  * derive: [AI Skill Installer Distribution](#ai-skill-installer-distribution)
  * derive: [AI Skills Instruction Contracts](#ai-skills-instruction-contracts)
  * derive: [AI Skills Markdown Implementation Artifacts](#ai-skills-markdown-implementation-artifacts)
  * specify: [AI-Assisted System Model Management](../IntegrationFeature.md#ai-assisted-system-model-management)
---

### AI Skill Installer Distribution

The system SHALL distribute a single assistant-neutral skill collection installable from the repository's main branch without requiring a user-managed clone.

#### Details
Distribution and installation follow the associated specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [AI Skill Installer Distribution Specification](#ai-skill-installer-distribution-specification)
  * derivedFrom: [AI Assistant Skill-Guided Reqvire Modeling](#ai-assistant-skill-guided-reqvire-modeling)
  * satisfiedBy: [README.md](../../../README.md)
  * satisfiedBy: [SKILLS.md](../../../doc/SKILLS.md)
  * satisfiedBy: [SKILL.md](../../../skills/reqvire-audit/SKILL.md)
  * satisfiedBy: [AnalyzeCoverage.md](../../../skills/reqvire-audit/references/AnalyzeCoverage.md)
  * satisfiedBy: [AnalyzeModel.md](../../../skills/reqvire-audit/references/AnalyzeModel.md)
  * satisfiedBy: [ChangeImpact.md](../../../skills/reqvire-audit/references/ChangeImpact.md)
  * satisfiedBy: [Lint.md](../../../skills/reqvire-audit/references/Lint.md)
  * satisfiedBy: [SKILL.md](../../../skills/reqvire-concept-authoring/SKILL.md)
  * satisfiedBy: [openai.yaml](../../../skills/reqvire-concept-authoring/agents/openai.yaml)
  * satisfiedBy: [ConceptAuthoring.md](../../../skills/reqvire-concept-authoring/references/ConceptAuthoring.md)
  * satisfiedBy: [SKILL.md](../../../skills/reqvire-ontology-authoring/SKILL.md)
  * satisfiedBy: [openai.yaml](../../../skills/reqvire-ontology-authoring/agents/openai.yaml)
  * satisfiedBy: [OntologyAuthoring.md](../../../skills/reqvire-ontology-authoring/references/OntologyAuthoring.md)
  * satisfiedBy: [OntologyRefactoring.md](../../../skills/reqvire-ontology-authoring/references/OntologyRefactoring.md)
  * satisfiedBy: [SKILL.md](../../../skills/reqvire-syseng/SKILL.md)
  * satisfiedBy: [AddCapability.md](../../../skills/reqvire-syseng/references/AddCapability.md)
  * satisfiedBy: [AddRequirement.md](../../../skills/reqvire-syseng/references/AddRequirement.md)
  * satisfiedBy: [AddVerification.md](../../../skills/reqvire-syseng/references/AddVerification.md)
  * satisfiedBy: [CapabilitySemanticContractRefactor.md](../../../skills/reqvire-syseng/references/CapabilitySemanticContractRefactor.md)
  * satisfiedBy: [Collect.md](../../../skills/reqvire-syseng/references/Collect.md)
  * satisfiedBy: [ConsolidateRequirements.md](../../../skills/reqvire-syseng/references/ConsolidateRequirements.md)
  * satisfiedBy: [Containment.md](../../../skills/reqvire-syseng/references/Containment.md)
  * satisfiedBy: [ContainmentStructureRefactor.md](../../../skills/reqvire-syseng/references/ContainmentStructureRefactor.md)
  * satisfiedBy: [CreatingTasks.md](../../../skills/reqvire-syseng/references/CreatingTasks.md)
  * satisfiedBy: [DesignDocOwnership.md](../../../skills/reqvire-syseng/references/DesignDocOwnership.md)
  * satisfiedBy: [Link.md](../../../skills/reqvire-syseng/references/Link.md)
  * satisfiedBy: [Move.md](../../../skills/reqvire-syseng/references/Move.md)
  * satisfiedBy: [Remove.md](../../../skills/reqvire-syseng/references/Remove.md)
  * satisfiedBy: [RenameElement.md](../../../skills/reqvire-syseng/references/RenameElement.md)
  * satisfiedBy: [Setup.md](../../../skills/reqvire-syseng/references/Setup.md)
  * satisfiedBy: [SpecificationLanguageCleanup.md](../../../skills/reqvire-syseng/references/SpecificationLanguageCleanup.md)
  * satisfiedBy: [SpecificationsExtractionLogic.md](../../../skills/reqvire-syseng/references/SpecificationsExtractionLogic.md)
  * satisfiedBy: [SubmodelRefactor.md](../../../skills/reqvire-syseng/references/SubmodelRefactor.md)
  * satisfiedBy: [VerificationAlignment.md](../../../skills/reqvire-syseng/references/VerificationAlignment.md)
  * satisfiedBy: [explore.md](../../../skills/reqvire-syseng/references/explore.md)
  * satisfiedBy: [CodingAssistants.tsx](../../../website/src/pages/CodingAssistants.tsx)
  * verifiedBy: [AI Skill Installer Manifest Verification](../../Verifications/Integration/AIAssistance/AISkillVerifications.md#ai-skill-installer-manifest-verification)
---

### AI Skills Instruction Contracts

The system shall define AI skill instruction contracts that describe MBSE-first Reqvire workflows, including boundary clarification and verification-aligned change sequencing.

#### Details
Instruction contract details shall follow the associated contract specification.

#### Metadata
  * type: requirement

#### Contract References
  * [Focused Element Editing Behavior](../../Operations/ModelOperations/Behaviors.md#focused-element-editing-behavior)
  * [MCP Update Element Tool Specification](../../Interfaces/MCP/Specifications.md#mcp-update-element-tool-specification)
  * [Create Element Override Behavior](../../Operations/ModelOperations/Behaviors.md#create-element-override-behavior)
  * [MCP Server Command Specification](../../Interfaces/MCP/Specifications.md#mcp-server-command-specification)
  * [MCP Stdio Process Lifecycle Specification](../../Interfaces/MCP/Specifications.md#mcp-stdio-process-lifecycle-specification)
  * [MCP Stdio Transport Specification](../../Interfaces/MCP/Specifications.md#mcp-stdio-transport-specification)
  * [MCP Formatting Tool Specification](../../Interfaces/MCP/Specifications.md#mcp-formatting-tool-specification)
  * [MCP Lint Fix Tool Specification](../../Interfaces/MCP/Specifications.md#mcp-lint-fix-tool-specification)
  * [Model Interface Operation Boundary Specification](../../Interfaces/Specifications.md#model-interface-operation-boundary-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)

#### Relations
  * definedBy: [AI Skills Instruction Contract Specification](#ai-skills-instruction-contract-specification)
  * derivedFrom: [AI Assistant Skill-Guided Reqvire Modeling](#ai-assistant-skill-guided-reqvire-modeling)
---

### AI Skills Markdown Implementation Artifacts

The system shall produce traceable markdown skill artifacts that implement the AI assistant skill guidance contracts for supported assistant ecosystems.

#### Details
Artifact contract details shall follow the associated contract specification.

#### Metadata
  * type: requirement

#### Contract References
  * [Focused Element Editing Behavior](../../Operations/ModelOperations/Behaviors.md#focused-element-editing-behavior)
  * [MCP Update Element Tool Specification](../../Interfaces/MCP/Specifications.md#mcp-update-element-tool-specification)
  * [Create Element Override Behavior](../../Operations/ModelOperations/Behaviors.md#create-element-override-behavior)
  * [MCP Server Command Specification](../../Interfaces/MCP/Specifications.md#mcp-server-command-specification)
  * [MCP Stdio Process Lifecycle Specification](../../Interfaces/MCP/Specifications.md#mcp-stdio-process-lifecycle-specification)
  * [MCP Stdio Transport Specification](../../Interfaces/MCP/Specifications.md#mcp-stdio-transport-specification)
  * [MCP Formatting Tool Specification](../../Interfaces/MCP/Specifications.md#mcp-formatting-tool-specification)
  * [MCP Lint Fix Tool Specification](../../Interfaces/MCP/Specifications.md#mcp-lint-fix-tool-specification)
  * [MCP Prompt Guidance Specification](../../Interfaces/MCP/Specifications.md#mcp-prompt-guidance-specification)
  * [Model Interface Operation Boundary Specification](../../Interfaces/Specifications.md#model-interface-operation-boundary-specification)
  * [Contract Reference Mutation Specification](../../ModelStructure/Specifications.md#contract-reference-mutation-specification)
  * [Contract Reference Semantics Specification](../../ModelStructure/Specifications.md#contract-reference-semantics-specification)
  * [Canonical Model Mutation Persistence Specification](../../Operations/ModelOperations/Specifications.md#canonical-model-mutation-persistence-specification)

#### Relations
  * definedBy: [AI Skills Markdown Artifact Specification](#ai-skills-markdown-artifact-specification)
  * derivedFrom: [AI Assistant Skill-Guided Reqvire Modeling](#ai-assistant-skill-guided-reqvire-modeling)
  * verifiedBy: [AI Skill CLI Command Reference Verification](../../Verifications/Integration/AIAssistance/AISkillVerifications.md#ai-skill-cli-command-reference-verification)
---

### AI Skill Installer Distribution Specification

AI skill distribution uses the standard skills CLI to install the canonical repository package.

#### Details
- Maintain exactly four shared skill packages under the repository-root `skills/` directory: `reqvire-syseng`, `reqvire-audit`, `reqvire-ontology-authoring`, and `reqvire-concept-authoring`.
- Each package contains `SKILL.md` with the matching stable skill name, its complete supporting references, and any applicable assistant metadata. The same package serves Claude Code and Codex.
- README and website installation examples explicitly select `https://github.com/reqvire-org/reqvire/tree/main/skills` with `npx skills add`; show agent selection for `codex` and `claude-code`, optional `--global` installation, and `--skill` selection.
- Project installation is the default. The skills CLI owns target-agent discovery, installation scope, and copying or symlinking; do not claim that custom Reqvire home-directory settings control that CLI.
- Use the same source URL to reinstall or update skills from main. Local checkout installation may use `npx skills add ./skills`.
- Deprecate the Claude marketplace plugin and discontinue marketplace packaging, duplicate assistant source trees, and custom curl-based installer scripts. Existing marketplace users receive guidance to disable or uninstall the old plugin and install the shared skills.
- Installation documentation distinguishes reusable skills from the Reqvire executable and MCP server. Installing skills alone does not start an MCP server or enable mutations.

#### Metadata
  * type: specification

#### Relations
  * define: [AI Skill Installer Distribution](#ai-skill-installer-distribution)
---

### AI Skills Instruction Contract Specification

AI skill instruction contracts define MBSE-first Reqvire modeling workflows, boundary checkpoints, and verification-aligned change sequencing.

#### Details
Instruction contract rules:
- Skill workflows start from capability and requirement context.
- Skill workflows separate semantic contracts from obligations.
- Skill workflows align verifications before implementation changes.
- Skill workflows run validation and lint after meaningful model updates.
- Cross-submodel refactors require explicit human confirmation before changing boundaries.
- Verification guidance distinguishes verification coverage from implementation coverage.
- Guidance MUST use contract dependency as the umbrella term for Contract Bindings and Contract References, retaining the specific names for their distinct authoring sections and semantics.
- Guidance MUST distinguish contract ownership through `definedBy`, shared implementation obligations through Contract Bindings, and content dependencies through Contract References. A service, report, view, or document consuming another requirement's contract MUST be assessed by its responsibility for fulfillment rather than by dependency alone.
- Guidance MUST place bindings on the requirements responsible for the obligation, preserve the mutual exclusion of binding and reference sections on one requirement, and preserve acyclic dependencies including mixed reference, binding, and ancestry paths.
- Model-refactor examples include correct and incorrect examples where that improves assistant behavior.

Interface guidance follows the owning requirement's boundary and maintenance references. Authoring/refactoring workflows use typed MCP tools with mutation enablement, including their previews; inspection, validation and exports may use retained CLI commands. Formatting is MCP-only, lint repair uses `reqvire.lint` with `fix: true`, and ordinary successful writes already normalize affected documents. Known migrations remain CLI preview/application before validated MCP startup. Do not prescribe removed CLI authoring commands, CLI formatting or CLI lint repair.

#### Metadata
  * type: specification

#### Relations
  * define: [AI Skills Instruction Contracts](#ai-skills-instruction-contracts)
---

### AI Skills Markdown Artifact Specification

AI skills markdown artifacts define the assistant-facing files that implement the Reqvire system engineering guidance.

#### Details
Artifact contract rules:
- All assistant-neutral skill artifacts live under the repository-root `skills/` directory, with package directories matching their stable `reqvire-*` skill names.
- Supporting references use the shared `references/` layout. Local and cross-skill links resolve within the delivered collection. Assistant-specific metadata may accompany a package without creating a second guidance tree.
- Skill invocation guidance uses the shared skill names rather than retired marketplace namespaces or slash commands.
- Authoring, refactoring, collection, search, and audit references MUST describe both contract dependency kinds consistently. Reference examples MUST use requirement-owned contract elements, retain change-impact visibility, and exclude references from implementation fulfillment and evidence.
- MCP mutation examples MUST cover `relation_type: "referenceContract"`, removal through the typed unlink tool, and rejected mixed-section edits. Search examples MUST use the dedicated binding/reference filters with normalized target identifier globs.
- CLI command references shall match supported report output modes. JSON-only `model`, `containment`, `resources`, and `traces` commands emit JSON to stdout without `--json` and accept `--output <file>` directly. Shared flag guidance shall distinguish these commands from commands whose file output requires `--json`.
- Assistant artifact changes should preserve MBSE workflow guidance and verification-aligned change sequencing.
- Ontology-authoring guidance should distinguish generic labels/descriptions from ontology slots: `rdfs:label` and `rdfs:comment` are appropriate for optional presentation metadata, while true domain concepts, canonical authored tokens, parser fields, interface enum values, report kinds, controlled-vocabulary payloads, and queryable domain meanings remain declared ontology properties even when their local names end with `Name` or `Meaning`.
- Ontology-authoring guidance should use canonical Reqvire ontology identity: the top parent ontology element in an ontology subgraph defines `ontology_base` and `ontology_prefix` metadata, authored Turtle uses the corresponding hash namespace for terms such as classes/properties/individuals, and export emits one generated `owl:Ontology` document declaration per resolved `ontology_base`. Guidance should state that child ontology elements with the same inherited base contribute vocabulary to that same ontology document, that cross-base ontology hierarchy can become `owl:imports`, that authored Turtle using the inherited prefix must explicitly declare it to `<ontology_base>#`, and that missing or conflicting declarations are invalid.
- Ontology-authoring guidance should preserve the OWL/SHACL block split: OWL `#### Ontology` blocks declare `owl:DatatypeProperty` and `owl:ObjectProperty` terms with stable `rdfs:domain`/`rdfs:range` values, including XSD ranges for datatype properties; SHACL `#### Shapes` blocks declare `sh:NodeShape` profiles using `sh:targetClass` and `sh:path` over reachable ontology terms and carry operational validation facets such as closed-world cardinality, numeric bounds, patterns, enumerations, and messages.
- Ontology-authoring skills should keep existing-ontology cleanup guidance in a separate refactoring reference document. The main skill should load that reference only when the task is to refactor, audit, clean up, or improve existing ontology content, not for greenfield ontology creation.

All shared skills, installed copies and reference pages MUST use the supported interface boundary. CRUD examples use typed MCP tool names and arguments rather than removed CLI verbs; no shell/CLI wrapper substitutes for an MCP authoring tool. Preserve read/report CLI examples where supported, including their actual output controls. Show MCP formatting preview/application, gated lint repair, automatic canonical writes and the CLI migration exception. Installed copies and canonical package guidance must agree with the released boundary; updating this contract alone does not update those artifacts.

#### Metadata
  * type: specification

#### Relations
  * define: [AI Skills Markdown Implementation Artifacts](#ai-skills-markdown-implementation-artifacts)
---
