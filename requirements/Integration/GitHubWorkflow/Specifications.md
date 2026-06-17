# Elements

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
  * refine: [GitHub Pages Deployment Workflow](GitHubWorkflowRequirements.md#github-pages-deployment-workflow)
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
  * refine: [Generate Change Logs for Pull Requests](GitHubWorkflowRequirements.md#generate-change-logs-for-pull-requests)
---

### Pull Request Validation Workflow Specification

#### Details
Pull request validation workflow behavior:
- Triggers when a pull request is updated.
- Runs `reqvire validate`.
- Runs `reqvire lint`.
- Reports pull-request validation results as workflow evidence.

#### Metadata
  * type: specification

#### Relations
  * refine: [Automate Pull Request Validations](GitHubWorkflowRequirements.md#automate-pull-request-validations)
---
