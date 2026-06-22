import { Link } from "react-router-dom";
import { DetailGrid } from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function SemanticModel() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">
        Reqvire Semantic Model
      </h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        The semantic model is Reqvire's typed representation of
        engineering knowledge. It defines the element types, relations, ownership
        rules, semantic references, verification links, and implementation evidence
        that make traceability queryable and validation possible.
      </p>

      <Section title="High-Level Structure">
        <p className="text-zinc-600 mb-4">
          Reqvire is not organized as one large parent-child tree. The model is
          built from independent layers and subgraphs that are connected through
          explicit relations where the modeling rules allow them.
        </p>
        <ul className="space-y-3">
          {[
            ["Ontology layer", "reusable structural semantic definitions, relations, rules, and model vocabulary"],
            ["Conceptual layer", "native concept-scheme and concept elements that generate SKOS thesaurus resources for terminology, taxonomy, and search"],
            ["Capability and requirement subgraphs", "one or more independent product or system intent structures with requirements and contracts"],
            ["Verification layer", "tests, proofs, analysis, inspection, and demonstration evidence linked to the requirements they verify"],
            ["Implementation evidence", "code, tests, reports, documents, and other artifacts that satisfy requirements or provide verification evidence"],
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
          Concept references bind prose to SKOS concepts. Reused Contract
          Context brings reusable requirement-owned contracts into scope without
          forcing unrelated concerns into the same hierarchy.
        </p>
      </Section>

      <Section title="Ownership Rules">
        <DetailGrid
          items={[
            {
              name: "Capability roots",
              desc: "A capability with no capability parent is a capability-rooted submodel boundary. Child capabilities use derive or derivedFrom within the capability family.",
            },
            {
              name: "Requirement ownership",
              desc: "A requirement resolves to exactly one owning capability. Top-level requirements use specify; child requirements inherit ownership through requirement hierarchy.",
            },
            {
              name: "Contract ownership",
              desc: "A non-semantic-contract is owned by exactly one compatible requirement through define or definedBy. Semantic contracts constrain requirements through constrain and constrainedBy.",
            },
            {
              name: "Cross-boundary reuse",
              desc: "Cross-capability semantic dependencies stay explicit through concept references and semantic-contract use relations so context, review impact, and AI collection remain auditable.",
            },
          ]}
        />
      </Section>

      <Section title="Submodels and Semantic References">
        <p className="text-zinc-600 mb-4">
          Capability-rooted submodels are intentionally independent. A capability
          can own its operational meaning, the requirements that specify it, and
          the contracts and verifications that prove it without becoming part
          of one universal hierarchy.
        </p>
        <div className="space-y-4">
          <div className="border border-zinc-200 rounded-lg p-4">
            <h4 className="font-semibold text-zinc-900 mb-1">
              Concept references
            </h4>
            <p className="text-sm text-zinc-600">
              Capabilities, requirements, contracts, verification objectives,
              and concrete verifications use concept references to bind
              readable labels to SKOS concepts.
            </p>
          </div>
          <div className="border border-zinc-200 rounded-lg p-4">
            <h4 className="font-semibold text-zinc-900 mb-1">
              Reused contract context
            </h4>
            <p className="text-sm text-zinc-600">
              Requirements reuse requirement-owned contracts such as
              specifications, constraints, behaviors, states, and input/output
              definitions. The consuming requirement declares that its subgraph
              must fulfill the reused contract across that requirement, its
              child requirements, and the contracts that detail those
              obligations. Semantic contracts are linked through constrainedBy.
            </p>
          </div>
          <div className="border border-zinc-200 rounded-lg p-4">
            <h4 className="font-semibold text-zinc-900 mb-1">
              Fulfillment evidence
            </h4>
            <p className="text-sm text-zinc-600">
              The reused context creates the contract dependency; fulfillment is
              shown by satisfied requirements, child requirement coverage, and
              verifications linked to evidence. Trace and change-impact views
              keep that dependency visible so affected contracts, requirements,
              contracts, verifications, and implementation artifacts can be
              reviewed and hardened after changes.
            </p>
          </div>
          <div className="border border-zinc-200 rounded-lg p-4">
            <h4 className="font-semibold text-zinc-900 mb-1">
              One-way dependency flow
            </h4>
            <p className="text-sm text-zinc-600">
              Reused contract context flow between capability-rooted subgraphs is
              one-directional. If two submodels reuse contracts from each other in
              both directions, the boundary becomes ambiguous, so validation
              rejects that pattern and forces the dependency direction to be
              explicit.
            </p>
          </div>
        </div>
      </Section>

      <Section title="Element Types">
        <div className="space-y-4">
          {[
            {
              name: "Ontology",
              desc: "Defines structural domain semantics, relations, rules, and ontology vocabulary as first-class OWL/Turtle content.",
            },
            {
              name: "Concept Scheme and Concept",
              desc: "Defines curated SKOS thesaurus terminology as native Markdown elements. Concept schemes and concepts generate skos:ConceptScheme, skos:Concept, labels, definitions, taxonomy, and mappings.",
            },
            {
              name: "Capability",
              desc: "Describes coherent operational, product, business, regulatory, or system abilities. Stable, decomposable, and verifiable.",
            },
            {
              name: "Requirement",
              desc: "Defines implementable obligations, constraints, guarantees, and behavioral expectations that specify capabilities.",
            },
            {
              name: "Contract",
              desc: "Defines source, specification, constraint, behavior, state, and input/output detail for obligations.",
            },
            {
              name: "Verification",
              desc: "Evidence that requirements are verified by tests, proofs, analysis, inspection, or demonstration.",
            },
          ].map((el) => (
            <div
              key={el.name}
              className="border border-zinc-200 rounded-lg p-4"
            >
              <h4 className="font-semibold text-zinc-900 mb-1">{el.name}</h4>
              <p className="text-sm text-zinc-600">{el.desc}</p>
            </div>
          ))}
        </div>
      </Section>

      <Section title="Ontology Contracts">
        <p className="text-zinc-600 mb-4">
          Ontology and semantic contracts are separate layers of meaning.
          Ontology defines reusable structural vocabulary. Native concept
          elements define curated SKOS terminology. Semantic contracts
          explicitly use ontology and constrain requirements to make obligations
          precise and machine-checkable.
        </p>
        <p className="text-zinc-600 mb-4">
          For ontology authoring rules, examples, validation, and export
          commands, see{" "}
          <Link to="/ontologies" className="text-blue-600 hover:text-blue-700">
            Ontologies
          </Link>
          .
        </p>
        <DetailGrid
          items={[
            {
              name: "Ontology",
              desc: "Use ontology elements for reusable structural meaning: classes, properties, ranges, restrictions, labels, comments, and stable domain vocabulary. Ontology elements require one Ontology Turtle block.",
            },
            {
              name: "Concept References",
              desc: "Use concept references when readable capability, requirement, contract, or verification prose should bind labels to SKOS concept IRIs without crowding the text.",
            },
            {
              name: "Semantic Contract",
              desc: "Use semantic-contract for a reusable SHACL profile that constrains requirements and explicitly uses ontology. It requires Shapes and must not contain Ontology.",
            },
          ]}
        />
      </Section>

      <Section title="Relations">
        <p className="text-zinc-600 mb-4">
          Relations connect elements in the model, creating traceability chains
          from domain meaning to obligations, verification, implementation, and evidence:
        </p>
        <div className="overflow-x-auto border border-zinc-200 rounded-lg">
          <table className="w-full text-sm">
            <thead className="bg-zinc-50 text-zinc-900">
              <tr>
                <th className="text-left p-3 font-semibold">Relation</th>
                <th className="text-left p-3 font-semibold">Meaning</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-200">
              {[
                ["derivedFrom / derive", "Hierarchy inside compatible families: capability, requirement, ontology, concept-scheme/concept context, or verification-family."],
                ["specify / specifiedBy", "Requirement specifies a capability."],
                ["define / definedBy", "Requirement owns a compatible contract element."],
                ["constrain / constrainedBy", "Semantic contract constrains one or more requirements."],
                ["use / usedBy", "Semantic contract uses ontology vocabulary."],
                ["broader / narrower", "SKOS taxonomy between native concept elements."],
                ["related", "Associative SKOS relation between native concept elements."],
                ["exactMatch / closeMatch", "SKOS mapping relation from a native concept to another concept or external concept IRI."],
                ["verify / verifiedBy", "Concrete verification records evidence scope for a requirement; capability coverage is computed from verified requirements."],
                ["satisfiedBy / satisfy", "Requirement or evidence-backed verification links to implementation or proof/test evidence."],
                ["reuse", "Requirement imports a one-way non-semantic requirement-owned contract dependency."],
                ["trace", "Soft traceability without ownership semantics."],
              ].map(([relation, meaning]) => (
                <tr key={relation}>
                  <td className="p-3 align-top">
                    <code className="text-blue-700 font-semibold">
                      {relation}
                    </code>
                  </td>
                  <td className="p-3 text-zinc-600">{meaning}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
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
