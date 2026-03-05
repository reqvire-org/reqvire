# Adding New Features & Requirements

**Key principle**: Requirements drive everything - no implementation without requirements. Follow the MBSE workflow to ensure properly specified functionality with full traceability.

**For common commands** (link, add, validate, etc.), see [SKILL.md Command Reference](../SKILL.md#command-reference) and [Quick Start Guide](../SKILL.md#quick-start-common-workflows).

## MBSE Workflow

```
1. Requirements first → Define what the system shall do (never skip)
2. Refinements       → Add specifications, constraints, behaviors as needed
3. Verifications     → Add verification elements for leaf requirements
4. Implementation    → Connect `requirement` elements (and test verifications) to code via `satisfiedBy` (when code exists)
```

## Step 1: Understand the Feature Scope

Before creating requirements, answer:
- What stakeholder need does this address? (user-requirement)
- What technical capabilities are needed? (`requirement`)
- Are there constraints or limits to define?
- How will this be verified?

## Step 2: Create Requirements Hierarchy

```
User Requirement (stakeholder need)
    ↓ derivedFrom
System Requirement(s) (technical implementation)
```

**Guidelines:**
- Start with user-requirement for the stakeholder need
- Derive `requirement` elements for technical details
- Keep requirements atomic and testable
- Use EARS patterns for clear statements

### Adding Requirements

Create requirements using the `--content` flag or by piping element content to the add command:

```bash
# Add user requirement using --content flag (preferred)
reqvire add requirements/UserStories.md --content '### Feature Name

The system shall provide feature capability.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Parent Requirement](path.md#parent)
'

# Add user requirement to file via stdin
echo '### Feature Name

The system shall provide feature capability.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Parent Requirement](path.md#parent)
' | reqvire add requirements/UserStories.md

# Add requirement (system-level) using heredoc (stdin)
reqvire add requirements/System/Features.md <<'EOF'
### System Feature Implementation

The system shall implement the feature using defined algorithms.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature Name](../UserStories.md#feature-name)
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
# Link requirement to user requirement
reqvire link "System Feature Implementation" "derivedFrom" "Feature Name"

# Link parent to child (opposite direction of derivedFrom)
reqvire link "Feature Name" "derive" "System Feature Implementation"
```

**Relation types**: `derivedFrom` (child → parent), `derive` (parent → child), `verifiedBy` (requirement → verification), `verify` (verification → requirement), `satisfiedBy` (requirement/test-verification → implementation), `satisfy` (implementation → requirement/test-verification), `refinedBy` (requirement → refinement), `refine` (refinement → requirement), `trace` (non-directional traceability)

## Step 3: Add Refinements (if needed)

Add refinements only when:
- **Specifications** - Detailed definitions needed, referenced by multiple requirements
- **Constraints** - Limits/boundaries exist (add to Constraints.md)
- **Behaviors** - Complex state/flow logic needs documentation

Link via `refinedBy` from the requirement that owns the refinement.

### Refinement Best Practices

- **Constraints** should always be in constraint element type
- Group constraints in single file (e.g., `Constraints.md` in requirements root)
- Define **Behaviors** and **Specifications** as elements only if other requirements depend on them
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

# Attach refinement element (only from requirements OUTSIDE the owner's hierarchy)
# The refinement must be owned by a requirement via refinedBy
reqvire link "Other Feature Requirement" attaching "Performance Constraint"

# Attach file (design document, specification document)
reqvire link "Architecture Requirement" attaching "docs/architecture.pdf"

# Link to implementation file or external URL
# Note: user-requirement must not use satisfiedBy/satisfy.
reqvire link "System Requirement" "satisfiedBy" "src/auth/login.rs"
reqvire link "Compliance Requirement" "trace" "https://example.com/spec.html"
```

**Attachment constraints:**
- Refinements must have a `refine` relation (established via requirement's `refinedBy`) before being attached
- Only requirements OUTSIDE the owner's derivation hierarchy can attach a refinement
- Requirements in the same hierarchy access refinements through the hierarchy, not attachments

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

### 1. User Requirement
```markdown
### User Authentication

The system shall authenticate users before granting access to protected resources.

#### Metadata
  * type: user-requirement
```

### 2. System Requirements (derived)
```markdown
### Password Authentication

The system shall validate user credentials against stored password hashes.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Authentication](../UserStories.md#user-authentication)
  * satisfiedBy: [auth.rs](../../src/auth.rs)
```

```markdown
### Session Management

The system shall create and manage user sessions after successful authentication.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Authentication](../UserStories.md#user-authentication)
  * refinedBy: [Session Timeout Constraint](../Constraints.md#session-timeout)
```

### 3. Constraint
```markdown
### Session Timeout

User sessions shall expire after 30 minutes of inactivity.

#### Metadata
  * type: constraint
```

### 4. Verification
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
├── UserStories.md           # User requirements
├── Constraints.md           # All constraints
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
