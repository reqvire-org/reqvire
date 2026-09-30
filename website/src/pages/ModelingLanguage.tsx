import { CodeBlock, Section, TermList } from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function ModelingLanguage() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">
        Reqvire Modeling Language
      </h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Reqvire uses semi-structured Markdown as a lightweight semantic
        engineering and MBSE modeling language. Models stay readable in Git
        while still forming a machine-validated, queryable, traceable
        engineering knowledge graph for humans and AI assistants.
      </p>

      <Section title="Core Elements">
        <TermList
          items={[
            [
              "Ontologies",
              "first-class OWL/Turtle vocabulary and reusable semantic model terms.",
            ],
            [
              "Concepts",
              "native concept-scheme and concept elements that generate SKOS thesaurus RDF from main body definitions, labels, taxonomy, and mappings.",
            ],
            [
              "Capabilities",
              "coherent operational, product, business, regulatory, or system abilities.",
            ],
            [
              "Requirements",
              "implementable obligations, constraints, guarantees, and behavioral expectations that specify capabilities.",
            ],
            [
              "Semantic contracts",
              "reusable SHACL profiles that use ontology vocabulary and constrain requirements.",
            ],
            [
              "Contracts",
              "requirement-owned source, specification, constraint, behavior, state, and input-output detail.",
            ],
            [
              "Verifications",
              "tests, proofs, analysis, inspection, or demonstrations linked to the requirements they verify. Capabilities receive verification coverage through requirement roll-up.",
            ],
            [
              "Relations",
              "explicit links between model elements and implementation or evidence artifacts.",
            ],
          ]}
        />
        <p className="text-zinc-600 mt-4">
          Elements are defined with{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            ###
          </code>{" "}
          Markdown headers. Metadata, Relations, Details, Contract Bindings,
          Contract References, Concept References, Ontology, External Ontology,
          Shapes, and Query use reserved{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            ####
          </code>{" "}
          subsections.
        </p>
      </Section>

      <Section title="Document Shape">
        <p className="text-zinc-600 mb-4">
          Model files begin with either a multi-element file or a
          single-element file. Element names must be globally unique; stable
          identifiers let links survive file moves and renames.
        </p>
        <h3 className="text-lg font-semibold text-zinc-900 mb-3">
          Multi-element file
        </h3>
        <CodeBlock>{`# Elements

### API Authentication

API authentication capability.

#### Metadata
  * type: capability

#### Concept References
  * [Access Token](../Thesaurus/Auth.md#access-token)

#### Relations
  * specifiedBy: [API Access Token Validation](Requirements.md#api-access-token-validation)`}</CodeBlock>
        <h3 className="text-lg font-semibold text-zinc-900 mb-3 mt-6">
          Single-element file
        </h3>
        <CodeBlock>{`# Element

## Metadata
  * type: specification

## Relations
  * define: [API Access Token Validation](Requirements.md#api-access-token-validation)

## Access Token Validation Specification

The access-token validator checks the token issuer, subject, audience, expiry,
and signature before the request reaches protected application logic.`}</CodeBlock>
      </Section>

      <Section title="Contract Bindings and Contract References">
        <p className="text-zinc-600 mb-4">
          Contract dependencies include Contract Bindings and Contract References.
          Requirements use these subsections to depend on a contract owned by
          another requirement. Each target is a source, specification,
          constraint, behavior, state, or input-output element with exactly one
          requirement owner.
        </p>
        <h3 className="text-lg font-semibold text-zinc-900 mb-3">
          Shared implementation obligation
        </h3>
        <p className="text-zinc-600 mb-4">
          A Contract Binding makes the consuming requirement responsible for
          fulfilling the shared contract. Its implementation coverage contributes
          to the contract owner's fulfillment. Here, Error Response Specification
          is owned by a separate requirement in ErrorContracts.md.
        </p>
        <CodeBlock>{`### Endpoint Error Responses

WHEN an endpoint rejects a request, the system SHALL return an error response conforming to the shared error contract.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Error Response Specification](ErrorContracts.md#error-response-specification)

#### Relations
  * specify: [API Endpoints](Capabilities.md#api-endpoints)
  * satisfiedBy: [error_response.rs](../src/error_response.rs)`}</CodeBlock>
        <h3 className="text-lg font-semibold text-zinc-900 mb-3 mt-6">
          Content dependency for review
        </h3>
        <p className="text-zinc-600 mb-4">
          A Contract Reference declares a content dependency that propagates change
          impact without contributing to the contract owner's implementation fulfillment.
          Changes to the error contract flag its documentation for review.
          The documentation artifact satisfies the documentation requirement.
        </p>
        <CodeBlock>{`### API Error Documentation

The system SHALL document error response fields and examples.

#### Metadata
  * type: requirement

#### Contract References
  * [Error Response Specification](ErrorContracts.md#error-response-specification)

#### Relations
  * specify: [API Documentation](Capabilities.md#api-documentation)
  * satisfiedBy: [errors.md](../docs/errors.md)`}</CodeBlock>
        <p className="text-zinc-600 mt-4">
          Each requirement can declare either Contract Bindings or Contract
          References. Using both on one element is invalid even when their
          targets differ. Reference dependencies must be acyclic, including
          paths through bindings and requirement ancestry. Validation identifies
          the dependency path when a cycle occurs.
        </p>
      </Section>

      <Section title="Physical Containment">
        <p className="text-zinc-600 mb-4">
          The logical graph is defined by metadata and relations. Folders and
          files provide review boundaries, navigation, and ownership context.
          The layout below is a suggested convention, not a schema obligation.
        </p>
        <CodeBlock>{`<model-root>/
  <Area>/<CapabilityName>/
  Ontologies/
  Verifications/`}</CodeBlock>
        <div className="mt-4">
          <TermList
            items={[
              [
                "<Area>/<CapabilityName>/",
                "root-level capability subgraphs with child capabilities, specifying requirements, and requirement-owned contracts.",
              ],
              [
                "Ontologies/",
                "reusable ontology elements referenced by model elements instead of nested into unrelated capability files.",
              ],
              [
                "Verifications/",
                "verification elements grouped by domain and linked through verify or verifiedBy.",
              ],
            ]}
          />
        </div>
      </Section>

      <Section title="Minimal Example">
        <CodeBlock>{`### API Access Token Validation
The system SHALL reject API requests whose access token does not conform to the access token semantic contract.

#### Metadata
  * type: requirement

#### Concept References
  * [Access Token](../Thesaurus/Auth.md#access-token)

#### Relations
  * specify: [API Authentication](#api-authentication)
  * constrainedBy: [Access Token Validation Shape Contract](#access-token-validation-shape-contract)
  * verifiedBy: [Access Token Contract Test](#access-token-contract-test)
  * satisfiedBy: [auth_middleware.rs](../src/auth_middleware.rs)

---

### Access Token Validation Shape Contract

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [API Access Token Validation](#api-access-token-validation)
  * use: [Auth Ontology](#auth-ontology)

#### Shapes
\`\`\`turtle
@prefix auth: <urn:reqvire:auth:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

auth:AccessTokenValidationShape
  a sh:NodeShape ;
  sh:targetClass auth:AccessToken ;
  sh:property [
    sh:path auth:subject ;
    sh:minCount 1 ;
  ] .
\`\`\``}</CodeBlock>
      </Section>

      <Footer />
    </div>
  );
}
