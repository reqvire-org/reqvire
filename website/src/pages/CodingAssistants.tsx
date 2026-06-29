import { BulletList, CodeBlock, DetailGrid, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function CodingAssistants() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">
        Coding Assistants
      </h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Reqvire integrates with coding assistants by exposing the engineering
        knowledge graph as structured, AI-ready context. Assistants can work
        from ontology, capability, requirement, contract, verification, and
        implementation evidence instead of scattered prose.
      </p>

      <Section title="Integration Options">
        <DetailGrid
          items={[
            {
              name: "Claude Code plugin",
              desc: "Marketplace plugin with Reqvire skills for model analysis, requirement authoring, verification work, impact analysis, and task generation.",
            },
            {
              name: "Codex skill package",
              desc: "Installable Reqvire skills for Codex workflows, including semantic engineering, ontology authoring, and concept/thesaurus authoring guidance.",
            },
            {
              name: "MCP server",
              desc: "Standard Streamable HTTP interface for MCP-capable clients. Read/report tools, resources, and prompts are available by default; mutation tools require explicit enablement.",
            },
          ]}
        />
      </Section>

      <Section title="Claude Code Plugin">
        <p className="text-zinc-600 mb-4">
          Add the Reqvire marketplace, install the plugin, then restart Claude
          Code. This is the default Claude Code installation path.
        </p>
        <CodeBlock>{`/plugin marketplace add https://github.com/reqvire-org/reqvire
/plugin install reqvire@reqvire-org`}</CodeBlock>
        <p className="text-zinc-600 mt-4 mb-4">
          Direct skill install is also available for environments that use the
          local Claude skills folder:
        </p>
        <CodeBlock>{`curl -fsSL https://raw.githubusercontent.com/reqvire-org/reqvire/main/scripts/install-claude-skill.sh | bash`}</CodeBlock>
        <div className="mt-4">
          <BulletList
            items={[
              "Skills: syseng, audit, ontology-authoring, and concept-authoring.",
              "The direct installer uses CLAUDE_HOME when set, otherwise it installs into ~/.claude/skills.",
              "Use the plugin for guided model exploration, authoring, impact review, coverage analysis, ontology work, and concept/thesaurus work.",
              "Skill workflows use the current Reqvire CLI and MCP tool surfaces instead of relying on legacy slash-command inventories.",
            ]}
          />
        </div>
      </Section>

      <Section title="Codex Skills">
        <p className="text-zinc-600 mb-4">
          Reqvire also provides Codex skills for semantic engineering workflows.
          They package guidance for system-model work without requiring users to
          memorize the full CLI and ontology conventions.
        </p>
        <CodeBlock>{`curl -fsSL https://raw.githubusercontent.com/reqvire-org/reqvire/main/scripts/install-codex-skill.sh | bash`}</CodeBlock>
        <p className="text-zinc-600 mt-4 mb-4">
          The installer uses{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            CODEX_HOME
          </code>{" "}
          when set, otherwise it installs into{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            ~/.codex/skills
          </code>
          . Restart Codex after installing or updating skills.
        </p>
        <BulletList
          items={[
            "reqvire-syseng: capability, requirement, contract, verification, traceability, and model-structure work.",
            "reqvire-audit: validation, linting, coverage, change-impact, and review evidence workflows.",
            "reqvire-ontology-authoring: authored OWL/Turtle vocabulary, semantic-contract boundaries, and ontology refactoring.",
            "reqvire-concept-authoring: native SKOS concept schemes, concepts, mappings, and concept-reference authoring.",
          ]}
        />
      </Section>

      <Section title="MCP Server">
        <p className="text-zinc-600 mb-4">
          Start Reqvire as an MCP Streamable HTTP server when an assistant
          should discover tools, resources, and prompts dynamically.
        </p>
        <CodeBlock>{`reqvire mcp
reqvire mcp --with-size-estimates
reqvire mcp --enable-mutations`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Convenience no-install form:
        </p>
        <CodeBlock>{`npx -y @reqvire-org/reqvire@latest --workspace /path/to/repository mcp`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Use{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            reqvire.semantic.ontologies
          </code>{" "}
          when an assistant needs ontology vocabulary. Use{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            reqvire.semantic.shapes
          </code>{" "}
          for SHACL,{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            reqvire.semantic.concepts
          </code>{" "}
          for SKOS concepts, and{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            reqvire.semantic.graph
          </code>{" "}
          for the combined graph. Pass{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            full: true
          </code>{" "}
          when it also needs generated model context triples for elements,
          relations, Contract Bindings entries, concept references,
          ontology declarations, and shape references.
        </p>
      </Section>

      <Section title="Recommended Workflow">
        <div className="bg-zinc-50 rounded-lg p-5 space-y-3">
          {[
            "Capability first: understand or define the operational ability being changed.",
            "Concept and ontology context: inspect or reuse curated terminology and structural semantic vocabulary that give the capability meaning.",
            "Requirements: define implementable obligations that specify the capability.",
            "Contracts: define requirements in precise terms: source basis, specifications, constraints, behavior, state, interfaces, and input/output semantics.",
            "Semantic contracts: add reusable SHACL shape profiles under the ontology plane and link them with constrainedBy/constrain and use/usedBy.",
            "Verifications: ensure leaf requirements are verified so capability coverage rolls up correctly.",
            "Implementation links: connect code, tests, proofs, and evidence with satisfiedBy.",
            "Validation: run validate, lint, coverage, traces, and change impact before review.",
          ].map((step, i) => (
            <div key={step} className="flex items-start gap-3">
              <span className="w-6 h-6 rounded-full bg-blue-100 text-blue-700 flex items-center justify-center text-xs font-semibold flex-shrink-0 mt-0.5">
                {i + 1}
              </span>
              <span className="text-zinc-700">{step}</span>
            </div>
          ))}
        </div>
      </Section>

      <Section title="Useful Prompts">
        <BulletList
          items={[
            "Show unverified leaf requirements and propose verifications.",
            "Explain the capability, concept and ontology context, requirements, and verification evidence for this change.",
            "Analyze change impact against main and summarize invalidated verifications.",
            "Create implementation tasks for the impacted requirements with traceability.",
            "Refactor this requirement to extract technical detail into specification elements.",
          ]}
        />
      </Section>

      <Section title="Use Cases">
        <div className="grid sm:grid-cols-2 gap-3">
          {[
            "Engineering graph exploration",
            "Coverage and traceability analysis",
            "Change impact assessment",
            "Capability planning",
            "Implementation task generation",
            "Concept and ontology context collection",
            "Requirement refactoring",
            "Verification hardening",
          ].map((use) => (
            <div
              key={use}
              className="flex items-start gap-2.5 bg-zinc-50 rounded-lg px-4 py-3"
            >
              <span className="w-1.5 h-1.5 rounded-full bg-blue-500 mt-2 flex-shrink-0" />
              <span className="text-sm text-zinc-700">{use}</span>
            </div>
          ))}
        </div>
      </Section>

      <Footer />
    </div>
  );
}
