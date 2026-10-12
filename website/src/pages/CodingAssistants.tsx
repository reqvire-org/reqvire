import { BulletList, CodeBlock, DetailGrid, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";

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
              name: "Agent skills",
              desc: "One shared skill collection for Codex, Claude Code, and other supported coding assistants. Install with the skills CLI for model analysis, authoring, verification, and task generation.",
            },
            {
              name: "MCP server",
              desc: "Standard Streamable HTTP interface for MCP-capable clients. Read/report tools, resources, and prompts are available by default; mutation tools require explicit enablement.",
            },
          ]}
        />
      </Section>

      <Section title="Install Skills">
        <p className="text-zinc-600 mb-4">
          Install the shared collection from the repository's main branch.
          The installer lets you choose skills and target agents; installation
          is scoped to the current project by default.
        </p>
        <CodeBlock>{`npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills`}</CodeBlock>
        <p className="text-zinc-600 mt-4 mb-4">
          To install all four skills globally for both Codex and Claude Code:
        </p>
        <CodeBlock>{`npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills --agent codex claude-code --skill '*' --global`}</CodeBlock>
        <p className="text-zinc-600 mt-4 mb-4">
          Or select one skill for a project:
        </p>
        <CodeBlock>{`npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills --agent codex --skill reqvire-syseng`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Rerun the same install command to refresh from main, then restart your
          assistant. Installing skills does not start the MCP server or enable
          model mutations.
        </p>
        <Dialog>
          <DialogTrigger asChild>
            <button
              type="button"
              className="mt-4 text-sm text-blue-700 underline underline-offset-4 hover:text-blue-900 focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-blue-700"
            >
              Migrating from the Claude plugin?
            </button>
          </DialogTrigger>
          <DialogContent
            aria-labelledby="skills-migration-title"
            aria-describedby="skills-migration-description"
            className="bg-white text-zinc-900 max-h-[calc(100dvh-2rem)] overflow-y-auto"
          >
            <DialogHeader>
              <DialogTitle id="skills-migration-title">
                Migrate to shared skills
              </DialogTitle>
              <DialogDescription id="skills-migration-description">
                The Claude marketplace plugin and custom curl installers are
                deprecated. The shared skills replace both assistant packages.
              </DialogDescription>
            </DialogHeader>
            <ol className="list-decimal pl-5 space-y-3 text-sm text-zinc-600">
              <li>
                In Claude Code, open <code>/plugin</code> and disable or uninstall
                the Reqvire marketplace plugin.
              </li>
              <li>Install the shared skills for Claude Code:</li>
            </ol>
            <CodeBlock>{`npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills --agent claude-code --skill '*' --global`}</CodeBlock>
            <p className="text-sm text-zinc-600">
              Restart Claude Code. Skill names now use the <code>reqvire-*</code>
              prefix, including <code>reqvire-syseng</code> and{" "}
              <code>reqvire-audit</code>.
            </p>
          </DialogContent>
        </Dialog>
      </Section>

      <Section title="Included Skills">
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
        <CodeBlock>{`npx -y @reqvire-org/reqvire@latest --workspace /path/to/workspace mcp`}</CodeBlock>
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
