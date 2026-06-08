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
              "Capabilities",
              "coherent operational, product, business, regulatory, or system abilities.",
            ],
            [
              "Requirements",
              "implementable obligations, constraints, guarantees, and behavioral expectations that specify capabilities.",
            ],
            [
              "Refinements",
              "requirement-owned source, semantic-contract, semantic-query-contract, specification, constraint, behavior, state, and input-output detail.",
            ],
            [
              "Verifications",
              "tests, proofs, analysis, inspection, or demonstrations linked to the capability or requirement they verify.",
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
          Markdown headers. Metadata, relations, details, attachments, ontology
          blocks, shapes, queries, and concept references use reserved{" "}
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

#### Attachments
  * [Access Token Ontology](AuthOntology.md#access-token-ontology)

#### Relations
  * specifiedBy: [API Access Token Validation](Requirements.md#api-access-token-validation)`}</CodeBlock>
        <h3 className="text-lg font-semibold text-zinc-900 mb-3 mt-6">
          Single-element file
        </h3>
        <CodeBlock>{`# Element

## Metadata
  * type: specification

## Relations
  * refine: [API Access Token Validation](Requirements.md#api-access-token-validation)

## Access Token Validation Specification

The access-token validator checks the token issuer, subject, audience, expiry,
and signature before the request reaches protected application logic.`}</CodeBlock>
      </Section>

      <Section title="Physical Containment">
        <p className="text-zinc-600 mb-4">
          The logical graph is defined by metadata and relations. Folders and
          files provide review boundaries, navigation, and ownership context.
          The layout below is a suggested convention, not a schema obligation.
        </p>
        <CodeBlock>{`<model-root>/
  Capabilities/
  Ontologies/
  Verifications/`}</CodeBlock>
        <div className="mt-4">
          <TermList
            items={[
              [
                "Capabilities/",
                "capability-rooted subgraphs with child capabilities, specifying requirements, and requirement-owned refinements.",
              ],
              [
                "Ontologies/",
                "reusable ontology elements attached by capabilities instead of nested into unrelated capability files.",
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
The system shall reject API requests whose access token does not conform to the access token semantic contract.

#### Metadata
  * type: requirement

#### Concept References
  * Access Token: auth:AccessToken

#### Relations
  * specify: [API Authentication](#api-authentication)
  * refinedBy: [Access Token Validation Shape Contract](#access-token-validation-shape-contract)
  * verifiedBy: [Access Token Contract Test](#access-token-contract-test)
  * satisfiedBy: [auth_middleware.rs](../src/auth_middleware.rs)

---

### Access Token Validation Shape Contract

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [API Access Token Validation](#api-access-token-validation)

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
