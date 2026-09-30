import { BulletList, CodeBlock, DetailGrid, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function ImplementationCoverage() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">
        Implementation Coverage
      </h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Implementation coverage shows which requirements have implementation
        evidence and which still need work. It is separate from verification
        coverage: a terminal obligation needs a direct artifact link, while a
        parent depends on every child and every required contract consumer.
        Evidence links track implementation; they do not prove correctness.
      </p>

      <Section title="Scope">
        <p className="text-zinc-600 mb-4">
          Implementation coverage is scoped to requirements. Capabilities are
          not directly implementation-covered because capability intent should
          remain implementation-independent; capability coverage is understood
          through the requirements that specify the capability.
        </p>
        <BulletList
          items={[
            "Included: requirement elements.",
            "Excluded: capability elements as direct implementation targets.",
            "Reported by the same reqvire coverage command that also reports verification coverage.",
          ]}
        />
      </Section>

      <Section title="Terminal Requirements and Roll-up">
        <p className="text-zinc-600 mb-4">
          A terminal requirement has no child requirements through <code>derive</code>
          {" "}and no requirements binding any of its owned contracts. Owning an
          unused specification does not change that status. Terminal requirements
          need direct <code>satisfiedBy</code> evidence.
        </p>
        <p className="text-zinc-600 mb-4">
          Every child and required consumer must be covered before a nonterminal
          requirement is covered. The owner needs no additional direct evidence
          when those contributions collectively fulfill it. Direct parent evidence
          stays visible even when a child or consumer is missing implementation.
        </p>
        <p className="text-zinc-600 mb-4">
          Contract dependencies include bindings and references. Bindings assign
          shared implementation obligations; references track content dependencies
          for change-impact review.
          A documentation requirement can reference an API contract while remaining
          responsible for its own documentation evidence. Its reference does not
          contribute evidence, consumers, or blockers to the API owner's coverage.
        </p>
        <p className="text-zinc-600 mb-4">
          Child and contract-consumer dependencies must be acyclic when combined,
          including within a single capability root. A circular chain is a model
          validation error: Reqvire rejects it before generating coverage and
          rejects mutations that would introduce it before saving. Direct evidence
          does not make a cycle valid. Follow the reported dependency path and
          correct the hierarchy or contract binding that closes the loop.
        </p>
        <p className="text-zinc-600 mb-4">
          Implementation percentages count distinct covered terminal requirements
          divided by distinct terminals in the selected scope. Classification
          totals still include parents. Capability completeness is reported
          separately: a capability containing only a covered contract owner can
          be complete even when all its terminal consumers are outside the scope
          and its local denominator is zero. A zero denominator displays zero
          percent. Verification leaves depend only on requirement children.
        </p>
      </Section>

      <Section title="Coverage Sources">
        <DetailGrid
          items={[
            {
              name: "direct_satisfied",
              desc: "A terminal requirement has direct satisfiedBy links to implementation artifacts.",
            },
            {
              name: "requirement_rollup",
              desc: "Every immediate requirement child is implementation-covered, recursively. No contract ownership is required.",
            },
            {
              name: "contract_consumer_rollup",
              desc: "Every requirement binding an owned contract is implementation-covered, recursively.",
            },
            {
              name: "combined_rollup",
              desc: "Every child requirement and every required contract consumer is implementation-covered.",
            },
            {
              name: "uncovered",
              desc: "A terminal requirement has no direct evidence, or at least one required child or contract consumer is uncovered. Direct parent evidence cannot override that gap.",
            },
          ]}
        />
      </Section>

      <Section title="Linking Implementation">
        <p className="text-zinc-600 mb-4">
          Requirements link to implementation artifacts with{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            satisfiedBy
          </code>
          . Multiple artifacts can satisfy one requirement when the
          implementation is spread across modules, tests, generated fixtures, or
          proof reports.
        </p>
        <CodeBlock>{`#### Relations
  * satisfiedBy: [auth_middleware.rs](../src/auth_middleware.rs)
  * satisfiedBy: [test_access_token.rs](../tests/test_access_token.rs)`}</CodeBlock>
      </Section>

      <Section title="Contract Fulfillment">
        <p className="text-zinc-600 mb-4">
          Contract Bindings can make one subgraph depend on a
          requirement-owned contract from another subgraph. The contract binding
          identifies an implementation obligation on its consuming requirement.
          Fulfillment depends on all required consumers and their complete
          implementation roll-up. Verification evidence remains a separate measure.
        </p>
        <BulletList
          items={[
            "The binding requirement declares the contract obligation for its requirement subtree.",
            "Every required consumer must be covered; one implemented consumer is insufficient.",
            "Specifications identify obligations but contribute neither evidence nor coverage units.",
            "Coverage and change-impact reports keep the bound contract visible for review and hardening.",
          ]}
        />
        <p className="text-zinc-600 mt-4">
          Place bindings where implementation responsibility belongs. Use
          separate child bindings when implementations fulfill the obligation
          independently and need separate coverage tracking. Use a parent
          binding when the obligation applies to its entire requirement subtree
          and fulfillment is intentionally assessed through that subtree's
          implementation roll-up. A common ancestor alone does not justify
          moving bindings upward; unrelated siblings do not inherit a child's
          binding. Descendants inherit a parent's binding and cannot repeat it.
        </p>
      </Section>

      <Section title="Coverage Within a Capability">
        <p className="text-zinc-600 mb-4">
          Select the subjects to report after calculating coverage against the
          complete model. Each selected requirement retains the same coverage
          classification, source, evidence, blockers, and terminal status as the whole-model
          report from that snapshot. Scope changes the subjects and totals;
          it does not change the evidence available to assess them.
        </p>
        <p className="text-zinc-600 mb-4">
          For example, a requirement in Alpha Root owns a contract consumed by
          a requirement in Beta Root. If every required Beta consumer is
          implementation-covered, the owner keeps its
          <code> contract_consumer_rollup </code>
          source, contributor links, and supporting artifacts when viewing Alpha.
          An uncovered Beta consumer instead remains a blocker. Beta's requirements
          stay outside Alpha's subject counts in either case.
          This remains indirect implementation coverage; it does not create
          direct satisfaction or verification of the owner.
        </p>
        <CodeBlock>{`reqvire coverage --from "Alpha Root"
reqvire coverage --from "Alpha Root" --json`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          The optional <code>from</code> argument on MCP's
          <code> reqvire.coverage</code> and Explorer's Scope selector use the
          same contract. Omit the selector for the unchanged whole-model
          report. Scoped JSON adds <code>scope</code> with the selected
          capability, distinct capability, requirement, and verification IDs,
          and the whole-model-only orphan diagnostic marker.
        </p>
      </Section>

      <Section title="Report Shape">
        <p className="text-zinc-600 mb-4">
          Text output summarizes totals, covered and uncovered counts, coverage
          percentage, and source counts. JSON output exposes the same data for
          CI, dashboards, and assistant workflows. Both covered and uncovered
          records include terminal status, direct and supporting artifact evidence,
          immediate contributing requirements, and recursively identified blockers.
        </p>
        <CodeBlock>{`reqvire coverage
reqvire coverage --json --output coverage.json`}</CodeBlock>
        <div className="mt-4">
          <BulletList
            items={[
              "total_requirements_in_scope",
              "covered_requirements",
              "uncovered_requirements",
              "total_terminal_requirements / covered_terminal_requirements / uncovered_terminal_requirements",
              "implementation_coverage_percentage (terminal requirements only)",
              "coverage_sources",
            ]}
          />
        </div>
      </Section>

      <Footer />
    </div>
  );
}
