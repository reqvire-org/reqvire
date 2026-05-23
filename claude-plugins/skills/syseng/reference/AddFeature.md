# Adding New Features & Requirements

**Key principle**: Requirements drive everything - no implementation without requirements. Follow the MBSE workflow to ensure properly specified functionality with full traceability.

**For common commands** (link, add, validate, etc.), see [SKILL.md Command Reference](../SKILL.md#command-reference) and [Quick Start Guide](../SKILL.md#quick-start-common-workflows).

## MBSE Workflow

```
1. Feature first       → Define the product/capability scope and owner
2. Semantic context    → Add source and ontology elements when domain meaning or external contracts matter
3. Requirements        → Define what the system shall do (never skip implementable obligations)
4. Requirement details → Add specifications, constraints, behaviors, state, and input-output refinements as needed
5. Verifications       → Add verification elements for leaf requirements
6. Implementation      → Connect `requirement` elements and evidence-backed verifications to code/evidence via `satisfiedBy`
```

## Construction Method

When constructing or refactoring a system model, work from model boundaries inward:

1. Run `reqvire submodels --json` and identify the existing feature-root subgraphs.
2. Run `reqvire search --filter-type="ontology" --short` and identify the ontology terms already available.
3. Decide whether the new content belongs under an existing feature root, a child feature, a new independent feature root, or the shared ontology hierarchy.
4. Add subfeatures only for meaningful capability slices; do not use feature hierarchy just to share ontology.
5. Put shared vocabulary and stable semantic relationships in ontology; attach that ontology from the owning or consuming feature so requirements inherit it through feature context.
6. Put implementable obligations in requirements that `specify` the local feature.
7. Put local details in requirement-owned refinements.
8. Use attachments, not hierarchy, when another feature root needs ontology or reusable requirement-owned contracts from this one.
9. Validate `submodels`, `collect`, and change-impact paths after each boundary slice.

## Step 1: Understand the Feature Scope

Before creating requirements, answer:
- What product capability, stakeholder need, regulation, or external obligation does this address? (`feature`)
- What semantic meaning, domain vocabulary, data shape, or policy contract must be shared? (`ontology` for vocabulary, `semantic-contract` for requirement-owned SHACL profiles)
- What technical capabilities are needed? (`requirement`)
- Are there constraints or limits to define?
- How will this be verified?

## Step 2: Create Feature and Requirement Hierarchy

```
Feature (product/capability scope)
    ├── refinedBy → source
    ├── attach → ontology
    ├── derive → Subfeature
    └── specifiedBy ← Requirement
                     └── derive → Child Requirement
```

**Guidelines:**
- Start with `feature` for the product/capability or external need
- Check existing feature roots before adding a new one; preserve independent submodels unless the new work truly belongs in the same capability root
- Use subfeatures for real product/interface/domain slices that should own local requirements
- Use `source` refinements for stakeholder, regulatory, contractual, or external context
- Use `ontology` elements for ontology/vocabulary contracts
- Use requirement-owned `semantic-contract` refinements for SHACL shape profiles over reachable ontology context
- Use `specify` / `specifiedBy` to connect requirements to features
- Derive `requirement` elements only from other requirements
- Keep requirements atomic and testable
- Use EARS patterns for clear statements

### Feature vs Requirement vs Semantic Contract

A `feature` answers:
- What capability, product area, stakeholder need, regulatory area, external obligation, or domain slice is this?
- Why does this area exist in the product model?
- What stakeholder, regulatory, source, or policy context owns it?
- What ontology defines its domain language?
- Which requirements belong under this capability?

A feature is not a weaker requirement. It is the capability anchor. Use a `feature` when the statement is about product scope, stakeholder value, regulatory domain, ownership, planning, or capability grouping. Features are not directly verified or satisfied; their coverage rolls up from the requirements that specify them.

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

Use a requirement-owned `semantic-contract` when a specific obligation needs a SHACL shape profile over reachable ontology concepts. Requirement-owned semantic contracts must have `#### Shapes` and must not have `#### Ontology`.

Do not turn every obligation into an ontology. If the statement says what the system must do, keep it as a requirement and point it to the semantic contract when a closed-world profile is needed. If the statement is `X is a Y`, `X has property Z`, `X relates to Y`, `these fields form a valid object`, or `this domain term means...`, keep it in ontology when it is stable shared domain meaning:

```text
Requirement:
  The system shall reject API requests whose access token does not conform to the Access Token semantic contract.

Ontology:
  auth:AccessToken, auth:subject, auth:expiresAt.

Requirement semantic contract:
  SHACL shape requiring the fields/properties needed by this specific obligation.
```

Good split:
- Feature: `API Authentication`
- Ontology: `Access Token Ontology`
- Requirement-owned semantic contract: `Access Token Validation Contract`
- Requirement: `The system shall reject API requests whose access token does not conform to the Access Token validation contract.`
- Input-output refinement, when needed: local request/response examples or API-specific representation details

Keep domain definitions and reusable vocabulary in ontology when they are stable and shared. Keep obligation-specific closed-world constraints in requirement-owned semantic contracts. Keep workflow behavior, implementation obligations, acceptance commitments, and verification scope in requirements.

When splitting existing prose, do not lose meaning:
- Feature prose keeps capability scope and why the area exists.
- Ontology keeps reusable terms and relationships.
- Requirement refinements keep exact command behavior, payload fields, outputs, state behavior, validation messages, file paths, and workflow steps.
- Verifications keep evidence expectations and test assertions.

### Adding Requirements

Create requirements using the `--content` flag or by piping element content to the add command:

```bash
# Add feature using --content flag (preferred)
reqvire add requirements/Features.md --content '### Feature Name

Feature capability scope and purpose.

#### Metadata
  * type: feature
'

# Add ontology attached by a feature
reqvire add requirements/Ontologies/FeatureName.md <<'EOF'
### Feature Ontology

Defines shared domain meaning for the feature.

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
reqvire add requirements/System/Features.md <<'EOF'
### System Feature Implementation

The system shall implement the feature using defined algorithms.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Feature Name](../Features.md#feature-name)
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
# Link requirement to feature
reqvire link "System Feature Implementation" "specify" "Feature Name"

# Link feature to requirement (opposite direction of specify)
reqvire link "Feature Name" "specifiedBy" "System Feature Implementation"

# Link child requirement to parent requirement
reqvire link "Child Requirement" "derivedFrom" "System Feature Implementation"
```

**Relation types**: `derivedFrom` (child → parent inside same family), `derive` (parent → child inside same family), `specify` (requirement → feature), `specifiedBy` (feature → requirement), `verifiedBy` (requirement → verification), `verify` (verification → requirement), `satisfiedBy` (requirement/test-verification/formal-proof-verification → implementation or evidence), `satisfy` (implementation/evidence → requirement/test-verification/formal-proof-verification), `refinedBy` (owner → refinement), `refine` (refinement → owner), `trace` (non-directional traceability)

## Step 3: Add Refinements (if needed)

Add refinements only when:
- **Specifications** - Detailed definitions needed, referenced by multiple requirements
- **Constraints** - Limits/boundaries exist (add to Constraints.md)
- **Behaviors** - Complex state/flow logic needs documentation
- **State** - State machines, lifecycle states, and state-dependent contracts
- **Input-output** - Payloads, messages, schemas, examples, and fixtures
- **Ontology** - Shared semantic meaning and vocabulary attached by features
- **Semantic contracts** - Requirement-owned SHACL shape profiles for one obligation

Link requirement-detail refinements via `refinedBy` from the requirement that owns the refinement. Link `source` refinements via `refinedBy` from the feature that owns them. Attach ontology elements from features; link `semantic-contract` refinements from the owning requirement when they define shapes.

### Refinement Best Practices

- **Constraints** should always be in constraint element type
- Group constraints in single file (e.g., `Constraints.md` in requirements root)
- Define **Behaviors** and **Specifications** as elements only if other requirements depend on them
- Define **ontology** elements when requirements need shared domain meaning or ontology vocabulary
- Define **semantic contracts** as requirement-owned refinements when one obligation needs a SHACL shape profile over reachable ontology context
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
reqvire link "Other Feature Requirement" attaching "Performance Constraint"

# Attach ontology element across feature subgraph boundaries
reqvire link "Other Feature" attaching "Feature Ontology"

# Attach requirement-owned semantic contract across requirement subgraph boundaries
reqvire link "Other Feature Requirement" attaching "Requirement Shape Contract"

# Attach file (design document, specification document)
reqvire link "Architecture Requirement" attaching "docs/architecture.pdf"

# Link to implementation file or external URL
# Note: feature must not use satisfiedBy/satisfy.
reqvire link "System Requirement" "satisfiedBy" "src/auth/login.rs"
reqvire link "Compliance Requirement" "trace" "https://example.com/spec.html"
```

**Attachment constraints:**
- Refinements must have a `refine` relation before being attached
- Features may attach only `ontology` elements
- Requirements may attach only requirement-owned `semantic-contract`, `specification`, `constraint`, `behavior`, `state`, or `input-output` refinements
- Cross-feature semantic dependencies must be explicit attachments so change impact is preserved

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
reqvire add requirements/Verifications/FeatureTests.md <<'EOF'
### Feature Test

Test verifies the feature works correctly:
1. Input validation passes
2. Output matches expected format
3. Error handling works

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Feature Requirement](../System/Features.md#feature-requirement)
  * satisfiedBy: [test_feature.rs](../../tests/test_feature.rs)
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
reqvire link "Feature Test" "verify" "Feature Requirement"

# Link from requirement to verification (verifiedBy relation, opposite direction)
reqvire link "Feature Requirement" "verifiedBy" "Feature Test"

# Link test verification to test implementation
reqvire link "Feature Test" "satisfiedBy" "tests/test_feature.rs"
```

## Step 5: Validate and Check Coverage

After adding requirements and verifications, follow the standard validation workflow. See [SKILL.md Validation & Quality Checklist](../SKILL.md#validation--quality-checklist) for the complete procedure:

1. `reqvire validate` - Check model structure
2. `reqvire lint --fix` - Fix auto-fixable issues
3. `reqvire coverage` - Verify leaf verification coverage and requirement-only implementation coverage
4. `reqvire format --fix` - Normalize formatting

Additionally, use `reqvire resources` to see all files referenced by the model through `satisfiedBy`, `trace` relations and attachments.

## Complete Example

### 1. Feature
```markdown
### User Authentication

Authentication capability for access-controlled product areas.

#### Metadata
  * type: feature

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
  * specify: [User Authentication](../Features.md#user-authentication)
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
  * specify: [User Authentication](../Features.md#user-authentication)
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
├── Features.md              # Feature roots
├── Constraints.md           # Shared requirement constraints
├── Ontologies/              # Canonical home for ontology elements
├── System/
│   ├── FeatureA.md          # System requirements for feature A
│   └── FeatureB.md
├── Verifications/
│   ├── FeatureATests.md     # Verifications for feature A
│   └── FeatureBTests.md
└── DesignDocuments/         # Design docs (not parsed as elements)
    └── Architecture.md
```

### Reorganizing Elements

Move elements between files or reposition within files:

```bash
# Move element to different file
reqvire mv "Feature Requirement" "requirements/System/NewFile.md"

# Move element to specific position (0-based index)
reqvire mv "Feature Requirement" "requirements/System/Features.md" 0  # Move to top

# Move entire file to new location
reqvire mv-file "requirements/Old.md" "requirements/System/New.md"

# Merge file into existing file (squash - combine contents)
reqvire mv-file --squash "requirements/Source.md" "requirements/Target.md"
```

**When to reorganize:**
- Grouping related requirements into feature-specific files
- Splitting large files into manageable sections
- Consolidating scattered constraints into Constraints.md
- Moving verifications to match requirement structure
