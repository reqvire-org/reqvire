import { Link } from "react-router-dom";
import { BulletList, CodeBlock, DetailGrid, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";

const integrationPages = [
  {
    name: "MCP Server",
    href: "/mcp-server",
    desc: "Streamable HTTP MCP server, tool discovery, read/report tools, mutation mode, and semantic evidence.",
  },
  {
    name: "Coding Assistants",
    href: "/coding-assistants",
    desc: "Claude Code plugin, Codex skills, MCP workflows, assistant prompts, and AI-native Reqvire use cases.",
  },
];

export default function Integrations() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">
        Integrations
      </h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Reqvire integrations expose the same semantic engineering graph to
        coding assistants, CI jobs, GitHub workflows, and local tools. The goal
        is to keep model evidence close to the engineering work that changes
        it.
      </p>

      <Section title="Integration Pages">
        <div className="space-y-4">
          {integrationPages.map((page) => (
            <Link
              key={page.href}
              to={page.href}
              className="block border border-zinc-200 rounded-lg p-5 hover:border-blue-200 hover:bg-blue-50/40 transition-colors"
            >
              <h3 className="font-semibold text-zinc-900 mb-1">{page.name}</h3>
              <p className="text-sm text-zinc-600 leading-relaxed">
                {page.desc}
              </p>
            </Link>
          ))}
        </div>
      </Section>

      <Section title="Common Entry Points">
        <DetailGrid
          items={[
            {
              name: "CLI",
              desc: "Use reqvire validate, search, model, coverage, traces, submodels, ontologies, and serve in local or CI workflows.",
            },
            {
              name: "MCP",
              desc: "Run reqvire mcp for Streamable HTTP tools that MCP-capable assistants can discover and call; npx is available as a no-install convenience.",
            },
            {
              name: "Assistant packages",
              desc: "Install Claude Code plugins or Codex skills when the assistant environment supports native commands or skill files.",
            },
            {
              name: "GitHub",
              desc: "Run validation, coverage, traces, and change-impact checks in pull requests or issue-comment workflows.",
            },
          ]}
        />
      </Section>

      <Section title="CI Example">
        <CodeBlock>{`curl -fsSL https://raw.githubusercontent.com/reqvire-org/reqvire/main/scripts/install.sh | bash
reqvire validate
reqvire coverage --json --output reports/coverage.json
reqvire traces --output reports/traces.json
reqvire change-impact --json --output reports/impact.json`}</CodeBlock>
      </Section>

      <Section title="Integration Guardrails">
        <BulletList
          items={[
            "Run integrations from the repository root or pass --workspace explicitly.",
            "Keep MCP mutation mode disabled unless the client is trusted to write model files.",
            "Use JSON output for CI and assistant workflows that need stable machine-readable data.",
            "Use ontology and collect outputs when the assistant needs semantic context before editing.",
          ]}
        />
      </Section>

      <Footer />
    </div>
  );
}
