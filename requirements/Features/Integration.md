# Elements

### AI-Assisted System Model Management

As a **System Engineer**, I want Reqvire to provide explicit AI-assistant modeling guidance, so that AI-assisted changes preserve MBSE traceability and verification discipline.

#### Details
AI-assisted system model management is the capability anchor for assistant skills, modeling instructions, workflow boundaries, and traceable AI-assisted model changes.

Requirements under this feature define the concrete skill contracts, generated assistant artifacts, and synchronization expectations.

#### Metadata
  * type: feature
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire AI Assistance Ontology](../Ontologies/Integration.md#reqvire-ai-assistance-ontology)

#### Relations
  * specifiedBy: [AI Assistant Skill-Guided Reqvire Modeling](../Functional/Integration/AISkills.md#ai-assistant-skill-guided-reqvire-modeling)
---

### Aligning Design with Code

As a **Developer**, I want Reqvire to connect implementation evidence to requirements, so that code and model intent remain traceable.

#### Details
Aligning design with code is the capability anchor for implementation traceability, source markers, supported comment styles, and validation of code-to-requirement evidence.

Requirements under this feature define concrete parsing, marker, comment-style, and validation behavior.

#### Metadata
  * type: feature
  * owner: syseng
  * priority: high
  * risk: medium
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Code Traceability Ontology](../Ontologies/Integration.md#reqvire-code-traceability-ontology)

#### Relations
  * specifiedBy: [Code Traceability](../Functional/Integration/CodeAlignment.md#code-traceability)
---

### GitHub Workflow Automation

As a **Contributor**, I want Reqvire to support repository workflow automation, so that model documentation, validation, and review evidence can participate in pull-request and merge workflows.

#### Details
GitHub workflow automation is the capability anchor for hosted Git workflow integration, documentation export automation, pull-request validation, and change-log generation.

Requirements under this feature define concrete workflow behavior and repository automation evidence.

#### Metadata
  * type: feature
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire GitHub Workflow Ontology](../Ontologies/Integration.md#reqvire-github-workflow-ontology)

#### Relations
  * specifiedBy: [Automate Documentation Export](../Functional/Integration/GitHubIntegration.md#automate-documentation-export)
  * specifiedBy: [Automate Pull Request Validations](../Functional/Integration/GitHubIntegration.md#automate-pull-request-validations)
  * specifiedBy: [Generate Change Logs for Pull Requests](../Functional/Integration/GitHubIntegration.md#generate-change-logs-for-pull-requests)
---

