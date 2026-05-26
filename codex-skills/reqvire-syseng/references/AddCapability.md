# Adding New Capabilities & Requirements

**Key principle**: Capabilities define stable system abilities; requirements define implementable obligations. No implementation without requirements, and no requirement cluster without a clear capability context.

**For common commands** (link, add, validate, etc.), see [SKILL.md Command Reference](../SKILL.md#command-reference) and [Quick Start Guide](../SKILL.md#quick-start-common-workflows).

## MBSE Workflow

```
1. Capability first       → Define the coherent operational/system ability
2. Semantic context    → Add source, ontology, and capability refinements when domain meaning or external contracts matter
3. Requirements        → Define what the system shall do (never skip implementable obligations)
4. Refinements         → Add specifications, constraints, behaviors, state, input-output, and semantic-contract refinements as needed
5. Verifications       → Add verification elements for capabilities or leaf requirements
6. Implementation      → Connect `requirement` elements and evidence-backed verifications to code/evidence via `satisfiedBy`
```

## Construction Method

When constructing or refactoring a system model, work from model boundaries inward:

1. Run `reqvire submodels --json` and identify the existing capability-root subgraphs.
2. Run `reqvire search --filter-type="ontology" --short` and identify the ontology terms already available.
3. Decide whether the new content belongs under an existing capability root, a child capability, a new independent capability root, or the shared ontology hierarchy.
4. Add subcapabilities only for meaningful capability slices; do not use capability hierarchy just to share ontology.
5. Put shared vocabulary and stable semantic relationships in ontology; attach that ontology from the owning or consuming capability so requirements inherit it through capability context.
6. Put implementable obligations in requirements that `specify` the local capability.
7. Put local details in capability-owned or requirement-owned refinements.
8. Use attachments, not hierarchy, when another capability root needs ontology or reusable requirement-owned contracts from this one.
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
- What semantic meaning, domain vocabulary, data shape, or policy contract must be shared? (`ontology` for vocabulary, `semantic-contract` for capability-owned or requirement-owned SHACL profiles)
- What technical capabilities are needed? (`requirement`)
- Are there constraints or limits to define?
- How will this be verified?

## Step 2: Create Capability and Requirement Hierarchy

```
Capability (coherent operational/system ability)
    ├── refinedBy → source / semantic-contract / specification / constraint / behavior / state / input-output
    ├── attach → ontology
    ├── derive → Subcapability
    ├── verifiedBy → Verification
    └── specifiedBy ← Requirement
                     └── derive → Child Requirement
```

**Guidelines:**
- Start with `capability` for the coherent ability, not a UI screen, task, module, or implementation detail
- Check existing capability roots before adding a new one; preserve independent submodels unless the new work truly belongs in the same capability root
- Use child capabilities for real independently verifiable operational, product, interface, stakeholder, regulatory, or domain slices
- Use `source` refinements for stakeholder, regulatory, contractual, or external context
- Use `semantic-contract`, `specification`, `constraint`, `behavior`, `state`, and `input-output` refinements when capability-level context needs a contract before requirements are derived
- Use `ontology` elements for ontology/vocabulary contracts
- Use capability-owned or requirement-owned `semantic-contract` refinements for SHACL shape profiles over reachable ontology context
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
- Which verification evidence directly verifies it when capability-level evidence is appropriate.

A capability is not a weaker requirement. Use a `capability` when the statement is about stable operational ability, product scope, stakeholder value, regulatory domain, ownership, planning, or capability grouping. Capabilities may be directly verified but are not directly satisfied; implementation coverage rolls up from the requirements that specify them.

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

Use a `semantic-contract` when a specific capability or obligation needs a SHACL shape profile over reachable ontology concepts. Semantic contracts must have `#### Shapes`, must refine exactly one compatible capability or requirement owner, and must not have `#### Ontology`.

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
- Input-output refinement, when needed: local request/response examples or API-specific representation details

Keep domain definitions and reusable vocabulary in ontology when they are stable and shared. Keep capability-level or obligation-specific closed-world constraints in semantic contracts. Keep workflow behavior, implementation obligations, acceptance commitments, and verification scope in requirements.

When splitting existing prose, do not lose meaning:
- Capability prose keeps capability scope and why the area exists.
- Ontology keeps reusable terms and relationships.
- Capability and requirement refinements keep exact command behavior, payload fields, outputs, state behavior, validation messages, file paths, and workflow steps.
- Verifications keep evidence expectations and test assertions.

### Adding Requirements

Create requirements using the `--content` flag or by piping element content to the add command:

```bash
# Add capability using --content flag (preferred)
reqvire add requirements/Capabilities.md --content '### Capability Name

Capability scope and purpose. Describe what the system is able to accomplish, not how it is implemented.

#### Metadata
  * type: capability
'

# Add ontology attached by a capability
reqvire add requirements/Ontologies/CapabilityName.md <<'EOF'
### Capability Ontology

Defines shared domain meaning for the capability.

#### Metadata
  * type: ontology

#### Ontology

```turtle
@prefix ex: <urn:reqvire:example:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:DomainConcept a owl:Class .
```
EOF

# Add requirement using heredoc (stdin)
reqvire add requirements/System/Capabilities.md <<'EOF'
### System Capability Implementation

The system shall implement the capability using defined algorithms.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Capability Name](../Capabilities.md#capability-name)
EOF

# Override existing element (replace by name) - useful for cleanup after merge
reqvire add requirements/File.md --override <<'EOF'
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

**Relation types**: `derivedFrom` (child → parent inside same family), `derive` (parent → child inside same family), `specify` (requirement → capability), `specifiedBy` (capability → requirement), `verifiedBy` (capability or requirement → verification), `verify` (verification → capability or requirement), `satisfiedBy` (requirement/test-verification/formal-proof-verification → implementation or evidence), `satisfy` (implementation/evidence → requirement/test-verification/formal-proof-verification), `refinedBy` (owner → refinement), `refine` (refinement → owner), `trace` (non-directional traceability)

## Step 3: Add Refinements (if needed)

Add refinements only when:
- **Specifications** - Detailed definitions needed, referenced by multiple requirements
- **Constraints** - Limits/boundaries exist (add to Constraints.md)
- **Behaviors** - Complex state/flow logic needs documentation
- **State** - State machines, lifecycle states, and state-dependent contracts
- **Input-output** - Payloads, messages, schemas, examples, and fixtures
- **Ontology** - Shared semantic meaning and vocabulary attached by capabilities
- **Semantic contracts** - Capability-owned or requirement-owned SHACL shape profiles for one capability or obligation

Link refinements via `refinedBy` from the capability or requirement that owns the refinement. Attach ontology elements from capabilities; link `semantic-contract` refinements from the owning capability or requirement when they define shapes.

### Refinement Best Practices

- **Constraints** should always be in constraint element type
- Group constraints in single file (e.g., `Constraints.md` in requirements root)
- Define **Behaviors** and **Specifications** as elements only if other requirements depend on them
- Define **ontology** elements when requirements need shared domain meaning or ontology vocabulary
- Define **semantic contracts** as capability-owned or requirement-owned refinements when one capability or obligation needs a SHACL shape profile over reachable ontology context
- Otherwise define them under `#### Behaviors` or `#### Specifications` subsection of the requirement

### Adding Refinement Elements

Create refinement elements when they need to be referenced by multiple requirements:

```bash
# Add specification element
reqvire add requirements/Specifications.md <<'EOF'
### Data Format Specification

The data format shall follow JSON Schema version 7 with strict validation.

#### Metadata
  * type: specification
EOF

# Add constraint element
reqvire add requirements/Constraints.md <<'EOF'
### Performance Constraint

All API responses shall complete within 200ms under normal load.

#### Metadata
  * type: constraint
EOF

# Add behavior element
reqvire add requirements/Behaviors.md <<'EOF'
### Error Recovery Behavior

When an error occurs, the system shall log the error, notify the user, and attempt recovery.

#### Metadata
  * type: behavior
EOF
```

Link refinements to requirements using relations or attachments:

```bash
# Link refinement to requirement using refinedBy relation (owner defines it)
reqvire link "Data Processing Requirement" "refinedBy" "Data Format Specification"

# Attach requirement refinement element across explicit requirement subgraph boundaries
# The refinement must be owned by a requirement via refinedBy
reqvire link "Other Capability Requirement" attaching "Performance Constraint"

# Attach ontology element across capability subgraph boundaries
reqvire link "Other Capability" attaching "Capability Ontology"

# Attach requirement-owned semantic contract across requirement subgraph boundaries
reqvire link "Other Capability Requirement" attaching "Requirement Shape Contract"

# Attach file (design document, specification document)
reqvire link "Architecture Requirement" attaching "docs/architecture.pdf"

# Link to implementation file or external URL
# Note: capability must not use satisfiedBy/satisfy.
reqvire link "System Requirement" "satisfiedBy" "src/auth/login.rs"
reqvire link "Compliance Requirement" "trace" "https://example.com/spec.html"
```

**Attachment constraints:**
- Refinements must have a `refine` relation before being attached
- Capabilities may attach only `ontology` elements
- Requirements may attach only requirement-owned `semantic-contract`, `specification`, `constraint`, `behavior`, `state`, or `input-output` refinements
- Cross-capability semantic dependencies must be explicit attachments so change impact is preserved

## Step 4: Add Verifications

**Bottom Roll-Up Strategy:**
- Add verification elements for **leaf requirements only**
- Parent requirements inherit verification from children
- Avoid redundant verify relations

### Verification Types

Choose appropriate type:
- `test` - Automated or manual testing (can have `satisfiedBy` to test code)
- `analysis` - Review, calculation, simulation
- `inspection` - Visual examination, audit
- `demonstration` - Showing capability works

### Adding Verification

Create verification elements for leaf requirements:

```bash
# Add test verification
reqvire add requirements/Verifications/CapabilityTests.md <<'EOF'
### Capability Test

Test verifies the capability works correctly:
1. Input validation passes
2. Output matches expected format
3. Error handling works

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Capability Requirement](../System/Capabilities.md#capability-requirement)
  * satisfiedBy: [test_capability.rs](../../tests/test_capability.rs)
EOF

# Add analysis verification
reqvire add requirements/Verifications/PerformanceAnalysis.md <<'EOF'
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

Additionally, use `reqvire resources` to see all files referenced by the model through `satisfiedBy`, `trace` relations and attachments.

## Complete Example

### 1. Capability
```markdown
### User Authentication

Authentication capability for access-controlled product areas.

#### Metadata
  * type: capability

#### Attachments
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

#### Ontology

```turtle
@prefix auth: <urn:reqvire:auth:> .
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
  * specify: [User Authentication](../Capabilities.md#user-authentication)
  * refinedBy: [Password Authentication Semantic Contract](../Contracts/Auth.md#password-authentication-semantic-contract)
  * satisfiedBy: [auth.rs](../../src/auth.rs)
```

### 4. Semantic Contract
````markdown
### Password Authentication Semantic Contract

Defines the closed-world SHACL profile used by the password-authentication obligation.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Password Authentication](../System/Auth.md#password-authentication)

#### Shapes

```turtle
@prefix auth: <urn:reqvire:auth:> .
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
  * specify: [User Authentication](../Capabilities.md#user-authentication)
  * refinedBy: [Session Timeout Constraint](../Constraints.md#session-timeout)
```

### 5. Constraint
```markdown
### Session Timeout

Session timeout limit used by session-management behavior.

#### Metadata
  * type: constraint

#### Relations
  * refine: [Session Management](../System/Auth.md#session-management)
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
  * verify: [Password Authentication](../System/Auth.md#password-authentication)
  * verify: [Session Management](../System/Auth.md#session-management)
  * satisfiedBy: [test_auth.rs](../../tests/test_auth.rs)
```

## File Organization

Typical structure:
```
requirements/
├── Capabilities.md              # Capability roots
├── Constraints.md           # Shared requirement constraints
├── Ontologies/              # Canonical home for ontology elements
├── System/
│   ├── CapabilityA.md          # System requirements for capability A
│   └── CapabilityB.md
├── Verifications/
│   ├── CapabilityATests.md     # Verifications for capability A
│   └── CapabilityBTests.md
└── DesignDocuments/         # Design docs (not parsed as elements)
    └── Architecture.md
```

### Reorganizing Elements

Move elements between files or reposition within files:

```bash
# Move element to different file
reqvire mv "Capability Requirement" "requirements/System/NewFile.md"

# Move element to specific position (0-based index)
reqvire mv "Capability Requirement" "requirements/System/Capabilities.md" 0  # Move to top

# Move entire file to new location
reqvire mv-file "requirements/Old.md" "requirements/System/New.md"

# Merge file into existing file (squash - combine contents)
reqvire mv-file --squash "requirements/Source.md" "requirements/Target.md"
```

**When to reorganize:**
- Grouping related requirements into capability-specific files
- Splitting large files into manageable sections
- Consolidating scattered constraints into Constraints.md
- Moving verifications to match requirement structure
