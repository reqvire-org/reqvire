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
        roll-up: coverage proves that an implementable obligation is connected
        to code, tests, proof evidence, reports, or other artifacts.
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

      <Section title="Coverage Sources">
        <DetailGrid
          items={[
            {
              name: "direct_satisfied",
              desc: "The requirement has direct satisfiedBy relation links to implementation or evidence artifacts.",
            },
            {
              name: "contract_satisfied_via_contract_bindings",
              desc: "The requirement owns a contract and a directly satisfied requirement reuses that contract.",
            },
            {
              name: "contract_satisfied_via_child",
              desc: "The requirement owns a contract and a derived descendant requirement is directly satisfied.",
            },
            {
              name: "uncovered",
              desc: "The requirement has no direct satisfaction, no contract-bindings coverage, and no child coverage path.",
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
          is the dependency edge; fulfillment comes from satisfied requirements,
          descendant requirement coverage, and verification evidence.
        </p>
        <BulletList
          items={[
            "The binding requirement declares the contract obligation for its requirement subtree.",
            "Child requirements and contracts can provide the detailed implementation route.",
            "Coverage and change-impact reports keep the bound contract visible for review and hardening.",
          ]}
        />
      </Section>

      <Section title="Report Shape">
        <p className="text-zinc-600 mb-4">
          Text output summarizes totals, covered and uncovered counts, coverage
          percentage, and source counts. JSON output exposes the same data for
          CI, dashboards, and assistant workflows.
        </p>
        <CodeBlock>{`reqvire coverage
reqvire coverage --json --output coverage.json`}</CodeBlock>
        <div className="mt-4">
          <BulletList
            items={[
              "total_requirements_in_scope",
              "covered_requirements",
              "uncovered_requirements",
              "implementation_coverage_percentage",
              "coverage_sources",
            ]}
          />
        </div>
      </Section>

      <Footer />
    </div>
  );
}
