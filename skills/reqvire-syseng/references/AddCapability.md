# Adding New Capabilities & Requirements

**Key principle**: Capabilities define stable system abilities; requirements define implementable obligations. No implementation without requirements, and no requirement cluster without a clear capability context.

**For common commands** (link, add, validate, etc.), see [SKILL.md Command Reference](../SKILL.md#command-reference) and [Quick Start Guide](../SKILL.md#quick-start-common-workflows).

## MBSE Workflow

```
1. Capability first       → Define the coherent operational/system ability
2. Semantic context    → Add concept references when shared SKOS concepts matter
3. Requirements        → Define what the system shall do (never skip implementable obligations)
4. Contracts         → Add specifications, constraints, behaviors, state, and input-output contracts as needed
   Semantic contracts  → Add reusable SHACL contracts with explicit ontology use when closed-world checks are needed
5. Verifications       → Add verification elements for leaf requirements; capabilities receive roll-up coverage
6. Implementation      → Connect `requirement` elements and evidence-backed verifications to code/evidence via `satisfiedBy`
```

## Construction Method

When constructing or refactoring a system model, work from model boundaries inward:

1. Run `reqvire submodels --json` and identify the existing capability-root subgraphs.
2. Run `reqvire search --filter-type="ontology" --short` and identify the ontology terms already available.
3. Decide whether the new content belongs under an existing capability root, a child capability, a new independent capability root, or the shared ontology hierarchy.
4. Add subcapabilities only for meaningful capability slices; do not use capability hierarchy just to share ontology.
5. Put curated terminology in native concept elements, put structural vocabulary and stable semantic relationships in ontology, and bind capability, requirement, contract, and verification prose to SKOS concepts with `#### Concept References`.
6. Put implementable obligations in requirements that `specify` the local capability.
7. Put local details in compatible contracts owned by the relevant requirement.
8. Use Contract Bindings for shared implementation obligations and Contract References for content dependencies when another requirement root needs a requirement-owned contract. Use `use`/`usedBy` for semantic-contract ontology dependencies and `constrain`/`constrainedBy` for semantic-contract requirement dependencies.
9. Validate `submodels`, `collect`, and change-impact paths after each boundary slice.

## Capability Design Rules

A capability represents a coherent operational, product, business, regulatory, or system ability that the system provides or supports.

Capabilities are first-class graph nodes and the primary semantic bridge between:
- ontology: meaning and domain concepts
- requirements: implementable obligations
- verification: evidence the capability is satisfied

Capabilities should describe what the system is able to accomplish rather than how it is implemented.

A capability is not:
- a UI screen
- a deployment artifact
- a code module
- a ticket/task
- a low-level implementation detail

A capability should represent one coherent operational/system concern, one meaningful traceability anchor, and one stable semantic concept. It should remain stable, composable, implementation-independent, verifiable, and understandable by both humans and AI systems.

If a capability becomes too broad or contains multiple independently verifiable concerns, decompose it into child capabilities. Use child capabilities when concerns differ in verification, ownership, lifecycle, architecture impact, operational semantics, or requirement clusters.

Optional subsections such as `#### Stakeholder Need`, `#### Feature`, `#### Operational Context`, `#### Regulatory Driver`, `#### Mission Objective`, `#### Service Context`, `#### AI Context`, and `#### Notes` can enrich capability content. They do not replace graph structure; create child capabilities when independent traceability or verification is required.

## Step 1: Understand the Capability Scope

Before creating requirements, answer:
- What coherent operational, product, business, regulatory, or system ability does this address? (`capability`)
- What semantic meaning, domain vocabulary, data shape, or policy contract must be shared? (`concept-scheme`/`concept` for curated vocabulary referenced through concept references, `ontology` for structural vocabulary, `semantic-contract` for reusable SHACL profiles that explicitly use ontology and constrain requirements)
- What technical capabilities are needed? (`requirement`)
- Are there constraints or limits to define?
- How will this be verified?

## Step 2: Create Capability and Requirement Hierarchy

```
Capability (coherent operational/system ability)
    ├── Concept References → SKOS concepts
    ├── derive → Subcapability
    └── specifiedBy ← Requirement
                     ├── definedBy → source / specification / constraint / behavior / state / input-output
                     ├── constrainedBy → semantic-contract → use → ontology
                     └── derive → Child Requirement
```

**Guidelines:**
- Start with `capability` for the coherent ability, not a UI screen, task, module, or implementation detail
- Check existing capability roots before adding a new one; preserve independent submodels unless the new work truly belongs in the same capability root
- Use child capabilities for real independently verifiable operational, product, interface, stakeholder, regulatory, or domain slices
- Use `source` contracts on requirements for stakeholder, regulatory, contractual, or external context
- Use `specification`, `constraint`, `behavior`, `state`, and `input-output` contracts when requirement-level context needs additional operational detail
- Use `ontology` elements for ontology/vocabulary contracts
- Use `semantic-contract` elements for SHACL shape profiles over explicitly used ontology; link them to requirements with `constrain`/`constrainedBy`
- Use `specify` / `specifiedBy` to connect requirements to capabilities
- Derive `requirement` elements only from other requirements
- Keep requirements atomic and testable
- Use EARS patterns for clear statements

### Capability vs Requirement vs Ontology vs Feature

A `capability` answers:
- What the system can accomplish.
- What coherent operational/system concern this represents.
- What stakeholder need, feature context, operational context, regulatory driver, mission objective, service context, AI context, source context, or ontology defines its meaning.
- Which requirements specify this capability.
- Which verified requirements provide coverage for this capability.

A capability is not a weaker requirement. Use a `capability` when the statement is about stable operational ability, product scope, stakeholder value, regulatory domain, ownership, planning, or capability grouping. Capabilities are not directly verified or directly satisfied; implementation and verification coverage roll up from the requirements that specify them.

A `requirement` answers:
- What must the system do?
- Under what condition, interface, state, or scope?
- What implementation or evidence can satisfy it?
- What verification proves it?

A requirement is the obligation anchor. It should stay testable, implementation-facing, and evidence-facing. Requirements usually read as "The system shall..." and are the elements verified by verification elements and satisfied by implementation.

Use an `ontology` element when the content defines domain meaning or a machine-checkable ontology vocabulary that requirements can rely on:
- vocabulary and ontology terms: "AccessToken is a kind of Credential"
- business object structure: "AccessToken has subject and expiration"
- allowed semantic relationships between concepts
- valid domain object structure: "these fields form a valid token"
- domain term meaning: "this term means..."
- external or cross-subgraph ontology terms that multiple requirements should apply

Use a `semantic-contract` when a requirement obligation needs a SHACL shape profile over ontology concepts. Semantic contracts must have `#### Shapes`, must use ontology through `use`/`usedBy`, constrain one or more requirements with `constrain`/`constrainedBy`, and must not have `#### Ontology`.

A feature is often user-facing, roadmap-oriented, or product-oriented. Capabilities are broader and more stable. Features may be described inside capability content, but the `capability` element remains the primary traceable graph node.

Do not turn every obligation into an ontology. If the statement says what the system must do, keep it as a requirement and point it to the semantic contract when a closed-world profile is needed. If the statement is `X is a Y`, `X has property Z`, `X relates to Y`, `these fields form a valid object`, or `this domain term means...`, keep it in ontology when it is stable shared domain meaning:

```text
Requirement:
  The system shall reject API requests whose access token does not conform to the Access Token semantic contract.

Ontology:
  auth:AccessToken, auth:subject, auth:expiresAt.

Semantic contract:
  SHACL shape requiring the fields/properties needed by this specific obligation.
```

Good split:
- Capability: `API Authentication`
- Ontology: `Access Token Ontology`
- Semantic contract: `Access Token Validation Contract`
- Requirement: `The system shall reject API requests whose access token does not conform to the Access Token validation contract.`
- Input-output contract, when needed: local request/response examples or API-specific representation details

Keep domain definitions and reusable vocabulary in ontology when they are stable and shared. Keep requirement-obligation-specific closed-world constraints in semantic contracts. Keep workflow behavior, implementation obligations, acceptance commitments, and verification scope in requirements.

When splitting existing prose, do not lose meaning:
- Capability prose keeps capability scope and why the area exists.
- Ontology keeps reusable terms and relationships.
- Requirement-owned contracts keep exact command behavior, payload fields, outputs, state behavior, validation messages, file paths, and workflow steps.
- Verifications keep evidence expectations and test assertions.

### Adding Requirements

Create requirements through typed `reqvire.add_element` calls with complete Markdown definitions:

```text
# Add capability
{"name": "reqvire.add_element", "arguments": {"file": "system-model/Product/CapabilityName/CapabilityFeature.md", "content": "### Capability Name\n\nCapability scope and purpose. Describe what the system is able to accomplish, not how it is implemented.\n\n#### Metadata\n  * type: capability\n"}}

# Add ontology reused by a capability
{"name": "reqvire.add_element", "arguments": {"file": "system-model/Ontologies/CapabilityName.md", "content": "### Capability Ontology\n\nDefines shared domain meaning for the capability.\n\n#### Metadata\n  * type: ontology\n  * ontology_base: https://example.org/ontology/capability-name\n  * ontology_prefix: ex\n\n#### Ontology\n\n```turtle\n@prefix ex: <https://example.org/ontology/capability-name#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\nex:DomainConcept a owl:Class .\n```"}}

# Add requirement with its capability parent
{"name": "reqvire.add_element", "arguments": {"file": "system-model/Product/CapabilityName/Requirements.md", "content": "### System Capability Implementation\n\nThe system shall implement the capability using defined algorithms.\n\n#### Metadata\n  * type: requirement\n\n#### Relations\n  * specify: [Capability Name](CapabilityFeature.md#capability-name)"}}

# Preview a complete replacement of the requirement created above
{"name": "reqvire.add_element", "arguments": {"file": "system-model/Product/CapabilityName/Requirements.md", "content": "### System Capability Implementation\n\nThe system shall implement the corrected capability behavior.\n\n#### Metadata\n  * type: requirement\n\n#### Relations\n  * specify: [Capability Name](CapabilityFeature.md#capability-name)\n", "override_existing": true, "dry_run": true}}
```

Read the current element before preparing an override. The supplied definition replaces its authored prose, metadata, relations and dependency sections, so preserve the outgoing declarations still intended. Externally authored children and contract/concept references remain attached when the complete candidate is valid; do not unlink children to edit their parent's definition. Review the preview and apply the same complete content with `dry_run: false`. Moving the definition to another file rewrites incoming destinations. A missing name follows ordinary creation; actual deletion still protects children from becoming orphaned.

Once requirements are created, establish traceability using the link command:

```text
# Link requirement to capability
{"name": "reqvire.link", "arguments": {"source": "System Capability Implementation", "relation_type": "specify", "target": "Capability Name"}}

# Link capability to requirement (opposite direction of specify)
{"name": "reqvire.link", "arguments": {"source": "Capability Name", "relation_type": "specifiedBy", "target": "System Capability Implementation"}}

# Link child requirement to parent requirement
{"name": "reqvire.link", "arguments": {"source": "Child Requirement", "relation_type": "derivedFrom", "target": "System Capability Implementation"}}
```

**Relation types**: `derivedFrom` (child -> parent inside same family, including verification-family hierarchy), `derive` (parent -> child inside same family), `specify` (requirement -> capability), `specifiedBy` (capability -> requirement), `verifiedBy` (requirement -> concrete verification), `verify` (concrete verification -> requirement), `satisfiedBy` (requirement/test-verification/formal-proof-verification -> implementation or evidence), `satisfy` (implementation/evidence -> requirement/test-verification/formal-proof-verification), `definedBy` (requirement -> contract), `define` (contract -> requirement)

## Step 3: Add Contracts (if needed)

Add contracts only when:
- **Specifications** - Detailed definitions needed, referenced by multiple requirements
- **Constraints** - Limits/boundaries exist (add to Constraints.md)
- **Behaviors** - Complex state/flow logic needs documentation
- **State** - State machines, lifecycle states, and state-dependent contracts
- **Input-output** - Payloads, messages, schemas, examples, and fixtures
- **Ontology** - Shared semantic meaning and vocabulary referenced by model elements
- **Semantic contracts** - Reusable SHACL shape profiles over explicitly used ontology that constrain requirements

Link requirement-owned contracts via `definedBy` from the requirement that owns the contract. Link semantic contracts to ontology with `use` and to requirements with `constrain`/`constrainedBy`. Use `#### Concept References` when capability, requirement, contract, or verification prose needs explicit SKOS concept bindings.

### Contract Best Practices

- **Constraints** should always be in constraint element type
- Group constraints in single file (e.g., `Constraints.md` in requirements root)
- Define **Behaviors** and **Specifications** as elements only if other requirements depend on them
- Define **ontology** elements when requirements need shared domain meaning or ontology vocabulary
- Define **semantic contracts** as reusable SHACL contracts when one or more requirement obligations need a shape profile over explicitly used ontology context
- Otherwise keep the EARS obligation in the requirement body and use `#### Details` for additional EARS obligations. Author implementation detail in a compatible requirement-owned contract.

### Adding Contract Elements

Create each contract with its existing requirement owner in the same typed request. The following examples use the requirement created in Step 2:

```text
# Add specification element
{"name": "reqvire.add_element", "arguments": {"file": "system-model/Specifications.md", "content": "### Data Format Specification\n\nThe data format shall follow JSON Schema version 7 with strict validation.\n\n#### Metadata\n  * type: specification\n\n#### Relations\n  * define: [System Capability Implementation](Product/CapabilityName/Requirements.md#system-capability-implementation)\n"}}

# Add constraint element
{"name": "reqvire.add_element", "arguments": {"file": "system-model/Constraints.md", "content": "### Performance Constraint\n\nAll API responses shall complete within 200ms under normal load.\n\n#### Metadata\n  * type: constraint\n\n#### Relations\n  * define: [System Capability Implementation](Product/CapabilityName/Requirements.md#system-capability-implementation)\n"}}

# Add behavior element
{"name": "reqvire.add_element", "arguments": {"file": "system-model/Behaviors.md", "content": "### Error Recovery Behavior\n\nWhen an error occurs, the system shall log the error, notify the user, and attempt recovery.\n\n#### Metadata\n  * type: behavior\n\n#### Relations\n  * define: [System Capability Implementation](Product/CapabilityName/Requirements.md#system-capability-implementation)\n"}}
```

Ownership is already declared by each contract’s `define` relation. Use bindings for shared implementation obligations and references for content dependencies:

```text
# Reuse requirement contract element across explicit requirement subgraph boundaries
# The contract must be owned by a requirement via definedBy
{"name": "reqvire.link", "arguments": {"source": "Other Capability Requirement", "relation_type": "bindContract", "target": "Performance Constraint"}}

# Link a reusable semantic contract to a requirement and ontology
{"name": "reqvire.link", "arguments": {"source": "Requirement Shape Contract", "relation_type": "constrain", "target": "Other Capability Requirement"}}
{"name": "reqvire.link", "arguments": {"source": "Requirement Shape Contract", "relation_type": "use", "target": "Capability Ontology"}}

# Reference a requirement-owned contract for documentation/change review
{"name": "reqvire.link", "arguments": {"source": "Architecture Documentation", "relation_type": "referenceContract", "target": "Performance Constraint"}}

# Link to an implementation artifact
# Note: capability must not use satisfiedBy/satisfy.
{"name": "reqvire.link", "arguments": {"source": "System Requirement", "relation_type": "satisfiedBy", "target": "src/auth/login.rs"}}
```

**Contract dependency constraints:**
- Contracts must have a `define` relation before being reused
- Only requirements author Contract Bindings or Contract References; use `#### Concept References` for SKOS concept dependencies
- Requirements may reuse only compatible requirement-owned `source`, `specification`, `constraint`, `behavior`, `state`, or `input-output` contracts
- A requirement cannot contain both Contract Bindings and Contract References
- Bindings cross requirement hierarchies and impose implementation obligations; references preserve content dependencies and must remain acyclic through contract owners, bindings, and requirement ancestry
- Semantic contracts must not author `#### Concept References`; they depend on ontology through `use`

## Step 4: Add Verifications

**Bottom Roll-Up Strategy:**
- Add verification elements for **leaf requirements only**
- Parent requirements inherit verification from children
- Avoid redundant verify relations

### Verification Types

Choose appropriate type:
- `verification-objective` - Planning/grouping objective for verification work; may parent concrete verification elements through `derivedFrom` but must not use `verify` or `satisfiedBy`
- `test` - Automated or manual testing (can have `satisfiedBy` to test code)
- `analysis` - Review, calculation, simulation
- `inspection` - Visual examination, audit
- `demonstration` - Showing capability works

### Adding Verification

Create verification elements for leaf requirements:

```text
# Add test verification
{"name": "reqvire.add_element", "arguments": {"file": "system-model/Verifications/Product/CapabilityName/CapabilityTests.md", "content": "### Capability Test\n\nTest verifies the capability works correctly:\n1. Input validation passes\n2. Output matches expected format\n3. Error handling works\n\n#### Metadata\n  * type: test-verification\n\n#### Relations\n  * derivedFrom: [Verification Objective](../../Objectives.md#verification-objective)\n  * verify: [Capability Requirement](../../../Product/CapabilityName/Requirements.md#capability-requirement)\n  * satisfiedBy: [test_capability.rs](../../../../tests/test_capability.rs)"}}

# Add analysis verification
{"name": "reqvire.add_element", "arguments": {"file": "system-model/Verifications/PerformanceAnalysis.md", "content": "### Performance Analysis\n\nAnalysis verifies system meets performance requirements through load testing.\n\n#### Metadata\n  * type: analysis-verification\n\n#### Relations\n  * derivedFrom: [Verification Objective](Objectives.md#verification-objective)\n  * verify: [Performance Requirement](../System/Performance.md#performance-requirement)"}}
```

Link verifications to requirements and test implementations:

```text
# Link verification to requirement (verify relation)
{"name": "reqvire.link", "arguments": {"source": "Capability Test", "relation_type": "verify", "target": "Capability Requirement"}}

# Link from requirement to verification (verifiedBy relation, opposite direction)
{"name": "reqvire.link", "arguments": {"source": "Capability Requirement", "relation_type": "verifiedBy", "target": "Capability Test"}}

# Link test verification to test implementation
{"name": "reqvire.link", "arguments": {"source": "Capability Test", "relation_type": "satisfiedBy", "target": "tests/test_capability.rs"}}
```

## Step 5: Validate and Check Coverage

After adding requirements and verifications, follow the standard validation workflow. See [SKILL.md Validation & Quality Checklist](../SKILL.md#validation--quality-checklist) for the complete procedure:

1. `reqvire validate` - Check model structure
2. `{"name": "reqvire.lint", "arguments": {"fix": true}}` - Fix auto-fixable issues
3. `reqvire coverage` - Verify leaf verification coverage and requirement-only implementation coverage
4. `{"name": "reqvire.format", "arguments": {"fix": true}}` - Normalize formatting

Additionally, use `reqvire resources` to see all files referenced by the model through `satisfiedBy` relations. Bound and referenced contracts are model elements, not file artifacts.

## Complete Example

### 1. Capability
```markdown
### User Authentication

Authentication capability for access-controlled product areas.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Password Authentication](System/Auth.md#password-authentication)
  * specifiedBy: [Session Management](System/Auth.md#session-management)
```

### 2. Ontology
````markdown
### Authentication Ontology

Defines the shared authentication domain vocabulary.

#### Metadata
  * type: ontology
  * ontology_base: https://example.org/ontology/auth
  * ontology_prefix: auth

#### Ontology

```turtle
@prefix auth: <https://example.org/ontology/auth#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

auth:AuthenticatedSession a owl:Class .
auth:UserCredential a owl:Class .
```
````

### 3. Requirements
```markdown
### Password Authentication

The system shall validate user credentials against stored password hashes.

#### Metadata
  * type: requirement

#### Relations
  * specify: [User Authentication](Feature.md#user-authentication)
  * constrainedBy: [Password Authentication Semantic Contract](../../../Ontologies/Auth.md#password-authentication-semantic-contract)
  * satisfiedBy: [auth.rs](../../src/auth.rs)
```

### 4. Semantic Contract
````markdown
### Password Authentication Semantic Contract

Defines the closed-world SHACL profile used by the password-authentication obligation.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Password Authentication](../Product/Auth/Requirements.md#password-authentication)
  * use: [Authentication Ontology](../Ontologies/Auth.md#authentication-ontology)

#### Shapes

```turtle
@prefix auth: <https://example.org/ontology/auth#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

auth:UserCredentialShape
  a sh:NodeShape ;
  sh:targetClass auth:UserCredential .
```
````

```markdown
### Session Management

The system shall create and manage user sessions after successful authentication.

#### Metadata
  * type: requirement

#### Relations
  * specify: [User Authentication](Feature.md#user-authentication)
  * definedBy: [Session Timeout Constraint](Constraints.md#session-timeout)
```

### 5. Constraint
```markdown
### Session Timeout

Session timeout limit used by session-management behavior.

#### Metadata
  * type: constraint

#### Relations
  * define: [Session Management](../System/Auth.md#session-management)
```

### 6. Verification
```markdown
### Authentication Test

Test verifies user authentication flow:
1. Valid credentials grant access
2. Invalid credentials are rejected
3. Sessions expire correctly

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Password Authentication](../Requirements.md#password-authentication)
  * verify: [Session Management](../Requirements.md#session-management)
  * satisfiedBy: [test_auth.rs](../../tests/test_auth.rs)
```

## File Organization

Typical structure:
```
system-model/
├── Product/
│   └── CapabilityA/
│       ├── Feature.md          # Capability roots and child capabilities
│       ├── Requirements.md     # Requirements that specify this capability
│       └── Specifications.md   # Requirement-owned contracts
├── Operations/
│   └── CapabilityB/
├── Ontologies/                  # Canonical home for ontology and semantic-contract elements
└── Verifications/               # Central verification plane, grouped by domain
```

### Reorganizing Elements

Move elements between files or reposition within files:

```text
# Move element to different file
{"name": "reqvire.move_element", "arguments": {"element_name": "Capability Requirement", "file": "system-model/Product/CapabilityName/NewFile.md"}}

# Move element to specific position (0-based index)
{"name": "reqvire.move_element", "arguments": {"element_name": "Capability Requirement", "file": "system-model/Product/CapabilityName/Requirements.md"}}

# Move entire file to new location
{"name": "reqvire.move_file", "arguments": {"source_file": "system-model/Old.md", "target_file": "system-model/Product/CapabilityName/New.md"}}

# Move an entire folder subtree to new location
{"name": "reqvire.move_folder", "arguments": {"source_folder": "system-model/Product/DraftCapability", "target_folder": "system-model/Product/CapabilityName"}}

# Merge file into existing file (squash - combine contents)
{"name": "reqvire.move_file", "arguments": {"source_file": "system-model/Source.md", "target_file": "system-model/Target.md", "squash": true}}
```

**When to reorganize:**
- Grouping related requirements into capability-specific files
- Splitting large files into manageable sections
- Consolidating scattered constraints into Constraints.md
- Moving verifications to match requirement structure
