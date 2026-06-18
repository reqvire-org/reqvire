# Elements

### AI Assistant Skill-Guided Reqvire Modeling

The system shall define a dedicated AI-assistant skills submodel that provides instruction contracts for Reqvire modeling workflows and traceable implementation artifacts.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Contract Specification](../../ModelStructure/Specifications.md#contract-specification)
  * [Relation Semantics Specification](../../ModelStructure/Specifications.md#relation-semantics-specification)
  * [Supported Element Types Specification](../../ModelStructure/Specifications.md#supported-element-types-specification)
  * [Traceability Reporting Specification](../../Reports/ModelReports/Specifications.md#traceability-reporting-specification)

#### Relations
  * derive: [AI Skills Instruction Contracts](#ai-skills-instruction-contracts)
  * derive: [AI Skills Markdown Implementation Artifacts](#ai-skills-markdown-implementation-artifacts)
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
  * satisfiedBy: [analyze-coverage.md](../../../claude-plugins/commands/analyze-coverage.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/ontology-authoring/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/syseng/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-ontology-authoring/SKILL.md)
  * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-syseng/SKILL.md)
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
- Codex ontology-authoring skill artifacts live under `codex-skills/reqvire-ontology-authoring`.
- Claude ontology-authoring skill artifacts live under `claude-plugins/skills/ontology-authoring`.
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

