import { Link } from "react-router-dom";
import { BulletList, DetailGrid, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";

const advancedPages = [
  {
    name: "Verifications",
    href: "/verifications",
    desc: "Verification methods, evidence-backed verification rules, leaf requirement roll-up, and coverage checks.",
  },
  {
    name: "Implementation Coverage",
    href: "/implementation-coverage",
    desc: "Requirement implementation evidence, satisfiedBy links, reused-context contract coverage, and JSON report shape.",
  },
  {
    name: "Submodels and Subgraphs",
    href: "/submodels",
    desc: "Capability-rooted subgraphs, cross-submodel couplings, one-way reused contract context, and boundary review.",
  },
];

export default function Advanced() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">Advanced</h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Advanced Reqvire workflows focus on proving the model, measuring
        evidence, reviewing change impact, and keeping independent subgraphs
        clean enough for humans and AI assistants to reason about.
      </p>

      <Section title="Advanced Pages">
        <div className="space-y-4">
          {advancedPages.map((page) => (
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

      <Section title="Reports to Know">
        <DetailGrid
          items={[
            {
              name: "reqvire traces",
              desc: "Generates upward trace trees from verifications to owning capability roots and identifies redundant verify relations.",
            },
            {
              name: "reqvire change-impact",
              desc: "Shows elements that need review because requirements, contracts, reused contract context, verification links, or semantic dependencies changed.",
            },
            {
              name: "reqvire ontologies",
              desc: "Exports authored ontology and SHACL content, with full mode for generated Reqvire model context triples.",
            },
            {
              name: "reqvire resources",
              desc: "Lists files referenced by the model through relations so evidence artifacts remain visible.",
            },
          ]}
        />
      </Section>

      <Section title="Review Discipline">
        <BulletList
          items={[
            "Verify leaf requirements where possible and let coverage roll up through the hierarchy.",
            "Use satisfiedBy for implementation and evidence artifacts, not as a substitute for verification scope.",
            "Model cross-subgraph contract reuse with one-way Reused Contract Context.",
            "Run validate, lint, coverage, traces, submodels, and change-impact before high-risk reviews.",
          ]}
        />
      </Section>

      <Footer />
    </div>
  );
}
