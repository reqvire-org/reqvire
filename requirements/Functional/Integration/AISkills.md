# Elements

### AI Assistant Skill-Guided Reqvire Modeling

The system shall define a dedicated AI-assistant skills submodel that provides instruction contracts for Reqvire modeling workflows and traceable implementation artifacts.

#### Metadata
 * type: requirement

#### Attachments
 * [Refinement Specification](../../Refinements.md#refinement-specification)
 * [Relation Semantics Specification](../../Refinements.md#relation-semantics-specification)
 * [Supported Element Types Specification](../../Refinements.md#supported-element-types-specification)
 * [Traceability Reporting Specification](../../Refinements.md#traceability-reporting-specification)

#### Relations
 * derive: [AI Skills Instruction Contracts](#ai-skills-instruction-contracts)
 * derive: [AI Skills Markdown Implementation Artifacts](#ai-skills-markdown-implementation-artifacts)
 * specify: [AI-Assisted System Model Management](../../Features/Integration.md#ai-assisted-system-model-management)
---

### AI Skills Instruction Contracts

The system shall define AI skill instruction contracts that describe MBSE-first Reqvire workflows, including boundary clarification and verification-aligned change sequencing.

#### Details
Instruction contract details shall follow the associated refinement specification.

#### Metadata
 * type: requirement

#### Relations
 * derivedFrom: [AI Assistant Skill-Guided Reqvire Modeling](#ai-assistant-skill-guided-reqvire-modeling)
 * refinedBy: [AI Skills Instruction Contract Specification](#ai-skills-instruction-contract-specification)
---

### AI Skills Markdown Implementation Artifacts

The system shall produce traceable markdown skill artifacts that implement the AI assistant skill guidance contracts for supported assistant ecosystems.

#### Details
Artifact contract details shall follow the associated refinement specification.

#### Metadata
 * type: requirement

#### Relations
 * derivedFrom: [AI Assistant Skill-Guided Reqvire Modeling](#ai-assistant-skill-guided-reqvire-modeling)
 * refinedBy: [AI Skills Markdown Artifact Specification](#ai-skills-markdown-artifact-specification)
 * satisfiedBy: [analyze-coverage.md](../../../claude-plugins/commands/analyze-coverage.md)
 * satisfiedBy: [SKILL.md](../../../claude-plugins/skills/syseng/SKILL.md)
 * satisfiedBy: [SKILL.md](../../../codex-skills/reqvire-syseng/SKILL.md)
---

### AI Skills Instruction Contract Specification

AI skill instruction contracts define MBSE-first Reqvire modeling workflows, boundary checkpoints, and verification-aligned change sequencing.

#### Details
Instruction contract rules:
- Skill workflows start from feature and requirement context.
- Skill workflows separate semantic contracts from obligations.
- Skill workflows align verifications before implementation changes.
- Skill workflows run validation and lint after meaningful model updates.
- Cross-submodel refactors require explicit human confirmation before changing boundaries.
- Verification guidance distinguishes verification coverage from implementation coverage.
- Model-refactor examples include correct and incorrect examples where that improves assistant behavior.

#### Metadata
 * type: specification

#### Relations
 * refine: [AI Skills Instruction Contracts](#ai-skills-instruction-contracts)
---

### AI Skills Markdown Artifact Specification

AI skills markdown artifacts define the assistant-facing files that implement the Reqvire system engineering guidance.

#### Details
Artifact contract rules:
- Codex skill artifacts live under `codex-skills/reqvire-syseng`.
- Claude skill artifacts live under `claude-plugins/skills/syseng`.
- Equivalent guidance should stay synchronized between Codex and Claude skill artifacts.
- Assistant artifact changes should preserve MBSE workflow guidance and verification-aligned change sequencing.

#### Metadata
 * type: specification

#### Relations
 * refine: [AI Skills Markdown Implementation Artifacts](#ai-skills-markdown-implementation-artifacts)
---
