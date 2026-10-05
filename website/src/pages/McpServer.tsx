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
          The server uses MCP Streamable HTTP. The endpoint is fixed at{" "}
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
            "reqvire.worktree.list",
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
        <BulletList
          items={[
            "Mutation mode adds add, remove, move, rename, merge, link, unlink, relink, move-asset, and remove-asset tools.",
            "Startup requires a clean Git worktree on a committed branch, a valid model, and Git author identity. Only one mutation-enabled MCP server may own the branch.",
            "Successful changes are validated and saved before the updated model is published. Automatic commits are off by default: HEAD and staging stay unchanged, and results omit the commit field.",
            "Add --enable-commits to commit each successful non-empty mutation locally and include its commit identifier in the result. This flag requires --enable-mutations. Previews, rejections, and no-ops never create commits.",
            "Both modes update subsequent MCP reads and refresh the embedded Explorer after a successful write.",
            "The running server owns its model snapshot. External edits are not imported and affected model files may be overwritten. Stop the server before editing its workspace externally.",
            "If mutation recovery fails, the last accepted model remains readable with a recovery diagnostic. Further edits, previews, commits, pushes and PR operations are blocked. Explorer labels the accepted snapshot; local file downloads and live change-impact analysis remain unavailable until recovery. Stop the server, repair and validate the worktree, then restart under the usual clean-worktree checks.",
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

      <Section title="Worktrees and Explorer">
        <CodeBlock>{`# Browse local branches without mutation permissions
reqvire serve
# Add read-only MCP on the same listener
reqvire serve --enable-mcp
# Manage worktrees and edit models through MCP
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
          Each browser tab keeps its own branch selection in the URL across reload
          and back/forward navigation. Switching retains cached context data and
          does not redirect MCP calls or preload other branches. Use
          <code> reqvire.worktree.list</code> to obtain admitted context IDs for MCP
          requests. Prepared worktrees and their changes survive server shutdown.
        </p>
        <DetailGrid items={[
          { name: "reqvire.worktree.create", desc: "Requires branch and base_ref. Uses a local commit or ref without fetching. With multiple contexts, pass from_worktree_id for relative bases such as HEAD. The selected source must be clean." },
          { name: "reqvire.worktree.open", desc: "Requires an existing local branch. Reuses a healthy context or admits its clean worktree, creating a managed worktree when necessary. It never forces a second checkout." },
          { name: "reqvire.worktree.remove", desc: "Requires worktree_id. Removes only clean, server-created secondary worktrees and preserves the branch and commits. Original and externally created worktrees are protected." },
          { name: "reqvire.git.commit", desc: "Requires a message and commits only accepted pending changes. Unrelated staged files and intervening external versions are excluded. No pending changes means no new commit. This tool works without GitHub enablement." },
        ]} />
        <CodeBlock>{`{
  "name": "reqvire.worktree.create",
  "arguments": { "branch": "model/review", "base_ref": "main" }
}

{
  "name": "reqvire.coverage",
  "arguments": { "worktree_id": "<returned worktree_id>" }
}`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Pass the returned <code>worktree_id</code> to workspace-bound tools
          and prompts. Resources use <code>?worktree_id=...</code>. Omission
          works with one admitted context; with multiple contexts it is an error.
          IDs belong to the current server session. Each result identifies its
          context and accepted model revision. Discovery and worktree inventory
          are server-wide. Restart preserves worktree files and branches but
          requires clean admission again.
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
          Only the startup repository and configured remote are supported; forks
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
