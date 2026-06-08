import { Footer } from "@/components/Footer";

export default function StrategicVision() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">
        Connected Engineering Knowledge for Verifiable Software
      </h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Reqvire turns scattered engineering intent into a structured semantic
        engineering model: ontologies, capabilities, requirements, verifications,
        implementation links, and evidence. That model stays close to the software
        lifecycle, can be reviewed and validated like code, and provides reliable
        context to engineers, AI agents, CI, reports, audits, and change-impact workflows.
      </p>

      <Section title="Constraining AI-Generated Engineering Work">
        <p className="text-zinc-600 mb-4">
          LLMs, AI agents, coding harnesses, and prompt-driven development are
          changing how software is produced. They can move quickly, but they also
          make it easier for engineering intent, assumptions, constraints, and
          verification obligations to disappear behind plausible code changes.
        </p>
        <p className="text-zinc-600">
          Reqvire gives these probabilistic workflows deterministic engineering
          anchors. The semantic engineering model provides explicit domain
          meaning, capabilities, requirements, contracts, verifications, and
          implementation links that humans and AI agents can review, query,
          validate, and use as controlled context during planning, coding, review,
          CI, and change-impact analysis.
        </p>
      </Section>

      <Section title="The Model is More Than a Requirements List">
        <p className="text-zinc-600 mb-4">
          It is a connected engineering knowledge model built from six core components:
        </p>
        <ul className="space-y-3">
          {[
            ["Ontologies", "define reusable domain meaning"],
            ["Capabilities", "define stable operational or system abilities"],
            ["Requirements", "define implementable obligations"],
            ["Refinements", "capture behavioral, state, semantic, and constraint detail"],
            ["Verifications", "prove that obligations and capability expectations are met"],
            ["Implementation artifacts", "show where requirements and evidence are realized"],
          ].map(([term, desc]) => (
            <li key={term} className="flex items-start gap-2.5">
              <span className="w-1.5 h-1.5 rounded-full bg-blue-500 mt-2 flex-shrink-0" />
              <span className="text-zinc-700">
                <strong className="text-zinc-900">{term}</strong> — {desc}.
              </span>
            </li>
          ))}
        </ul>
      </Section>

      <Section title="Where Reqvire Fits">
        <p className="text-zinc-600 mb-4">
          Reqvire sits at the intersection of semantic engineering, MBSE discipline,
          connected engineering knowledge, and AI-enabled engineering workflows.
        </p>
        <div className="flex flex-wrap gap-2 mb-4">
          {[
            "SysML and MBSE",
            "Semantic engineering",
            "Ontology-driven engineering",
            "Connected engineering knowledge",
            "Context engineering",
            "AI-enabled engineering workflows",
          ].map((tag) => (
            <span
              key={tag}
              className="px-3 py-1.5 bg-blue-50 text-blue-600 rounded-full text-[13px] font-medium"
            >
              {tag}
            </span>
          ))}
        </div>
        <p className="text-zinc-600">
          It keeps the traceability and lifecycle discipline of MBSE, but makes
          the engineering model explicit, reviewable, queryable, and usable as
          reliable context for software teams, AI agents, CI, reports, audits,
          and change-impact workflows.
        </p>
      </Section>

      <Section title="What MBSE Means Here">
        <p className="text-zinc-600 mb-4">
          Model-Based Systems Engineering shifts engineering from static
          documents toward connected models of system meaning, behavior,
          obligations, interfaces, verification, and evidence.
        </p>
        <p className="text-zinc-600 mb-4">
          In Reqvire, that model is lightweight, text-based, and close to the
          software lifecycle:
        </p>
        <ul className="space-y-3">
          {[
            ["Ontologies", "define reusable domain meaning"],
            ["Capabilities", "define stable operational or product abilities"],
            ["Requirements", "define implementable obligations"],
            ["Refinements", "capture behavioral, semantic, state, I/O contract, constraint, and specification detail"],
            ["Verifications", "prove that obligations and capability expectations are met"],
            ["Implementation artifacts", "show where requirements and evidence are realized"],
          ].map(([term, desc]) => (
            <li key={term} className="flex items-start gap-2.5">
              <span className="w-1.5 h-1.5 rounded-full bg-blue-500 mt-2 flex-shrink-0" />
              <span className="text-zinc-700">
                <strong className="text-zinc-900">{term}</strong> — {desc}.
              </span>
            </li>
          ))}
        </ul>
        <p className="text-zinc-600 mt-4">
          This makes the model useful to systems engineers, software engineers,
          reviewers, compliance stakeholders, AI agents, and automated engineering workflows.
        </p>
      </Section>

      <Section title="Why the Model Has Six Parts">
        <p className="text-zinc-600 mb-4">
          Reqvire separates engineering knowledge into six connected parts
          because each one answers a different question. Ontologies define domain
          semantics, relations, rules, and reusable engineering concepts. Semantic
          contracts apply that knowledge as SHACL-based constraints and machine-readable
          engineering context, giving teams and AI agents implementation-facing
          contracts that can be checked during validation, automation, and review
          workflows. Capabilities describe what the product or system must be able
          to do. Requirements define obligations. Refinements add behavioral and
          semantic detail. Verifications prove expectations are met. Implementation
          artifacts connect the model back to code, tests, and evidence.
        </p>
        <p className="text-zinc-600">
          Keeping these concerns separate but linked makes the model easier to
          review, query, validate, and evolve as the software changes.
        </p>
      </Section>

      <Footer />
    </div>
  );
}

function Section({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <section className="mb-12">
      <h2 className="text-2xl font-semibold text-zinc-900 mb-4">{title}</h2>
      {children}
    </section>
  );
}
