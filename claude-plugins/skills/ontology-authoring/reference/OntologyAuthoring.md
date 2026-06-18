# Reqvire Ontology Authoring Reference

## Element Template

~~~markdown
# Elements

### Managed Platform Domain Ontology

The Managed Platform Domain ontology defines the first domain-level vocabulary for a Reqvire-managed system of interest. It starts with reusable classes that can later be specialized into product, engineering, operations, interface, governance, support, billing, or verification concepts.

#### Metadata
  * type: ontology
  * ontology_base: https://example.org/ontology/managed-platform
  * ontology_prefix: ex

#### Ontology
```turtle
@prefix ex: <https://example.org/ontology/managed-platform#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:DomainConcept a owl:Class ;
  rdfs:comment "Base class for stable vocabulary terms used to model the system of interest and its engineering context." .

ex:SystemOfInterest a owl:Class ;
  rdfs:subClassOf ex:DomainConcept ;
  rdfs:comment "System being engineered, analyzed, verified, operated, or governed by the Reqvire project." .

ex:Actor a owl:Class ;
  rdfs:subClassOf ex:DomainConcept ;
  rdfs:comment "Human, machine, organization, team, external system, or agent that interacts with or governs the system of interest." .

ex:ManagedResource a owl:Class ;
  rdfs:subClassOf ex:DomainConcept ;
  rdfs:comment "Resource whose lifecycle, configuration, access, operation, or ownership is managed by the system of interest." .

ex:InterfaceSurface a owl:Class ;
  rdfs:subClassOf ex:DomainConcept ;
  rdfs:comment "Boundary through which actors, systems, services, or tools interact with the system of interest." .

ex:LifecycleState a owl:Class ;
  rdfs:subClassOf ex:DomainConcept ;
  rdfs:comment "Named state used to classify readiness, availability, operation, or lifecycle of a modeled thing." .
```

#### Relations
  * derivedFrom: [Parent Ontology](Parent.md#parent-ontology)
---
~~~

## Ontology Building Blocks

A useful ontology deliberately defines five kinds of content. Check all five before treating an ontology as complete enough for the domain being modeled.

| Building block | Purpose | General examples | System-of-interest examples |
|----------------|---------|------------------|-----------------------------|
| Classes / concepts | General categories or types of things in the domain. | `Person`, `Organization`, `Project`, `Document`, `Asset` | `SystemOfInterest`, `Actor`, `CapabilityArea`, `ManagedResource`, `InterfaceSurface`, `LifecycleState` |
| Instances / individuals | Specific named examples or controlled vocabulary records that belong to a class. | `AcmeCorp`, `ProjectApollo`, `HighPriority` | `theSystem`, `productionEnvironment`, `activeDeploymentState`, `criticalRiskLevel` |
| Properties / slots | Literal-valued attributes or characteristics of a class or individual. | `name`, `identifier`, `status`, `createdDate` | `resourceIdentifier`, `environmentRegion`, `stateName`, `criticality`, `protocol` |
| Relationships | Object-valued links showing how concepts or individuals connect. | `ownsAsset`, `approvesDocument`, `assignedToProject` | `ownedByActor`, `deployedToEnvironment`, `exposedThroughInterface`, `hasLifecycleState`, `dependsOnResource` |
| Axioms / rules | Logical statements that constrain or enrich domain meaning. | subclass rules, domain/range rules, disjointness, equivalence, inverse properties, cardinality restrictions | `Deployment` is a `ManagedResource`; `Environment` hosts `Deployment`; `Snapshot` is of one or more `Deployment` resources; `PublicInterface` and `PrivateInterface` may be disjoint |

Classes and properties should be developed together. A class without relationships or slots is often too vague; a property without a clear domain, range, or usage intent is often too broad. Individuals should be used for stable named records or controlled vocabularies, not for arbitrary runtime data. Axioms should express true domain semantics, not just labels or visualization preferences.

## Domain Concepts First

For new ontology work, start top-down. Define the domain-level concepts before adding concrete leaves such as a specific payment provider, support tool, API endpoint, cloud service, product screen, or workflow.

Good first-stage classes for IT engineering and MBSE Reqvire projects often include:

- `SystemOfInterest`: the engineered system managed by the Reqvire project.
- `Actor`: human, organization, team, machine client, external system, AI agent, operator, or stakeholder.
- `CapabilityArea`: coherent ability or mission area supported by the system.
- `Function`: behavior or activity needed to realize capabilities.
- `ManagedResource`: product, platform, data, infrastructure, or system resource under lifecycle control.
- `BusinessArtifact`: commercial, contractual, customer, account, support, billing, or organizational object.
- `OperationalArtifact`: incident, event, metric, log, health status, operation, diagnostic finding, or maintenance object.
- `InterfaceSurface`: web, API, CLI, MCP, message, integration, port, or service boundary.
- `ExternalSystem` / `EnablingSystem`: outside systems that provide identity, payment, telemetry, cloud, data, control, or support functions.
- `LifecycleState`: state category used for readiness, availability, operation progress, failure, approval, or retirement.
- `Policy` / `Constraint`: reusable governance, security, compliance, access, reliability, or operational rule category.
- `VerificationEvidence`: test result, analysis result, proof, inspection, demonstration, trace, report, or runtime evidence.

Then specialize the hierarchy for the actual system of interest. For example, a managed database platform may specialize `ManagedResource` into `Organization`, `Environment`, `Deployment`, `Database`, `NetworkAccess`, `Snapshot`, and `Replica`; specialize `InterfaceSurface` into `ConsoleInterface`, `ManagementApi`, and `McpInterface`; and specialize `ExternalSystem` into identity, cloud, payment, tax, monitoring, or support providers only when those providers are relevant to competency questions.

For refactoring existing ontology/source material, first inventory the existing leaves bottom-up, identify duplicates and mixed abstractions, then place them under the top-down domain hierarchy. Do not preserve existing structure just because it exists.

## Ontology From Existing Model Content

Ontology work does not have to be first in the systems-engineering process. Reqvire models may start with capabilities, requirements, specifications, behaviors, constraints, input/output refinements, verifications, and evidence. Ontology and semantic contracts can be extracted later from that authored model.

Use this workflow when requirements or refinements exist but ontology coverage is missing, partial, or uneven:

1. Read the capability and requirement subgraph before creating ontology.
2. Inventory repeated nouns, states, lifecycle labels, role names, relation words, artifact types, interface concepts, payload fields, governed tokens, and validation conditions.
3. Separate reusable meaning from one-off implementation detail.
4. Promote stable shared vocabulary into ontology elements when it improves shared meaning, queryability, semantic validation, change impact, or agent retrieval.
5. Promote repeated closed-world validation rules into semantic-contract SHACL shapes only when machine-checkable constraints add value.
6. Keep implementation-specific details in requirements, specifications, behaviors, constraints, state, or input/output refinements.
7. Leave requirements prose-only when formal ontology or SHACL modeling does not add enough value.
8. Keep verification traceability anchored on requirements.

The target semantic-contract model is:

```text
Requirement --constrainedBy--> Semantic Contract
Semantic Contract --constrain--> Requirement
Semantic Contract --use--> Ontology
```

In that model, the shape may come before the requirement as the formal rule, and the requirement is the human-facing obligation/interface. The inverse workflow is also valid: write requirements first, then extract ontology and semantic contracts later.

Do not force every requirement into ontology terms or semantic contracts. Ontology and SHACL are semantic enrichment, not mandatory structure.

## Competency Questions

After the domain frame exists, sketch competency questions: questions an ontology-backed model should be able to answer. Use them to decide scope, detail level, and whether a term belongs in ontology, requirement, or semantic contract.

For Reqvire managed projects, competency questions should usually be about IT engineering, systems engineering, MBSE, and the system of interest:

- Which systems, subsystems, external actors, and enabling systems are in the system-of-interest boundary?
- Which capabilities does the system of interest provide, and which stakeholder needs do they support?
- Which system functions realize each capability?
- Which logical or physical components are allocated to each function?
- Which interfaces connect two components or systems, and what information, control, or resource flows over them?
- Which requirements constrain a capability, component, interface, function, operational mode, or deployment environment?
- Which verification methods and evidence prove that a requirement or capability is satisfied?
- Which software services depend on which platform, network, identity, data, or observability services?
- Which deployment environments host each component, and which configuration constraints apply there?
- Which risks, hazards, failure modes, or change-impact paths affect critical capabilities or interfaces?
- Which operational states or modes change valid behavior, interfaces, or verification expectations?
- Which artifacts provide implementation or evidence for requirements, verifications, or architecture decisions?

Use the answers to identify ontology elements:

| Competency question asks about | Likely ontology content |
|--------------------------------|-------------------------|
| Kinds of system things | OWL classes such as system, subsystem, component, function, interface, environment, evidence |
| How things connect | Object properties such as realizes, allocatedTo, exposesInterface, exchangesFlowWith, deployedTo, verifies |
| Literal facts | Datatype properties such as identifier, criticality, protocol, modeName, environmentName |
| Enumerated project vocabulary | Typed individuals such as lifecycle states, risk levels, interface kinds, verification methods |
| Closed-world validity | A semantic-contract SHACL shape, not local ontology text |

Treat the list as a litmus test. After drafting the ontology, check whether a model using it can answer the competency questions without relying on raw prose only.

## Ontology IRI And Term Namespace

Use a canonical split between root ontology metadata and the term namespace:

```turtle
@prefix ex: <https://example.org/ontology/managed-platform#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:ManagedResource a owl:Class .
```

The top parent ontology element defines `ontology_base` metadata, normally as an absolute IRI without a fragment, and `ontology_prefix` metadata, the canonical CURIE label for authored terms. The prefix namespace identifies terms in that ontology and normally uses the ontology base plus `#`. A parser expands `ex:ManagedResource` to `<https://example.org/ontology/managed-platform#ManagedResource>`; it does not infer term existence from the prefix alone. The class exists because the ontology asserts `ex:ManagedResource a owl:Class`. If authored Turtle uses the inherited `ontology_prefix`, it must explicitly declare that prefix to the canonical namespace. Missing or conflicting declarations fail validation.

The root ontology block should declare the document IRI itself, for example `<https://example.org/ontology/managed-platform> a owl:Ontology`. Do not use `ex:ManagedPlatformOntology a owl:Ontology` as the default pattern. That makes the ontology itself look like a named term inside the vocabulary namespace.

When one ontology element depends on another, model the hierarchy with `derivedFrom`. Reqvire emits one generated `owl:Ontology` document declaration per resolved `ontology_base`; ontology elements that inherit the same base contribute vocabulary to that same document. A hierarchy edge becomes `owl:imports` only when the source and target ontology elements resolve to different ontology bases.

## Class Hierarchy And Slots

Develop classes and properties together. A class candidate is incomplete until its expected relationships and literal-valued slots are considered, and a property is suspect until its intended domain and range are understood.

For new ontology work, use a top-down pass:

- Define the top-level domain concepts.
- Specialize them into middle-level concepts that match the system of interest.
- Add concrete leaves only when they are needed by competency questions or by reusable system modeling.
- For each class, define the object properties and datatype properties needed to answer the questions.

For refactoring, use a discovery pass first:

- Inventory existing concrete classes, individuals, properties, requirements, source terms, and repeated nouns.
- Group them under the intended top-down domain classes.
- Merge synonyms, split overloaded concepts, and remove terms that are actually requirements, workflow details, UI labels, command names, or implementation fields.
- Add middle-level classes only when they reduce duplication or clarify competency-question answers.

## Class Hierarchy Correctness

There is rarely one perfect class hierarchy for a domain. The hierarchy depends on the ontology purpose, competency questions, required detail level, compatibility needs, and modeling style. After adding many classes, pause and check the hierarchy against these rules.

### Use Is-A Semantics

Subclassing means "is-a" or "kind-of": class A is a subclass of class B only when every instance of A is also an instance of B.

Good examples:

- `Deployment` is a kind of `PlatformResource`.
- `Environment` is a kind of `PlatformResource`.
- `ConsoleInterface` is a kind of `WebInterface`.
- `WebInterface` is a kind of `InterfaceSurface`.
- `TestVerification` is a kind of `Verification`.
- `PrivateEndpointConnection` is a kind of `NetworkAccess`.

Bad examples:

- `Deployment` is not a subclass of `Environment`; a deployment belongs to or is hosted by an environment.
- `Organization` is not a subclass of `Actor` if organizations own resources but do not act through the same identity/session semantics as users or machine clients.
- `Snapshot` is not a subclass of `Deployment`; it is an operational artifact related to a deployment.
- `CreateDeploymentOperation` is not a subclass of `Deployment`; it is an operation targeting a deployment.

When the phrase "is a kind of" sounds wrong, use an object property instead of subclassing. For example, use `deploymentBelongsToEnvironment`, `operationTargetsDeployment`, `snapshotOfDeployment`, or `interfaceExposesResource`.

### Avoid Singular/Plural Duplicate Classes

Do not model singular and plural versions of the same concept as separate classes. A single `Deployment` is not a subclass of `Deployments`; they are the same concept named two ways.

Pick a naming convention and follow it consistently. Prefer singular class names for reusable ontology classes: `Deployment`, `Environment`, `InterfaceSurface`, `Requirement`, `Verification`, `Snapshot`, `Replica`.

### Remember Subclass Transitivity

Subclassing is transitive: if B is a subclass of A, and C is a subclass of B, then C is also a subclass of A.

Examples:

- If `WebInterface` is a subclass of `InterfaceSurface`, and `ConsoleInterface` is a subclass of `WebInterface`, then `ConsoleInterface` is also an indirect subclass of `InterfaceSurface`.
- If `NetworkAccess` is a subclass of `PlatformConcept`, and `VpcPeeringConnection` is a subclass of `NetworkAccess`, then `VpcPeeringConnection` is also an indirect subclass of `PlatformConcept`.
- If `TestVerification` is a subclass of `Verification`, and `Verification` is a subclass of `Element`, then `TestVerification` is also an indirect subclass of `Element`.

Model only the closest direct superclass. Do not redundantly declare every indirect superclass unless compatibility with another tool requires it.

### Expect Hierarchy Evolution

Class hierarchies change as the system and domain understanding evolve. When a concept no longer fits under one parent, split or move the class rather than preserving a misleading hierarchy.

Examples:

- If all interfaces were originally treated as public, then private/internal surfaces appear, split `ApiSurface` into `PublicApiSurface`, `PrivateApiSurface`, and possibly `ManagementApiSurface`.
- If every deployment was originally assumed to be single-primary, then multi-primary deployments appear, introduce `SinglePrimaryDeployment` and `MultiPrimaryDeployment` under `Deployment` instead of adding misleading flags everywhere.
- If verification evidence was originally only test evidence, then formal proof and analysis evidence appear, specialize `VerificationEvidence` or `Verification` into more precise subclasses.

When refactoring, update slot domains/ranges, concept references, semantic contracts, and visualization expectations to match the new hierarchy.

### Separate Classes From Names

A class represents a domain concept, not the current word used to name it. Renaming a class label should not create a new class when the concept is unchanged.

Do not create separate classes for synonyms:

- Do not model both `DatabaseDeployment` and `DbDeployment` if they mean the same concept.
- Do not model both `ApiClient` and `MachineClient` as separate classes unless the ontology distinguishes their semantics.
- Do not model both `ConsoleInterface` and `WebConsole` if one is only a presentation name.

Choose one canonical class name and document aliases, synonyms, product labels, translations, or presentation names in comments or annotation properties.

### Separate Annotations From Domain Slots

`rdfs:label` and `rdfs:comment` are annotation properties for optional presentation labels and explanatory descriptions. Create a domain class or property instead when the label or description value is itself a true domain concept, contract token, or query/validation target.

Keep a custom datatype property when the literal is part of the modeled contract:

- authored Markdown metadata tokens such as element type, status, priority, or risk values
- parser-recognized field names and reserved section names
- CLI or API enum values such as report kinds, search filter kinds, or side-effect classes
- generated export fields, rule conditions, rule outcomes, and queryable semantic attributes
- controlled-vocabulary payload values that semantic contracts validate directly

Default to `rdfs:label` and `rdfs:comment` for presentation text. Do not decide from suffix alone: a property ending in `Name` may be a canonical token, and a property ending in `Meaning` may be a domain definition that users query or validate. Keep a custom property only when the literal is part of a parser, CLI/API, report, SHACL, query, or payload contract. If an existing SHACL shape validates a presentation-only custom field, refactor the shape to target `rdfs:label` or `rdfs:comment`.

For controlled vocabulary records, the formal ontology meaning comes from the IRI, explicit `owl:NamedIndividual` typing, class membership, hierarchy, and axioms. A literal token remains a slot if it is how project files, CLI output, API payloads, reports, semantic contracts, or validation rules identify the value.

### Avoid Class Cycles

Avoid cycles in `rdfs:subClassOf`. If `A` is a subclass of `B` and `B` is also a subclass of `A`, the hierarchy effectively says the classes are equivalent. If they are truly the same concept, merge them or state equivalence deliberately. If they are not the same concept, replace one subclass edge with an object property.

Bad cycles:

- `Deployment rdfs:subClassOf PlatformResource` and `PlatformResource rdfs:subClassOf Deployment`.
- `ApiInterface rdfs:subClassOf InterfaceSurface` and `InterfaceSurface rdfs:subClassOf ApiInterface`.
- `Requirement rdfs:subClassOf Verification` and `Verification rdfs:subClassOf Requirement`.

### Analyze Siblings

Sibling classes are direct subclasses of the same superclass. Except at the ontology root, siblings should be at the same level of generality and should divide the parent along a coherent modeling dimension.

Good sibling sets:

- `WebInterface`, `ApiInterface`, `McpInterface`, and `BffInterface` under `InterfaceSurface`.
- `PublicApiSurface`, `HiddenApiSurface`, and `ManagementApiSurface` under `ApiSurface`.
- `Environment`, `Deployment`, `Database`, and `Replica` under `PlatformResource` when each is modeled as a managed resource.
- `TestVerification`, `FormalProofVerification`, `AnalysisVerification`, `InspectionVerification`, and `DemonstrationVerification` under `Verification`.

Bad sibling sets:

- `InterfaceSurface`, `ApiInterface`, and `PublicApiSurface` under the same parent; these are different abstraction levels.
- `Deployment`, `Replica`, and `PrimaryReplicaA` under `PlatformResource`; `PrimaryReplicaA` is likely an individual or a more specific subclass under `Replica`.
- `Requirement`, `SecurityRequirement`, and `PasswordResetRequirement` under `Element`; these mix a broad family, a specialization, and a concrete requirement concern.

Review branch width:

- If a class has only one direct subclass, check whether the subclass adds useful meaning. If not, merge it or wait until another sibling exists.
- If a class has more than about a dozen direct subclasses, introduce intermediate categories when they clarify competency-question answers.
- Root-level classes may represent major domain divisions and do not need to be as uniform as lower-level siblings.

Examples:

- If `ApiSurface` has only `PublicApiSurface`, do not create the subclass unless private, hidden, or management surfaces are expected or the distinction is already needed.
- If `InterfaceSurface` has many direct subclasses such as `ConsoleInterface`, `MissionControlInterface`, `PublicApiSurface`, `HiddenApiSurface`, `ManagementApiSurface`, `McpInterface`, `ToolSurface`, and `ResourceSurface`, consider intermediate parents such as `WebInterface`, `ApiInterface`, and `AgentInterface`.
- If `Verification` has many direct method classes, group them only when the grouping adds semantics, such as `EvidenceBackedVerification` and `ReviewBasedVerification`.

### Use Multiple Inheritance Carefully

OWL allows a class to have multiple superclasses. Use multiple inheritance when every instance of the class truly belongs to each parent class and should inherit the meaning, slots, and restrictions of each parent.

Good examples:

- `McpInterface` can be both an `InterfaceSurface` and an `AgentInterface` if all MCP interfaces are interaction surfaces and agent-facing interfaces.
- `OrganizationApiClient` can be both a `MachineClient` and an `OrganizationScopedResource` if it is both an authenticating machine actor and a resource scoped to an organization.
- `TestVerification` can be both `Verification` and `EvidenceBackedVerification` if test verification always requires evidence.

Use caution:

- Do not use multiple inheritance just to express that two things are related. Use object properties such as `interfaceExposesResource`, `organizationHasApiClient`, or `verificationCoversRequirement`.
- Check inherited slots and restrictions from all parents. Conflicting parent meanings often indicate that one edge should be a relationship instead of a superclass.
- If a class has many parents, review whether a compositional property model would be clearer.

### Introduce A New Class Only When It Adds Meaning

A subclass should usually add at least one of the following:

- New slots that the superclass does not have.
- Different slot facets, values, allowed ranges, or cardinality restrictions.
- Different relationships to other classes.
- Domain language that experts need for communication, navigation, or compatibility with another model.

Good class distinctions:

- `PublicApiSurface`, `HiddenApiSurface`, and `ManagementApiSurface` under `ApiSurface` when access mode changes authentication, exposure, verification, or governance rules.
- `VpcPeeringConnection` and `PrivateEndpointConnection` under `NetworkAccess` when they have different lifecycle states, provider requirements, or operational behavior.
- `TestVerification`, `FormalProofVerification`, and `AnalysisVerification` under `Verification` when they have different evidence requirements and coverage semantics.
- `SinglePrimaryDeployment` and `MultiPrimaryDeployment` under `Deployment` when topology changes failover, replica, write-routing, or verification rules.

Avoid unnecessary classes:

- Do not create `ActiveDeployment`, `PausedDeployment`, and `FailedDeployment` if state changes frequently; use a `deploymentHasState` slot.
- Do not create `EuWestEnvironment` and `UsEastEnvironment` unless region-specific classes have different relationships or restrictions; use `environmentLocatedInRegion`.
- Do not create `HighRiskRequirement` unless high-risk requirements participate in different rules; use risk metadata or a risk slot.
- Do not create a class for every concrete deployment, environment, or test run; use individuals or evidence artifacts when stable, and source data/exported facts when volatile.

Create a new class when the distinction is important to competency questions and affects ontology structure. Use a slot value when the distinction is just a value, status, label, location, count, or frequently changing condition.

Decision test:

| Question | Prefer a new class when yes | Prefer a slot value when yes |
|----------|-----------------------------|------------------------------|
| Does the distinction change inherited slots or restrictions? | `EvidenceBackedVerification` requires evidence. | `deploymentState` is active or paused. |
| Does it change valid relationships to other classes? | `PrivateEndpointConnection` connects to endpoint services. | `environmentRegion` is a named region. |
| Is the distinction stable for the lifetime of an instance? | A `McpInterface` remains an MCP interface. | A deployment can move from active to failed to recovered. |
| Do domain experts use it as a meaningful kind of thing? | `ManagementApiSurface` is a meaningful interface kind. | `resourceName` is only a label. |
| Would many rules or queries target this category? | `FormalProofVerification` has distinct verification semantics. | `replicaCount` is only a number. |

### Decide Class Or Instance By Granularity

Whether a concept is a class or an individual depends on the ontology's intended use. Decide the lowest level of granularity the ontology needs to represent. The most specific things that answer competency questions are usually individuals. More general categories that organize those answers are usually classes.

Use individuals when the model needs stable named records:

- `theSystem` as an instance of `SystemOfInterest`.
- `productionEnvironment` as an instance of `Environment` when the ontology models one stable named environment.
- `activeDeploymentState` as an instance of `DeploymentState`.
- `publicAccessMode` as an instance of `InterfaceAccessMode`.
- `testVerificationType` as an instance of `VerificationType`.

Use classes when the concept can have subclasses, inherited slots, different restrictions, or many instances:

- `Environment` is a class because many environments can exist and may specialize into `ProductionEnvironment`, `StagingEnvironment`, or `EphemeralEnvironment` if those categories carry different rules.
- `Deployment` is a class because many deployment instances can exist and subclasses such as `SinglePrimaryDeployment` or `MultiPrimaryDeployment` may add topology semantics.
- `ApiSurface` is a class because it can specialize into `PublicApiSurface`, `HiddenApiSurface`, and `ManagementApiSurface`.
- `Verification` is a class because it specializes into verification methods with different evidence semantics.

Avoid arbitrary mixed levels:

- Do not make `EnvironmentRegion` a class while `euWestRegion` is a class and `usEastRegion` is an individual unless there is a clear modeling reason.
- Do not make `ProductionEnvironment` an individual in one part of the ontology and a class in another part unless one is a named environment and the other is a category. Use distinct names such as `productionEnvironmentType` for the category and `productionEnvironment` for the individual.
- Do not make concrete customer resources ontology classes just because they have names. A named deployment such as `primaryGraphDeployment` is usually an individual, while `Deployment` remains the class.

If concepts form a natural hierarchy, model them as classes. For example, `InterfaceSurface` -> `ApiInterface` -> `PublicApiSurface` is a class hierarchy. If a term is only a stable value under a class, model it as an individual. For example, `activeDeploymentState` is an individual of `DeploymentState`.

### Limit Class Hierarchy Scope

Do not model everything that could be true about the domain. Model the classes, slots, relationships, and axioms needed for the ontology's competency questions and expected applications. As a practical rule, specialize or generalize at most one extra level beyond the known need unless reuse or compatibility clearly requires more.

Avoid overspecializing:

- Do not model every cloud provider SKU, CPU type, storage implementation, or region variant unless the ontology must reason about those distinctions.
- Do not model every possible API endpoint as a class when `ApiSurface`, `ApiOperation`, or requirement-owned interface contracts are enough.
- Do not model every deployment operation subclass if lifecycle behavior can be represented with `DeploymentOperation` plus operation type/state slots.
- Do not model every test run or log event as ontology classes; use evidence artifacts, source data, or generated facts.

Avoid over-generalizing:

- Do not classify `Operator` as a biological person just because operators are humans. In this ontology, `Operator` is an actor role for access, authorization, and operational workflows.
- Do not classify `Organization` as `Actor` unless organizations participate in the same action/session semantics as users, operators, machine clients, or agents.
- Do not classify `PaymentProvider` under every possible external-system hierarchy unless provider distinctions matter to billing, compliance, or integration behavior.

Avoid unnecessary properties:

- Do not add slots such as UI color, marketing copy, label text, favorite resource, internal implementation field, or arbitrary preference unless they answer competency questions.
- Do not add all conceivable relationships among included terms. Add relationships that support navigation, validation, reasoning, traceability, visualization, or known queries.

Document intentional omissions. If a term could be classified another way in a broader ontology, add a short comment explaining the chosen scope. For example, document that `Operator` is modeled as an `Actor` role rather than as a general `Person`, or that region/provider details are intentionally handled by deployment configuration refinements instead of ontology class expansion.

### Use Disjointness For Mutually Exclusive Classes

Disjoint classes cannot share instances. Declare disjointness when overlap would be a modeling error and validation should catch it.

Good disjointness candidates:

- `PublicApiSurface` and `PrivateApiSurface` if an API surface cannot be both public and private in the ontology's access model.
- `ActiveDeploymentState`, `PausedDeploymentState`, `FailedDeploymentState`, and `TerminatedDeploymentState` if deployment state individuals or classes are mutually exclusive.
- `TestVerification` and `AnalysisVerification` if each verification element has exactly one verification method type.
- `ProductionEnvironmentType` and `EphemeralEnvironmentType` if environment type classification is exclusive.

Do not declare disjointness when multiple inheritance or overlap is valid:

- `McpInterface` and `ManagementApiSurface` may overlap if an MCP surface exposes management operations.
- `EvidenceBackedVerification` and `TestVerification` should not be disjoint if tests are evidence-backed.
- `OperationalArtifact` and `BusinessArtifact` should be disjoint only if the ontology deliberately forbids artifacts that are both operational and business-facing.

Compact OWL/Turtle example:

```turtle
ex:PublicApiSurface owl:disjointWith ex:PrivateApiSurface .

ex:TestVerification owl:disjointWith ex:AnalysisVerification .
```

## Defining Slots

Classes alone rarely provide enough information to answer competency questions. Once a few classes exist, describe the internal structure of each concept by defining its slots: the datatype properties and object properties that characterize instances of the class.

Use the candidate term list from discovery and competency questions. Terms that are not classes often become slots. For each slot candidate, decide which class it describes and attach it to the most general class that can validly have that property. Subclasses inherit the slots of their superclasses.

For a managed platform or system-of-interest ontology:

- `ManagedResource` may have slots such as `resourceIdentifier`, `resourceName`, `resourceCriticality`, and `hasLifecycleState`.
- `Environment` may inherit managed-resource slots and add `environmentRegion`, `environmentType`, and `environmentHasNetworkAccess`.
- `Deployment` may inherit managed-resource slots and add `deploymentUsesVersion`, `deploymentUsesPlan`, `deploymentHasReplica`, `deploymentHasSnapshot`, and `deploymentBelongsToEnvironment`.
- `InterfaceSurface` may have slots such as `interfaceProtocol`, `interfaceAccessMode`, and `interfaceExposesResource`.
- `VerificationEvidence` may have slots such as `evidenceIdentifier`, `evidenceMethod`, `evidenceResult`, and `verifiesRequirement`.

Common slot categories:

| Slot category | Meaning | System-of-interest examples |
|---------------|---------|-----------------------------|
| Intrinsic slots | Properties inherent to the thing being modeled. | deployment version, environment type, interface protocol, lifecycle state, resource criticality |
| Extrinsic slots | Contextual or externally assigned properties. | display name, external provider identifier, region, owner, billing account, support plan |
| Part slots | Physical or logical parts of a structured concept. | deployment has replica, environment has network access, interface has endpoint, system has subsystem |
| Relationship slots | Links from instances of one class to instances of another. | deployment belongs to environment, resource owned by actor, interface exposes resource, evidence verifies requirement |

Attach a slot at the most general valid class. For example, `resourceIdentifier` belongs on `ManagedResource` if every managed resource needs an identifier. `deploymentHasReplica` belongs on `Deployment`, not on `ManagedResource`, because not every managed resource has replicas. `interfaceProtocol` belongs on `InterfaceSurface` if all interface surfaces have a protocol or interaction mechanism.

## Defining Slot Facets

After identifying a slot, define its facets and decide where each facet belongs. Stable semantic facets belong in OWL: property type, domain, range, and true class/property axioms. Operational validation facets belong in SHACL: required fields, closed-world cardinality, numeric bounds, regex patterns, allowed values, and validation messages.

### Cardinality

Cardinality describes how many values a slot may or must have.

| Cardinality facet | Meaning | System-of-interest examples |
|-------------------|---------|-----------------------------|
| Single value | At most one value. | A deployment has one current lifecycle state; an interface has one primary access mode. |
| Multiple values | Any number of values. | An environment can host many deployments; a deployment can have many replicas; a resource can have many metrics. |
| Minimum cardinality | At least N values are required. | A deployment must belong to at least one environment; a verification result must have at least one evidence artifact. |
| Maximum cardinality | At most N values are allowed. | A single-primary deployment may have at most one primary replica; a resource may have at most one owning organization. |
| Zero maximum for subclasses | A subclass explicitly cannot use a slot. | A read-only interface may have zero mutation operations; an unmanaged external system may have zero deployment operations. |

### Value Type

Value type describes what kind of value can fill the slot.

| Value type | Meaning | System-of-interest examples |
|------------|---------|-----------------------------|
| String | Text value. | `resourceName`, `resourceIdentifier`, `interfaceProtocol`, `regionName` |
| Number / integer / float | Numeric value. | `replicaCount`, `cpuLimit`, `retentionDays`, `errorRateThreshold` |
| Boolean | True/false value. | `publiclyReachable`, `readOnly`, `requiresApproval`, `evidencePassed` |
| Enumerated value | One value from a controlled set. | lifecycle state: active, paused, failed, terminated; risk level: low, medium, high, critical |
| Instance value | Link to an individual of an allowed class. | deployment belongs to an `Environment`; resource owned by an `Actor`; interface exposes a `ManagedResource` |

For datatype properties, express value type with an RDF datatype such as `xsd:string`, `xsd:boolean`, `xsd:integer`, or `xsd:decimal` when appropriate. For object properties, express the allowed class with `rdfs:range` when the range is stable. Use semantic-contract SHACL shapes when one or more requirements need closed-world cardinality, datatype, enumeration, numeric range, regex, or allowed-class validation.

### OWL / SHACL Hybrid Datatype Pattern

Use OWL as the single source of truth for what the domain concepts and slots are. Use SHACL as the validation graph for what data must comply with in a specific closed-world contract.

Common XSD ranges for OWL datatype properties:

| Literal kind | XSD datatype examples |
|--------------|-----------------------|
| Text | `xsd:string` |
| Integers | `xsd:integer`, `xsd:int`, `xsd:positiveInteger` |
| Decimal or floating point values | `xsd:decimal`, `xsd:float`, `xsd:double` |
| Booleans | `xsd:boolean` |
| Dates and times | `xsd:dateTime`, `xsd:date` |

In Reqvire, keep the split at the element boundary:

- `ontology` elements under `requirements/Ontologies` own `#### Ontology` Turtle blocks with OWL class declarations plus `owl:DatatypeProperty`/`owl:ObjectProperty` declarations and stable `rdfs:domain`/`rdfs:range`.
- `semantic-contract` elements own `#### Shapes` Turtle blocks.
- The SHACL block declares `sh:NodeShape` resources that use `sh:targetClass` and `sh:path` over OWL classes and properties declared by ontology elements reached through explicit `use` relations.
- Do not redeclare `ex:Employee a owl:Class` or another OWL class inside the SHACL block just to make a shape parse.

OWL ontology example:

~~~markdown
### Workforce Ontology

#### Ontology
```turtle
@prefix ex: <https://example.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:Employee a owl:Class ;
  rdfs:label "Employee"@en .

ex:hasAge a owl:DatatypeProperty ;
  rdfs:domain ex:Employee ;
  rdfs:range xsd:integer ;
  rdfs:label "has age" .
```

#### Metadata
  * type: ontology
---
~~~

SHACL semantic-contract example:

~~~markdown
### Employee Age Validation Contract

#### Shapes
```turtle
@prefix ex: <https://example.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:EmployeeValidationShape a sh:NodeShape ;
  sh:targetClass ex:Employee ;
  sh:property [
    sh:path ex:hasAge ;
    sh:datatype xsd:integer ;
    sh:minInclusive 18 ;
    sh:maxInclusive 65 ;
    sh:message "An employee must be between 18 and 65 years old." ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Owning Requirement](../Product/Example/Requirements.md#owning-requirement)
  * use: [Example Domain Ontology](../Ontologies/Example.md#example-domain-ontology)
---
~~~

This split keeps OWL clean for ontology tools and reasoners while keeping SHACL rules isolated for validation. It also keeps target classes stable for future materialization pipelines: if a SPARQL CONSTRUCT or SHACL Rules workflow later writes inferred triples back into the graph, those triples can type resources as OWL classes that downstream validators or reasoners already understand. Do not imply that Reqvire executes those pipelines unless a requirement explicitly scopes query execution, SHACL-AF rule execution, or persistent RDF store behavior.

## Domain And Range

For object-valued slots, the range is the allowed class of values that may fill the slot. The domain is the class or classes described by the slot. For datatype slots, the range is usually a literal datatype such as `xsd:string`, `xsd:boolean`, or `xsd:integer`.

Choose the most general useful domain and range:

- General enough to cover all valid uses.
- Specific enough that every class in the domain can genuinely have the slot.
- Specific enough that every class in the range is a plausible value.
- Not so broad that the slot becomes meaningless, such as using a root class for every relationship.

System-of-interest examples:

| Slot | Good domain | Good range | Why |
|------|-------------|------------|-----|
| `ownedByActor` | `PlatformResource` | `Actor` | Any managed platform resource can have an owning or accountable actor. |
| `deploymentBelongsToEnvironment` | `Deployment` | `Environment` | Deployments are hosted inside environments; the slot should not be attached to every managed resource. |
| `deploymentHasReplica` | `Deployment` | `Replica` | Replicas are parts of deployments, not of every platform resource. |
| `interfaceExposesResource` | `InterfaceSurface` | `PlatformResource` | Web, API, MCP, and BFF surfaces can expose platform resources. |
| `environmentHasNetworkAccess` | `Environment` | `NetworkAccess` | Network access belongs to the environment boundary. |
| `verificationCoversRequirement` | `Verification` | `Requirement` | Verification evidence or methods cover requirements in the engineering model. |

Reqvire and `red`-style ontology refactoring rules:

- If a proposed domain or range includes a class and its subclass, keep the superclass and remove the subclass.
- If a proposed domain or range lists all subclasses of a parent class, use the parent class instead.
- If a proposed domain or range lists almost all subclasses of a parent class, consider the parent only if the exceptions are not semantically important.
- If a slot applies only to a specialized class, keep it there. For example, `deploymentHasReplica` belongs on `Deployment`, not on `PlatformResource`.
- If a slot applies to all managed resources, attach it at the shared parent. For example, `resourceIdentifier` belongs on `PlatformResource` or `ManagedResource`, not separately on `Deployment`, `Environment`, and `Database`.

Compact OWL/Turtle mapping:

```turtle
@prefix ex: <https://example.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:Deployment a owl:Class ;
  rdfs:subClassOf ex:PlatformResource .

ex:Environment a owl:Class ;
  rdfs:subClassOf ex:PlatformResource .

ex:deploymentBelongsToEnvironment a owl:ObjectProperty ;
  rdfs:domain ex:Deployment ;
  rdfs:range ex:Environment .

ex:resourceIdentifier a owl:DatatypeProperty ;
  rdfs:domain ex:PlatformResource ;
  rdfs:range xsd:string .
```

## Property Details

### Inverse Slots

Two object properties are inverse slots when one relation is the reverse direction of the other. Inverse slots avoid forcing users or tools to author the same relationship twice while still allowing navigation in both directions.

Examples:

| Forward slot | Inverse slot | Meaning |
|--------------|--------------|---------|
| `deploymentBelongsToEnvironment` | `environmentHostsDeployment` | If a deployment belongs to an environment, the environment hosts that deployment. |
| `snapshotOfDeployment` | `deploymentHasSnapshot` | If a snapshot is of a deployment, the deployment has that snapshot. |
| `interfaceExposesResource` | `resourceExposedThroughInterface` | If an interface exposes a resource, the resource is exposed through that interface. |
| `ownedByActor` | `actorOwnsResource` | If a resource is owned by an actor, the actor owns that resource. |
| `verificationCoversRequirement` | `requirementVerifiedBy` | If a verification covers a requirement, the requirement is verified by that verification. |

Compact OWL/Turtle example:

```turtle
ex:deploymentBelongsToEnvironment a owl:ObjectProperty ;
  rdfs:domain ex:Deployment ;
  rdfs:range ex:Environment ;
  owl:inverseOf ex:environmentHostsDeployment .

ex:environmentHostsDeployment a owl:ObjectProperty ;
  rdfs:domain ex:Environment ;
  rdfs:range ex:Deployment .
```

Use inverse slots when both directions are meaningful to competency questions, navigation, visualization, or authoring ergonomics. Do not create inverse properties for every relationship automatically; add them when the reverse name is clear and useful.

### Default Values

Default values are authoring or tool convenience, not hard ontology restrictions. A default value may be filled in when a new instance is created, but it can be changed when the actual instance differs.

Examples:

- A new `InterfaceSurface` may default to `privateAccessMode`, but a public API surface can override it.
- A new `DeploymentOperation` may default to `pendingOperationState`, then move to running, completed, or failed.
- A new requirement may default to medium priority in a requirements tool, but the actual priority can be changed.

Do not confuse default values with fixed values or restrictions. If every instance of a class must always have a value, model that as a restriction or a SHACL rule rather than as a default.

Examples:

- If every `TestVerification` must be evidence-backed, define that as a class restriction, semantic rule, or SHACL shape.
- If every `Deployment` must belong to at least one `Environment`, define a minimum-cardinality rule.
- If `PublicApiSurface` must have `publicAccessMode`, define a fixed value or validation rule instead of relying on a default.

OWL itself does not provide operational default-value behavior. Store defaults as annotations or tool metadata only when the authoring workflow needs them; use SHACL or explicit axioms for enforceable constraints.

## Naming Conventions

Define naming conventions for classes, properties, and individuals before adding many terms, then follow them consistently. Consistent naming improves readability and prevents duplicate classes, synonym drift, and class/property confusion.

Consider the target syntax and tools:

- OWL/Turtle IRIs are easier to maintain when local names avoid spaces and punctuation.
- RDF tooling is case-sensitive; `Deployment` and `deployment` are different local names.
- Some tools share one namespace for classes, properties, and individuals. Avoid relying on case alone to distinguish meanings.
- Prefer stable local names that survive UI label changes, product renames, and translations.

Recommended convention:

| Element kind | Convention | Examples |
|--------------|------------|----------|
| Classes | Singular UpperCamelCase nouns. | `Deployment`, `Environment`, `InterfaceSurface`, `TestVerification` |
| Object properties | lowerCamelCase verb phrase or relation phrase. | `deploymentBelongsToEnvironment`, `interfaceExposesResource`, `verificationCoversRequirement` |
| Datatype properties | lowerCamelCase noun phrase. | `resourceIdentifier`, `environmentRegion`, `verificationMethodName`, `evidencePassed` |
| Individuals | lowerCamelCase stable name. | `theSystem`, `productionEnvironment`, `activeDeploymentState`, `testVerificationType` |
| Controlled values | lowerCamelCase value plus class/context suffix when useful. | `publicAccessMode`, `criticalRiskLevel`, `pendingOperationState` |

Avoid:

- Spaces in local names: use `InterfaceSurface`, not `Interface Surface`.
- Punctuation-heavy names: use `deploymentBelongsToEnvironment`, not `deployment-belongs-to-environment`.
- Plural class names unless the domain term is inherently plural.
- Class and slot names that differ only by case, such as `Environment` and `environment`, when a clearer relation name exists.
- Product/UI labels as canonical class names when a stable domain concept exists.

Use comments or annotation properties for presentation labels, aliases, translations, old names, and product terms. For example, keep `MissionControlInterface` as the canonical class only if it is a stable domain concept; otherwise model it as a label or individual attached to `WebInterface`.

### Singular Or Plural

Class names represent collections of instances, but use one grammatical form consistently. Prefer singular class names because they read well with instance statements:

- `primaryGraphDeployment a Deployment`
- `productionEnvironment a Environment`
- `createDeploymentApiTest a TestVerification`

Do not create both singular and plural forms as separate classes. Use `Deployment`, not both `Deployment` and `Deployments`; use `InterfaceSurface`, not both `InterfaceSurface` and `InterfaceSurfaces`.

### Prefix And Suffix Conventions

Property names should make direction and meaning clear. Use prefixes or suffixes only when they improve readability.

Good patterns:

- `has...` for part or attribute-like object properties: `deploymentHasReplica`, `environmentHasNetworkAccess`, `interfaceHasAccessMode`.
- `...Of...` or `...For...` when the inverse direction is clearer: `snapshotOfDeployment`, `supportCaseForOrganization`.
- Verb phrases for relationships: `deploymentBelongsToEnvironment`, `interfaceExposesResource`, `verificationCoversRequirement`.

Avoid applying `has` mechanically to every property. `hasDeploymentBelongsToEnvironment` is worse than `deploymentBelongsToEnvironment`.

### Other Naming Rules

- Do not add redundant type words such as `Class`, `Property`, `Slot`, or `Individual` to local names. Use `Deployment`, not `DeploymentClass`; use `resourceIdentifier`, not `resourceIdentifierProperty`.
- Avoid abbreviations unless they are standard domain terms. Prefer `ApiInterface` over `ApiIf`, `DeploymentConfiguration` over `DepCfg`, and `VerificationEvidence` over `VerifEvd`.
- Name sibling classes consistently. If one sibling includes the parent concept, the others should too: use `PublicApiSurface`, `HiddenApiSurface`, and `ManagementApiSurface`, not `PublicApiSurface`, `Hidden`, and `Management`.
- Keep property names stable even if product UI labels change. Put UI text in comments or annotations.

## Creating Instances

Create individual instances when the ontology needs named examples, controlled vocabulary records, system-of-interest anchors, external providers, lifecycle states, verification methods, or other stable records. Do not create ontology individuals for arbitrary runtime data unless the ontology explicitly models that data as part of the system definition.

Creating an instance requires three decisions:

1. Choose the class that the individual belongs to.
2. Create a stable individual IRI.
3. Fill slot values with literals or links to other individuals.

System-of-interest example:

```turtle
@prefix ex: <https://example.org/ontology#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:productionEnvironment a ex:Environment ;
  ex:resourceIdentifier "env-prod" ;
  ex:resourceName "Production" ;
  ex:environmentLocatedInRegion ex:euWestRegion ;
  ex:environmentHasType ex:productionEnvironmentType .

ex:primaryGraphDeployment a ex:Deployment ;
  ex:resourceIdentifier "deployment-primary-graph" ;
  ex:resourceName "Primary graph database deployment" ;
  ex:deploymentBelongsToEnvironment ex:productionEnvironment ;
  ex:deploymentUsesVersion ex:neo4j5Version ;
  ex:deploymentUsesPlan ex:businessCriticalPlan ;
  ex:deploymentHasState ex:activeDeploymentState ;
  ex:deploymentHasReplica ex:primaryGraphReplicaA .

ex:primaryGraphReplicaA a ex:Replica ;
  ex:resourceIdentifier "replica-a" ;
  ex:hasLifecycleState ex:activeDeploymentState .
```

Verification and requirement example:

```turtle
@prefix ex: <https://example.org/ontology#> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .

ex:deploymentCreationRequirement a reqvire:Requirement ;
  ex:requirementIdentifier "REQ-DEPLOY-CREATE" .

ex:createDeploymentApiTest a reqvire:TestVerification ;
  ex:verificationMethodName "API integration test" ;
  ex:verificationCoversRequirement ex:deploymentCreationRequirement ;
  ex:evidencePassed true .
```

Prefer individuals for stable vocabulary values such as `activeDeploymentState`, `failedDeploymentState`, `publicAccessMode`, `privateAccessMode`, `testVerificationMethod`, or `criticalRiskLevel`. Type stable vocabulary records explicitly as `owl:NamedIndividual` and their domain class. Prefer requirements, refinements, evidence files, source data, or generated semantic export for volatile facts such as every customer deployment, every test run, every log entry, or every observed metric sample.

## Core Building Blocks

### Classes / Concepts

Use OWL classes for stable semantic categories.

```turtle
ex:DomainConcept a owl:Class ;
  rdfs:comment "Base class for stable terms in the system-of-interest domain model." .

ex:SystemOfInterest a owl:Class ;
  rdfs:subClassOf ex:DomainConcept ;
  rdfs:comment "System being engineered, analyzed, verified, operated, or governed by the Reqvire project." .

ex:ManagedResource a owl:Class ;
  rdfs:subClassOf ex:DomainConcept ;
  rdfs:comment "Resource whose lifecycle, configuration, access, operation, or ownership is managed by the system of interest." .

ex:InterfaceSurface a owl:Class ;
  rdfs:subClassOf ex:DomainConcept ;
  rdfs:comment "Boundary through which actors, systems, services, or tools interact with the system of interest." .

ex:LifecycleState a owl:Class ;
  rdfs:subClassOf ex:DomainConcept ;
  rdfs:comment "Named state used to classify readiness, availability, operation, or lifecycle of a modeled thing." .
```

### Object Properties / Relationships

Use object properties when a relation connects resources.

```turtle
ex:exposedThroughInterface a owl:ObjectProperty ;
  rdfs:domain ex:ManagedResource ;
  rdfs:range ex:InterfaceSurface ;
  rdfs:comment "Associates a managed resource with an interface surface through which it is exposed or operated." .

ex:hasLifecycleState a owl:ObjectProperty ;
  rdfs:domain ex:ManagedResource ;
  rdfs:range ex:LifecycleState ;
  rdfs:comment "Associates a managed resource with a lifecycle or operational state category." .
```

Use `rdfs:domain` and `rdfs:range` only when they are stable. If the property intentionally spans many unrelated classes, describe usage in `rdfs:comment` or a semantic contract rather than forcing misleading global domain/range axioms.

### Datatype Properties / Slots

Use datatype properties for literal-valued attributes.

```turtle
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:resourceIdentifier a owl:DatatypeProperty ;
  rdfs:domain ex:ManagedResource ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable project or platform identifier for a managed resource." .
```

### Individuals / Vocabulary Records

Use explicit `owl:NamedIndividual` plus domain-class typing for enum-like vocabulary, rule records, report kinds, lifecycle values, or other named records. Keep canonical token slots when those literal values are consumed by Markdown metadata, CLI/API payloads, reports, SHACL constraints, or queries.

```turtle
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:stateName a owl:DatatypeProperty ;
  rdfs:domain ex:LifecycleState ;
  rdfs:range xsd:string .

ex:stateMeaning a owl:DatatypeProperty ;
  rdfs:domain ex:LifecycleState ;
  rdfs:range xsd:string .

ex:theSystem a owl:NamedIndividual, ex:SystemOfInterest ;
  rdfs:comment "The concrete system of interest governed by this Reqvire project." .

ex:criticalLifecycleState a owl:NamedIndividual, ex:LifecycleState ;
  ex:stateName "critical" ;
  ex:stateMeaning "State category indicating that resource behavior may block a critical capability." .

ex:collectReportKind a owl:NamedIndividual, ex:ReportKind ;
  ex:reportKindName "collect" ;
  rdfs:comment "Report kind that gathers element context, refinements, attachments, and reachable semantic context." .
```

### Axioms

Use OWL axioms when they are true domain semantics, not just visualization decoration.

```turtle
ex:InterfaceSurface a owl:Class .
ex:IntegrationBoundary a owl:Class ;
  owl:equivalentClass ex:InterfaceSurface .

ex:dependsOnResource a owl:ObjectProperty ;
  owl:inverseOf ex:supportsResource .

ex:impactedByEnvironment a owl:ObjectProperty ;
  owl:propertyChainAxiom (ex:deployedTo ex:environmentHasConstraint) .
```

For disjointness:

```turtle
ex:BusinessArtifact a owl:Class .
ex:LifecycleState a owl:Class .

ex:BusinessArtifact owl:disjointWith ex:LifecycleState .
```

## Reqvire-Specific Placement

- Put ontology elements in `requirements/Ontologies`.
- Use one `#### Ontology` fenced Turtle block per ontology element.
- Use `derivedFrom` only to relate ontology elements to ontology parents.
- Keep shared ontology roots independent from capability roots; non-ontology, non-semantic-contract elements consume ontology terms through `#### Concept References`.
- Do not put `#### Shapes` in ontology elements.

Concept reference example:

```markdown
#### Concept References
  * Managed Resource: https://example.org/ontology#ManagedResource
```

Capability, requirement, refinement, verification-objective, and concrete verification prose can bind readable terms with `#### Concept References` when useful. The referenced IRI or CURIE must be declared by an ontology element in the model. Semantic contracts must not author concept references; their semantic dependencies are declared with `use`/`usedBy`.

## Semantic Contract Boundary

Use SHACL in `semantic-contract` elements when requirements need closed-world validation rules. Link each semantic contract to ontology with `use` and to governed requirements with `constrain`.

~~~markdown
### Interface Contract Shape

#### Shapes
```turtle
@prefix ex: <https://example.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:SystemInterfaceShape
  a sh:NodeShape ;
  sh:targetClass ex:SystemInterface ;
  sh:property [
    sh:path ex:interfaceProtocol ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Owning Requirement](../Product/Example/Requirements.md#owning-requirement)
  * use: [Example Domain Ontology](../Ontologies/Example.md#example-domain-ontology)
---
~~~

## Authoring Checklist

- The element has `type: ontology`.
- A top-down domain frame is written or clearly summarized before adding project-specific terms.
- Competency questions are written or clearly summarized before adding detailed leaves.
- Each class/property/individual supports a competency question, reusable MBSE meaning, or a known system-of-interest modeling need.
- The Turtle parses and is not empty.
- Classes, properties, and individuals use stable IRIs.
- `rdfs:comment` explains non-obvious terms.
- Object properties and datatype properties are not confused.
- Domain/range axioms are stable and useful.
- Vocabulary individuals are typed by ontology classes.
- SHACL shape references point to declared, reachable ontology terms.
- Non-ontology, non-semantic-contract elements use concept references for ontology term bindings.
- Semantic contracts use ontology through `use`/`usedBy` and do not author concept references.
- `reqvire validate` passes.
- `reqvire ontologies` emits the expected terms.
