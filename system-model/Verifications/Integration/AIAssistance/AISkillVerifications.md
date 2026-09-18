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
