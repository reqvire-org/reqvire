---
name: reqvire-ontology-authoring
description: Expert workflow for creating, extending, and validating Reqvire ontology elements for IT engineering, systems engineering, MBSE, and system-of-interest modeling. Use for competency-question-driven ontology scoping, OWL/Turtle vocabulary, concept-reference context, semantic-contract boundaries, ontology hierarchy, domain/range/property modeling, individuals, axioms, and Reqvire validation; trigger when Codex needs to add or revise files under requirements/Ontologies, author #### Ontology Turtle blocks, decide whether meaning belongs in ontology vs requirement/specification/semantic-contract, or prepare ontology terms for Reqvire HTML visualization and semantic export.
---

# Reqvire Ontology Authoring

Author Reqvire ontology content as reusable building blocks for system-of-interest models, not implementation detail. Reqvire managed-project ontologies are primarily for IT engineering, systems engineering, MBSE, architecture, interfaces, verification, operations, and other system model concerns. Use ontology elements for stable nouns, relationships, categories, and model meaning; use requirements for obligations; use semantic-contract elements for reusable SHACL profiles that explicitly `use` ontology and `constrain` requirements.

## Workflow

1. Establish the domain-concept frame before project-specific examples.
   - Identify the system of interest and the domain viewpoints the ontology must support: product/domain, engineering/MBSE, operations, governance, interfaces, verification, assurance, or business process.
   - Name the most reusable top-level concepts before leaves such as concrete providers or specific workflows.
   - Typical first-pass concepts for IT/MBSE projects include `SystemOfInterest`, `Actor`, `CapabilityArea`, `ManagedResource`, `BusinessArtifact`, `OperationalArtifact`, `InterfaceSurface`, `ExternalSystem` or `EnablingSystem`, `LifecycleState`, `Operation`, `Policy`, `Risk`, and `VerificationEvidence`.
   - Check the five ontology building blocks: classes/concepts, instances/individuals, properties/slots, relationships, and axioms/rules.
2. Define the system-of-interest scope and 5-10 competency questions.
   - Ask what an ontology-backed model must answer about the system, subsystems, capabilities, functions, interfaces, requirements, verification, deployment, operations, risks, or evidence.
   - Treat these questions as the litmus test for scope. The ontology should contain enough vocabulary and relationships to answer them at the required level of detail.
   - Keep the questions sketch-level; they do not need to be exhaustive.
3. Develop the class hierarchy and properties together.
   - For new ontology work, use top-down modeling by default: define the domain-level concepts first, then specialize them into project-specific concepts.
   - For refactoring existing ontology or source material, first inventory the concrete existing terms bottom-up, then reorganize them into the target top-down domain hierarchy.
   - Use a combined pass only after the top-level hierarchy exists: add important leaves from project sources, then insert middle-level concepts that make competency questions answerable.
   - For each class candidate, immediately ask what object properties and datatype properties are needed: ownership, state, allocation, interface exposure, verification evidence, operational effect, constraints, and identifiers.
4. Inspect the ontology plane:
   - `reqvire search --filter-type=ontology --short`
   - `reqvire ontologies`
   - Read the existing `requirements/Ontologies/*.md` file that owns nearby vocabulary.
5. Decide whether to extend an existing ontology or create a new ontology element.
   - Extend when the new terms belong to an existing vocabulary root.
   - Create a new ontology when the terms form a coherent reusable vocabulary with separate ownership, lifecycle, or concept-reference scope.
6. Derive candidate classes, properties, individuals, and axioms from the domain frame and competency questions.
7. For a top parent ontology element, define `ontology_base` and `ontology_prefix` metadata before authoring Turtle. Descendant ontology elements inherit both through `derivedFrom` hierarchy. Use the corresponding hash namespace for terms, normally `<ontology_base>#`, with the inherited prefix as the canonical CURIE label. When rebasing an existing ontology element, use `add --override` as the command path and require it to rewrite the dependent ontology boundary chain atomically, including `ontology_base`, `ontology_prefix`, inherited prefix bindings, imports, and any reachable SHACL references.
8. Place ontology elements under `requirements/Ontologies`.
9. Author exactly one `#### Ontology` fenced Turtle block per ontology element.
10. Link ontology hierarchy with `derivedFrom` only between ontology elements.
11. Use `#### Concept References` on non-ontology, non-semantic-contract elements when their prose needs explicit bindings to ontology terms.
12. Add or update semantic contracts only when a closed-world SHACL profile is needed; link each contract to ontology with `use`/`usedBy` and to governed requirements with `constrain`/`constrainedBy`. Do not add `#### Concept References` to semantic contracts.
13. Use `reqvire ontologies` for the clean authored ontology/SHACL document. Use `reqvire ontologies --full` when downstream graph/database tooling also needs model-context facts for element relations, attachments, concept references, term declarations, shape references, and ontology projection facts. Concept references are term-reference edges, not generated `OntologyConstruct` records.
14. Validate before finishing.

## Ontology From Existing Model Content

Ontology work does not have to come first. If capabilities, requirements, specifications, behaviors, constraints, input/output refinements, or verifications already exist, derive ontology and semantic contracts from that authored model rather than forcing a greenfield ontology pass.

Use this workflow when a project has partial or no ontology coverage:

1. Read the existing capability and requirement subgraph first.
2. Extract repeated domain nouns, states, relation words, artifact types, payload concepts, governed tokens, and validation conditions from requirements and refinements.
3. Promote stable reusable vocabulary into ontology elements only where it improves shared meaning, queryability, impact analysis, or semantic validation.
4. Promote repeated closed-world validation rules into semantic-contract SHACL shapes only when machine-checkable constraints add value.
5. Leave requirements prose-only when ontology or SHACL formalization is not useful.
6. Link requirements to reusable shapes with `constrainedBy`/`constrain` and link shapes to ontology with `use`/`usedBy`.

## Modeling Split

Use `ontology` for:
- Reusable ontology vocabulary under root `ontology_base` and `ontology_prefix` metadata values. Authored Turtle normally declares terms in the hash namespace such as `@prefix ex: <https://example.org/ontology/managed-platform#>`; Reqvire emits one generated `owl:Ontology` document declaration per resolved `ontology_base`, with same-base child ontology elements contributing vocabulary to that document. Cross-base ontology hierarchy can become `owl:imports`. If authored Turtle uses the inherited prefix, it must explicitly declare that prefix to `<ontology_base>#`; missing or conflicting declarations fail validation.
- Classes/concepts, such as `reqvire:RequirementCoverage a owl:Class`.
- Object properties and datatype properties declared in `#### Ontology` blocks with stable `rdfs:domain` and `rdfs:range` when those semantics are true.
- Stable vocabulary individuals, such as element type records, rule records, report kinds, governance values, and enum-like categories. For new controlled vocabulary records, explicitly type the record as both `owl:NamedIndividual` and its domain class, for example `ex:collectReportKind a owl:NamedIndividual, ex:ReportKind`.
- Stable hierarchy and axioms, such as `rdfs:subClassOf`, `rdfs:domain`, `rdfs:range`, `owl:inverseOf`, equivalence, disjointness, and property chains when semantically true.
- MBSE/system-of-interest concepts, such as system, subsystem, function, interface, port, deployment environment, operational mode, verification evidence, risk, hazard, dependency, allocation, and traceability category.

Use `requirement` for implementable obligations, especially statements that naturally read as "The system shall...".

Use `specification`, `behavior`, `constraint`, `state`, or `input-output` for exact commands, file paths, outputs, workflows, schemas, UI behavior, and implementation-specific details.

Use `semantic-contract` for reusable SHACL `sh:NodeShape` profiles. A semantic contract is a first-class ontology-plane element and should be authored under `requirements/Ontologies` near the ontology it uses. It must have `#### Shapes`, use `sh:targetClass` and `sh:path` over ontology terms reachable through explicit `use` relations, constrain requirements through `constrain`/`constrainedBy`, and must not contain `#### Ontology`.

For greenfield ontology creation templates and examples, read `references/OntologyAuthoring.md`.

For refactoring or improving existing ontology files, read `references/OntologyRefactoring.md`. Do not load the refactoring reference for new ontology creation unless the user explicitly asks to refactor or improve existing ontology content.

## Labels, Definitions, and Domain Tokens

Use `rdfs:label` and `rdfs:comment` for labels and descriptions unless the value is a true domain concept, contract token, or query/validation target. Treat them as the default for presentation text, not replacements for ontology slots.

- `rdfs:label` is optional presentation label text when no more specific domain property is needed.
- `rdfs:comment` is optional explanatory description text for a class, property, individual, or axiom.
- Create a domain class or property when a label/description value is itself system meaning that requirements, SHACL, queries, reports, or payloads depend on.
- Keep custom datatype properties only when the literal is part of the system contract: authored metadata tokens, parser fields, export fields, interface enum values, report kinds, rule conditions, queryable attributes, or controlled-vocabulary payloads.
- If a SHACL shape validates a presentation-only `*Name`, `*Label`, `*Meaning`, or description field, refactor the SHACL `sh:path` to `rdfs:label` or `rdfs:comment` instead of preserving a custom property.
- Do not infer that a property is only a label because it ends with `Name` or only a comment because it ends with `Meaning`.
- Controlled-vocabulary individuals get formal semantics from their IRI, explicit `owl:NamedIndividual` typing, typed class membership, hierarchy, and axioms. Domain token and definition properties may still be required for validation and queries.
- Do not replace canonical token properties with `rdfs:label` when project files, CLI output, API payloads, semantic contracts, reports, or queries depend on the literal. For example, keep `ex:reportKindName "collect"` for a report command token; `rdfs:label` may be added for presentation but is not the contract token.

## OWL / SHACL Block Split

Use OWL datatype/object properties for slots and relationships with stable domain/range semantics. Use SHACL shapes for operational validation facets.

- In `#### Ontology`, define classes, `owl:DatatypeProperty` and `owl:ObjectProperty` declarations, stable `rdfs:domain`/`rdfs:range`, hierarchy, metadata annotations, and stable OWL axioms.
- For datatype properties, set `rdfs:range` to an XSD datatype such as `xsd:string`, `xsd:boolean`, `xsd:integer`, `xsd:decimal`, `xsd:float`, `xsd:double`, `xsd:dateTime`, or `xsd:date`.
- For object properties, set `rdfs:domain` to the subject class and `rdfs:range` to the object class when both are stable.
- In `#### Shapes`, declare `sh:NodeShape` resources, target existing OWL classes with `sh:targetClass`, and validate existing properties with `sh:path`.
- Do not redefine `ex:Class a owl:Class` inside a SHACL shape block. The shape consumes the ontology; it does not own the class declaration.
- Put closed-world cardinality, patterns, enumerations, numeric bounds, messages, and data-quality rules in SHACL, not in the ontology block. Use OWL cardinality restrictions only when they are true domain axioms and reasoner semantics are intended.
- Keep OWL and SHACL separable so ontology exports remain useful to OWL/RDFS tools while semantic contracts remain useful to SHACL validators.

## OWL/Turtle Quick Reference

Reqvire ontology blocks are Turtle using OWL/RDFS vocabulary. Prefer compact, stable CURIEs and define terms explicitly before referencing them.

Use canonical Reqvire ontology identity boundaries:

- The top parent ontology element in an ontology subgraph must define non-empty `ontology_base` and `ontology_prefix` metadata, for example `ontology_base: https://example.org/ontology/managed-platform` and `ontology_prefix: ex`. When rebasing an existing ontology element, use `add --override` instead of hand-editing metadata so the rewrite chain remains atomic.
- The term namespace identifies classes, properties, and individuals and normally uses the inherited ontology base plus `#`, for example `@prefix ex: <https://example.org/ontology/managed-platform#>`. The prefix label comes from inherited `ontology_prefix`; the namespace comes from inherited `ontology_base`.
- Authored Turtle that uses the inherited prefix must explicitly declare it to `<ontology_base>#`; missing or conflicting declarations fail validation.
- The root ontology Turtle block should declare `<ontology_base> a owl:Ontology` for authored OWL document identity. Child ontology blocks normally define vocabulary terms only. Do not manually model the ontology itself as a vocabulary term inside the term namespace, such as `ex:ManagedPlatformOntology a owl:Ontology`.
- Link ontology hierarchy with `derivedFrom` between ontology elements. Reqvire derives one ontology document declaration per resolved `ontology_base`; same-base `derivedFrom` contributes to the same document, while cross-base hierarchy can become `owl:imports`.

```turtle
@prefix ex: <https://example.org/ontology/managed-platform#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:ManagedResource a owl:Class .

ex:Deployment a owl:Class ;
  rdfs:subClassOf ex:ManagedResource .

ex:Environment a owl:Class ;
  rdfs:subClassOf ex:ManagedResource .

ex:InterfaceSurface a owl:Class .
ex:PublicApiSurface a owl:Class ;
  rdfs:subClassOf ex:InterfaceSurface .
ex:PrivateApiSurface a owl:Class ;
  rdfs:subClassOf ex:InterfaceSurface .

ex:productionEnvironment a owl:NamedIndividual, ex:Environment ;
  ex:resourceIdentifier "env-prod" .

ex:resourceIdentifier a owl:DatatypeProperty ;
  rdfs:domain ex:ManagedResource ;
  rdfs:range xsd:string .

ex:deploymentBelongsToEnvironment a owl:ObjectProperty ;
  rdfs:domain ex:Deployment ;
  rdfs:range ex:Environment ;
  owl:inverseOf ex:environmentHostsDeployment .

ex:environmentHostsDeployment a owl:ObjectProperty ;
  rdfs:domain ex:Environment ;
  rdfs:range ex:Deployment .

ex:DeploymentWithEnvironment a owl:Class ;
  rdfs:subClassOf ex:Deployment,
    [ a owl:Restriction ;
      owl:onProperty ex:deploymentBelongsToEnvironment ;
      owl:someValuesFrom ex:Environment ] .

ex:IntegrationBoundary a owl:Class ;
  owl:equivalentClass ex:InterfaceSurface .

ex:interfaceExposesResource a owl:ObjectProperty .
ex:exposesResource a owl:ObjectProperty ;
  owl:equivalentProperty ex:interfaceExposesResource .

ex:environmentHasPolicy a owl:ObjectProperty .
ex:resourceImpactedByPolicy a owl:ObjectProperty ;
  owl:propertyChainAxiom (ex:deploymentBelongsToEnvironment ex:environmentHasPolicy) .

ex:PublicApiSurface owl:disjointWith ex:PrivateApiSurface .
```

Use `owl:Class` for concepts, explicit `owl:NamedIndividual` plus domain-class typing for stable vocabulary records, `owl:DatatypeProperty` for literal slots, `owl:ObjectProperty` for relationship slots, `rdfs:subClassOf` for is-a hierarchy, `rdfs:domain`/`rdfs:range` for stable slot attachment and allowed values, and OWL axioms such as `owl:Restriction`, `owl:inverseOf`, `owl:equivalentClass`, `owl:equivalentProperty`, `owl:disjointWith`, and `owl:propertyChainAxiom` only when the semantics are true. Use SHACL shapes for closed-world validation such as required fields, cardinality, enumerations, or datatype checks.

## Validation

Run focused validation after ontology edits:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" ontologies
```

When working in the Reqvire source repository, the local Rust CLI is also acceptable:

```bash
cargo run -q -p cli -- --workspace "$PWD" validate
```

## Guardrails

- Keep ontology text reusable and implementation-independent.
- Start from the domain frame and competency questions, and remove ontology terms that do not help answer them or support future system-of-interest modeling.
- Declare referenced ontology terms before SHACL shapes reference them.
- Prefer deterministic IRIs and stable CURIE prefixes.
- Avoid random UUIDs in authored ontology unless the domain truly requires non-semantic identifiers.
- Do not put governance metadata on ontology elements.
- Do not claim implementation satisfaction from ontology elements.
- If adding many property semantics, include domain/range only when the domain and range are stable and not misleadingly broad.
