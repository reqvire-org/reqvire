import { BulletList, CodeBlock, DetailGrid, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function Ontologies() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">Ontologies</h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Ontologies are Reqvire&apos;s reusable semantic vocabulary layer. They
        define domain meaning, model terms, relationships, and semantic rules
        as first-class OWL/Turtle content with explicit boundary metadata.
      </p>

      <Section title="What Belongs Here">
        <DetailGrid
          items={[
            {
              name: "Reusable meaning",
              desc: "Use ontology when content says that X is a Y, X has property Z, X relates to Y, or a domain term has stable meaning across requirements.",
            },
            {
              name: "Semantic vocabulary",
              desc: "Ontology elements define classes, object properties, datatype properties, subclass relations, domain and range, restrictions, labels, comments, inverse properties, and property chains.",
            },
            {
              name: "Not implementation behavior",
              desc: "Commands, fields, URI patterns, workflow steps, file paths, emitted outputs, and reject/write behavior belong in specifications, behaviors, states, input-output contracts, or semantic contracts.",
            },
          ]}
        />
      </Section>

      <Section title="Reachability Model">
        <p className="text-zinc-600 mb-4">
          Reqvire keeps ontology orthogonal to capability and requirement
          hierarchy. Capabilities attach ontology; requirements inherit ontology
          context from their owning capability path.
        </p>
        <BulletList
          items={[
            "Author shared ontology elements under the ontology plane, commonly requirements/Ontologies.",
            "Capabilities attach ontology elements to make vocabulary reachable for descendant capabilities and specifying requirements.",
            "Requirements do not attach ontology directly; requirement attachments are for reusable requirement-owned non-semantic-contract refinements.",
            "Ontology hierarchy uses derive or derivedFrom only with other ontology elements.",
            "Ontology elements do not author attachments.",
          ]}
        />
      </Section>

      <Section title="Ontology Element">
        <p className="text-zinc-600 mb-4">
          An ontology element carries one{" "}
          <code className="text-sm bg-zinc-100 px-1.5 py-0.5 rounded">
            #### Ontology
          </code>{" "}
          Turtle block.
        </p>
        <CodeBlock>{`### Access Token Ontology

Defines access token domain meaning.

#### Metadata
  * type: ontology
  * ontology_base: https://example.org/ontology/auth
  * ontology_prefix: auth

#### Ontology
\`\`\`turtle
@prefix auth: <https://example.org/ontology/auth#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

<https://example.org/ontology/auth> a owl:Ontology .

auth:AccessToken a owl:Class ;
  rdfs:label "Access token" ;
  rdfs:comment "Bearer credential presented to access protected resources." .

auth:subject a owl:ObjectProperty ;
  rdfs:domain auth:AccessToken .
\`\`\``}</CodeBlock>
      </Section>

      <Section title="Concept References">
        <p className="text-zinc-600 mb-4">
          Concept references let requirement prose stay readable while binding
          terms to reachable ontology CURIEs or IRIs. The referenced term must
          be reachable from the requirement through the capability that owns the
          requirement.
        </p>
        <CodeBlock>{`### API Authentication

API authentication capability.

#### Metadata
  * type: capability

#### Attachments
  * [Access Token Ontology](AuthOntology.md#access-token-ontology)

#### Relations
  * specifiedBy: [API Access Token Validation](#api-access-token-validation)

---

### API Access Token Validation
The system shall reject API requests whose access token is invalid.

#### Metadata
  * type: requirement

#### Concept References
  * Access Token: auth:AccessToken

#### Relations
  * specify: [API Authentication](#api-authentication)`}</CodeBlock>
      </Section>

      <Section title="Semantic Contracts">
        <p className="text-zinc-600 mb-4">
          Semantic contracts are reusable SHACL profiles that explicitly use
          ontology and constrain requirements. They make obligations
          machine-checkable without redefining ontology locally.
        </p>
        <BulletList
          items={[
            "semantic-contract may constrain zero, one, or many requirements.",
            "semantic-contract requires one Shapes block.",
            "semantic-contract must not contain an Ontology block.",
            "semantic-contract must use ontology through explicit use relations, and SHACL references must resolve through that used ontology graph.",
          ]}
        />
        <div className="mt-4">
          <CodeBlock>{`### Access Token Validation Shape Contract

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
        </div>
      </Section>

      <Section title="Validation">
        <DetailGrid
          items={[
            {
              name: "Turtle parsing",
              desc: "Malformed ontology Turtle is rejected during validation.",
            },
            {
              name: "Declaration conflicts",
              desc: "Duplicate or incompatible declarations for the same ontology term are rejected.",
            },
            {
              name: "SHACL reachability",
              desc: "A SHACL reference must point to a term declared by the semantic contract's explicit ontology-use graph. Missing or outside-context references are validation errors.",
            },
            {
              name: "Change impact",
              desc: "Ontology and semantic-contract dependencies remain explicit so trace and change-impact reports can show what must be reviewed after changes.",
            },
          ]}
        />
      </Section>

      <Section title="Export and Tools">
        <p className="text-zinc-600 mb-4">
          The ontology command collects authored ontology and SHACL content.
          Full mode adds generated Reqvire model context triples for downstream
          semantic tools and the Explorer ontology view.
        </p>
        <CodeBlock>{`reqvire ontologies
reqvire ontologies --output ontologies.ttl
reqvire ontologies --jsonld --output ontologies.jsonld
reqvire ontologies --full

# MCP tool
reqvire.ontologies({ "format": "turtle", "full": true })`}</CodeBlock>
      </Section>

      <Footer />
    </div>
  );
}
