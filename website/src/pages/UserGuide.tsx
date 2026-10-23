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
          workspace root. For a model spanning repositories, use a Central Git
          worktree with a versioned Repo manifest as described below.
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
        <p className="text-zinc-600 mt-4">
          For a client-managed project process, add <code>--transport stdio</code>
          to MCP startup. Use an absolute workspace path in a client&apos;s command
          and arguments. HTTP remains the default transport; mutation startup
          retains validation and clean-worktree ownership checks.
        </p>
      </Section>

      <Section title="Multi-repository Models">
        <p className="text-zinc-600 mb-4">
          Keep the model&apos;s composition in a versioned Google Repo manifest in
          a Central Git repository. Central is the workspace root; declared
          component checkouts contribute to the same model. Element paths stay
          relative to Central, even when an element belongs to a component.
        </p>
        <BulletList items={[
          "Install Git, Python 3, and Google Repo. Initialize the Repo client beside Central's Git checkout, then explicitly sync the component repositories before using Reqvire.",
          "Track the selected manifest and all its includes in Central. Declare unique, non-overlapping project paths inside Central and revisions available in the local checkouts. Each checkout must match the declared repository identity and selected revision.",
          "Exclude .repo/ and component checkout directories from Central's Git tracking. Declared component Git inputs remain visible to Reqvire even when Central ignores their directories.",
          "An initialized .repo/ selects manifest mode automatically. Use --manifest for an explicit Central-relative manifest selection. A selected manifest that cannot resolve fails; Reqvire never silently falls back or fetches missing commits.",
          "CLI, MCP and Explorer use the same combined source scope. Workspace .reqvireignore rules apply globally; each repository's Git ignores apply within its ownership boundary. Undeclared repositories and Repo metadata are excluded. Repo submanifests are not supported.",
        ]} />
        <CodeBlock>{`# The Central repository already tracks default.xml and its includes
# and ignores /.repo/ and /repositories/.
git clone https://github.com/example/central.git central
cd central
repo init -u https://github.com/example/central.git -b main -m default.xml
repo sync

# Initialization selects the manifest automatically
reqvire validate
# Or select another versioned manifest explicitly
reqvire --manifest default.xml search --json
reqvire --manifest default.xml serve --enable-mcp --enable-mutations`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Inspection does not require clean content. Mutation startup requires
          clean committed Central and component checkouts and Git author identity.
          Branch selection chooses Central&apos;s branch and its manifest; it does
          not require identically named component branches. See the MCP Server
          guide for private branch checkouts and component-first commits.
        </p>
      </Section>

      <Section title="Core Commands">
        <CommandList
          items={[
            { cmd: "reqvire validate", desc: "Parse and validate model structure, relations, Contract Bindings, Contract References, ontology, and semantic contracts." },
            { cmd: "reqvire lint", desc: "Find model quality issues such as redundant relations and cross-boundary hierarchy problems." },
            { cmd: "reqvire lint --auditable", desc: "Report remediation-ready structural findings." },
            { cmd: "reqvire search", desc: "Filter the model by type, file, name, content, relations, and governance metadata." },
            { cmd: "reqvire model", desc: "Emit the ontology/concept/capability-rooted model view as structured JSON." },
            { cmd: "reqvire traces", desc: "Generate verification trace graphs from verifications to owning capability roots." },
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

      <Section title="Authoring through MCP">
        <p className="text-zinc-600 mb-4">
          Start MCP with mutations enabled to create, replace, remove, move,
          rename, and merge elements or move files, folders, and assets.
          Typed authoring previews also require mutation enablement. Paths are
          relative to the selected worktree context's workspace root.
        </p>
        <CodeBlock>{`reqvire mcp --enable-mutations

// MCP tool calls
reqvire.add_element({file: "system-model/Auth.md", content: "<complete element Markdown>", dry_run: true})
reqvire.update_element({element_name: "Token Expiry Requirement", target: "Element Text", operation: "append", content: "When a token expires, the system SHALL reject it.", dry_run: true})
reqvire.update_element({element_name: "Token Expiry Requirement", target: "Element Text", operation: "append", content: "When a token expires, the system SHALL reject it.", expected_model_revision: "<base_model_revision from preview>"})
reqvire.add_element({file: "system-model/Auth.md", content: "<replacement Markdown>", override_existing: true})
reqvire.remove_element({element_name: "Obsolete Requirement", dry_run: true})
reqvire.move_element({element_name: "Token Expiry Requirement", file: "system-model/Identity/Auth.md"})
reqvire.rename_element({element_name: "Token Expiry Requirement", new_name: "Access Token Expiry Requirement"})
reqvire.merge_elements({target: "Access Token Validation", sources: ["Legacy Token Validation"], dry_run: true})
reqvire.move_file({source_file: "system-model/AuthDrafts.md", target_file: "system-model/Identity/Auth.md", squash: true})
reqvire.move_folder({source_folder: "system-model/IdentityDrafts", target_folder: "system-model/Identity"})
reqvire.move_asset({old_path: "docs/auth-flow.pdf", new_path: "docs/identity/auth-flow.pdf"})
reqvire.remove_asset({file_path: "docs/obsolete-auth-flow.pdf", dry_run: true})`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Each accepted mutation validates the complete candidate and persists
          canonical formatting in affected model documents. CLI authoring
          commands and their previews have been removed.
        </p>
        <p className="text-zinc-600 mt-4">
          For small edits, target <code>Element Text</code> or an exact subsection
          name with <code>reqvire.update_element</code>. Append/replace take
          <code> content</code>; replace_text takes exactly one literal
          <code> old_text</code> match and <code>new_text</code>. Optional prose
          subsections support explicit create/delete. Preview, review the diff,
          then apply with the preview's <code>base_model_revision</code> as
          <code> expected_model_revision</code> in the same worktree context.
          Protected declarations stay attached; Ontology/Shapes/Query support
          only replace and replace_text with complete fenced Markdown payloads.
        </p>
        <p className="text-zinc-600 mt-4">
          For a full definition change, read the current definition and submit
          complete replacement Markdown with <code>override_existing: true</code>.
          Its authored prose, metadata, relations, and dependency sections are
          replaced; include every outgoing declaration still intended. Children
          and incoming contract or concept references authored on other elements
          remain attached when the complete candidate is valid. Preview with
          <code> dry_run: true</code>, then apply the reviewed definition with
          <code> dry_run: false</code>. A cross-file replacement rewrites incoming
          destinations. Actual deletion still protects children from becoming
          orphaned.
        </p>
      </Section>

      <Section title="Linking and Contract Dependencies">
        <p className="text-zinc-600 mb-4">
          The typed MCP link and unlink tools manage relations, Contract Bindings,
          and Contract References. Use the dependency form that matches the
          consuming requirement's responsibility.
        </p>
        <CodeBlock>{`// MCP tool calls
reqvire.link({source: "Authentication Requirement", relation_type: "verifiedBy", target: "Auth Test Case"})
reqvire.link({source: "System Requirement", relation_type: "satisfiedBy", target: "src/auth/login.rs"})
reqvire.link({source: "Performance Requirement", relation_type: "bindContract", target: "Rate Limiting Constraint"})
reqvire.link({source: "API Documentation", relation_type: "referenceContract", target: "Error Response Specification"})
reqvire.unlink({source: "Authentication Requirement", target: "Auth Test Case"})
reqvire.relink({source: "Child Requirement", relation_type: "derivedFrom", from_target: "Old Parent", to_target: "New Parent", dry_run: true})`}</CodeBlock>
        <p className="text-zinc-600 mb-4">
          Use <code>bindContract</code> for an implementation obligation and
          <code> referenceContract</code> for a content dependency that should
          propagate change impact. A requirement cannot contain both dependency
          sections. The unlink tool detects either kind by its target.
        </p>
      </Section>

      <Section title="Validation, Formatting, and Linting">
        <p className="text-zinc-600 mb-4">
          MCP mutations validate before they write. Formatting application and safe lint repair require mutation enablement. Run validation and
          hygiene commands explicitly in pull requests and before publishing
          generated reports.
        </p>
        <CodeBlock>{`reqvire validate
reqvire validate --json --output reports/validate.json

// MCP tool calls: previews/inspection work in read-only mode
reqvire.format({fix: false})
reqvire.format({fix: true})
reqvire.format({fix: true, with_full_relations: true})
reqvire.lint({fix: true})

reqvire lint
reqvire lint --fixable
reqvire lint --auditable
reqvire lint --json --output reports/lint.json`}</CodeBlock>
      </Section>

      <Section title="Migrating Existing Models">
        <p className="text-zinc-600 mb-4">
          Migration remains CLI-only. MCP validates the model before starting,
          so migrate incompatible source before launching an MCP server.
          The migrate command previews changes by default; add --fix to apply them.
        </p>
        <CodeBlock>{`reqvire migrate
reqvire migrate --fix
reqvire validate`}</CodeBlock>
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
        <p className="text-zinc-600 mb-4">
          Each verification in a traces report includes a <code>trace_graph</code>
          {" "}with unique nodes and labelled edges. Shared ancestors appear once
          while preserving every path to owning capabilities. Direct and total
          requirement counts exclude capability nodes.
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
        <p className="text-zinc-600 mb-4">
          Model Flow supports directional model browsing. Select an element,
          file, or folder, then use Downstream to follow its realization chains
          or Upstream to inspect its ancestors. Page selections merge the chains
          of their elements and show shared nodes once. Upstream uses the
          model's inverse relation names and arrows toward ancestors. The
          separate layout control chooses horizontal or vertical placement;
          hover either icon group for its text tooltips. Both horizons are
          available in served and exported Explorer.
        </p>
        <p className="text-zinc-600 mb-4">
          In Traces, select a verification to open its interactive flow from
          directly verified requirements through their ancestors and owning
          capabilities. Switch between left-to-right and top-to-bottom layouts,
          expand branches, or open element details and source files from the
          cards. Select a file to see its verification overview. These controls
          are also available in exported Explorer.
        </p>
        <p className="text-zinc-600 mb-4">
          Selections in Model, Traces, Thesaurus, Ontologies and Coverage have
          shareable URLs. Copy the address to reopen the same item and worktree,
          or use reload, Back and Forward to restore earlier selections. Model
          links also retain the workspace mode; Traces links can identify a
          verification or file overview. Explicit links override remembered
          selections. Unavailable items show an overview with an explanation.
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
          retry automatically. Committed external artifact edits can be adopted
          through Refresh HEAD; changed model inputs require full validated admission.
        </p>
        <p className="text-zinc-600 mb-4">
          If a commit times out, MCP checks local Git automatically. A verified
          published commit is reconciled and returned as success without repeating
          the commit. Confirmed non-publication returns an error: an automatic
          mutation rolls back its changes, while <code>reqvire.git.commit</code>{" "}
          keeps accepted pending edits uncommitted. If verification cannot complete,
          the accepted snapshot remains readable and writes are disabled. Resolve
          conflicting evidence, then use <code>reqvire.git.reconcile</code> with the
          recorded <code>attempted_commit</code>; preview is the default, and{" "}
          <code>dry_run: false</code> applies verified recovery. Repair a stalled
          Git hook before committing again; reconciliation does not fix the hook.
        </p>
        <p className="text-zinc-600 mb-4">
          The branch picker at the far right of the shared header, after Search, theme and Help, is available with plain
          serving and read-only MCP too, and stays visible when the left pane is collapsed. It lists every local branch in the startup
          repository and loads only the branch you select, reusing its worktree or
          creating an isolated managed worktree. Each tab can browse a different model; its URL
          preserves the selection across reload and back/forward navigation.
          A successful switch updates the model, branch label, and asset links
          together and closes old element details. If loading fails or the worktree
          disappears, Explorer keeps the last valid view labelled with its branch
          and shows an error. During loading, a centered spinner appears over a
          dimmed, blurred view. On failure, click outside the error dialog, press
          Escape, or choose Close to return to the retained worktree; select the
          branch again to retry. Existing checkouts remain
          unchanged. Read-only selection accepts valid uncommitted content and
          checks source freshness through the existing cache. Mutation-enabled
          serving rejects dirty targets before first admission. Reselecting an
          active mutation context reuses its accepted model, including its own
          uncommitted changes. Unchanged data stays cached across switches, and
          prepared worktrees survive shutdown. Exported pages contain only their
          exported snapshot.
        </p>
        <p className="text-zinc-600 mb-4">
          In mutation-enabled serving, branch selection defaults to HEAD mode:
          selecting an admitted context checks and adopts an eligible external
          branch tip without replacing its worker. Use Refresh HEAD to do the same
          for the displayed context, or call <code>reqvire.worktree.refresh_head</code>
          with its exact local <code>branch</code>. Apply is the default; use
          <code> dry_run: true</code> for a check-only preview. Clean implementation
          commits and same-branch amendments retain model semantics while updating
          Git provenance and resource text. The tool makes no commit and works
          with automatic commits disabled.
        </p>
        <p className="text-zinc-600 mb-4">
          Branch-selected MCP calls use tool contract version 3. Open or create the branch
          first, then use <code>branch</code> on model reads, edits, commits,
          refresh and publication. Use <code>from_branch</code> to resolve a
          relative creation base. With multiple open contexts, explicitly select
          the branch on each request; there is no client-global branch switch.
          Prompts use the same selector and MCP resource URIs encode it as
          <code> ?branch=model%2Freview</code>. Branch names are the public interface in MCP results and inventory as well as Explorer’s picker, browser URLs and HTTP routes;
          opaque IDs remain private to runtime and cache handling. Clients on the same branch share the existing
          mutation serialization and revision checks.
        </p>
        <p className="text-zinc-600 mb-4">
          Changes to currently eligible model documents, consumed ontology sources
          and resolution dependencies block refresh; dependency protection follows
          currently eligible declaring owners. Changes to root ignore files are
          allowed, and their current rules govern classification and normal
          loading. Successful refresh also adopts those rules for later tools;
          preview and rejection keep the accepted rules. Policy-only changes may
          change model membership and revision. Ordinary evidence content may
          change while referenced targets must remain valid. Pending
          model edits, a dirty checkout, branch changes and unresolved Git/recovery
          operations also block it. Explorer keeps the accepted view and commit
          label, shows the affected paths and observed commit, and disables writes
          for an external HEAD mismatch. Resolve the diagnostic or explicitly
          readmit changed model inputs through <code>reqvire.worktree.open</code>.
          Polling observes staleness without automatically adopting external commits.
        </p>
      </Section>

      <Section title="Coverage in Explorer">
        <p className="text-zinc-600 mb-4">
          Coverage opens on Whole Model with progress metrics, compact verification-type
          and implementation-source breakdowns, capability drill-down and issue lists.
          Verification types share one proportional bar and an inline legend with all five counts, including zeros.
          The left pane nests every capability under a selectable Whole Model root. Select
          that root to clear capability selection and return to the complete model scope.
        </p>
        <p className="text-zinc-600 mb-4">
          Select a capability name to show its full scoped dashboard. Its authored name
          appears in the page title, with scoped metrics and requirement
          drill-down followed by unverified requirements, unimplemented requirements and
          unsatisfied test verifications. Empty capabilities keep the same dashboard with
          zero totals and explanatory empty issue sections. Orphaned verifications appear
          only in Whole Model because they have no capability membership.
        </p>
        <p className="text-zinc-600 mb-4">
          Capability-tree roots and siblings are sorted by name. Its chevrons expand child
          capabilities without changing selection. Main report rows rank sibling capabilities
          by verification coverage, then implementation coverage, then name, keeping each
          subtree together. There is no separate mode selector or View issues page.
        </p>
        <p className="text-zinc-600 mb-4">
          Capability selection updates the URL. Copy it to share the complete scoped view,
          or use Back and Forward to revisit scopes. Explicit scope links override saved
          preferences; older mode/category links resolve to their scope in the unified view.
          Branch changes restore that branch&apos;s scope independently. A capability removed
          during refresh returns to Whole Model with an explanation.
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
          Labelled bars, percentages and counts describe model coverage; they do not imply
          test execution results. Requirement blocker counts retain diagnostic detail.
        </p>
        <BulletList items={[
          "Opening evidence outside the selected subtree keeps the current scope.",
          "A shared verification is counted once within each scope and once globally. Scope totals need not add up to the whole-model total.",
          "Orphan verifications belong to the whole model. Select Whole Model in the capability tree to inspect them.",
          "Capability scope is remembered per project/worktree. Valid selections survive refresh; removed capabilities return to Whole Model with an explanation.",
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
