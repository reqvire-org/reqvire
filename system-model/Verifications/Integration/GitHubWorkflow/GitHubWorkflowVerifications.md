# Elements

### Repository Workflow Verification Objective

Verify that repository automation executes isolated checks and preserves trustworthy validation evidence.

#### Metadata
  * type: verification-objective
---

### End-to-End Test Runner Lifecycle Verification

Verify temporary fixture ownership, independent run evidence, interruption cleanup, and accurate timing categories in the shell E2E runner.

#### Details
- Run disposable passing and failing suites through a copy of the real runner and timing wrapper. Assert initialized Git state, successful fixture removal, failed fixture retention at the reported path, and accurate pass/fail exit status and counts.
- Run the same suite concurrently and assert distinct fixture and log directories with both runs' evidence preserved.
- Interrupt an active suite with a child process, a server launched in a separate session, and a child ignoring graceful termination. Assert bounded escalation, no surviving test child, removal of the interrupted fixture, retention of logs, and a signal-derived nonzero result. Confirm unrelated processes remain alive.
- Make a suite fail during fixture setup and assert it is reported as a failure without running its test body.
- Make descendant discovery fail during cleanup. Assert that the run fails, retains the affected fixture, does not start another suite, and retries cleanup for the same active process group on exit.
- Include work outside Reqvire invocations and compare the reported suite/test wall-time categories and invocation totals with retained timing records. Run a selected suite and a mixed full suite so aggregation and selection behavior are covered.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Repository Workflow Verification Objective](#repository-workflow-verification-objective)
  * verify: [Isolated End-to-End Test Execution](../../../Integration/GitHubWorkflow/GitHubWorkflowRequirements.md#isolated-end-to-end-test-execution)
  * satisfiedBy: [test.sh](../../../../tests/test-runner-lifecycle/test.sh)
---

### Browser End-to-End Harness Verification

Verify consistent executable selection and bounded browser lifecycle behavior across Explorer browser suites.

#### Details
- With no browser on PATH, select the executable supplied through `REQVIRE_TEST_BROWSER`, including a path containing spaces. Check override precedence, the `CHROME_BIN` compatibility alias, PATH fallback, and explicit failure for invalid overrides or missing executables.
- Exercise server/browser startup failure, early process exit, assertion failure, and a second browser failing to start. Confirm startup failures are reported as failures rather than successful skips, already-started browsers are stopped, successful profiles are removed after exit, and failure profiles remain available.
- Exercise debugging-command rejection, timeout, connection loss with pending commands/navigation, and shutdown escalation. Assert all waits settle and the launched process is reaped before profile deletion.
- Exercise reload with a load event before its command response, a missing event, and connection loss. Force the scoped-coverage browser subprocess to exceed its deadline after writing progress; assert failure and retention of partial stdout/stderr.
- Run the served route, live refresh, scoped coverage, and worktree browser checks through the common helper using only the configured executable. Retain their existing functional assertions.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Repository Workflow Verification Objective](#repository-workflow-verification-objective)
  * verify: [Isolated End-to-End Test Execution](../../../Integration/GitHubWorkflow/GitHubWorkflowRequirements.md#isolated-end-to-end-test-execution)
  * satisfiedBy: [test.sh](../../../../tests/test-browser-support/test.sh)
---

### Current Revision Workflow Gates Verification

Verify that CI tests and validates the checked-out revision and propagates failures.

#### Details
- Execute the workflows' host build and Rust test commands with an empty target and a deliberately stale worker executable. Assert the build precedes tests and `REQVIRE_TEST_BIN` points to the resulting executable in both PR and release jobs.
- Make the build or worker test fail and assert the workflow command sequence fails without proceeding as a success.
- Confirm PR validation consumes the current run's compiled binary, targets development-branch PRs as well as main, and preserves the required check name.
- Execute the model validation report command with a valid and an invalid model. The invalid model must fail despite report capture, and the workflow must retain the diagnostic report rather than tolerate the failure.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Repository Workflow Verification Objective](#repository-workflow-verification-objective)
  * verify: [Automate Pull Request Validations](../../../Integration/GitHubWorkflow/GitHubWorkflowRequirements.md#automate-pull-request-validations)
  * satisfiedBy: [test.sh](../../../../tests/test-workflow-gates/test.sh)
---
