import {
  BulletList,
  CodeBlock,
  DetailGrid,
  Section,
  TermList,
} from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function RequirementsCapabilities() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">
        Requirements and Capabilities
      </h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Reqvire models requirements inside a semantic engineering graph.
        Capabilities define stable system abilities, requirements define
        implementable obligations, ontology defines reusable meaning, and
        verification and implementation links prove the model is real.
      </p>

      <Section title="Conceptual Split">
        <TermList
          items={[
            [
              "Capability",
              "a coherent operational, product, business, regulatory, or system ability that gives scope and language to a model area.",
            ],
            [
              "Requirement",
              "a testable obligation that specifies a capability and can be verified, satisfied, decomposed, and defined by contracts.",
            ],
            [
              "Ontology",
              "reusable domain and model meaning: classes, properties, vocabulary, restrictions, labels, and semantic relationships.",
            ],
            [
              "Feature",
              "a product or roadmap term that may be described inside capability content, but is not the primary traceability node.",
            ],
          ]}
        />
      </Section>

      <Section title="Capabilities">
        <p className="text-zinc-600 mb-4">
          A capability is not a weaker requirement. It answers why a system area
          exists, what domain context it uses, who owns the scope, and which
          requirements belong under that ability. Capabilities can be directly
          verified, but implementation coverage rolls up from their specifying
          requirements.
        </p>
        <BulletList
          items={[
            "Use capability hierarchy only between capability elements with derive or derivedFrom.",
            "Use Concept References when capability prose should bind readable labels to ontology terms.",
            "Create child capabilities when ownership, lifecycle, verification, stakeholder scope, architecture impact, or requirement clusters differ.",
            "Do not create one universal top capability that hides all submodel boundaries.",
          ]}
        />
        <div className="mt-5">
          <CodeBlock>{`### API Authentication

API authentication capability and access-token domain context.

#### Metadata
  * type: capability
  * status: review
  * priority: high
  * risk: medium
  * owner: Identity Team

#### Concept References
  * Access Token: https://example.org/ontology/auth#AccessToken

#### Relations
  * specifiedBy: [API Access Token Validation](AuthRequirements.md#api-access-token-validation)`}</CodeBlock>
        </div>
      </Section>

      <Section title="Requirements">
        <p className="text-zinc-600 mb-4">
          A requirement is the obligation anchor. It should say what the system
          shall do, under what condition or scope, what implementation or
          evidence can satisfy it, and what verification proves it.
        </p>
        <DetailGrid
          items={[
            {
              name: "Top-level ownership",
              desc: "A top-level requirement uses specify to resolve to exactly one owning capability.",
            },
            {
              name: "Requirement hierarchy",
              desc: "Child requirements use derive or derivedFrom inside the requirement family and inherit ownership from their parent requirement.",
            },
            {
              name: "Implementation evidence",
              desc: "Requirements use satisfiedBy to link code, tests, generated reports, proof outputs, or other implementation evidence.",
            },
            {
              name: "Verification evidence",
              desc: "Requirements use verifiedBy to link test, proof, analysis, inspection, or demonstration verifications.",
            },
          ]}
        />
        <div className="mt-5">
          <CodeBlock>{`### API Access Token Validation

The system shall reject API requests whose access token is invalid.

#### Metadata
  * type: requirement
  * status: review
  * priority: high
  * risk: medium
  * owner: Identity Team

#### Concept References
  * Access Token: auth:AccessToken

#### Relations
  * specify: [API Authentication](Auth.md#api-authentication)
  * verifiedBy: [Access Token Contract Test](../Verifications/Auth.md#access-token-contract-test)
  * satisfiedBy: [auth_middleware.rs](../../src/auth_middleware.rs)`}</CodeBlock>
        </div>
      </Section>

      <Section title="Requirement-Owned Contracts and Semantic Contracts">
        <p className="text-zinc-600 mb-4">
          Contracts carry detailed engineering terms for a requirement.
          Non-semantic-contract elements are owned through define or definedBy
          and should not author governance metadata. Semantic contracts are separate
          ontology-plane elements that constrain requirements through constrainedBy and constrain,
          and use ontology through use and usedBy.
        </p>
        <BulletList
          items={[
            "source captures stakeholder, regulatory, policy, contractual, or external source material.",
            "specification, constraint, behavior, state, and input-output contract elements carry detailed contract content.",
            "semantic-contract is a reusable SHACL profile over explicitly used ontology and must include Shapes.",
          ]}
        />
        <div className="mt-5">
          <CodeBlock>{`# Element

## Metadata
  * type: specification

## Relations
  * define: [API Access Token Validation](AuthRequirements.md#api-access-token-validation)

## Access Token Validation Specification

The access-token validator checks the token issuer, subject, audience, expiry,
and signature before the request reaches protected application logic.`}</CodeBlock>
        </div>
      </Section>

      <Section title="Reused Contract Context">
        <p className="text-zinc-600 mb-4">
          Reused Contract Context makes cross-boundary requirement contracts
          explicit. Ontology term usage is modeled with concept references. A
          requirement can reuse a one-way contract dependency from a compatible
          requirement-owned non-semantic-contract element in another subgraph;
          semantic-contract dependencies use constrainedBy/constrain and
          use/usedBy instead.
        </p>
        <DetailGrid
          items={[
            {
              name: "Concept references",
              desc: "Capabilities, requirements, contracts, and verifications bind prose to ontology terms with Concept References.",
            },
            {
              name: "Reused contract context",
              desc: "Requirements reuse specifications, constraints, behaviors, states, and input-output contracts owned by requirement subgraphs. Semantic contracts are linked through constrainedBy/constrain.",
            },
            {
              name: "One-way flow",
              desc: "The consuming requirement declares that its subgraph must fulfill the reused contract. Reciprocal cross-submodel reuse is rejected because it hides the intended dependency direction.",
            },
            {
              name: "Review impact",
              desc: "Verifications provide evidence, while traces and change-impact reports show which contracts, child requirements, and implementation artifacts need review after changes.",
            },
          ]}
        />
        <div className="mt-5">
          <CodeBlock>{`### API Consumer Token Handling

The consumer service shall fulfill the shared access-token validation behavior.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Access Token Validation Behavior](../Identity/AuthBehaviors.md#access-token-validation-behavior)

#### Relations
  * specify: [API Consumer](Consumer.md#api-consumer)`}</CodeBlock>
        </div>
      </Section>

      <Section title="Governance Metadata">
        <p className="text-zinc-600 mb-4">
          Governance metadata belongs on capability and requirement elements.
          It supports planning, ownership routing, readiness review, risk
          review, search filters, and assistant task generation.
        </p>
        <div className="overflow-x-auto border border-zinc-200 rounded-lg mb-5">
          <table className="w-full text-sm">
            <thead className="bg-zinc-50 text-zinc-900">
              <tr>
                <th className="text-left p-3 font-semibold">Key</th>
                <th className="text-left p-3 font-semibold">Values</th>
                <th className="text-left p-3 font-semibold">Default</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-200">
              {[
                ["status", "draft, review, approved", "approved"],
                ["priority", "low, medium, high, critical", "medium"],
                ["risk", "low, medium, high, critical", "low"],
                [
                  "owner",
                  "free-form person, role, team, or subsystem",
                  "unassigned",
                ],
              ].map(([key, values, defaultValue]) => (
                <tr key={key}>
                  <td className="p-3 align-top">
                    <code className="text-blue-700 font-semibold">{key}</code>
                  </td>
                  <td className="p-3 text-zinc-600">{values}</td>
                  <td className="p-3 text-zinc-600">{defaultValue}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <BulletList
          items={[
            "Capabilities inherit missing governance fields from parent capabilities.",
            "Top-level requirements inherit missing governance fields from their owning capability.",
            "Child requirements inherit missing fields from the nearest parent requirement.",
            "Contracts and verifications must not declare status, priority, risk, or owner directly.",
          ]}
        />
        <div className="mt-5">
          <CodeBlock>{`reqvire search --filter-status review
reqvire search --filter-priority high,critical
reqvire search --filter-risk high,critical --json
reqvire search --filter-owner "Identity Team"`}</CodeBlock>
        </div>
      </Section>

      <Section title="Model Containment">
        <p className="text-zinc-600 mb-4">
          Folders and files are physical containment only. They should make the
          model easy to browse, but authoritative semantics come from metadata,
          relations, and Reused Contract Context.
        </p>
        <CodeBlock>{`requirements/
  Product/
    Collaboration/
      Collaboration.md
      CollaborationRequirements.md
      CollaborationBehaviors.md
      Architecture/
        CollaborationServiceSpecifications.md
  Platform/
    Identity/
      Identity.md
      IdentityRequirements.md
  Ontologies/
    Collaboration.md
    Identity.md
  Verifications/
    Collaboration/
      CollaborationVerifications.md
    Identity/
      IdentityVerifications.md`}</CodeBlock>
        <div className="mt-5">
          <BulletList
            items={[
              "Capabilities holds capability-rooted subgraphs with child capabilities, specifying requirements, and requirement-owned contracts.",
              "Ontologies holds reusable semantic vocabulary referenced by model elements instead of nested into unrelated capability files.",
              "Verifications holds verification elements grouped by domain and linked through verify or verifiedBy.",
              "Folder names are guidance, not schema. Reqvire validates element metadata and graph relations.",
            ]}
          />
        </div>
      </Section>

      <Footer />
    </div>
  );
}
