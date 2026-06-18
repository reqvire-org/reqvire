import { BulletList, CodeBlock, DetailGrid, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function Submodels() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">
        Submodels and Subgraphs
      </h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        The submodels report analyzes independent capability-rooted subgraphs
        and the explicit couplings between them. It is useful when reviewing
        architecture boundaries, attachment contracts, refactors, and AI
        context collection.
      </p>

      <Section title="What Is a Submodel">
        <p className="text-zinc-600 mb-4">
          A submodel is a graph rooted at a capability with no capability
          parent. Reqvire resolves capability hierarchy, specified requirements,
          requirement hierarchy, contracts, verifications, attachments, and
          implementation evidence from that root.
        </p>
        <BulletList
          items={[
            "Capability hierarchy uses derive and derivedFrom between capabilities.",
            "Requirements enter the graph through specify and specifiedBy.",
            "Requirement hierarchy uses derive and derivedFrom between requirements.",
            "Full mode reports each capability root as a submodel.",
            "Scoped mode reports the first independent branch roots below a selected capability or requirement.",
          ]}
        />
      </Section>

      <Section title="Why It Matters">
        <DetailGrid
          items={[
            {
              name: "Boundary review",
              desc: "Submodels reveal whether independent capability areas are cleanly separated or coupled through hidden hierarchy links.",
            },
            {
              name: "Attachment validation",
              desc: "Cross-submodel dependencies should be explicit one-way attachment contracts rather than hierarchy relations that blur ownership.",
            },
            {
              name: "Change impact",
              desc: "Change-impact analysis follows native relations and explicit attachments. Clear submodel boundaries make the resulting review scope easier to interpret and route.",
            },
            {
              name: "AI context",
              desc: "Assistants can collect the right capability-rooted context without dragging unrelated model areas into the prompt.",
            },
          ]}
        />
      </Section>

      <Section title="Cross-Submodel Couplings">
        <p className="text-zinc-600 mb-4">
          The report includes user-authored identifier relations from one
          requirement to another requirement when those requirements resolve to
          different capability-root ownership boundaries. Attachments are not
          counted in this coupling list; they are explicit dependency edges with
          their own attachment semantics.
        </p>
        <p className="text-zinc-600 mb-4">
          Use lint with auditable output when you need cleanup hints for the
          hierarchical subset of those problems, such as cross-boundary
          requirement hierarchy links that should usually be modeled with
          ownership-preserving hierarchy or explicit attachments instead.
        </p>
        <CodeBlock>{`reqvire submodels
reqvire submodels --json
reqvire submodels --from "API Authentication"

reqvire lint --auditable
reqvire lint --auditable --json`}</CodeBlock>
      </Section>

      <Section title="Output Shape">
        <DetailGrid
          items={[
            {
              name: "Submodels",
              desc: "Capability-rooted entries with requirements, contracts, verification context, and summary counts.",
            },
            {
              name: "Cross-Submodel Couplings",
              desc: "Explicit links that cross capability-root boundaries and require review.",
            },
            {
              name: "Summary",
              desc: "Totals for submodels, requirements, and coupling counts. JSON exposes submodels, cross_submodel_couplings, and summary.",
            },
          ]}
        />
      </Section>

      <Section title="Modeling Rule of Thumb">
        <p className="text-zinc-600">
          Keep shared domain meaning in ontology and attach it to consuming
          capabilities. Keep reusable obligation detail as requirement-owned
          contracts and attach those contracts from consuming requirements. Use
          hierarchy only when ownership really belongs inside the same
          capability-rooted subgraph.
        </p>
      </Section>

      <Footer />
    </div>
  );
}
