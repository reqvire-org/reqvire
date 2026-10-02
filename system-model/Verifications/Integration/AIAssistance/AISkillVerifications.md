# Elements

### AI Skill Installer Verification Objective

Verify that assistant skill installers publish complete Claude and Codex skill packages without requiring a local repository clone.

#### Metadata
  * type: verification-objective
---

### AI Skill CLI Command Reference Verification

Verify that installed Claude and Codex system engineering skill references can be followed to obtain JSON verification traces.

#### Details

##### Acceptance Criteria
- The installed `traces` reference does not require the removed `--json` flag and advertises optional file output.
- Following the reference produces valid JSON on stdout and equivalent JSON through `--output`.
- Shared flag guidance identifies the JSON-only reports that accept file output without `--json`.
- Installed reference pages do not prescribe `--json` or `--mmd` for JSON-only reports.
- Local and no-clone installers deliver equivalent guidance for both assistant ecosystems.

##### Test Criteria
- Use the standard skill installer E2E suite and a minimal traced model in its temporary Git workspace.
- Inspect both installed system engineering skill references for each installation mode and execute the documented `traces` output options against the test CLI.
- Compare stdout and file JSON and fail on unsupported documented flags or unconditional `--json` requirements for JSON-only commands.
- Scan installed Markdown reference pages for unsupported JSON-only report flags.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [AI Skill Installer Verification Objective](#ai-skill-installer-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-ai-skill-installers/test.sh)
  * verify: [AI Skills Markdown Implementation Artifacts](../../../Integration/AIAssistance/AISkills.md#ai-skills-markdown-implementation-artifacts)
---

### AI Skill Contract Dependency Guidance Inspection

Inspect the delivered Claude and Codex guidance for correct contract ownership, implementation obligations, and review dependencies.

#### Details
- Confirm that contract dependency names the umbrella category, Contract Bindings identify shared implementation obligations, and Contract References identify content dependencies for change-impact review. The specific Markdown section names and command examples retain their canonical names.
- Compare authoring, extraction, ownership, submodel-refactoring, collection, search, mutation, and audit guidance in both skill packages. Context-only consumers use Contract References; requirements responsible for shared implementation obligations use Contract Bindings; the owning requirement uses `definedBy`.
- Follow examples involving a kernel-service consumer, a coverage report, documentation, and a shared endpoint obligation. Confirm the examples preserve the intended fulfillment direction and explain changes to implementation coverage when a binding becomes a reference.
- Confirm references remain visible to collection, search, and change-impact review while contributing no implementation evidence, blockers, terminal dependencies, or coverage units.
- Confirm guidance rejects mixed binding/reference sections on one requirement, file-path reference targets, self-dependencies, and cycles through references, bindings, or requirement ancestry.
- Confirm command examples use `referenceContract`, `unlink`, and the dedicated binding/reference search filters with identifier globs. Confirm both installed packages retain the same dependency semantics and the Claude marketplace advertises the updated plugin version.

Review artifacts: [SKILL.md](../../../../claude-plugins/skills/syseng/SKILL.md), [SKILL.md](../../../../codex-skills/reqvire-syseng/SKILL.md).

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [AI Skill Installer Verification Objective](#ai-skill-installer-verification-objective)
  * verify: [AI Skills Instruction Contracts](../../../Integration/AIAssistance/AISkills.md#ai-skills-instruction-contracts)
  * verify: [AI Skills Markdown Implementation Artifacts](../../../Integration/AIAssistance/AISkills.md#ai-skills-markdown-implementation-artifacts)
---

### AI Skill Installer Manifest Verification

Verify that local and remote assistant skill installers install exactly the files present in the checked-in Claude and Codex skill source trees.

#### Details
The verification shall run the Codex and Claude installers in local-copy mode and in remote mode using `REQVIRE_REPO_RAW=file://...`, then compare installed file manifests against `codex-skills` and `claude-plugins/skills`.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [AI Skill Installer Verification Objective](#ai-skill-installer-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-ai-skill-installers/test.sh)
  * verify: [AI Skill Installer Distribution](../../../Integration/AIAssistance/AISkills.md#ai-skill-installer-distribution)
---
