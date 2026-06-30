# Elements

### AI Assistant Skill-Guided Reqvire Modeling

The system shall define a dedicated AI-assistant skills submodel that provides instruction contracts for Reqvire modeling workflows and traceable implementation artifacts.

#### Metadata
  * type: requirement

#### Contract Bindings
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

The system shall provide remote, no-clone skill installers whose downloadable manifests stay synchronized with the supported Claude and Codex skill artifact files.

#### Details
Remote installer distribution details shall follow the associated contract specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [AI Skill Installer Distribution Specification](#ai-skill-installer-distribution-specification)
  * derivedFrom: [AI Assistant Skill-Guided Reqvire Modeling](#ai-assistant-skill-guided-reqvire-modeling)
  * satisfiedBy: [README.md](../../../README.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/audit/SKILL.md)
  * satisfiedBy: [AnalyzeCoverage.md](../../../claude-plugins/skills/audit/reference/AnalyzeCoverage.md)
  * satisfiedBy: [AnalyzeModel.md](../../../claude-plugins/skills/audit/reference/AnalyzeModel.md)
  * satisfiedBy: [ChangeImpact.md](../../../claude-plugins/skills/audit/reference/ChangeImpact.md)
  * satisfiedBy: [Lint.md](../../../claude-plugins/skills/audit/reference/Lint.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/concept-authoring/SKILL.md)
  * satisfiedBy: [ConceptAuthoring.md](../../../claude-plugins/skills/concept-authoring/reference/ConceptAuthoring.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/ontology-authoring/SKILL.md)
  * satisfiedBy: [OntologyAuthoring.md](../../../claude-plugins/skills/ontology-authoring/reference/OntologyAuthoring.md)
  * satisfiedBy: [OntologyRefactoring.md](../../../claude-plugins/skills/ontology-authoring/reference/OntologyRefactoring.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/syseng/SKILL.md)
  * satisfiedBy: [AddCapability.md](../../../claude-plugins/skills/syseng/reference/AddCapability.md)
  * satisfiedBy: [AddRequirement.md](../../../claude-plugins/skills/syseng/reference/AddRequirement.md)
  * satisfiedBy: [AddVerification.md](../../../claude-plugins/skills/syseng/reference/AddVerification.md)
  * satisfiedBy: [CapabilitySemanticContractRefactor.md](../../../claude-plugins/skills/syseng/reference/CapabilitySemanticContractRefactor.md)
  * satisfiedBy: [Collect.md](../../../claude-plugins/skills/syseng/reference/Collect.md)
  * satisfiedBy: [ConsolidateRequirements.md](../../../claude-plugins/skills/syseng/reference/ConsolidateRequirements.md)
  * satisfiedBy: [Containment.md](../../../claude-plugins/skills/syseng/reference/Containment.md)
  * satisfiedBy: [ContainmentStructureRefactor.md](../../../claude-plugins/skills/syseng/reference/ContainmentStructureRefactor.md)
  * satisfiedBy: [CreatingTasks.md](../../../claude-plugins/skills/syseng/reference/CreatingTasks.md)
  * satisfiedBy: [DesignDocOwnership.md](../../../claude-plugins/skills/syseng/reference/DesignDocOwnership.md)
  * satisfiedBy: [Link.md](../../../claude-plugins/skills/syseng/reference/Link.md)
  * satisfiedBy: [Move.md](../../../claude-plugins/skills/syseng/reference/Move.md)
  * satisfiedBy: [Remove.md](../../../claude-plugins/skills/syseng/reference/Remove.md)
  * satisfiedBy: [RenameElement.md](../../../claude-plugins/skills/syseng/reference/RenameElement.md)
  * satisfiedBy: [Setup.md](../../../claude-plugins/skills/syseng/reference/Setup.md)
  * satisfiedBy: [SpecificationLanguageCleanup.md](../../../claude-plugins/skills/syseng/reference/SpecificationLanguageCleanup.md)
  * satisfiedBy: [SpecificationsExtractionLogic.md](../../../claude-plugins/skills/syseng/reference/SpecificationsExtractionLogic.md)
  * satisfiedBy: [SubmodelRefactor.md](../../../claude-plugins/skills/syseng/reference/SubmodelRefactor.md)
  * satisfiedBy: [VerificationAlignment.md](../../../claude-plugins/skills/syseng/reference/VerificationAlignment.md)
  * satisfiedBy: [explore.md](../../../claude-plugins/skills/syseng/reference/explore.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-audit/SKILL.md)
  * satisfiedBy: [AnalyzeCoverage.md](../../../codex-skills/reqvire-audit/references/AnalyzeCoverage.md)
  * satisfiedBy: [AnalyzeModel.md](../../../codex-skills/reqvire-audit/references/AnalyzeModel.md)
  * satisfiedBy: [ChangeImpact.md](../../../codex-skills/reqvire-audit/references/ChangeImpact.md)
  * satisfiedBy: [Lint.md](../../../codex-skills/reqvire-audit/references/Lint.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-concept-authoring/SKILL.md)
  * satisfiedBy: [openai.yaml](../../../codex-skills/reqvire-concept-authoring/agents/openai.yaml)
  * satisfiedBy: [ConceptAuthoring.md](../../../codex-skills/reqvire-concept-authoring/references/ConceptAuthoring.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-ontology-authoring/SKILL.md)
  * satisfiedBy: [openai.yaml](../../../codex-skills/reqvire-ontology-authoring/agents/openai.yaml)
  * satisfiedBy: [OntologyAuthoring.md](../../../codex-skills/reqvire-ontology-authoring/references/OntologyAuthoring.md)
  * satisfiedBy: [OntologyRefactoring.md](../../../codex-skills/reqvire-ontology-authoring/references/OntologyRefactoring.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-syseng/SKILL.md)
  * satisfiedBy: [AddCapability.md](../../../codex-skills/reqvire-syseng/references/AddCapability.md)
  * satisfiedBy: [AddRequirement.md](../../../codex-skills/reqvire-syseng/references/AddRequirement.md)
  * satisfiedBy: [AddVerification.md](../../../codex-skills/reqvire-syseng/references/AddVerification.md)
  * satisfiedBy: [CapabilitySemanticContractRefactor.md](../../../codex-skills/reqvire-syseng/references/CapabilitySemanticContractRefactor.md)
  * satisfiedBy: [Collect.md](../../../codex-skills/reqvire-syseng/references/Collect.md)
  * satisfiedBy: [ConsolidateRequirements.md](../../../codex-skills/reqvire-syseng/references/ConsolidateRequirements.md)
  * satisfiedBy: [Containment.md](../../../codex-skills/reqvire-syseng/references/Containment.md)
  * satisfiedBy: [ContainmentStructureRefactor.md](../../../codex-skills/reqvire-syseng/references/ContainmentStructureRefactor.md)
  * satisfiedBy: [CreatingTasks.md](../../../codex-skills/reqvire-syseng/references/CreatingTasks.md)
  * satisfiedBy: [DesignDocOwnership.md](../../../codex-skills/reqvire-syseng/references/DesignDocOwnership.md)
  * satisfiedBy: [Link.md](../../../codex-skills/reqvire-syseng/references/Link.md)
  * satisfiedBy: [Move.md](../../../codex-skills/reqvire-syseng/references/Move.md)
  * satisfiedBy: [Remove.md](../../../codex-skills/reqvire-syseng/references/Remove.md)
  * satisfiedBy: [RenameElement.md](../../../codex-skills/reqvire-syseng/references/RenameElement.md)
  * satisfiedBy: [Setup.md](../../../codex-skills/reqvire-syseng/references/Setup.md)
  * satisfiedBy: [SpecificationLanguageCleanup.md](../../../codex-skills/reqvire-syseng/references/SpecificationLanguageCleanup.md)
  * satisfiedBy: [SpecificationsExtractionLogic.md](../../../codex-skills/reqvire-syseng/references/SpecificationsExtractionLogic.md)
  * satisfiedBy: [SubmodelRefactor.md](../../../codex-skills/reqvire-syseng/references/SubmodelRefactor.md)
  * satisfiedBy: [VerificationAlignment.md](../../../codex-skills/reqvire-syseng/references/VerificationAlignment.md)
  * satisfiedBy: [explore.md](../../../codex-skills/reqvire-syseng/references/explore.md)
  * satisfiedBy: [CODEX_SKILLS.md](../../../doc/CODEX_SKILLS.md)
  * satisfiedBy: [install-claude-skill.sh](../../../scripts/install-claude-skill.sh)
  * satisfiedBy: [install-codex-skill.sh](../../../scripts/install-codex-skill.sh)
  * satisfiedBy: [CodingAssistants.tsx](../../../website/src/pages/CodingAssistants.tsx)
  * verifiedBy: [AI Skill Installer Manifest Verification](../../Verifications/Integration/AIAssistance/AISkillVerifications.md#ai-skill-installer-manifest-verification)
---

### AI Skills Instruction Contracts

The system shall define AI skill instruction contracts that describe MBSE-first Reqvire workflows, including boundary clarification and verification-aligned change sequencing.

#### Details
Instruction contract details shall follow the associated contract specification.

#### Metadata
  * type: requirement

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

#### Contract Bindings
  * [MCP Prompt Guidance Specification](../../Interfaces/MCP/Specifications.md#mcp-prompt-guidance-specification)

#### Relations
  * definedBy: [AI Skills Markdown Artifact Specification](#ai-skills-markdown-artifact-specification)
  * derivedFrom: [AI Assistant Skill-Guided Reqvire Modeling](#ai-assistant-skill-guided-reqvire-modeling)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/audit/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/concept-authoring/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/ontology-authoring/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/syseng/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-audit/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-concept-authoring/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-ontology-authoring/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-syseng/SKILL.md)
---

### AI Skill Installer Distribution Specification

AI skill installer distribution defines the supported no-clone installer behavior for assistant skill packages.

#### Details
Installer distribution rules:
- Claude Code marketplace installation remains the default Claude Code installation path.
- The Claude direct installer shall install the checked-in Claude skill package into `$CLAUDE_HOME/skills`, defaulting to `~/.claude/skills` when `CLAUDE_HOME` is unset.
- The Codex direct installer shall install the checked-in Codex skill package into `$CODEX_HOME/skills`, defaulting to `~/.codex/skills` when `CODEX_HOME` is unset.
- Direct installers shall support local checkout execution by copying local skill directories.
- Direct installers shall support remote `curl ... | bash` execution by downloading from `REQVIRE_REPO_RAW`, defaulting to the Reqvire GitHub raw URL.
- Remote installers shall stage downloads in a temporary directory and replace a target skill directory only after that skill has been fully staged.
- Installer manifests shall list every file that belongs to the corresponding remote skill package, including reference documents and assistant-specific metadata files.
- Any addition, removal, or rename under `claude-plugins/skills` or `codex-skills` shall be treated as a change-impact trigger for the direct installer manifests.

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
- Model-refactor examples include correct and incorrect examples where that improves assistant behavior.

#### Metadata
  * type: specification

#### Relations
  * define: [AI Skills Instruction Contracts](#ai-skills-instruction-contracts)
---

### AI Skills Markdown Artifact Specification

AI skills markdown artifacts define the assistant-facing files that implement the Reqvire system engineering guidance.

#### Details
Artifact contract rules:
- Codex skill artifacts live under `codex-skills/reqvire-syseng`.
- Claude skill artifacts live under `claude-plugins/skills/syseng`.
- Codex audit skill artifacts live under `codex-skills/reqvire-audit`.
- Claude audit skill artifacts live under `claude-plugins/skills/audit`.
- Codex ontology-authoring skill artifacts live under `codex-skills/reqvire-ontology-authoring`.
- Claude ontology-authoring skill artifacts live under `claude-plugins/skills/ontology-authoring`.
- Codex concept-authoring skill artifacts live under `codex-skills/reqvire-concept-authoring`.
- Claude concept-authoring skill artifacts live under `claude-plugins/skills/concept-authoring`.
- Equivalent guidance should stay synchronized between Codex and Claude skill artifacts.
- Assistant artifact changes should preserve MBSE workflow guidance and verification-aligned change sequencing.
- Ontology-authoring guidance should distinguish generic labels/descriptions from ontology slots: `rdfs:label` and `rdfs:comment` are appropriate for optional presentation metadata, while true domain concepts, canonical authored tokens, parser fields, interface enum values, report kinds, controlled-vocabulary payloads, and queryable domain meanings remain declared ontology properties even when their local names end with `Name` or `Meaning`.
- Ontology-authoring guidance should use canonical Reqvire ontology identity: the top parent ontology element in an ontology subgraph defines `ontology_base` and `ontology_prefix` metadata, authored Turtle uses the corresponding hash namespace for terms such as classes/properties/individuals, and export emits one generated `owl:Ontology` document declaration per resolved `ontology_base`. Guidance should state that child ontology elements with the same inherited base contribute vocabulary to that same ontology document, that cross-base ontology hierarchy can become `owl:imports`, that authored Turtle using the inherited prefix must explicitly declare it to `<ontology_base>#`, and that missing or conflicting declarations are invalid.
- Ontology-authoring guidance should preserve the OWL/SHACL block split: OWL `#### Ontology` blocks declare `owl:DatatypeProperty` and `owl:ObjectProperty` terms with stable `rdfs:domain`/`rdfs:range` values, including XSD ranges for datatype properties; SHACL `#### Shapes` blocks declare `sh:NodeShape` profiles using `sh:targetClass` and `sh:path` over reachable ontology terms and carry operational validation facets such as closed-world cardinality, numeric bounds, patterns, enumerations, and messages.
- Ontology-authoring skills should keep existing-ontology cleanup guidance in a separate refactoring reference document. The main skill should load that reference only when the task is to refactor, audit, clean up, or improve existing ontology content, not for greenfield ontology creation.

#### Metadata
  * type: specification

#### Relations
  * define: [AI Skills Markdown Implementation Artifacts](#ai-skills-markdown-implementation-artifacts)
---
