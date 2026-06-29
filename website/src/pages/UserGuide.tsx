import { BulletList, CodeBlock, CommandList, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function UserGuide() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">User Guide</h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-8">
        This guide covers the day-to-day Reqvire CLI workflow for validating,
        querying, refactoring, and serving a semantic engineering model.
      </p>

      <Section title="Installation">
        <h3 className="text-lg font-semibold text-zinc-900 mb-3">
          Quick Install
        </h3>
        <CodeBlock>{`curl -fsSL https://raw.githubusercontent.com/reqvire-org/reqvire/main/scripts/install.sh | bash`}</CodeBlock>
      </Section>

      <Section title="Workspace Selection">
        <p className="text-zinc-600 mb-4">
          Use the global{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            --workspace
          </code>{" "}
          option when running Reqvire from outside the model repository. The
          option applies to normal CLI commands and the MCP server.
        </p>
        <CodeBlock>{`reqvire --workspace /path/to/repository validate
reqvire --workspace /path/to/repository search --filter-type requirement
reqvire --workspace /path/to/repository mcp`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Convenience no-install command form:
        </p>
        <CodeBlock>{`npx -y @reqvire-org/reqvire@latest --workspace /path/to/repository validate
npx -y @reqvire-org/reqvire@latest --workspace /path/to/repository search --filter-type requirement
npx -y @reqvire-org/reqvire@latest --workspace /path/to/repository mcp`}</CodeBlock>
      </Section>

      <Section title="Core Commands">
        <CommandList
          items={[
            { cmd: "reqvire validate", desc: "Parse and validate model structure, relations, Contract Bindings, ontology, and semantic contracts." },
            { cmd: "reqvire format", desc: "Preview canonical formatting, ordering, and relation layout." },
            { cmd: "reqvire format --fix", desc: "Apply formatting fixes." },
            { cmd: "reqvire lint", desc: "Find model quality issues such as redundant relations and cross-boundary hierarchy problems." },
            { cmd: "reqvire lint --auditable", desc: "Report remediation-ready structural findings." },
            { cmd: "reqvire search", desc: "Filter the model by type, file, name, content, relations, and governance metadata." },
            { cmd: "reqvire model", desc: "Emit the ontology/concept/capability-rooted model view as structured JSON." },
            { cmd: "reqvire traces", desc: "Generate verification trace trees from verifications to owning capability roots." },
            { cmd: "reqvire coverage", desc: "Report verification coverage and requirement implementation coverage." },
            { cmd: "reqvire change-impact", desc: "Analyze review impact from changed model content and relations." },
          ]}
        />
      </Section>

      <Section title="JSON and Output Files">
        <p className="text-zinc-600 mb-4">
          Commands such as validate, search, lint, coverage, collect,
          submodels, and change-impact support selectable{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            --json
          </code>
          . Model, containment, resources, and traces are JSON-only; use{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            --output
          </code>{" "}
          to write their JSON directly to a file.
        </p>
        <CodeBlock>{`reqvire validate --json --output results.json
reqvire search --json --output search-results.json
reqvire lint --json --output lint-report.json
reqvire change-impact --json --output impact.json
reqvire semantic export --layer ontologies --output ontologies.ttl
reqvire semantic export --jsonld --output semantic-graph.jsonld`}</CodeBlock>
      </Section>

      <Section title="Working with Elements">
        <CommandList
          items={[
            { cmd: "reqvire add <file>", desc: "Append a new Markdown element from inline content, stdin, or a file." },
            { cmd: "reqvire add <file> --dry-run", desc: "Preview a new or overridden element without writing." },
            { cmd: "reqvire rm <element>", desc: "Remove an element and validate remaining links." },
            { cmd: "reqvire mv <element> <file>", desc: "Move an element to another Markdown file while preserving references." },
            { cmd: "reqvire rename <old> <new>", desc: "Rename an element and update links." },
            { cmd: "reqvire merge <target> <source>", desc: "Merge source element content and relations into a target." },
            { cmd: "reqvire mv-file <old> <new>", desc: "Move or rename a model file while updating model references." },
            { cmd: "reqvire mv-file <old> <new> --squash", desc: "Merge all elements from one file into another file." },
            { cmd: "reqvire mv-asset <old> <new>", desc: "Move a referenced non-model file and update references." },
            { cmd: "reqvire rm-asset <path>", desc: "Remove a referenced asset from the model." },
          ]}
        />
        <div className="mt-5">
          <CodeBlock>{`reqvire add system-model/Auth.md --content '### Token Expiry Requirement

The system shall reject expired access tokens.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Authentication](Auth.md#api-authentication)'

reqvire add system-model/Auth.md --dry-run < new-requirement.md
reqvire add system-model/Auth.md --override < cleaned-merged-requirement.md

reqvire rm "Obsolete Requirement" --dry-run
reqvire rm "Obsolete Requirement"

reqvire mv "Token Expiry Requirement" system-model/Identity/AuthRequirements.md
reqvire rename "Token Expiry Requirement" "Access Token Expiry Requirement"
reqvire merge "Access Token Validation" "Legacy Token Validation" --dry-run

reqvire mv-file system-model/OldAuth.md system-model/Identity/Auth.md
reqvire mv-file system-model/AuthDrafts.md system-model/Identity/Auth.md --squash
reqvire mv-asset docs/auth-flow.pdf docs/identity/auth-flow.pdf
reqvire rm-asset docs/obsolete-auth-flow.pdf --dry-run`}</CodeBlock>
        </div>
      </Section>

      <Section title="Linking and Contract Bindings">
        <p className="text-zinc-600 mb-4">
          The link and unlink commands manage both relations and Reused Contract
          Context. Reused context uses the `bindContract` command form so
          cross-subgraph contract dependencies stay visible.
        </p>
        <CodeBlock>{`reqvire link "Authentication" "specifiedBy" "Authentication Requirement"
reqvire link "Authentication Requirement" "verifiedBy" "Auth Test Case"
reqvire link "System Requirement" "satisfiedBy" "src/auth/login.rs"
reqvire link "Performance Requirement" bindContract "#rate-limiting-constraint"

reqvire unlink "Authentication Requirement" "Auth Test Case"
reqvire unlink "Performance Requirement" "#rate-limiting-constraint"
reqvire relink "Child Requirement" "derivedFrom" "Old Parent" "New Parent"

reqvire link "Password Login Requirement" "derivedFrom" "Authentication Requirement" --dry-run
reqvire relink "Child Requirement" "derivedFrom" "Old Parent" "New Parent" --json`}</CodeBlock>
      </Section>

      <Section title="Validation, Formatting, and Linting">
        <p className="text-zinc-600 mb-4">
          Mutating commands validate before they write. Run validation and
          hygiene commands explicitly in pull requests and before publishing
          generated reports.
        </p>
        <CodeBlock>{`reqvire validate
reqvire validate --json --output reports/validate.json

reqvire format
reqvire format --fix
reqvire format --fix --with-full-relations

reqvire lint
reqvire lint --fixable
reqvire lint --auditable
reqvire lint --json --output reports/lint.json`}</CodeBlock>
      </Section>

      <Section title="Search and Collection">
        <p className="text-zinc-600 mb-4">
          Search is the fastest way to inspect a large model. Collect gathers an
          element and related upstream or downstream context with source
          citations, including contract bindings.
        </p>
        <CodeBlock>{`reqvire search --filter-type requirement --short
reqvire search --filter-type capability,requirement --filter-name ".*auth.*"
reqvire search --filter-status review --filter-priority high,critical
reqvire search --filter-type test-verification --not-have-relations satisfiedBy
reqvire search --filter-risk high,critical --json --output reports/risk.json

reqvire collect "Capability Requirement"
reqvire collect "Capability Requirement" --direction DOWNSTREAM --json
reqvire collect "Capability Requirement" --direction UPSTREAM --json --output context.json`}</CodeBlock>
      </Section>

      <Section title="Reports">
        <p className="text-zinc-600 mb-4">
          Model, containment, resources, and traces reports emit JSON directly.
          Commands that still support human-readable review output keep their
          selectable JSON mode for automation.
        </p>
        <CodeBlock>{`reqvire model
reqvire model --output reports/model.json
reqvire model --from "API Authentication"
reqvire model --filter-type capability,requirement

reqvire traces
reqvire traces --output reports/traces.json
reqvire traces --filter-type test-verification

reqvire resources
reqvire resources --output reports/resources.json

reqvire submodels
reqvire submodels --from "API Authentication"
reqvire submodels --json --output reports/submodels.json`}</CodeBlock>
      </Section>

      <Section title="Change Impact Workflow">
        <p className="text-zinc-600 mb-4">
          Change impact turns repository diffs into a review queue. Use it to
          find changed requirements, affected descendants, linked verifications,
          bound contracts, satisfied implementation artifacts, and governance
          context that should guide review order.
        </p>
        <CodeBlock>{`reqvire change-impact
reqvire change-impact --git-commit origin/main
reqvire change-impact --git-commit origin/main --json --output reports/impact.json

reqvire search --filter-risk high,critical --filter-status review
reqvire traces --output reports/traces.json
reqvire coverage --json --output reports/coverage.json`}</CodeBlock>
        <div className="mt-5">
          <BulletList
            items={[
              "Start with changed high-risk or critical requirements.",
              "Review bound contracts and child requirements before treating a change as isolated.",
              "Use traces to find verification evidence that must be rerun or hardened.",
              "Use coverage to confirm affected obligations still have implementation or evidence links.",
            ]}
          />
        </div>
      </Section>

      <Section title="Serve Explorer">
        <p className="text-zinc-600 mb-4">
          Serve starts a local Explorer for the current workspace with model
          views, verification traces, coverage reports, resources, and ontology
          explorer output.
        </p>
        <CodeBlock>{`reqvire serve
reqvire serve --host 0.0.0.0 --port 3000`}</CodeBlock>
      </Section>

      <Section title="Ignore Files">
        <BulletList
          items={[
            ".gitignore excludes files from structured parsing and from file relations.",
            ".reqvireignore excludes files from structured parsing but still allows file relations to reference them.",
            "Common repository docs such as README.md, CHANGELOG.md, CONTRIBUTING.md, LICENSE.md, SECURITY.md, and AI assistant instruction files are reserved and skipped as model files.",
          ]}
        />
      </Section>

      <Section title="GitHub Workflows">
        <p className="text-zinc-600 mb-4">
          Reqvire fits naturally into pull request checks and issue-comment
          workflows. Use pull request jobs for required validation and report
          artifacts; use comment-triggered jobs for review-time impact and trace
          questions that should not run on every push.
        </p>

        <h3 className="text-lg font-semibold text-zinc-900 mb-3">
          Pull Request Validation
        </h3>
        <p className="text-zinc-600 mb-4">
          This pattern validates the model, surfaces lint findings, writes JSON
          reports, and uploads the report folder as an artifact. Fetch full git
          history when change-impact needs to compare against the pull request
          base branch.
        </p>
        <CodeBlock>{`name: Reqvire PR Checks

on:
  pull_request:
    branches: [main]

jobs:
  reqvire:
    runs-on: ubuntu-latest
    permissions:
      contents: read

    steps:
      - name: Checkout repository
        uses: actions/checkout@v6
        with:
          fetch-depth: 0

      - name: Install Reqvire
        run: curl -fsSL https://raw.githubusercontent.com/reqvire-org/reqvire/main/scripts/install.sh | bash

      - name: Generate Reqvire reports
        run: |
          mkdir -p reports
          git fetch origin "\${{ github.base_ref }}"
          reqvire validate --json --output reports/validate.json
          reqvire lint --auditable --json --output reports/lint.json
          reqvire coverage --json --output reports/coverage.json
          reqvire traces --output reports/traces.json
          reqvire change-impact --git-commit "origin/\${{ github.base_ref }}" --json --output reports/impact.json

      - name: Upload Reqvire reports
        uses: actions/upload-artifact@v6
        with:
          name: reqvire-reports
          path: reports/`}</CodeBlock>

        <h3 className="text-lg font-semibold text-zinc-900 mb-3 mt-6">
          Issue Comment Commands
        </h3>
        <p className="text-zinc-600 mb-4">
          Comment workflows run only when a reviewer asks for them. The job must
          check out the pull request branch, compute the merge-base commit
          against the base branch, then pass that commit to change-impact.
        </p>
        <CommandList
          items={[
            { cmd: "/reqvire impact", desc: "Run change-impact against the pull request merge base and comment with the report." },
            { cmd: "/reqvire traces", desc: "Run verification traces and comment with the report." },
            { cmd: "/reqvire coverage", desc: "Run coverage and comment with the report." },
          ]}
        />
        <div className="mt-5">
          <CodeBlock>{`name: Reqvire PR Commands

on:
  issue_comment:
    types: [created]

jobs:
  run-reqvire:
    if: |
      github.event.issue.pull_request != null &&
      (
        contains(github.event.comment.body, '/reqvire impact') ||
        contains(github.event.comment.body, '/reqvire traces') ||
        contains(github.event.comment.body, '/reqvire coverage')
      )
    runs-on: ubuntu-latest
    permissions:
      pull-requests: read
      issues: write
      contents: read

    steps:
      - name: Resolve pull request refs
        env:
          GH_TOKEN: \${{ secrets.GITHUB_TOKEN }}
        run: |
          HEAD_REF=$(gh pr view \${{ github.event.issue.number }} --json headRefName --jq '.headRefName')
          BASE_REF=$(gh pr view \${{ github.event.issue.number }} --json baseRefName --jq '.baseRefName')
          echo "HEAD_REF=$HEAD_REF" >> "$GITHUB_ENV"
          echo "BASE_REF=$BASE_REF" >> "$GITHUB_ENV"

      - name: Checkout pull request branch
        uses: actions/checkout@v6
        with:
          ref: \${{ env.HEAD_REF }}
          fetch-depth: 0

      - name: Compute merge base
        run: |
          git fetch origin "$BASE_REF"
          BASE_COMMIT=$(git merge-base "origin/$BASE_REF" HEAD)
          echo "BASE_COMMIT=$BASE_COMMIT" >> "$GITHUB_ENV"

      - name: Install Reqvire
        run: curl -fsSL https://raw.githubusercontent.com/reqvire-org/reqvire/main/scripts/install.sh | bash

      - name: Run requested report
        run: |
          if grep -q '/reqvire impact' <<< "\${{ github.event.comment.body }}"; then
            reqvire change-impact --git-commit "$BASE_COMMIT" > reqvire-report.md
          elif grep -q '/reqvire traces' <<< "\${{ github.event.comment.body }}"; then
            echo '<pre><code class="language-json">' > reqvire-report.md
            reqvire traces >> reqvire-report.md
            echo '</code></pre>' >> reqvire-report.md
          elif grep -q '/reqvire coverage' <<< "\${{ github.event.comment.body }}"; then
            reqvire coverage > reqvire-report.md
          fi

      - name: Comment with report
        uses: peter-evans/create-or-update-comment@v5
        with:
          issue-number: \${{ github.event.issue.number }}
          body-path: reqvire-report.md`}</CodeBlock>
        </div>
      </Section>

      <Footer />
    </div>
  );
}
