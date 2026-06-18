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
          Start the server from the repository that contains the Reqvire model,
          or pass a workspace from another directory. Reqvire validates the
          model before the MCP server starts.
        </p>
        <CodeBlock>{`reqvire mcp
reqvire --workspace /path/to/repository mcp
reqvire mcp --host 127.0.0.1 --port 8081`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Convenience no-install form:
        </p>
        <CodeBlock>{`npx -y @reqvire-org/reqvire@latest --workspace /path/to/repository mcp`}</CodeBlock>
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
          Browser-originated requests are local-safe by default: missing Origin
          headers and loopback origins are accepted, while non-loopback origins
          are rejected.
        </p>
      </Section>

      <Section title="Default Tools">
        <p className="text-zinc-600 mb-4">
          Default mode advertises read and report tools only. Tool results
          include text content for chat clients and structuredContent for
          clients that consume machine-readable data.
        </p>
        <div className="grid sm:grid-cols-2 gap-2">
          {[
            "reqvire.workspace_status",
            "reqvire.tool_contract",
            "reqvire.model_revision",
            "reqvire.read_element",
            "reqvire.search",
            "reqvire.model",
            "reqvire.containment",
            "reqvire.collect",
            "reqvire.submodels",
            "reqvire.semantic.ontologies",
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

      <Section title="Semantic Model Evidence">
        <DetailGrid
          items={[
            {
              name: "Ontology search",
              desc: "reqvire.search can filter ontology and semantic-contract elements and return parsed semantic content in full results.",
            },
            {
              name: "Read element",
              desc: "reqvire.read_element returns element details, relations, Reused Contract Context entries, concept references, and optional size estimates.",
            },
            {
              name: "Collect",
              desc: "reqvire.collect includes authored concept references for capability and requirement elements, and semantic-contract use context where the underlying operation returns it.",
            },
            {
              name: "Ontologies",
              desc: "reqvire.semantic.ontologies returns authored RDF ontology, SHACL, or both as Turtle or JSON-LD, with full mode for generated model context and include_external for local external ontology source triples.",
            },
            {
              name: "Semantic prefixes",
              desc: "reqvire.semantic.prefixes returns ontology-defined prefixes, namespaces, source element prose content, and a SPARQL prefix block for query construction.",
            },
            {
              name: "Semantic vocabulary",
              desc: "reqvire.semantic.vocabulary returns paged classes, properties, relation families, controlled vocabularies, semantic contracts, query patterns, source maps, diagnostics, and prefixes for query construction.",
            },
            {
              name: "SPARQL",
              desc: "reqvire.semantic.sparql runs read-only SPARQL queries against the model-owned Oxigraph semantic store and returns structured SELECT, ASK, CONSTRUCT, or DESCRIBE results.",
            },
            {
              name: "Prompts",
              desc: "MCP prompts provide build-time guidance for regular Reqvire exploration, change planning, coverage review, semantic query construction, semantic verification search, and contract-context search.",
            },
          ]}
        />
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
            "reqvire.workflow.verify_coverage",
            "reqvire.semantic.query",
            "reqvire.semantic.verification_search",
            "reqvire.semantic.contract_context_search",
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
            "Mutation tools use Reqvire core operations and return structured diffs.",
            "Most mutation tools support dry_run.",
            "HTTP mutation requests are serialized so concurrent clients cannot interleave filesystem writes.",
          ]}
        />
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
            "reused_contract_context_contract_violation",
            "single_root_ownership_violation",
            "filesystem_error",
          ]}
        />
      </Section>

      <Footer />
    </div>
  );
}
