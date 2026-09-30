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
        architecture boundaries, contract bindings, refactors, and AI
        context collection.
      </p>

      <Section title="What Is a Submodel">
        <p className="text-zinc-600 mb-4">
          A submodel is a graph rooted at a capability with no capability
          parent. Reqvire resolves capability hierarchy, specified requirements,
          requirement hierarchy, owned contracts, verifications, and implementation
          evidence from that root. Contract Bindings and Contract References
          record dependencies on contracts owned by other requirements.
        </p>
        <BulletList
          items={[
            "Capability hierarchy uses derive and derivedFrom between capabilities.",
            "Requirements enter the graph through specify and specifiedBy.",
            "Requirement hierarchy uses derive and derivedFrom between requirements.",
            "Full mode reports each capability root as a submodel.",
            "Selecting a capability with submodels --from reports that capability itself and its subtree.",
            "Selecting a requirement with submodels --from reports the first independent requirement branch roots below that boundary.",
          ]}
        />
      </Section>

      <Section title="Coverage Scope and Ownership">
        <p className="text-zinc-600 mb-4">
          <code>coverage --from</code> accepts capability names only, unlike
          <code> submodels --from</code>, which also accepts a requirement
          boundary. Coverage includes the selected capability, descendant
          capabilities, their specified requirements, and requirement
          descendants. It follows every valid parent path and counts each
          subject once within the selected scope. A nested capability scope
          does not create a new independent root or change ownership.
        </p>
        <p className="text-zinc-600 mb-4">
          A requirement still resolves to one independent capability root.
          External contract evidence can support a scoped requirement without
          becoming an owned subject of that scope. Shared verification totals
          may overlap between scopes because verification membership follows
          requirement targets rather than ownership. Explorer may choose one
          parent for a hierarchy row while preserving all model relationships.
        </p>
        <CodeBlock>{`reqvire coverage --from "Alpha Root" --json`}</CodeBlock>
      </Section>

      <Section title="Why It Matters">
        <DetailGrid
          items={[
            {
              name: "Boundary review",
              desc: "Submodels reveal whether independent capability areas are cleanly separated or coupled through hidden hierarchy links.",
            },
            {
              name: "Contract dependency validation",
              desc: "Contract Bindings express shared implementation obligations; Contract References express content dependencies for review. Both preserve the requirements' submodel ownership.",
            },
            {
              name: "Change impact",
              desc: "Change-impact analysis follows native relations, Contract Bindings, and Contract References. Clear submodel boundaries make the resulting review scope easier to interpret and route.",
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
          different capability-root ownership boundaries. Contract Bindings and
          Contract References target contract elements and remain separate from
          this requirement-to-requirement coupling list.
        </p>
        <p className="text-zinc-600 mb-4">
          Use lint with auditable output when you need cleanup hints for the
          hierarchical subset of those problems, such as cross-boundary
          requirement hierarchy links that should usually be modeled with
          ownership-preserving hierarchy or explicit Contract Bindings instead.
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
        <p className="text-zinc-600 mb-4">
          Keep shared domain meaning in ontology and reference it from consuming
          capabilities. Keep reusable obligation detail as requirement-owned
          contracts and reuse those contracts from consuming requirements. Use
          hierarchy only when ownership really belongs inside the same
          capability-rooted subgraph.
        </p>
        <p className="text-zinc-600">
          Place bindings on the requirements responsible for implementation.
          Use separate child bindings for independently implemented obligations;
          use a parent binding when the obligation applies to its entire
          requirement subtree and fulfillment is assessed through that
          subtree's implementation roll-up. A common ancestor alone does not
          justify moving bindings upward. Descendants inherit a parent binding
          and cannot repeat it.
        </p>
      </Section>

      <Footer />
    </div>
  );
}
