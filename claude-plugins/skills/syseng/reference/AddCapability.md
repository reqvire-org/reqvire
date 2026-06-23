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
5. Verifications       → Add verification elements for capabilities or leaf requirements
6. Implementation      → Connect `requirement` elements and evidence-backed verifications to code/evidence via `satisfiedBy`
```

## Construction Method

When constructing or refactoring a system model, work from model boundaries inward:

1. Run `reqvire submodels --json` and identify the existing capability-root subgraphs.
2. Run `reqvire search --filter-type="ontology" --short` and identify the ontology terms already available.
3. Decide whether the new content belongs under an existing capability root, a child capability, a new independent capability root, or the shared ontology hierarchy.
4. Add subcapabilities only for meaningful capability slices; do not use capability hierarchy just to share ontology.
5. Put shared vocabulary and stable semantic relationships in ontology; bind capability, requirement, contract, and verification prose to SKOS concepts with `#### Concept References`.
6. Put implementable obligations in requirements that `specify` the local capability.
7. Put local details in compatible contracts owned by the relevant requirement.
8. Use contract_bindings, not hierarchy, when another requirement root needs reusable requirement-owned contracts. Use `use`/`usedBy` for semantic-contract ontology dependencies and `constrain`/`constrainedBy` for semantic-contract requirement dependencies.
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
- What semantic meaning, domain vocabulary, data shape, or policy contract must be shared? (`ontology` for vocabulary referenced through concept references, `semantic-contract` for reusable SHACL profiles that explicitly use ontology and constrain requirements)
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
- Capability and requirement contracts keep exact command behavior, payload fields, outputs, state behavior, validation messages, file paths, and workflow steps.
- Verifications keep evidence expectations and test assertions.

### Adding Requirements

Create requirements using the `--content` flag or by piping element content to the add command:

```bash
# Add capability using --content flag (preferred)
reqvire add system-model/Product/CapabilityName/CapabilityFeature.md --content '### Capability Name

Capability scope and purpose. Describe what the system is able to accomplish, not how it is implemented.

#### Metadata
  * type: capability
'

# Add ontology reused by a capability
reqvire add system-model/Ontologies/CapabilityName.md <<'EOF'
### Capability Ontology

Defines shared domain meaning for the capability.

#### Metadata
  * type: ontology
  * ontology_base: https://example.org/ontology/capability-name
  * ontology_prefix: ex

#### Ontology

```turtle
@prefix ex: <https://example.org/ontology/capability-name#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:DomainConcept a owl:Class .
```
EOF

# Add requirement using heredoc (stdin)
reqvire add system-model/Product/CapabilityName/Requirements.md <<'EOF'
### System Capability Implementation

The system shall implement the capability using defined algorithms.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Capability Name](CapabilityFeature.md#capability-name)
EOF

# Override existing element (replace by name) - useful for cleanup after merge
reqvire add system-model/File.md --override <<'EOF'
### Existing Element Name

Updated content with corrections.

#### Metadata
  * type: requirement
---
EOF
```

Once requirements are created, establish traceability using the link command:

```bash
# Link requirement to capability
reqvire link "System Capability Implementation" "specify" "Capability Name"

# Link capability to requirement (opposite direction of specify)
reqvire link "Capability Name" "specifiedBy" "System Capability Implementation"

# Link child requirement to parent requirement
reqvire link "Child Requirement" "derivedFrom" "System Capability Implementation"
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
- Otherwise define them under `#### Behaviors` or `#### Specifications` subsection of the requirement

### Adding Contract Elements

Create contract elements when they need to be referenced by multiple requirements:

```bash
# Add specification element
reqvire add system-model/Specifications.md <<'EOF'
### Data Format Specification

The data format shall follow JSON Schema version 7 with strict validation.

#### Metadata
  * type: specification
EOF

# Add constraint element
reqvire add system-model/Constraints.md <<'EOF'
### Performance Constraint

All API responses shall complete within 200ms under normal load.

#### Metadata
  * type: constraint
EOF

# Add behavior element
reqvire add system-model/Behaviors.md <<'EOF'
### Error Recovery Behavior

When an error occurs, the system shall log the error, notify the user, and attempt recovery.

#### Metadata
  * type: behavior
EOF
```

Link contracts to requirements using relations or contract_bindings:

```bash
# Link contract to requirement using definedBy relation (owner defines it)
reqvire link "Data Processing Requirement" "definedBy" "Data Format Specification"

# Reuse requirement contract element across explicit requirement subgraph boundaries
# The contract must be owned by a requirement via definedBy
reqvire link "Other Capability Requirement" bindContract "Performance Constraint"

# Link a reusable semantic contract to a requirement and ontology
reqvire link "Requirement Shape Contract" "constrain" "Other Capability Requirement"
reqvire link "Requirement Shape Contract" "use" "Capability Ontology"

# Reuse file (design document, specification document)
reqvire link "Architecture Requirement" bindContract "docs/architecture.pdf"

# Link to implementation file or external URL
# Note: capability must not use satisfiedBy/satisfy.
reqvire link "System Requirement" "satisfiedBy" "src/auth/login.rs"
reqvire link "Compliance Requirement" "trace" "https://example.com/spec.html"
```

**Contract Bindings constraints:**
- Contracts must have a `define` relation before being reused
- Capabilities do not author contract_bindings; use `#### Concept References` for SKOS concept bindings
- Requirements may reuse only requirement-owned `specification`, `constraint`, `behavior`, `state`, or `input-output` contracts
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

```bash
# Add test verification
reqvire add system-model/Verifications/Product/CapabilityName/CapabilityTests.md <<'EOF'
### Capability Test

Test verifies the capability works correctly:
1. Input validation passes
2. Output matches expected format
3. Error handling works

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Capability Requirement](../../../Product/CapabilityName/Requirements.md#capability-requirement)
  * satisfiedBy: [test_capability.rs](../../../../tests/test_capability.rs)
EOF

# Add analysis verification
reqvire add system-model/Verifications/PerformanceAnalysis.md <<'EOF'
### Performance Analysis

Analysis verifies system meets performance requirements through load testing.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Performance Requirement](../System/Performance.md#performance-requirement)
EOF
```

Link verifications to requirements and test implementations:

```bash
# Link verification to requirement (verify relation)
reqvire link "Capability Test" "verify" "Capability Requirement"

# Link from requirement to verification (verifiedBy relation, opposite direction)
reqvire link "Capability Requirement" "verifiedBy" "Capability Test"

# Link test verification to test implementation
reqvire link "Capability Test" "satisfiedBy" "tests/test_capability.rs"
```

## Step 5: Validate and Check Coverage

After adding requirements and verifications, follow the standard validation workflow. See [SKILL.md Validation & Quality Checklist](../SKILL.md#validation--quality-checklist) for the complete procedure:

1. `reqvire validate` - Check model structure
2. `reqvire lint --fix` - Fix auto-fixable issues
3. `reqvire coverage` - Verify leaf verification coverage and requirement-only implementation coverage
4. `reqvire format --fix` - Normalize formatting

Additionally, use `reqvire resources` to see all files referenced by the model through `satisfiedBy` relations and contract_bindings.

## Complete Example

### 1. Capability
```markdown
### User Authentication

Authentication capability for access-controlled product areas.

#### Metadata
  * type: capability

#### Contract Bindings
  * [Authentication Ontology](Ontologies/Auth.md#authentication-ontology)

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

```bash
# Move element to different file
reqvire mv "Capability Requirement" "system-model/Product/CapabilityName/NewFile.md"

# Move element to specific position (0-based index)
reqvire mv "Capability Requirement" "system-model/Product/CapabilityName/Requirements.md" 0  # Move to top

# Move entire file to new location
reqvire mv-file "system-model/Old.md" "system-model/Product/CapabilityName/New.md"

# Merge file into existing file (squash - combine contents)
reqvire mv-file --squash "system-model/Source.md" "system-model/Target.md"
```

**When to reorganize:**
- Grouping related requirements into capability-specific files
- Splitting large files into manageable sections
- Consolidating scattered constraints into Constraints.md
- Moving verifications to match requirement structure
