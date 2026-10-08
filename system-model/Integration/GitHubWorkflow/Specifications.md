# Elements

### End-to-End Test Runner Specification

#### Details
- Each invocation of the shell E2E runner owns a unique temporary run directory. Each test fixture is a child of that directory and receives its own initialized Git repository.
- Before starting any fixture, resolve the configured Reqvire executable and copy it into the owned run directory. Every timing-wrapper invocation and child CLI worker in that run MUST use this private copy, even if the original executable is removed or rebuilt. A later run MUST capture the then-current executable. Missing, non-executable or uncopyable inputs MUST fail startup before running suites, without retrying commands against a replacement build. Report both the configured source and private executable paths. Remove the copy after owned test processes stop; retain it with affected fixtures if cleanup cannot be confirmed.
- Logs and timing records are retained in a unique directory under `/tmp/reqvire-test-logs`, or under `REQVIRE_TEST_LOG_DIR` when set. Concurrent runs MUST NOT overwrite each other's records. Print the actual run and log locations.
- Remove successful test fixtures after completion. Retain failed fixtures at their reported paths for diagnosis, remove other temporary run data, and remove an empty run directory. A failed fixture setup is a failed test, not a skipped or successful test.
- On interruption, stop the active test process group and its owned descendants, including servers launched in separate sessions, with bounded escalation; reap the direct child before removing its interrupted fixture. Preserve completed failure evidence and logs and return a nonzero signal-derived exit status. If shutdown cannot be confirmed, retain the fixture and process identity, report the cleanup failure, and stop scheduling further suites. Do not signal unrelated processes or remove another run's files.
- Report per-test wall time including fixture setup and test execution, accumulated Reqvire invocation time, and invocation count separately. The final summary distinguishes suite wall time from accumulated Reqvire process time; overlapping invocation lifetimes are not interpreted as elapsed suite time.
- Preserve single-suite selection, full-suite execution, failure exit status, the configured Reqvire binary, and timing-wrapper signal forwarding. Timing records and logs remain available after fixture cleanup.
- Browser suites share executable discovery: use `REQVIRE_TEST_BROWSER` when set, otherwise the compatible `CHROME_BIN` override, otherwise Chromium/Chrome from `PATH`. An invalid explicit executable or unavailable browser fails the check instead of falling back or skipping it. A server that cannot start makes its dependent suite fail rather than reporting a pass.
- Browser startup, debugging commands, navigation, and shutdown have bounded waits. Pending commands and navigation fail when the debugging connection closes. Await browser exit before deleting its owned profile; keep failure diagnostics and profiles. All browser consumers use the same lifecycle behavior, including cleanup when assertions or a second browser startup fail.
- Reload uses the same bounded load-event handling as navigation. Scoped-coverage browser checks retain progress and partial stdout/stderr when their outer deadline expires, identifying the active scenario and wait instead of losing the diagnostics.

#### Metadata
  * type: specification

#### Relations
  * define: [Isolated End-to-End Test Execution](GitHubWorkflowRequirements.md#isolated-end-to-end-test-execution)
---

### GitHub Pages Deployment Workflow Specification

#### Details
GitHub Pages deployment workflow behavior:
- Triggers on push to the main branch and supports manual dispatch.
- Builds the Explorer SPA bundle using `npm ci && npm run build` in the `explorer/` directory.
- Builds the Reqvire binary with `REQVIRE_BUILD_EXPLORER=1` to embed the compiled bundle.
- Runs `reqvire export --output ./site` against the repository workspace to generate the Project Store and write all static assets.
- Uses `actions/configure-pages`, `actions/upload-pages-artifact`, and `actions/deploy-pages` to publish the `./site` directory to GitHub Pages.
- Requires `pages: write` and `id-token: write` permissions on the workflow job.
- Uses the `github-pages` environment with concurrency group "pages" and `cancel-in-progress: false` to prevent interrupted deployments.

#### Metadata
  * type: specification

#### Relations
  * define: [GitHub Pages Deployment Workflow](GitHubWorkflowRequirements.md#github-pages-deployment-workflow)
---

### Pull Request Change Log Workflow Specification

#### Details
Pull request change-log workflow behavior:
- Triggers during pull-request review.
- Summarizes system model and related component changes.
- Produces a pull-request change log that is reviewable by contributors.

#### Metadata
  * type: specification

#### Relations
  * define: [Generate Change Logs for Pull Requests](GitHubWorkflowRequirements.md#generate-change-logs-for-pull-requests)
---

### Pull Request Validation Workflow Specification

#### Details
Pull request validation workflow behavior:
- Triggers when a pull request is opened or updated, including pull requests targeting another development branch.
- Builds the checked-out revision's host CLI before Rust tests that spawn CLI workers. Pass that exact executable through `REQVIRE_TEST_BIN`; a clean or stale target directory must not change which implementation is tested. Release validation follows the same build-before-worker-test rule.
- Runs `reqvire validate` and `reqvire lint` using the CLI built from the same checked-out revision, without installing a released validator. Preserve the required `Run Rust Tests` check name.
- A build, Rust test, or model-validation failure fails its check. Capturing reports through pipelines must preserve failure exit status; model validation must not use `continue-on-error`.
- Reports pull-request validation results as workflow evidence.

#### Metadata
  * type: specification

#### Relations
  * define: [Automate Pull Request Validations](GitHubWorkflowRequirements.md#automate-pull-request-validations)
---
