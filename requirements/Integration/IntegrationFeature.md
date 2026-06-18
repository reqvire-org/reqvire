# Elements

### AI-Assisted System Model Management

As a **System Engineer**, I want Reqvire to provide explicit AI-assistant modeling guidance, so that AI-assisted changes preserve MBSE traceability and verification discipline.

#### Details
AI-assisted system model management is the capability for assistant skills, modeling instructions, workflow boundaries, and traceable AI-assisted model changes.

Requirements under this capability define the concrete skill contracts, generated assistant artifacts, and synchronization expectations.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * specifiedBy: [AI Assistant Skill-Guided Reqvire Modeling](AIAssistance/AISkills.md#ai-assistant-skill-guided-reqvire-modeling)
---

### Aligning Design with Code

As a **Developer**, I want Reqvire to connect implementation evidence to requirements, so that code and model intent remain traceable.

#### Details
Aligning design with code is the capability for implementation traceability, source markers, supported comment styles, and validation of code-to-requirement evidence.

Requirements under this capability define concrete parsing, marker, comment-style, and validation behavior.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: medium
  * status: approved

#### Relations
  * specifiedBy: [Code Traceability](CodeAlignment/CodeAlignmentRequirements.md#code-traceability)
---

### GitHub Workflow Automation

As a **Contributor**, I want Reqvire to support repository workflow automation, so that validation and review evidence can participate in pull-request and merge workflows.

#### Details
GitHub workflow automation is the capability for hosted Git workflow integration, pull-request validation, and change-log generation.

Requirements under this capability define concrete workflow behavior and repository automation evidence.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * specifiedBy: [Automate Pull Request Validations](GitHubWorkflow/GitHubWorkflowRequirements.md#automate-pull-request-validations)
  * specifiedBy: [Generate Change Logs for Pull Requests](GitHubWorkflow/GitHubWorkflowRequirements.md#generate-change-logs-for-pull-requests)
---
