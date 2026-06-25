import { BulletList, CodeBlock, DetailGrid, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function Verifications() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">Verifications</h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Verification confirms that system behavior, implementation evidence, or
        operational evidence satisfies the capabilities and requirements it is
        linked to. Reqvire keeps those verification links inside the same
        default semantic export as requirements, contracts, and implementation
        artifacts.
      </p>

      <Section title="Verification Types">
        <DetailGrid
          items={[
            {
              name: "verification-objective",
              desc: "Mandatory planning or grouping parent for concrete verification work through derivedFrom. It does not verify capabilities or requirements and must not have satisfiedBy evidence.",
            },
            {
              name: "test-verification",
              desc: "Formal or automated testing with documented expected outcomes. This is evidence-backed and must have satisfiedBy links to test implementations or reports.",
            },
            {
              name: "formal-proof-verification",
              desc: "Proof, model checking, theorem proving, generated fixtures, or proof reports. This is evidence-backed and must have satisfiedBy proof evidence.",
            },
            {
              name: "analysis-verification",
              desc: "Systematic analysis, calculation, simulation, or review of documentation or code. It does not require satisfiedBy evidence.",
            },
            {
              name: "inspection-verification",
              desc: "Formal examination of documentation, code, design, or physical components. It does not require satisfiedBy evidence.",
            },
            {
              name: "demonstration-verification",
              desc: "Showing the capability or requirement behavior in an operational-like environment. It does not require satisfiedBy evidence.",
            },
          ]}
        />
      </Section>

      <Section title="Two-Level Evidence Model">
        <p className="text-zinc-600 mb-4">
          Capabilities and requirements link to concrete verification elements with{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            verifiedBy
          </code>
          . Each concrete verification must derive from a verification-objective parent.
          Evidence-backed concrete verification elements then link to concrete test or proof
          artifacts with{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            satisfiedBy
          </code>
          .
        </p>
        <CodeBlock>{`### Response Time Requirement
The system shall process data within 500ms.

#### Relations
  * verifiedBy: [Performance Test](Verifications.md#performance-test)

---

### Performance Verification Objective

#### Metadata
  * type: verification-objective

---

### Performance Test

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Performance Verification Objective](#performance-verification-objective)
  * verify: [Response Time Requirement](Requirements.md#response-time-requirement)
  * satisfiedBy: [test_performance.sh](../../tests/test-performance/test_performance.sh)`}</CodeBlock>
      </Section>

      <Section title="Coverage Philosophy">
        <BulletList
          items={[
            "Verification objectives are mandatory parents for concrete verifications and are excluded from concrete verification coverage counts.",
            "Leaf requirements are the preferred verification targets because they represent concrete testable obligations.",
            "Parent requirement coverage rolls up through the requirement hierarchy when leaf requirements are verified.",
            "Capability coverage rolls up from the verified requirements that specify each capability.",
            "One verification may verify multiple leaf requirements when a single test or proof covers a coherent behavior.",
          ]}
        />
      </Section>

      <Section title="Coverage Command">
        <p className="text-zinc-600 mb-4">
          The coverage report includes verification coverage for leaf
          requirements plus evidence satisfaction status for test and formal
          proof verifications.
        </p>
        <CodeBlock>{`reqvire coverage
reqvire coverage --json`}</CodeBlock>
      </Section>

      <Section title="What Gets Flagged">
        <DetailGrid
          items={[
            {
              name: "Verified leaf requirements",
              desc: "Leaf requirements with verifiedBy relations to verification elements.",
            },
            {
              name: "Unsatisfied test verifications",
              desc: "test-verification elements that are missing satisfiedBy links to test implementations or evidence.",
            },
            {
              name: "Unsatisfied formal proofs",
              desc: "formal-proof-verification elements that are missing satisfiedBy links to proof artifacts, generated fixtures, or reports.",
            },
            {
              name: "Analysis, inspection, and demonstration",
              desc: "These verification methods are considered satisfied by the verification element itself and do not require satisfiedBy evidence.",
            },
          ]}
        />
      </Section>

      <Footer />
    </div>
  );
}
