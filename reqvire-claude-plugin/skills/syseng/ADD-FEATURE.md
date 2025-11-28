# Adding New Features & Requirements

Use this reference when extending the system with new capabilities. Follow the MBSE workflow to ensure properly specified functionality with full traceability.

**Key principle**: Requirements drive everything - no implementation without requirements.

## MBSE Workflow

```
1. Requirements first → Define what the system shall do (never skip)
2. Refinements       → Add specifications, constraints, behaviors as needed
3. Verifications     → Add verification elements for leaf requirements
4. Implementation    → Connect to code via satisfiedBy (when code exists)
```

## Step 1: Understand the Feature Scope

Before creating requirements, answer:
- What stakeholder need does this address? (user-requirement)
- What technical capabilities are needed? (system-requirement)
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
- Derive system-requirements for technical details
- Keep requirements atomic and testable
- Use EARS patterns for clear statements

### Adding Requirements

```bash
# Add element to file (reads from stdin)
echo '### Feature Name

The system shall provide feature capability.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Parent Requirement](path.md#parent)
' | reqvire add requirements/Path/File.md
```

## Step 3: Add Refinements (if needed)

Add refinements only when:
- **Specifications** - Detailed definitions needed, referenced by multiple requirements
- **Constraints** - Limits/boundaries exist (add to Constraints.md)
- **Behaviors** - Complex state/flow logic needs documentation

Link via `satisfiedBy` from the requirement that asks for the refinement.

### Refinement Best Practices

- **Constraints** should always be in constraint element type
- Group constraints in single file (e.g., `Constraints.md` in requirements root)
- Define **Behaviors** and **Specifications** as elements only if other requirements depend on them
- Otherwise define them under `#### Behaviors` or `#### Specifications` subsection of the requirement

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

```bash
echo '### Feature Test

Test verifies the feature works correctly.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Feature Requirement](../Requirements.md#feature-requirement)
' | reqvire add requirements/Verifications/FeatureTests.md
```

## Step 5: Validate and Check Coverage

```bash
reqvire validate           # Check model consistency
reqvire coverage           # Check verification coverage
reqvire resources          # List files referenced by model
reqvire lint --fix         # Fix any issues
```

Use `reqvire resources` to see all files referenced by the model through `satisfiedBy`, `trace` relations and attachments. This helps identify:
- Which implementation files are linked to requirements
- Which design documents are traced
- Orphaned files that should be linked

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
  * type: system-requirement

#### Relations
  * derivedFrom: [User Authentication](../UserStories.md#user-authentication)
  * satisfiedBy: [auth.rs](../../src/auth.rs)
```

```markdown
### Session Management

The system shall create and manage user sessions after successful authentication.

#### Metadata
  * type: system-requirement

#### Relations
  * derivedFrom: [User Authentication](../UserStories.md#user-authentication)

#### Attachments
  * [Session Timeout Constraint](../Constraints.md#session-timeout)
```

### 3. Constraint
```markdown
### Session Timeout

User sessions shall expire after 30 minutes of inactivity.

#### Metadata
  * type: constraint

#### Relations
  * satisfy: [User Authentication](../UserStories.md#user-authentication)
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
