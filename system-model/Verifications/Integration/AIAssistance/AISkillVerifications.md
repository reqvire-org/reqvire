# Elements

### AI Skill Installer Verification Objective

Verify that assistant skill installers publish complete Claude and Codex skill packages without requiring a local repository clone.

#### Metadata
  * type: verification-objective
---

### AI Skill CLI Command Reference Verification

Verify that the shared system engineering skill references can be followed to obtain JSON verification traces and respect the supported CLI/MCP operation boundary.

#### Details
- The `traces` reference advertises optional file output and uses no removed `--json` flag.
- Following the reference against the minimal fixture produces equivalent stdout and file JSON; compare the normalized results with a unified diff.
- Shared flag guidance identifies JSON-only reports that accept file output directly, and supporting references contain no unsupported JSON-only report flags.
- Shared and installed guidance uses typed MCP authoring tools and correct mutation permissions, MCP format preview/application and lint repair, automatic canonical writes, and CLI-only migration before validated MCP startup.
- Run the canonical package E2E checks and inspect actual skills CLI installation results for both target agents.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [AI Skill Installer Verification Objective](#ai-skill-installer-verification-objective)
  * satisfiedBy: [check_skills.sh](../../../../tests/test-ai-skill-installers/check_skills.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-ai-skill-installers/test.sh)
  * verify: [AI Skills Instruction Contracts](../../../Integration/AIAssistance/AISkills.md#ai-skills-instruction-contracts)
  * verify: [AI Skills Markdown Implementation Artifacts](../../../Integration/AIAssistance/AISkills.md#ai-skills-markdown-implementation-artifacts)
---

### AI Skill Contract Dependency Guidance Inspection

Inspect the shared assistant guidance for correct contract ownership, implementation obligations, and review dependencies.

#### Details
- Confirm that contract dependency names the umbrella category, Contract Bindings identify shared implementation obligations, and Contract References identify content dependencies for change-impact review. The specific Markdown section names and command examples retain their canonical names.
- Compare authoring, extraction, ownership, submodel-refactoring, collection, search, mutation, and audit guidance in the shared skill collection. Context-only consumers use Contract References; requirements responsible for shared implementation obligations use Contract Bindings; the owning requirement uses `definedBy`.
- Follow examples involving a kernel-service consumer, a coverage report, documentation, and a shared endpoint obligation. Confirm the examples preserve the intended fulfillment direction and explain changes to implementation coverage when a binding becomes a reference.
- Confirm references remain visible to collection, search, and change-impact review while contributing no implementation evidence, blockers, terminal dependencies, or coverage units.
- Confirm guidance rejects mixed binding/reference sections on one requirement, file-path reference targets, self-dependencies, and cycles through references, bindings, or requirement ancestry.
- Confirm command examples use `referenceContract`, `reqvire.unlink` with typed operation fields and `dry_run: false`, and the dedicated binding/reference search filters with identifier globs. Confirm installed packages for both target agents retain the same dependency semantics and use the shared skill names.
- Review authoring guidance and its complete replacement example against the override acceptance cases: incoming children/consumers survive, omitted authored declarations are removed, requirement parents remain explicit in payloads, invalid candidates preserve accepted state, and actual deletion retains orphan protection.
- Review setup guidance against native/npm stdio acceptance results, including client-managed startup, HTTP default, absolute workspace selection, permissions, rejected HTTP-only flags and shutdown preservation. Compare installed bytes with the canonical packages.

Review artifact: [SKILL.md](../../../../skills/reqvire-syseng/SKILL.md).

#### Metadata
  * type: demonstration-verification

#### Relations
  * derivedFrom: [AI Skill Installer Verification Objective](#ai-skill-installer-verification-objective)
  * verify: [AI Skills Instruction Contracts](../../../Integration/AIAssistance/AISkills.md#ai-skills-instruction-contracts)
  * verify: [AI Skills Markdown Implementation Artifacts](../../../Integration/AIAssistance/AISkills.md#ai-skills-markdown-implementation-artifacts)
---

### AI Skill Installer Manifest Verification

Verify that the canonical four-skill collection is complete, discoverable, and documented for installation from main through the standard skills CLI.

#### Details
- Compare discovered package names and supporting file manifests with fixtures using unified diffs.
- Check stable skill frontmatter, supporting-file and cross-skill link resolution, and preservation of applicable assistant metadata.
- Check README and website examples for the explicit main-branch skills source, correct agent names, scope options, and skill selection.
- Confirm marketplace packaging, duplicate assistant source trees, and custom installers have been retired; migration guidance directs existing plugin users to the shared packages.
- Exercise actual skills CLI discovery and project-scoped installation from a local checkout into an isolated fixture workspace. Compare installed files and bytes for Codex and Claude Code with the canonical packages.
- Remote main-branch installation becomes available after the updated collection is merged to main; do not substitute an older published package for the local acceptance check.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [AI Skill Installer Verification Objective](#ai-skill-installer-verification-objective)
  * satisfiedBy: [check_skills.sh](../../../../tests/test-ai-skill-installers/check_skills.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-ai-skill-installers/test.sh)
  * verify: [AI Skill Installer Distribution](../../../Integration/AIAssistance/AISkills.md#ai-skill-installer-distribution)
---
