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
          option when running Reqvire from outside the effective workspace. The
          option applies to normal CLI commands and the MCP server. A workspace
          must contain at least one eligible Git worktree; files in non-Git
          workspace folders are ignored, and model paths remain relative to the
          workspace root. Mutating commands may operate across eligible
          worktrees in the same workspace, but they reject targets in non-Git
          workspace folders.
        </p>
        <CodeBlock>{`reqvire --workspace /path/to/workspace validate
reqvire --workspace /path/to/workspace search --filter-type requirement
reqvire --workspace /path/to/workspace mcp`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Convenience no-install command form:
        </p>
        <CodeBlock>{`npx -y @reqvire-org/reqvire@latest --workspace /path/to/workspace validate
npx -y @reqvire-org/reqvire@latest --workspace /path/to/workspace search --filter-type requirement
npx -y @reqvire-org/reqvire@latest --workspace /path/to/workspace mcp`}</CodeBlock>
      </Section>

      <Section title="Core Commands">
        <CommandList
          items={[
            { cmd: "reqvire validate", desc: "Parse and validate model structure, relations, Contract Bindings, Contract References, ontology, and semantic contracts." },
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

      <Section title="Managed Query Artifacts">
        <p className="text-zinc-600 mb-4">
          Discover native queries, validate their vocabulary context, and export
          standalone SPARQL for downstream tools. Namespace filtering selects the
          ontologies a query uses. Use the exact name or generated IRI from discovery
          to select one artifact.
        </p>
        <CodeBlock>{`reqvire semantic query list --json
reqvire semantic query list --namespace-base https://example.org/items
reqvire semantic query validate --json
reqvire semantic query export --name "Active Items Lookup" --output active-items.sparql
reqvire semantic query export --name "Active Items Lookup" --json
reqvire semantic query check --name "Active Items Lookup" --artifact active-items.sparql
reqvire semantic export --layer queries --output queries.ttl`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          JSON export includes the exact content and SHA-256 digest. Check compares
          the complete file and fails for stale or missing artifacts without rewriting
          them. File export replaces an artifact atomically after validation.
        </p>
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
            { cmd: "reqvire mv-folder <old> <new>", desc: "Move or rename a model folder while updating model references and local evidence paths." },
            { cmd: "reqvire mv-asset <old> <new>", desc: "Move a referenced non-model file and update references." },
            { cmd: "reqvire rm-asset <path>", desc: "Remove a referenced asset from the model." },
          ]}
        />
        <p className="text-zinc-600 mt-4">
          Element, file, folder, and asset paths are workspace-root-relative.
          In a parent workspace, move and add commands can target any eligible
          Git worktree under that root while rejecting non-Git workspace
          folders.
        </p>
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
reqvire mv-folder system-model/IdentityDrafts system-model/Identity
reqvire mv-asset docs/auth-flow.pdf docs/identity/auth-flow.pdf
reqvire rm-asset docs/obsolete-auth-flow.pdf --dry-run`}</CodeBlock>
        </div>
      </Section>

      <Section title="Linking and Contract Dependencies">
        <p className="text-zinc-600 mb-4">
          The link and unlink commands manage relations, Contract Bindings,
          and Contract References. Use the dependency form that matches the
          consuming requirement's responsibility.
        </p>
        <CodeBlock>{`reqvire link "Authentication" "specifiedBy" "Authentication Requirement"
reqvire link "Authentication Requirement" "verifiedBy" "Auth Test Case"
reqvire link "System Requirement" "satisfiedBy" "src/auth/login.rs"
reqvire link "Performance Requirement" bindContract "#rate-limiting-constraint"
reqvire link "API Documentation" referenceContract "Error Response Specification"

reqvire unlink "Authentication Requirement" "Auth Test Case"
reqvire unlink "Performance Requirement" "#rate-limiting-constraint"
reqvire relink "Child Requirement" "derivedFrom" "Old Parent" "New Parent"

reqvire link "Password Login Requirement" "derivedFrom" "Authentication Requirement" --dry-run
reqvire relink "Child Requirement" "derivedFrom" "Old Parent" "New Parent" --json`}</CodeBlock>
        <p className="text-zinc-600 mb-4">
          Use <code>bindContract</code> for an implementation obligation and
          <code> referenceContract</code> for a content dependency that should
          propagate change impact. A requirement cannot contain both dependency
          sections. The unlink command detects either kind by its target.
        </p>
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
          citations, including contracts reached through ownership, Contract
          Bindings, and Contract References. Repeated contract targets are
          included once.
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

      <Section title="Scoped Coverage">
        <p className="text-zinc-600 mb-4">
          Coverage defaults to the whole model. Use <code>--from</code> with an
          exact capability name to report its subtree. Both root and nested
          capabilities are supported; the selected capability itself is
          included. In this example, Alpha Root is a root capability and
          Shared Branch is a nested capability. Substitute your model's names.
        </p>
        <CodeBlock>{`reqvire coverage
reqvire coverage --json
reqvire coverage --from "Alpha Root"
reqvire coverage --from "Alpha Root" --json
reqvire coverage --from "Shared Branch"
reqvire coverage --from "Shared Branch" --json`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Names are case-sensitive. An unknown name or a requirement name is
          an error, with no whole-model fallback. Text and JSON report the
          same scope. Requirement classifications and evidence are retained
          from the complete model; external evidence does not enter the
          selected subject counts. A capability without requirements reports
          an empty scope with zero counts.
        </p>
      </Section>

      <Section title="Change Impact Workflow">
        <p className="text-zinc-600 mb-4">
          Change impact compares a base workspace snapshot with the current
          workspace snapshot and turns the difference into a review queue. The
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded mx-1">
            --git-commit
          </code>
          option materializes the base snapshot from the current eligible Git
          worktree; it is not a multi-repository commit selector.
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
        <p className="text-zinc-600 mb-4">
          With <code>reqvire serve --enable-mcp --enable-mutations</code>,
          visible Explorer tabs check the server's published manifest every five
          seconds and automatically adopt successful MCP mutations. Unchanged
          checks transfer no model content. Changed checks download only missing
          content chunks and replace the view after the complete update has been
          verified. Polling reads cached server data without scanning or rebuilding
          the model; successful MCP mutations rebuild the published snapshot.
          Updates preserve navigation without restarting the server or
          interrupting MCP access. Failed updates keep the last valid view and
          retry automatically. External file edits require a server restart.
        </p>
      </Section>

      <Section title="Coverage in Explorer">
        <p className="text-zinc-600 mb-4">
          The Coverage view starts with Scope set to Whole model beside the page title. Scope selects
          any capability subtree and updates all summary tiles, breakdowns, gap
          lists and sidebar counts together. Capabilities always
          appear in a hierarchy, with parents above their children. Roots and
          siblings are ordered by lowest verification coverage first, then
          implementation coverage, then name. Each subtree stays together so
          coverage gaps remain visible in context.
        </p>
        <p className="text-zinc-600 mb-4">
          Expand a capability to see its attached requirements, then expand a
          requirement to see its immediate children and their coverage status.
          Rows with children or binding consumers expand one level at a time.
          Terminal requirements show their coverage metrics directly in the row.
          Each capability provides access to its attached requirements, including
          requirements whose parent appears under another capability.
          Binding consumers lists
          requirements responsible for implementing the owner's shared contract obligations.
          Blocker counts accompany the requirement's
          implementation status.
          Select a requirement name to inspect its evidence in element details and follow
          local artifact links to the file-content viewer. Returning to Coverage keeps your selected scope.
          Verified, Partially verified, and Not verified describe the model's
          verification coverage; they are coverage labels rather than test execution results.
        </p>
        <BulletList items={[
          "Opening evidence outside the selected subtree keeps the current scope.",
          "A shared verification is counted once within each scope and once globally. Scope totals need not add up to the whole-model total.",
          "Orphan verifications belong to the whole-model view. Use View whole model in the Orphaned verifications panel to inspect them.",
          "Scope is remembered per project and preserved across valid live refreshes. If the selected capability disappears, Explorer returns to Whole model and explains the reset.",
        ]} />
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
          history when change-impact needs to compare the workspace against the
          pull request base branch in the eligible Git worktree.
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
