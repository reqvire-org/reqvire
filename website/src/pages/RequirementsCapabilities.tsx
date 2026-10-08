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
            "Use Concept References when capability prose should bind readable labels to SKOS concepts.",
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
  * [Access Token](../Thesaurus/Auth.md#access-token)

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
  * [Access Token](../Thesaurus/Auth.md#access-token)

#### Relations
  * specify: [API Authentication](Auth.md#api-authentication)
  * verifiedBy: [Access Token Contract Test](../Verifications/Auth.md#access-token-contract-test)
  * satisfiedBy: [auth_middleware.rs](../../src/auth_middleware.rs)`}</CodeBlock>
        </div>
      </Section>

      <Section title="Requirement-Owned Contracts and Semantic Contracts">
        <p className="text-zinc-600 mb-4">
          Contracts define requirements in precise terms: source basis,
          specifications, constraints, behavior, state, interfaces, and
          input/output semantics.
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

      <Section title="Contract References">
        <p className="text-zinc-600 mb-4">
          Contract dependency is the umbrella term for Contract Bindings and
          Contract References. A Contract Reference declares a content dependency
          that propagates change impact without contributing to the contract
          owner's implementation fulfillment.
        </p>
        <p className="text-zinc-600 mb-4">
          Contract References must be acyclic. Reqvire rejects circular chains,
          including loops through contract bindings or requirement ancestry,
          before reporting or saving edits, and identifies the dependency path.
        </p>
        <p className="text-zinc-600 mb-4">
          Use Contract References when a requirement depends on a contract's content
          for review, such as documentation describing an API. Contract changes reach
          the referencing requirement, its descendants, verifications, and artifacts
          through change impact. The documentation's implementation does not fulfill
          the API contract owner's obligation.
        </p>
        <p className="text-zinc-600 mb-4">
          A requirement can declare Contract Bindings or Contract References. Using
          both sections on the same requirement is a validation error, even for
          different contracts. Separate documentation and implementation requirements
          when they have different responsibilities. References target the same six
          requirement-owned contract types as bindings and may stay within a hierarchy.
        </p>
        <CodeBlock>{`### Error Response Documentation

The system SHALL document endpoint error responses.

#### Metadata
  * type: requirement

#### Contract References
  * [Error Response Specification](Specifications.md#error-response-specification)

#### Relations
  * specify: [API Documentation](Capabilities.md#api-documentation)
  * satisfiedBy: [Errors page](../website/errors.html)`}</CodeBlock>
      </Section>

      <Section title="Contract Bindings">
        <p className="text-zinc-600 mb-4">
          Contract Bindings assign shared implementation obligations across
          requirement subgraphs. Concept references bind prose to curated SKOS concepts;
          semantic-contract dependencies use structural ontology through
          constrainedBy/constrain and use/usedBy. A requirement can reuse a
          one-way contract dependency from a compatible requirement-owned
          non-semantic-contract element in another subgraph.
        </p>
        <DetailGrid
          items={[
            {
              name: "Concept references",
              desc: "Capabilities, requirements, contracts, and verifications bind prose to SKOS concepts with Concept References.",
            },
            {
              name: "Shared implementation obligations",
              desc: "Requirements bind source, specification, constraint, behavior, state, and input-output contracts owned by other requirements. Semantic contracts are linked through constrainedBy/constrain.",
            },
            {
              name: "One-way flow",
              desc: "The consuming requirement declares the bound obligation for itself and its requirement descendants. Reciprocal cross-submodel reuse is rejected because it hides the intended dependency direction.",
            },
            {
              name: "Review impact",
              desc: "Verifications provide evidence, while traces and change-impact reports show which contracts, child requirements, and implementation artifacts need review after changes.",
            },
          ]}
        />
        <p className="text-zinc-600 mt-5 mb-4">
          Place each binding on the requirement responsible for implementing
          the obligation. Bind individual child requirements when their
          implementations fulfill it independently and need separate coverage
          tracking. Bind a parent when the obligation applies to its entire
          requirement subtree and fulfillment is intentionally assessed through
          that subtree's implementation roll-up. Sharing an ancestor alone is
          not a reason to move bindings upward.
        </p>
        <p className="text-zinc-600 mb-4">
          For example, if Order Error Responses and Payment Error Responses
          independently implement a shared error-schema contract, bind each
          child requirement. An unrelated API Metrics sibling does not acquire
          that obligation. A parent binding is appropriate when the parent's
          entire requirement subtree implements the shared obligation and is
          assessed together.
        </p>
        <p className="text-zinc-600 mb-4">
          Descendants inherit a parent's binding and cannot repeat it. Siblings
          can bind the same contract independently when no ancestor already
          binds it, provided each binding satisfies the ownership and dependency
          direction rules.
        </p>
        <div className="mt-5">
          <CodeBlock>{`### API Consumer Token Handling

The consumer service shall fulfill the shared access-token validation behavior.

#### Metadata
  * type: requirement

#### Contract Bindings
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
          relations, and contract dependencies.
        </p>
        <CodeBlock>{`system-model/
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
              "Ontologies holds structural ontology and semantic contracts; Thesaurus holds curated concept schemes instead of nesting reusable semantic material into unrelated capability files.",
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
