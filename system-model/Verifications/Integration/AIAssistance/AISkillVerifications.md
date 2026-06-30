# Elements

### AI Skill Installer Verification Objective

Verify that assistant skill installers publish complete Claude and Codex skill packages without requiring a local repository clone.

#### Metadata
  * type: verification-objective
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
