import { BulletList, CodeBlock, DetailGrid, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function McpServer() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">MCP Server</h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Reqvire can run as a Model Context Protocol server so MCP-capable
        coding assistants can inspect and operate on the engineering knowledge
        graph through structured tools instead of shell commands.
      </p>

      <Section title="Startup">
        <p className="text-zinc-600 mb-4">
          Start the server from the effective workspace root, or pass a
          workspace from another directory. The workspace must contain at least
          one eligible Git worktree; non-Git workspace folders are ignored.
          Reqvire validates the model before the MCP server starts.
        </p>
        <CodeBlock>{`reqvire mcp
reqvire --workspace /path/to/workspace mcp
reqvire mcp --host 127.0.0.1 --port 8081`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Convenience no-install form:
        </p>
        <CodeBlock>{`npx -y @reqvire-org/reqvire@latest --workspace /path/to/workspace mcp`}</CodeBlock>
      </Section>

      <Section title="HTTP Transport">
        <p className="text-zinc-600 mb-4">
          HTTP is the default MCP transport. The endpoint is fixed at{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            /mcp
          </code>
          , and Reqvire reports MCP protocol version{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            2025-11-25
          </code>
          .
        </p>
        <CodeBlock>{`{
  "mcpServers": {
    "reqvire": {
      "type": "http",
      "url": "http://127.0.0.1:8081/mcp"
    }
  }
}`}</CodeBlock>
        <CodeBlock>{`curl -sS \\
  -H 'Content-Type: application/json' \\
  -H 'Accept: application/json, text/event-stream' \\
  -H 'Mcp-Protocol-Version: 2025-11-25' \\
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' \\
  http://127.0.0.1:8081/mcp`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Requests without an Origin header and HTTP(S) loopback origins are
          accepted by default. Add other browser origins explicitly with the
          repeatable <code>--allow-origin</code> option.
        </p>
      </Section>

      <Section title="Local Project Stdio">
        <p className="text-zinc-600 mb-4">
          A stdio-capable client launches a dedicated Reqvire process for each
          configured project without a listening port. HTTP remains the default;
          select stdio explicitly in the client&apos;s command
          and arguments:
        </p>
        <CodeBlock>{`{
  "mcpServers": {
    "reqvire": {
      "type": "stdio",
      "command": "npx",
      "args": [
        "-y", "@reqvire-org/reqvire@latest",
        "--workspace", "/absolute/path/to/project",
        "mcp", "--transport", "stdio"
      ]
    }
  }
}`}</CodeBlock>
        <p className="text-zinc-600 mb-4">
          Configuration syntax depends on the client. An absolute workspace path
          selects the project even when the client launches from another directory.
          Native startup also accepts <code>reqvire mcp --transport stdio</code>.
          Both transports expose the same tools, resources, prompts, and
          permission checks. Append <code>--enable-mutations</code> for authoring;
          clean-worktree admission and exclusive ownership still apply, and
          <code> --enable-commits</code> remains a separate opt-in.
        </p>
        <p className="text-zinc-600">
          HTTP-only options <code>--host</code>, <code>--port</code>,
          <code> --allow-origin</code>, and <code>--allow-host</code> are rejected
          with stdio. Protocol messages use stdout; diagnostics use stderr. Closing
          stdin stops the process and its workers, releases ownership locks,
          and preserves pending model changes and managed worktrees. Embedded MCP
          through <code>serve --enable-mcp</code> continues to use HTTP.
        </p>
      </Section>

      <Section title="Hosting MCP">
        <p className="text-zinc-600 mb-4">
          <code>--host</code> selects the listening address. When you bind to
          a specific hostname or IP address, Reqvire accepts that endpoint host
          at the listening port automatically. Use your server&apos;s address:
        </p>
        <CodeBlock>{`reqvire mcp --host 192.0.2.50 --port 8081`}</CodeBlock>
        <p className="text-zinc-600 mb-4">
          For all interfaces, use <code>--host 0.0.0.0</code> and explicitly
          allow the hostname or IP that clients use. Repeat
          <code> --allow-host</code> for additional endpoint names:
        </p>
        <CodeBlock>{`reqvire mcp --host 0.0.0.0 --port 8081 \\
  --allow-host mcp.example.com --allow-host 192.0.2.50:8081

reqvire serve --enable-mcp --host 0.0.0.0 \\
  --allow-host explorer.example.com`}</CodeBlock>
        <p className="text-zinc-600 mb-4">
          Host values contain no scheme or path. A hostname without a port
          accepts that exact name on any port; adding a port restricts it to
          that authority. Subdomains need separate entries. IPv6 authorities
          use brackets, for example <code>[2001:db8::1]:8081</code>.
        </p>
        <p className="text-zinc-600 mb-4">
          For <code>https://mcp.example.com/mcp</code> behind a reverse proxy,
          keep the backend on loopback and permit the public host:
        </p>
        <CodeBlock>{`reqvire mcp --host 127.0.0.1 --port 8081 \\
  --allow-host mcp.example.com`}</CodeBlock>
        <p className="text-zinc-600">
          Configure the proxy to forward <code>/mcp</code> to
          <code> http://127.0.0.1:8081/mcp</code>, preserve the allowed Host
          header, and handle HTTPS and authentication. Forwarded-host headers
          alone do not authorize a hostname. Native MCP clients normally need
          this endpoint configuration without an additional browser origin.
        </p>
      </Section>

      <Section title="Browser Origins">
        <CodeBlock>{`reqvire mcp --allow-origin https://app.example.com \\
  --allow-origin http://192.0.2.10:3000

reqvire serve --enable-mcp --allow-origin https://app.example.com`}</CodeBlock>
        <p className="text-zinc-600 mb-4">
          Use the origin of the browser application: its scheme, hostname, and
          port, without a path. For example, a page at
          <code> https://app.example.com/chat</code> has origin
          <code> https://app.example.com</code>. This is separate from the
          Reqvire endpoint URL ending in <code>/mcp</code>.
        </p>
        <p className="text-zinc-600 mb-4">
          Additional origins retain loopback access. Matching uses the exact
          scheme, hostname, and effective port; omitted ports mean 80 for HTTP
          and 443 for HTTPS. Other ports and subdomains need their own entries.
          Wildcards and <code>null</code> origins are rejected. Permitted
          origins receive CORS preflight and response headers; unlisted origins
          receive HTTP 403.
        </p>
        <p className="text-zinc-600">
          The <code>--host</code> option controls the listening address.
          <code> --allow-origin</code> controls browser access to the MCP
          endpoint and applies to embedded MCP when <code>--enable-mcp</code>
          is present. Origin permission does not provide authentication;
          deployments use their own authentication and authorization controls.
        </p>
      </Section>

      <Section title="Default Tools">
        <p className="text-zinc-600 mb-4">
          Default mode advertises read and report tools only. Tool results
          include text content for chat clients and structuredContent for
          clients that consume machine-readable data.
        </p>
        <p className="text-zinc-600 mb-4">
          Read-only requests can run concurrently, using the machine&apos;s available
          CPU parallelism as the admission limit shared by clients. When capacity
          is full, MCP returns a retryable busy error (JSON-RPC code -32000 with
          data.retryable set to true). Clients should retry with backoff. Model
          reads continue checking for external changes.
        </p>
        <div className="grid sm:grid-cols-2 gap-2">
          {[
            "reqvire.workspace_status",
            "reqvire.branch.list",
            "reqvire.tool_contract",
            "reqvire.model_revision",
            "reqvire.read_element",
            "reqvire.search",
            "reqvire.model",
            "reqvire.containment",
            "reqvire.collect",
            "reqvire.submodels",
            "reqvire.semantic.export",
            "reqvire.semantic.ontologies",
            "reqvire.semantic.shapes",
            "reqvire.semantic.concepts",
            "reqvire.semantic.model",
            "reqvire.concepts.list",
            "reqvire.concepts.get",
            "reqvire.concept_schemes.list",
            "reqvire.concept_mappings.list",
            "reqvire.semantic.graph",
            "reqvire.semantic.queries",
            "reqvire.semantic.queries.validate",
            "reqvire.semantic.prefixes",
            "reqvire.semantic.vocabulary",
            "reqvire.semantic.sparql",
            "reqvire.lint",
            "reqvire.coverage",
            "reqvire.traces",
            "reqvire.resources",
            "reqvire.change_impact",
            "reqvire.format",
          ].map((tool) => (
            <code
              key={tool}
              className="text-xs font-mono text-blue-700 bg-blue-50 border border-blue-100 rounded px-2 py-1"
            >
              {tool}
            </code>
          ))}
        </div>
      </Section>

      <Section title="Scoped Coverage">
        <p className="text-zinc-600 mb-4">
          Call <code>reqvire.coverage</code> without arguments for the whole
          model. The optional string <code>from</code> selects a capability by
          its exact, case-sensitive name, including a nested capability.
          Replace the example name with a capability in your model.
        </p>
        <CodeBlock>{`{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": { "name": "reqvire.coverage", "arguments": {} }
}`}</CodeBlock>
        <CodeBlock>{`{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "reqvire.coverage",
    "arguments": { "from": "Alpha Root" }
  }
}`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Unknown names and names of non-capability elements return a
          structured tool error; they never fall back to the whole model.
          Scoped results use the same semantics as CLI and Explorer coverage:
          coverage is calculated against the complete current model before
          selecting the reported subjects. Evidence outside the selected
          subtree remains visible without adding its elements to the scope's
          subject counts. Scoped results include membership IDs in
          <code> scope</code>; orphan diagnostics remain a whole-model concern.
        </p>
      </Section>

      <Section title="Semantic Model Evidence">
        <DetailGrid
          items={[
            {
              name: "Ontology search",
              desc: "reqvire.search can filter ontology and semantic-contract elements and return parsed semantic content in full results.",
            },
            {
              name: "Read element",
              desc: "reqvire.read_element returns element details, relations, separate Contract Bindings and Contract References, concept references, and optional size estimates.",
            },
            {
              name: "Collect",
              desc: "reqvire.collect includes authored concept references for capability and requirement elements, and semantic-contract use context where the underlying operation returns it.",
            },
            {
              name: "Ontologies",
              desc: "reqvire.semantic.ontologies returns authored OWL/RDF ontology vocabulary as Turtle or JSON-LD. Used external subset materialization is requested through reqvire.semantic.export with the external-used layer.",
            },
            {
              name: "Semantic layers",
              desc: "reqvire.semantic.shapes returns SHACL shapes, reqvire.semantic.concepts returns SKOS concept scheme/thesaurus triples, reqvire.semantic.model returns generated model facts, and reqvire.semantic.export composes layers: ontologies, shapes, concepts, queries, model, external-used, and prefixes.",
            },
            {
              name: "Concept tools",
              desc: "reqvire.concept_schemes.list, reqvire.concepts.list, reqvire.concepts.get, and reqvire.concept_mappings.list expose standalone Thesaurus schemes, generated SKOS concepts, and mapsToConcept bridge inventory.",
            },
            {
              name: "Managed query artifacts",
              desc: "reqvire.semantic.queries lists native semantic-query elements. Optional name or iri selects one, namespace_base filters used ontology namespaces, and include_content adds standalone SPARQL plus SHA-256. reqvire.semantic.queries.validate returns candidate diagnostics and model validation status without executing query content. The queries RDF layer is also included in full semantic exports.",
            },
            {
              name: "Semantic prefixes",
              desc: "reqvire.semantic.prefixes returns ontology-defined prefixes, namespaces, source element prose content, and a SPARQL prefix block; include_external adds imported external prefixes with used-subset source metadata only.",
            },
            {
              name: "Semantic vocabulary",
              desc: "reqvire.semantic.vocabulary returns paged classes, properties, relation families, controlled vocabularies, semantic contracts, query patterns, source maps, diagnostics, and prefixes; ontology_document or ontology_base filters terms to one OWL document or used external source, and include_external adds used external vocabulary entries marked as external.",
            },
            {
              name: "SPARQL",
              desc: "reqvire.semantic.sparql runs read-only SPARQL queries against the model-owned Oxigraph semantic store; full mode is default, and include_external queries the store with the used external subset.",
            },
            {
              name: "Prompts",
              desc: "MCP prompts provide build-time guidance for Reqvire exploration, authoring, refactoring, change-impact audit, task generation, model-quality review, coverage review, semantic query construction, semantic verification search, contract-context search, and ontology/semantic-contract authoring.",
            },
          ]}
        />
      </Section>

      <Section title="Contract Reference Search">
        <p className="text-zinc-600 mb-4">
          Set <code>has_contract_references</code> to <code>true</code> to find
          requirements with at least one Contract Reference. Use{" "}
          <code>filter_contract_references</code> to match a referenced contract's
          normalized, workspace-relative identifier with a case-sensitive glob,
          such as <code>specifications/Errors.md#error-response-specification</code>.
          Matching uses the target identifier rather than the Markdown link label.
        </p>
        <CodeBlock>{`{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "reqvire.search",
    "arguments": {
      "has_contract_references": true,
      "filter_contract_references": "*#error-response-specification",
      "filter_name": ".*Documentation.*"
    }
  }
}`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          This request returns requirements whose names contain Documentation
          and that reference a matching error contract. Every supplied filter
          must match; omitting these filters preserves the usual search behavior.
          Invalid globs return a structured tool error.
        </p>
        <p className="text-zinc-600 mt-4">
          Search selects the reported requirements. Change-impact analysis
          follows Contract References from changed contracts to their consumers
          using the full model. A reference-only consumer contributes no
          implementation coverage or evidence to the contract owner.
        </p>
      </Section>

      <Section title="Prompts">
        <p className="text-zinc-600 mb-4">
          The server advertises MCP prompts in addition to tools and resources.
          Prompt templates are compiled into the Reqvire binary and retrieved
          through standard{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            prompts/list
          </code>{" "}
          and{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            prompts/get
          </code>{" "}
          requests.
        </p>
        <BulletList
          items={[
            "reqvire.workflow.explore_model",
            "reqvire.workflow.plan_change",
            "reqvire.workflow.generate_implementation_tasks",
            "reqvire.workflow.author_capability_requirement",
            "reqvire.workflow.author_or_align_verification",
            "reqvire.workflow.refactor_model_structure",
            "reqvire.workflow.audit_change_impact",
            "reqvire.workflow.author_concepts",
            "reqvire.workflow.model_quality_audit",
            "reqvire.workflow.verify_coverage",
            "reqvire.semantic.query",
            "reqvire.semantic.verification_search",
            "reqvire.semantic.contract_context_search",
            "reqvire.semantic.author_ontology_contract",
          ]}
        />
      </Section>

      <Section title="Size Estimates">
        <p className="text-zinc-600 mb-4">
          Start with{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            --with-size-estimates
          </code>{" "}
          when clients need approximate context sizing for model evidence. The
          flag is a server startup option, not a per-tool argument.
        </p>
        <CodeBlock>{`reqvire mcp --with-size-estimates`}</CodeBlock>
      </Section>

      <Section title="Mutation Mode">
        <p className="text-zinc-600 mb-4">
          Mutation tools are disabled by default. Enable them explicitly when an
          assistant should be allowed to modify the model.
        </p>
        <CodeBlock>{`reqvire mcp --enable-mutations`}</CodeBlock>
        <p className="text-zinc-600 mt-4 mb-4">
          Use <code>reqvire.update_element</code> for small content edits. Target
          <code> Element Text</code> for opening prose, or an exact subsection
          name. Append and replace take <code>content</code>; replace_text takes
          <code> old_text</code> matching exactly once within that target and
          <code> new_text</code>. Optional prose subsections support explicit
          create and delete. Preview first, then apply with
          <code> expected_model_revision</code> set to the preview's
          <code> base_model_revision</code> in the same worktree context. Stale
          revisions are rejected; identity and declarations stay attached.
        </p>
        <CodeBlock>{`// MCP tool calls: preview, review the diff, then apply
reqvire.update_element({element_name: "Token Expiry Requirement", target: "Element Text", operation: "append", content: "When a token expires, the system SHALL reject it.", dry_run: true})
reqvire.update_element({element_name: "Token Expiry Requirement", target: "Element Text", operation: "append", content: "When a token expires, the system SHALL reject it.", expected_model_revision: "<base_model_revision from preview>"})`}</CodeBlock>
        <p className="text-zinc-600 mt-4 mb-4">
          Ontology, Shapes and Query accept only replace or replace_text on their
          complete fenced Markdown payloads. Declaration sections such as
          Metadata, Relations, Contract References and Labels are protected.
          Missing subsections require explicit creation; Element Text cannot be
          created or deleted. Empty append and same-content replacement return
          empty diffs without changing the accepted revision.
        </p>
        <p className="text-zinc-600 mt-4 mb-4">
          For a full definition change, use <code>reqvire.add_element</code> with
          <code> override_existing: true</code> and its complete replacement
          Markdown. Read the current definition first: omitted authored metadata
          and outgoing declarations are removed. Incoming children, ownership,
          contract consumers, and concept references remain when the complete
          candidate is valid. Preview with <code>dry_run: true</code> before
          applying the same definition. A cross-file replacement updates incoming
          destinations; invalid replacements retain the accepted model and files.
        </p>
        <p className="text-zinc-600 mt-4 mb-4">
          MCP validates the model before starting. Migration remains CLI-only:
          preview with <code>reqvire migrate</code> and apply with{" "}
          <code>reqvire migrate --fix</code> before starting the server.
          There is no MCP migration tool or startup mode that bypasses validation.
        </p>
        <BulletList
          items={[
            "Mutation mode exposes typed CRUD, relation, file/folder, and asset tools, including their previews. It also enables format application and safe lint repair with fix: true; format preview and lint inspection remain available in read-only mode. Authored-model writes automatically format affected documents.",
            "Startup requires a clean Git worktree on a committed branch, a valid model, and Git author identity. Only one mutation-enabled MCP server may own the branch.",
            "Successful changes are validated and saved before the updated model is published. Automatic commits are off by default: HEAD and staging stay unchanged, and results omit the commit field.",
            "Add --enable-commits to commit each successful non-empty mutation locally and include its commit identifier in the result. This flag requires --enable-mutations. Previews, rejections, and no-ops never create commits.",
            "Both modes update subsequent MCP reads and refresh the embedded Explorer after a successful write.",
            "The running server owns its model snapshot. External edits are not imported and affected model files may be overwritten. Stop the server before editing its workspace externally.",
            "After a commit timeout, MCP checks local Git automatically. A verified published commit is reconciled and returns success without repeating the ref side effect. Confirmed non-publication returns an error: single-repository automatic mutations roll back their changes, while reqvire.git.commit keeps accepted pending edits uncommitted. Repo workspaces retain confirmed component commits and accepted pending changes until the final Central commit. If verification or recovery cannot complete, the accepted snapshot remains readable with a diagnostic and further writes are blocked. Resolve conflicting evidence, then inspect reqvire.git.reconcile with the exact attempted_commit (dry_run defaults to true) and apply dry_run: false for verified live recovery. Missing live evidence and other recovery failures require preservation, manual repair, validation, and normal admission. Repair a stalled hook before committing again.",
            "Restart requires the same clean-start checks. Use reqvire.git.commit for accepted pending changes before stopping, or resolve saved changes before restarting. Branch creation and remote publication require explicit tool calls.",
            "Leftover lock files are reused after their holders exit. A worker that survives its parent still owns its locks. If startup reports a lock error, check the named path and OS error: fix permissions for the server/container user or use a filesystem with locking support as indicated. Do not delete lock files to bypass an active owner.",
            "Most mutation tools support dry_run.",
            "Mutations, commits, and publication serialize within each worktree. Independent worktrees can progress separately.",
            "Model reads can run concurrently from the accepted snapshot. A read already in progress can finish with the earlier revision after a write; its context metadata identifies that revision. Reads admitted after publication use the updated model.",
            "Read and control requests have separate admission budgets based on available CPU parallelism. Full capacity returns a retryable busy error, so slow reads do not consume every slot needed for writes. Cancelling a client request does not cancel work already dispatched.",
          ]}
        />
        <CodeBlock>{`# Opt into automatic commits
reqvire mcp --enable-mutations --enable-commits

# Explorer with the same opt-in commit behavior
reqvire serve --enable-mcp --enable-mutations --enable-commits`}</CodeBlock>
      </Section>

      <Section title="Model branches and Explorer">
        <CodeBlock>{`# Browse local branches without mutation permissions
reqvire serve
# Add read-only MCP on the same listener
reqvire serve --enable-mcp
# Manage model branches and edit models through MCP
reqvire serve --enable-mcp --enable-mutations
# Explorer: http://localhost:8080/
# MCP:      http://localhost:8080/mcp`}</CodeBlock>
        <p className="text-zinc-600 mb-4">
          The branch picker lists local branches in every serving mode. Models load
          on demand: selecting a branch reuses its worktree or creates an isolated
          managed worktree, without switching an existing checkout. Read-only loads
          accept valid uncommitted content and check the existing model cache for
          changed inputs. Mutation-enabled serving requires a clean worktree and
          normal ownership admission before loading a new context; dirty targets
          are rejected. Active contexts reuse their accepted model, including
          accepted uncommitted writes when automatic commits are disabled.
        </p>
        <p className="text-zinc-600 mb-4">
          Mutation-enabled Explorer selects branches in HEAD mode. Selecting an
          admitted context checks an external branch-tip change through the same
          operation as the Refresh HEAD button and MCP tool. Eligible implementation
          commits and same-branch amendments update accepted Git provenance and
          resource bytes through normal validated model loading, retaining the semantic
          revision for unchanged model inputs and the same branch context. Polling reports staleness without adopting a new commit.
        </p>
        <p className="text-zinc-600 mb-4">
          Each browser tab keeps its own branch selection in the URL across reload
          and back/forward navigation. Switching retains cached context data and
          does not redirect MCP calls or preload other branches. Use
          <code> reqvire.branch.list</code> to find open branches for MCP
          requests. Prepared worktrees and their changes survive server shutdown.
        </p>
        <p className="text-zinc-600 mb-4">
          The branch-management workflow below uses tool contract version 4.
          Check discovery and contract compatibility when upgrading an older
          client or build. Previous <code>reqvire.worktree.*</code> tool names
          are unavailable. Opening a branch registers
          its model session; subsequent requests select it explicitly without
          switching the original checkout or a global active branch.
        </p>
        <DetailGrid items={[
          { name: "reqvire.branch.create", desc: "Requires destination branch and base_ref. Uses a local commit or ref without fetching. With multiple contexts, pass from_branch for relative bases such as HEAD. The selected source branch must already be open and clean." },
          { name: "reqvire.branch.open", desc: "Requires an existing local branch. Reuses a healthy context or admits its clean worktree, creating a managed worktree when necessary. It never forces a second checkout." },
          { name: "reqvire.branch.refresh", desc: "Checks and adopts the selected owned branch's local HEAD. dry_run defaults to false; true checks only. After the guard passes, uses normal validated loading. Requires a clean tree, no accepted pending changes, the same branch and no unresolved Git/recovery operation. Creates no commit and works without --enable-commits." },
          { name: "reqvire.branch.close", desc: "Requires branch. Closes a clean, server-created secondary context and cleans up its managed working directory while preserving the Git branch and commits. Original and externally prepared contexts are protected. Reopen the retained branch explicitly before using it again." },
          { name: "reqvire.git.commit", desc: "Requires a message and commits only accepted pending changes. Unrelated staged files and intervening external versions are excluded. No pending changes means no new commit. This tool works without GitHub enablement." },
          { name: "reqvire.git.reconcile", desc: "Verifies a live recorded unknown-outcome commit using exact attempted_commit and context routing. Preview is the default; dry_run: false completes only its affected index entries and accepted state after all checks pass. It preserves unrelated recorded staging and never updates a ref or retries a commit." },
        ]} />
        <CodeBlock>{`{
  "name": "reqvire.branch.create",
  "arguments": { "branch": "model/review", "base_ref": "main" }
}

{
  "name": "reqvire.coverage",
  "arguments": { "branch": "model/review" }
}

{
  "name": "reqvire.branch.refresh",
  "arguments": { "branch": "model/review", "dry_run": true }
}`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Refresh permits ordinary implementation, evidence, documentation and
          configuration edits, including changed <code>satisfiedBy</code> content,
          while referenced targets must remain valid. It blocks recognised model
          Markdown under current exclusions, consumed semantic sources and
          source-resolution changes whose declaring owners remain eligible.
          This includes new or converted model documents,
          ignored or outside-worktree semantic sources, and new files shadowing
          an existing source fallback. A protected role wins over an evidence role;
          unreferenced RDF and excluded examples have no automatic protected role.
          Changes to root <code>.gitignore</code> and <code>.reqvireignore</code>
          are allowed: current rules govern both versions of changed paths, normal
          loading and subsequent tools. Policy-only changes can include or exclude
          unchanged documents and change the model revision. Preview and rejection
          keep the accepted exclusion rules; successful refresh adopts the current rules.
          Results distinguish <code>accepted_head</code> from <code>observed_head</code>
          and return <code>refreshed</code>, <code>unchanged</code>, <code>checked</code>
          or <code>blocked</code>, with bounded path-specific blockers and next actions.
          Every apply repeats the checks. A rejection keeps accepted reads and
          the prior Explorer publication; external HEAD staleness disables writes.
          Changed model inputs require explicit full validated admission through
          <code> reqvire.branch.open</code>. Unknown commit outcomes still use
          <code> reqvire.git.reconcile</code>.
        </p>
        <p className="text-zinc-600 mt-4">
          Pass the exact local <code>branch</code> to workspace-bound tools
          and prompts. MCP resources use <code>?branch=model%2Freview</code>.
          Optional selection may be omitted with one open context; multiple
          contexts require a branch. Unknown or unopened branches fail without
          fallback or automatic opening. Version 4 rejects the removed
          <code> worktree_id</code> and <code>from_worktree_id</code> inputs.
          Branch names are the public interface for selectors, response context, inventory, the Explorer picker, browser URLs and HTTP routes. Opaque IDs stay private to worker sessions, runtime publication and caches.
          Clients targeting the same branch share its serialization and revision
          checks; a branch selector does not grant exclusive client ownership.
          Each result identifies its
          context and accepted model revision. Discovery and branch inventory
          are server-wide. Restart preserves worktree files and branches but
          requires clean admission again.
        </p>
      </Section>

      <Section title="Repo Workspaces">
        <p className="text-zinc-600 mb-4">
          Repo&apos;s active manifest makes Central and its declared components one
          model. Install Git, Python 3 and Google Repo, initialize and sync the
          client, then start from Central. The User Guide covers preparation.
          An initialized <code>.repo/manifest.xml</code> selects the native manifest,
          including its includes and local overlays. No root copy is needed. An explicit
          <code> --manifest</code> selects another manifest file location,
          preserving the same format and Git versioning.
        </p>
        <CodeBlock>{`reqvire mcp --enable-mutations

# Or expose Explorer and MCP together
reqvire serve --enable-mcp --enable-mutations

# Optional: use another manifest file location
reqvire --manifest .repo/manifests/review.xml mcp --enable-mutations`}</CodeBlock>
        <BulletList items={[
          "The branch argument selects a Central worktree. Repo-managed manifest inputs are copied privately and retained across removal and reopening. Files belonging to Central follow that worktree's Git history. Preparation uses local objects without a network sync.",
          "The same typed mutation tools edit Central-relative paths. Each file is saved in its owning repository. By default the edits remain pending and create no commits.",
          "reqvire.git.commit saves changed components first and updates pins in the selected manifest declarations, preserving their composition. Repo-managed files retain their manifest project's Git workflow; Central-owned files are checkpointed with Central. Unchanged components create no commit; --enable-commits uses the same workflow automatically.",
          "If a later save fails, confirmed component commits remain retained. The accepted model stays readable and pending changes remain until the selected checkpoint succeeds. Inspect per-project outcomes before retrying; an unknown ref outcome blocks writes until reconciled.",
          "With GitHub publication enabled, required component commits are pushed before Central. Manifest Git commits and publication remain with their owning repository; --manifest does not change ownership. Pull-request operations address Central; callers cannot substitute arbitrary repositories.",
          "Changing the selected manifest or project HEAD externally requires full validated admission. Refresh HEAD does not silently replace the composition of an accepted model.",
        ]} />
        <CodeBlock>{`{
  "name": "reqvire.branch.create",
  "arguments": { "branch": "model/review", "base_ref": "main" }
}
{
  "name": "reqvire.rename_element",
  "arguments": {
    "branch": "model/review",
    "element_name": "Component Requirement",
    "new_name": "Reviewed Component Requirement",
    "dry_run": true
  }
}
{
  "name": "reqvire.git.commit",
  "arguments": { "branch": "model/review", "message": "Review component requirement" }
}`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Preview the mutation, apply it with <code>dry_run: false</code>, then
          commit. Context metadata reports the selected manifest, project commits
          and per-project pending paths. No project selector or separate model
          namespace is needed.
        </p>
        <h3 className="text-lg font-semibold text-zinc-900 mt-6 mb-3">
          Historical Change Impact
        </h3>
        <p className="text-zinc-600 mb-4">
          <code>reqvire.change_impact</code> compares complete Repo compositions
          using the same baseline rules as the CLI. <code>git_commit</code>
          is the sole baseline selector, defaulting to the workspace-root
          repository&apos;s accepted HEAD. That commit supplies the root model,
          manifest and includes. The historical manifest supplies full commit
          IDs for components; the root ref is never applied independently to
          them. At server startup, use global <code>--manifest</code> to select
          a manifest tracked in the workspace-root repository:
        </p>
        <CodeBlock>{`reqvire --manifest default.xml mcp --enable-mutations

{
  "name": "reqvire.change_impact",
  "arguments": {
    "branch": "model/review",
    "git_commit": "origin/main"
  }
}`}</CodeBlock>
        <p className="text-zinc-600 mb-4">
          A selected manifest with independent Git history reports
          <code> repo_impact_manifest_history_unavailable</code>, naming the
          input and directing the caller to a tracked workspace-root manifest.
          The comparison never guesses a second revision. Mutable
          historical project pins report <code>repo_impact_base_unlocked</code>,
          and unavailable project objects report
          <code> repo_impact_base_unavailable</code>. Inputs must be versioned
          and locally available; analysis never fetches or syncs.
        </p>
        <p className="text-zinc-600">
          In mutation-enabled contexts, including embedded MCP, the current side
          is the selected branch&apos;s accepted model, captured composition and
          accepted pending edits. An impact request does not import external
          file or manifest changes. Baseline failures preserve that model,
          revision, pending changes and write availability. The report&apos;s
          <code> comparison</code> records base/current workspace-root commits,
          manifest inputs and project provenance. Inspect pending-state metadata
          separately: recorded commits do not imply pending bytes are committed.
        </p>
      </Section>

      <Section title="GitHub Publication">
        <CodeBlock>{`reqvire mcp --enable-mutations --enable-github
reqvire serve --enable-mcp --enable-mutations --enable-github
# Optional remote override (default: origin)
reqvire mcp --enable-mutations --enable-github --github-remote upstream`}</CodeBlock>
        <p className="text-zinc-600 mb-4">
          GitHub support is off by default and requires mutation mode. At startup,
          Reqvire checks the installed <code>gh</code> command, active authentication
          for the remote host, and repository access. If a check fails, publication
          tools are hidden and direct calls are rejected. Local model, worktree,
          and commit tools remain available. Inspect <code>reqvire.tool_contract</code>
          or worktree inventory for the reason, fix the setup, and restart.
        </p>
        <DetailGrid items={[
          { name: "reqvire.git.push", desc: "Pushes the context's accepted committed HEAD to the same branch in the pinned repository. Requires a clean worktree with no pending accepted changes. It never force-pushes or implicitly commits." },
          { name: "reqvire.github.pr.create", desc: "Requires base, title, and body; draft is optional. Push the branch first. The base can be main or another feature branch for a stacked PR. An existing matching open PR is returned without duplication or edits; conflicting bases are reported." },
          { name: "reqvire.github.pr.comment", desc: "Requires a positive pr_number and non-empty body. Adds the exact comment to a PR in the pinned repository, including a PR on another branch. Pending local changes are allowed; commenting never commits or pushes them." },
        ]} />
        <p className="text-zinc-600 mt-4">
          Publication uses the startup repository and configured remote. In Repo mode, declared component repositories are also checked and pinned at admission; dependency commits are published first. Forks
          are not supported. Automatic commits remain a separate opt-in through
          <code> --enable-commits</code>. Repository access at startup does not
          guarantee permission for every operation: push rules and later permission
          changes appear in the tool result. Results distinguish completed, no-op,
          failed, and unknown outcomes. If the response was lost after a possible
          remote change, inspect the returned branch or PR before retrying; comments
          are never automatically posted again.
        </p>
      </Section>

      <Section title="Error Handling">
        <p className="text-zinc-600 mb-4">
          Tool execution errors are returned as MCP tool results with{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            isError: true
          </code>
          . Structured payloads include a stable Reqvire error code, message,
          tool name, recoverability hint, and related validation errors when
          available.
        </p>
        <BulletList
          items={[
            "validation_failed",
            "duplicate_element",
            "element_not_found",
            "invalid_relation_type",
            "contract_bindings_contract_violation",
            "single_root_ownership_violation",
            "filesystem_error",
          ]}
        />
      </Section>

      <Footer />
    </div>
  );
}
