# Elements

### Isolated End-to-End Test Execution

When end-to-end tests are executed, the system SHALL isolate concurrent runs, preserve failure evidence, release completed test resources, and distinguish elapsed run time from accumulated Reqvire invocation time.

#### Metadata
  * type: requirement

#### Relations
  * specify: [GitHub Workflow Automation](../IntegrationFeature.md#github-workflow-automation)
  * definedBy: [End-to-End Test Runner Specification](Specifications.md#end-to-end-test-runner-specification)
  * satisfiedBy: [run_tests.sh](../../../tests/run_tests.sh)
  * satisfiedBy: [stop_test_processes.py](../../../tests/stop_test_processes.py)
  * satisfiedBy: [browser.mjs](../../../tests/browser.mjs)
  * verifiedBy: [End-to-End Test Runner Lifecycle Verification](../../Verifications/Integration/GitHubWorkflow/GitHubWorkflowVerifications.md#end-to-end-test-runner-lifecycle-verification)
---

### Automate Pull Request Validations

The system shall automate validations of pull requests in the GitHub workflow to ensure model consistency before merging.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Pull Request Validation Workflow Specification](Specifications.md#pull-request-validation-workflow-specification)
  * specify: [GitHub Workflow Automation](../IntegrationFeature.md#github-workflow-automation)
  * satisfiedBy: [pr.yml](../../../.github/workflows/pr.yml)
  * satisfiedBy: [release.yml](../../../.github/workflows/release.yml)
---

### Generate Change Logs for Pull Requests

The system shall generate detailed change logs for pull requests, summarizing modifications to the System model and related components.

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [change_impact.yml](../../../.github/workflows/change_impact.yml)
  * definedBy: [Pull Request Change Log Workflow Specification](Specifications.md#pull-request-change-log-workflow-specification)
  * specify: [GitHub Workflow Automation](../IntegrationFeature.md#github-workflow-automation)
---

### GitHub Pages Deployment Workflow

The system SHALL provide a GitHub Actions workflow that builds the Reqvire binary with the embedded Explorer bundle and deploys the exported static Explorer site to GitHub Pages on every push to the main branch.

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [pages.yml](../../../.github/workflows/pages.yml)
  * specify: [GitHub Workflow Automation](../IntegrationFeature.md#github-workflow-automation)
---
