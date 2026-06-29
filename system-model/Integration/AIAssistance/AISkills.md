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
  * derive: [AI Skills Instruction Contracts](#ai-skills-instruction-contracts)
  * derive: [AI Skills Markdown Implementation Artifacts](#ai-skills-markdown-implementation-artifacts)
  * derive: [AI Skill Installer Distribution](#ai-skill-installer-distribution)
  * specify: [AI-Assisted System Model Management](../IntegrationFeature.md#ai-assisted-system-model-management)
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

#### Relations
  * definedBy: [AI Skills Markdown Artifact Specification](#ai-skills-markdown-artifact-specification)
  * derivedFrom: [AI Assistant Skill-Guided Reqvire Modeling](#ai-assistant-skill-guided-reqvire-modeling)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/concept-authoring/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/ontology-authoring/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/audit/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/syseng/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-concept-authoring/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-ontology-authoring/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-audit/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-syseng/SKILL.md)
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
  * verifiedBy: [AI Skill Installer Manifest Verification](../../Verifications/Integration/AIAssistance/AISkillVerifications.md#ai-skill-installer-manifest-verification)
  * satisfiedBy: [install-claude-skill.sh](../../../scripts/install-claude-skill.sh)
  * satisfiedBy: [install-codex-skill.sh](../../../scripts/install-codex-skill.sh)
  * satisfiedBy: [CodingAssistants.tsx](../../../website/src/pages/CodingAssistants.tsx)
  * satisfiedBy: [README.md](../../../README.md)
  * satisfiedBy: [CODEX_SKILLS.md](../../../doc/CODEX_SKILLS.md)
  * satisfiedBy: [Claude audit SKILL.md](../../../claude-plugins/skills/audit/SKILL.md)
  * satisfiedBy: [Claude AnalyzeCoverage.md](../../../claude-plugins/skills/audit/reference/AnalyzeCoverage.md)
  * satisfiedBy: [Claude AnalyzeModel.md](../../../claude-plugins/skills/audit/reference/AnalyzeModel.md)
  * satisfiedBy: [Claude ChangeImpact.md](../../../claude-plugins/skills/audit/reference/ChangeImpact.md)
  * satisfiedBy: [Claude Lint.md](../../../claude-plugins/skills/audit/reference/Lint.md)
  * satisfiedBy: [Claude concept-authoring SKILL.md](../../../claude-plugins/skills/concept-authoring/SKILL.md)
  * satisfiedBy: [Claude ConceptAuthoring.md](../../../claude-plugins/skills/concept-authoring/reference/ConceptAuthoring.md)
  * satisfiedBy: [Claude ontology-authoring SKILL.md](../../../claude-plugins/skills/ontology-authoring/SKILL.md)
  * satisfiedBy: [Claude OntologyAuthoring.md](../../../claude-plugins/skills/ontology-authoring/reference/OntologyAuthoring.md)
  * satisfiedBy: [Claude OntologyRefactoring.md](../../../claude-plugins/skills/ontology-authoring/reference/OntologyRefactoring.md)
  * satisfiedBy: [Claude syseng SKILL.md](../../../claude-plugins/skills/syseng/SKILL.md)
  * satisfiedBy: [Claude AddCapability.md](../../../claude-plugins/skills/syseng/reference/AddCapability.md)
  * satisfiedBy: [Claude AddRequirement.md](../../../claude-plugins/skills/syseng/reference/AddRequirement.md)
  * satisfiedBy: [Claude AddVerification.md](../../../claude-plugins/skills/syseng/reference/AddVerification.md)
  * satisfiedBy: [Claude CapabilitySemanticContractRefactor.md](../../../claude-plugins/skills/syseng/reference/CapabilitySemanticContractRefactor.md)
  * satisfiedBy: [Claude Collect.md](../../../claude-plugins/skills/syseng/reference/Collect.md)
  * satisfiedBy: [Claude ConsolidateRequirements.md](../../../claude-plugins/skills/syseng/reference/ConsolidateRequirements.md)
  * satisfiedBy: [Claude Containment.md](../../../claude-plugins/skills/syseng/reference/Containment.md)
  * satisfiedBy: [Claude ContainmentStructureRefactor.md](../../../claude-plugins/skills/syseng/reference/ContainmentStructureRefactor.md)
  * satisfiedBy: [Claude CreatingTasks.md](../../../claude-plugins/skills/syseng/reference/CreatingTasks.md)
  * satisfiedBy: [Claude DesignDocOwnership.md](../../../claude-plugins/skills/syseng/reference/DesignDocOwnership.md)
  * satisfiedBy: [Claude Link.md](../../../claude-plugins/skills/syseng/reference/Link.md)
  * satisfiedBy: [Claude Move.md](../../../claude-plugins/skills/syseng/reference/Move.md)
  * satisfiedBy: [Claude Remove.md](../../../claude-plugins/skills/syseng/reference/Remove.md)
  * satisfiedBy: [Claude RenameElement.md](../../../claude-plugins/skills/syseng/reference/RenameElement.md)
  * satisfiedBy: [Claude Setup.md](../../../claude-plugins/skills/syseng/reference/Setup.md)
  * satisfiedBy: [Claude SpecificationLanguageCleanup.md](../../../claude-plugins/skills/syseng/reference/SpecificationLanguageCleanup.md)
  * satisfiedBy: [Claude SpecificationsExtractionLogic.md](../../../claude-plugins/skills/syseng/reference/SpecificationsExtractionLogic.md)
  * satisfiedBy: [Claude SubmodelRefactor.md](../../../claude-plugins/skills/syseng/reference/SubmodelRefactor.md)
  * satisfiedBy: [Claude VerificationAlignment.md](../../../claude-plugins/skills/syseng/reference/VerificationAlignment.md)
  * satisfiedBy: [Claude explore.md](../../../claude-plugins/skills/syseng/reference/explore.md)
  * satisfiedBy: [Codex audit SKILL.md](../../../codex-skills/reqvire-audit/SKILL.md)
  * satisfiedBy: [Codex AnalyzeCoverage.md](../../../codex-skills/reqvire-audit/references/AnalyzeCoverage.md)
  * satisfiedBy: [Codex AnalyzeModel.md](../../../codex-skills/reqvire-audit/references/AnalyzeModel.md)
  * satisfiedBy: [Codex ChangeImpact.md](../../../codex-skills/reqvire-audit/references/ChangeImpact.md)
  * satisfiedBy: [Codex Lint.md](../../../codex-skills/reqvire-audit/references/Lint.md)
  * satisfiedBy: [Codex concept-authoring SKILL.md](../../../codex-skills/reqvire-concept-authoring/SKILL.md)
  * satisfiedBy: [Codex concept-authoring openai.yaml](../../../codex-skills/reqvire-concept-authoring/agents/openai.yaml)
  * satisfiedBy: [Codex ConceptAuthoring.md](../../../codex-skills/reqvire-concept-authoring/references/ConceptAuthoring.md)
  * satisfiedBy: [Codex ontology-authoring SKILL.md](../../../codex-skills/reqvire-ontology-authoring/SKILL.md)
  * satisfiedBy: [Codex ontology-authoring openai.yaml](../../../codex-skills/reqvire-ontology-authoring/agents/openai.yaml)
  * satisfiedBy: [Codex OntologyAuthoring.md](../../../codex-skills/reqvire-ontology-authoring/references/OntologyAuthoring.md)
  * satisfiedBy: [Codex OntologyRefactoring.md](../../../codex-skills/reqvire-ontology-authoring/references/OntologyRefactoring.md)
  * satisfiedBy: [Codex syseng SKILL.md](../../../codex-skills/reqvire-syseng/SKILL.md)
  * satisfiedBy: [Codex AddCapability.md](../../../codex-skills/reqvire-syseng/references/AddCapability.md)
  * satisfiedBy: [Codex AddRequirement.md](../../../codex-skills/reqvire-syseng/references/AddRequirement.md)
  * satisfiedBy: [Codex AddVerification.md](../../../codex-skills/reqvire-syseng/references/AddVerification.md)
  * satisfiedBy: [Codex CapabilitySemanticContractRefactor.md](../../../codex-skills/reqvire-syseng/references/CapabilitySemanticContractRefactor.md)
  * satisfiedBy: [Codex Collect.md](../../../codex-skills/reqvire-syseng/references/Collect.md)
  * satisfiedBy: [Codex ConsolidateRequirements.md](../../../codex-skills/reqvire-syseng/references/ConsolidateRequirements.md)
  * satisfiedBy: [Codex Containment.md](../../../codex-skills/reqvire-syseng/references/Containment.md)
  * satisfiedBy: [Codex ContainmentStructureRefactor.md](../../../codex-skills/reqvire-syseng/references/ContainmentStructureRefactor.md)
  * satisfiedBy: [Codex CreatingTasks.md](../../../codex-skills/reqvire-syseng/references/CreatingTasks.md)
  * satisfiedBy: [Codex DesignDocOwnership.md](../../../codex-skills/reqvire-syseng/references/DesignDocOwnership.md)
  * satisfiedBy: [Codex Link.md](../../../codex-skills/reqvire-syseng/references/Link.md)
  * satisfiedBy: [Codex Move.md](../../../codex-skills/reqvire-syseng/references/Move.md)
  * satisfiedBy: [Codex Remove.md](../../../codex-skills/reqvire-syseng/references/Remove.md)
  * satisfiedBy: [Codex RenameElement.md](../../../codex-skills/reqvire-syseng/references/RenameElement.md)
  * satisfiedBy: [Codex Setup.md](../../../codex-skills/reqvire-syseng/references/Setup.md)
  * satisfiedBy: [Codex SpecificationLanguageCleanup.md](../../../codex-skills/reqvire-syseng/references/SpecificationLanguageCleanup.md)
  * satisfiedBy: [Codex SpecificationsExtractionLogic.md](../../../codex-skills/reqvire-syseng/references/SpecificationsExtractionLogic.md)
  * satisfiedBy: [Codex SubmodelRefactor.md](../../../codex-skills/reqvire-syseng/references/SubmodelRefactor.md)
  * satisfiedBy: [Codex VerificationAlignment.md](../../../codex-skills/reqvire-syseng/references/VerificationAlignment.md)
  * satisfiedBy: [Codex explore.md](../../../codex-skills/reqvire-syseng/references/explore.md)
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
