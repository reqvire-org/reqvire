# Elements

### AI Assistant Skill-Guided Reqvire Modeling

The system shall define a dedicated AI-assistant skills submodel that provides instruction contracts for Reqvire modeling workflows and traceable implementation artifacts.

#### Metadata
 * type: user-requirement

#### Attachments
 * [Refinement Specification](../../Refinements.md#refinement-specification)
 * [Relation Semantics Specification](../../Refinements.md#relation-semantics-specification)
 * [Supported Element Types Specification](../../Refinements.md#supported-element-types-specification)
 * [Traceability Reporting Specification](../../Refinements.md#traceability-reporting-specification)

#### Relations
 * derive: [AI Skills Instruction Contracts](#ai-skills-instruction-contracts)
 * derive: [AI Skills Markdown Implementation Artifacts](#ai-skills-markdown-implementation-artifacts)
 * derivedFrom: [AI Assistant Skill Authoring for Reqvire](../../UserStories.md#ai-assistant-skill-authoring-for-reqvire)
---

### AI Skills Instruction Contracts

The system shall define AI skill instruction contracts that describe MBSE-first Reqvire workflows, including boundary clarification and verification-aligned change sequencing.

#### Details
Implementation details shall follow the associated refinement specification.

#### Metadata
 * type: requirement

#### Relations
 * derivedFrom: [AI Assistant Skill-Guided Reqvire Modeling](#ai-assistant-skill-guided-reqvire-modeling)
 * refinedBy: [AI Skills Instruction Contract Specification](#ai-skills-instruction-contract-specification)
---

### AI Skills Markdown Implementation Artifacts

The system shall produce traceable markdown skill artifacts that implement the AI assistant skill guidance contracts for supported assistant ecosystems.

#### Details
Implementation details shall follow the associated refinement specification.

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

#### Details
The AI skills instruction contract is expected to define:
- MBSE change sequence: requirements -> refinements -> verifications -> implementation links.
- Explicit human boundary checkpoint before cross-submodel refactors.
- Verification alignment policy: verification criteria must match explicit assertions in test cases.
- Coverage interpretation guidance for verification and implementation coverage.
- Mandatory validation and lint checks after meaningful model updates.

#### Metadata
 * type: specification

#### Relations
 * refine: [AI Skills Instruction Contracts](#ai-skills-instruction-contracts)
---

### AI Skills Markdown Artifact Specification

#### Details
The markdown implementation artifacts is expected to:
- Be versioned in repository paths consumed by supported assistant runtimes.
- Capture task patterns with explicit "Do It When" triggers.
- Include correct and incorrect examples for model-refactor workflows.
- Require circle-back confirmation for expected submodel boundaries.
- Remain synchronized across Codex and Claude plugin variants.

#### Metadata
 * type: specification

#### Relations
 * refine: [AI Skills Markdown Implementation Artifacts](#ai-skills-markdown-implementation-artifacts)
---
