import { BulletList, CodeBlock, DetailGrid, Section } from "@/components/Doc";
import { Footer } from "@/components/Footer";

export default function Ontologies() {
  return (
    <div className="max-w-[768px]">
      <h1 className="text-4xl font-bold text-zinc-900 mb-5">Ontologies</h1>
      <p className="text-base text-zinc-600 leading-relaxed mb-10">
        Ontologies are Reqvire&apos;s reusable structural semantic vocabulary
        layer. They define domain meaning, model terms, relationships, and
        semantic rules as first-class OWL/Turtle content with explicit boundary
        metadata. Curated thesaurus terminology is authored separately as
        native concept-scheme and concept elements that generate SKOS RDF.
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
              name: "Curated concepts",
              desc: "Use native concept-scheme and concept elements for SKOS thesaurus content: human labels, definitions, synonyms, scope notes, examples, taxonomy, and concept mappings.",
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
          hierarchy. Non-ontology elements reference SKOS concepts explicitly;
          semantic contracts use ontology through use relations.
        </p>
        <BulletList
          items={[
            "Author shared ontology elements under the ontology plane, commonly system-model/Ontologies.",
            "Capabilities, requirements, contracts, verification objectives, and concrete verifications use Concept References for SKOS concept bindings.",
            "Semantic contracts do not author Concept References; they use ontology through explicit use relations.",
            "Reused Contract Context is for reusable requirement-owned contracts from other subgraphs.",
            "Ontology hierarchy uses derive or derivedFrom only with other ontology elements.",
            "Ontology elements do not author Reused Contract Context.",
          ]}
        />
      </Section>

      <Section title="Concept Thesaurus Root">
        <p className="text-zinc-600 mb-4">
          For mixed conceptual and structural modeling, create a native
          concept-scheme root with its own concept_base and concept_prefix.
          Keep structural OWL ontology elements separate. Structural terms can
          point back to curated concepts with reqvire:mapsToConcept when that
          bridge is useful.
        </p>
        <CodeBlock>{`Concept scheme root
  type: concept-scheme
  concept_base: https://example.org/concepts
  concept_prefix: concept
  owns: labels, definitions, synonyms, scope notes, examples,
        broader/narrower/related concept taxonomy, mappings

Structural ontology root
  ontology_base: https://example.org/ontology/managed-platform
  ontology_prefix: ex
  owns: owl:Class, owl:ObjectProperty, owl:DatatypeProperty,
        individuals, axioms, SHACL targets
  optional bridge: ex:StructuralTerm reqvire:mapsToConcept concept:CuratedConcept`}</CodeBlock>
        <div className="mt-4">
          <BulletList
            items={[
              "Use SKOS concepts for stakeholder vocabulary, labels, synonyms, definitions, search terms, and broader/narrower taxonomy.",
              "Use OWL structural terms for formal schema, reasoning semantics, SHACL targets, and model-operation meaning.",
              "Use reqvire:mapsToConcept only where a structural term needs an explicit concept anchor.",
              "Concept or concept-scheme changes propagate forward to mapped ontology terms; the authored RDF predicate is reqvire:mapsToConcept, and the change-impact edge is reported as mappedByOntology.",
              "Do not use SKOS mapping properties as the default bridge from OWL classes or model resources to concepts.",
            ]}
          />
        </div>
      </Section>

      <Section title="Native Concept Elements">
        <p className="text-zinc-600 mb-4">
          Native concepts are authored as Markdown elements. Reqvire generates
          skos:ConceptScheme and skos:Concept triples from element names,
          main body, Labels, Scope Note, Examples, Mappings, and concept
          relations. Do not add concept_id, concept_kind, pref_label, or
          language metadata.
        </p>
        <CodeBlock>{`### Engineering Concepts

Curated engineering terminology for system-model authoring.

#### Metadata
  * type: concept-scheme
  * concept_base: https://example.org/concepts
  * concept_prefix: concept

---

### Traceability

The conceptual practice of connecting intent, implementation, verification, and evidence.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Engineering Concepts](#engineering-concepts)

#### Labels
  * altLabel: Trace link analysis

#### Scope Note
Use for engineering artifact traceability, not runtime distributed tracing.`}</CodeBlock>
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
          Concept references let non-ontology prose stay readable while binding
          terms to native `concept` elements. Reqvire derives generated SKOS concept IRIs from those native targets for semantic export and tooling.
        </p>
        <CodeBlock>{`### API Authentication

API authentication capability.

#### Metadata
  * type: capability

#### Concept References
  * [Access Token](../Thesaurus/Auth.md#access-token)

#### Relations
  * specifiedBy: [API Access Token Validation](#api-access-token-validation)

---

### API Access Token Validation
The system shall reject API requests whose access token is invalid.

#### Metadata
  * type: requirement

#### Concept References
  * [Access Token](../Thesaurus/Auth.md#access-token)

#### Relations
  * specify: [API Authentication](#api-authentication)`}</CodeBlock>
      </Section>

      <Section title="External Ontology Sources">
        <p className="text-zinc-600 mb-4">
          Ontology elements can declare local Turtle/TTL, RDF/XML, or JSON-LD files for external
          vocabularies that are not authored by the Reqvire model. These files
          are loaded for validation and can be included in semantic exports on
          demand as the used external subset with `include_external`. Full raw
          external ontology dependencies remain internal dependency inputs and
          are not exposed by public export, MCP, Explorer, or SPARQL surfaces.
        </p>
        <BulletList
          items={[
            "Use External Ontology sections only on ontology elements.",
            "The source must be a local Turtle/TTL, RDF/XML, or JSON-LD file; Reqvire does not fetch remote ontology URLs during validation.",
            "Authored Turtle and SHACL blocks still declare their own prefixes explicitly. External source sections do not inject hidden Turtle.",
            "OWL/RDF/RDFS/XSD reserved vocabulary and core SHACL shape syntax are recognized by Reqvire without local External Ontology declarations.",
          ]}
        />
        <div className="mt-4">
          <CodeBlock>{`#### External Ontology
  * prefix: ext
  * namespace: https://example.org/external#
  * resource: https://example.org/external
  * source: references/ontologies/external.ttl
  * format: turtle`}</CodeBlock>
        </div>
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
            "semantic-contract must not contain Concept References.",
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
          Semantic export commands keep authored ontology vocabulary, SHACL
          shapes, SKOS concepts, and the combined semantic graph as separate
          surfaces. Use the combined graph only when downstream graph/database
          consumers need generated Reqvire model context triples, including
          relations, Reused Contract Context entries, concept references, term
          declarations, shape references, and ontology projection facts.
        </p>
        <BulletList
          items={[
            "Use reqvire semantic ontologies when a tool needs authored OWL/RDF ontology vocabulary only.",
            "Use reqvire semantic shapes when a tool needs semantic-contract SHACL shapes only.",
            "Use reqvire semantic concepts when a tool needs SKOS concept scheme and thesaurus triples only.",
            "Use reqvire concepts export or reqvire concepts validate when the workflow is specifically standalone Thesaurus concept-scheme work.",
            "Use reqvire semantic graph --full when a graph/database should also know which model elements reference SKOS concepts.",
            "Use reqvire semantic graph --full --include-external when a graph/database should receive authored triples, authored model facts, generated facts, and the used external subset.",
            "Reqvire emits generated rdfs:isDefinedBy links from authored named ontology resources to the resolved ontology_base document IRI; Explorer uses this as OWL document metadata for grouping, search, and modals rather than rendering ontology document nodes or isDefinedBy edges.",
            "Use reqvire.semantic.ontologies, reqvire.semantic.shapes, reqvire.semantic.concepts, or reqvire.semantic.graph through MCP depending on the semantic layer an assistant needs.",
            "Use reqvire.semantic.prefixes through MCP when an assistant needs ontology-defined namespaces and source prose before writing SPARQL; set include_external when used external subset prefixes are needed.",
            "Use reqvire.semantic.vocabulary through MCP when an assistant needs paged classes, properties, relation families, controlled vocabularies, semantic contracts, query patterns, source maps, diagnostics, and prefixes before writing SPARQL; set ontology_document or ontology_base to filter terms to one OWL document, and set include_external when used external vocabulary should be listed.",
            "Use reqvire.semantic.sparql through MCP when an assistant needs to query the model-owned Oxigraph semantic store directly; full mode is default and set include_external to query the used external subset.",
            "Reqvire parses complete external ontology files internally for validation and term resolution, but raw full external dependency triples are not public semantic output.",
            "Use MCP prompts such as reqvire.semantic.query when an assistant needs query-construction guidance before calling vocabulary, prefix, or SPARQL tools.",
            "Concept References appear in full export as authored-model facts such as conceptReference and referencesTerm; they do not rewrite authored OWL/SHACL semantics.",
            "Concept References are authored-model term-reference edges, not generated OntologyConstruct records. OntologyConstruct is reserved for projected OWL/RDFS/SHACL patterns such as subclass, membership, restriction, property-chain, inverse-property, and shape-overlay constructs.",
          ]}
        />
        <CodeBlock>{`reqvire semantic ontologies
reqvire semantic ontologies --include-external
reqvire semantic shapes
reqvire semantic concepts --include-mappings
reqvire concepts export
reqvire concepts validate
reqvire semantic graph --full
reqvire semantic graph --full --include-external

# MCP tool
reqvire.semantic.ontologies({ "format": "turtle" })
reqvire.semantic.ontologies({ "include_external": true })
reqvire.concept_schemes.list({})
reqvire.concepts.list({ "filter": "verification" })
reqvire.semantic.shapes()
reqvire.semantic.concepts({ "include_mappings": true })
reqvire.semantic.graph({ "full": true })
reqvire.semantic.prefixes()
reqvire.semantic.prefixes({ "include_external": true })
reqvire.semantic.vocabulary({ "section": "relation_families", "limit": 50 })
reqvire.semantic.vocabulary({ "section": "classes", "ontology_document": "https://example.org/ontology/auth" })
reqvire.semantic.vocabulary({ "section": "classes", "include_external": true })
reqvire.semantic.vocabulary({ "section": "properties", "include_external": true, "ontology_document": "https://example.org/external" })
reqvire.semantic.sparql({ "query": "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 20" })
reqvire.semantic.sparql({ "include_external": true, "query": "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 20" })

# MCP prompts
prompts/list
prompts/get({ "name": "reqvire.semantic.query" })`}</CodeBlock>
      </Section>

      <Footer />
    </div>
  );
}
