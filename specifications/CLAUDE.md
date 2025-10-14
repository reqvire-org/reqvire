# Specifications Writing Guide for Reqvire

## Overview
This guide helps you write and maintain specifications in Reqvire's requirements management system format. All specifications are written in Markdown with specific conventions for elements and relationships.

## Key Specification Files
- **SpecificationsRequirements.md**: Defines the structure and rules for all specifications
- **UserRequirements.md**: High-level user needs and requirements
- **UserStories.md**: User stories and scenarios describing system usage
- **Usecases.md**: Detailed use cases and user interactions
- **MissionRequirements.md**: Mission-level objectives and goals
- **SystemRequirements/**/*.md**: Detailed system requirements
- **Verifications/**/*.md: Folder containing verification specifications for requirements
- **LogicalArchitecture.md**: System's logical architecture and components
- **PhysicalArchitecture.md**: System's physical architecture and deployment

## Element Structure

### Basic Element Format
```markdown
### Element Name

Element content describing the requirement, specification, or verification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: parent-element-id
  * verifiedBy: verification-element-id
```

### Sections (##) vs Elements (###)
- **Sections** (`##`): Used for grouping similar requirements for easier management
- **Elements** (`###`): Uniquely identifiable system elements that can have relations

Sections may have a text which gives some additional context on what is the section about and which requriements to expect in the section.

## Element Types

### Requirements
```markdown
### User Authentication Requirement

The system SHALL provide user authentication using OAuth 2.0.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [High-Level Security Requirement](#high-level-security-requirement)
  * verifiedBy: [OAuth Authentication Test](Verifications/AuthTests.md#oauth-authentication-test)
  * satisfiedBy: ../core/src/auth.rs
```

### Verifications
```markdown
### OAuth Authentication Test

Test that verifies OAuth 2.0 authentication flow works correctly.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [User Authentication Requirement](../SystemRequirements/Requirements.md#user-authentication-requirement)
```

## Supported Element Types via Metadata

- `requirement`: System requirements (default for all elements without explicit type metadata)
- `user-requirement`: User-level requirements (must be explicitly specified in metadata)
- `verification`: General verification (defaults to test-verification)
- `test-verification`: Verification through testing
- `analysis-verification`: Verification through analysis
- `inspection-verification`: Verification through inspection
- `demonstration-verification`: Verification through demonstration

**Note**: Element type is determined by the `type` property in the `#### Metadata` subsection. If no type is specified, all elements default to `requirement` type regardless of file location. This behavior is location-independent.

## Relation Types

### Hierarchical Relations
- `derivedFrom`/`derive`: Derivation from higher-level elements

**Note**: Containment is managed through file structure, sections, and folders - not through explicit relations.

### Satisfaction Relations
- `satisfiedBy`/`satisfy`: Links requirements to implementations or architectural diagrams and other non-requirement system elements

### Verification Relations
- `verifiedBy`/`verify`: Links requirements to verification artifacts

### Traceability Relations
- `trace`: Simple traceability without change propagation

## Identifier Formats

### Internal References
```markdown
#### Relations
  * derivedFrom: [Element Name](#element-name)
  * derivedFrom: [Other Element](../OtherFile.md#element-id)
```

### Implementation References
```markdown
#### Relations
  * satisfiedBy: [Implementation](../core/src/module.rs)
```

## Subsection Structure

### Reserved Subsections (must follow exact format)
- `#### Metadata`: Element metadata and type
- `#### Relations`: Element relationships
- `#### Details`: Extended requirement details (can use `<details>` tags)
- `#### Properties`: Additional properties

### Format Rules
- Subsections use level 4 headers (`####`)
- Relation entries use two-space indentation: `  * relationType: target`
- Metadata entries use two-space indentation: `  * property: value`

## Writing Guidelines

### 1. Use EARS Requirements Standard
Follow the EARS (Easy Approach to Requirements Syntax) standard for writing clear, unambiguous requirements:

**EARS Keywords (based on RFC 2119):**
- **SHALL/MUST**: Mandatory requirements
- **SHOULD**: Recommended but not mandatory
- **MAY**: Optional features

**EARS Requirement Patterns:**
- **Ubiquitous**: "The system SHALL [capability]"
- **Event-driven**: "WHEN [trigger] the system SHALL [response]"
- **State-driven**: "WHILE [state] the system SHALL [capability]"
- **Unwanted behavior**: "IF [condition] THEN the system SHALL [response]"
- **Optional**: "WHERE [feature is included] the system SHALL [capability]"

### 2. Write Atomic Requirements
Each requirement should:
- Address a single capability or constraint
- Be testable and verifiable
- Have clear acceptance criteria
- Follow EARS patterns for consistency

### 3. Maintain Traceability
- Every requirement SHOULD have at least one verification
- Link child requirements to parent requirements
- Connect requirements to implementing code

### 4. Element Naming Conventions
- Use descriptive names that indicate the element's purpose
- Element names become URL fragments (spaces → hyphens, lowercase)
- Keep names unique within each file

## Validation Commands

Check for unverified requirements:
```bash
cargo run -- model-summary --filter-is-not-verified
```

Check for unsatisfied requirements:
```bash
cargo run -- model-summary --filter-is-not-satisfied
```

Preview formatting changes:
```bash
cargo run -- format
```

Apply formatting:
```bash
cargo run -- format --fix
```

Generate traceability matrix:
```bash
cargo run -- traces --json > /tmp/traces.json
```

## Common Patterns

### Hierarchical Requirements
```markdown
### High-Level Feature

System SHALL provide data export functionality.

#### Metadata
  * type: requirement

### CSV Export

System SHALL export data in CSV format.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #high-level-feature
  * verifiedBy: [CSV Export Test](../Verifications/ExportTests.md#csv-export-test)
```

### Verification Strategy: Verify Leaf Requirements

**Preferable Approach**: Focus verification efforts on leaf requirements (lowest-level requirements with no derived children).

#### Why Verify Leaf Requirements:
- **Traces roll up**: Verification traces automatically propagate up the requirement hierarchy
- **Efficient coverage**: One verification can verify multiple leaf requirements and cover the entire chain of parent requirements up to the root
- **Avoid redundancy**: No need to create separate verifications for every level of the hierarchy
- **Clear test scope**: Leaf requirements are concrete and testable, while parent requirements are often abstract

#### Example:
```markdown
### Authentication (Root)
High-level requirement about authentication.

### Password Authentication (Parent)
System SHALL support password authentication.

#### Relations
  * derivedFrom: #authentication

### Password Strength Validation (Leaf)
System SHALL enforce minimum 8 character passwords.

#### Relations
  * derivedFrom: #password-authentication
  * verifiedBy: [Password Strength Test](Verifications/Tests.md#password-strength-test)

### Login Rate Limiting (Leaf)
System SHALL limit failed login attempts to 5 per minute.

#### Relations
  * derivedFrom: #password-authentication
  * verifiedBy: [Rate Limit Test](Verifications/Tests.md#rate-limit-test)
```

In this example:
- Only the two leaf requirements have `verifiedBy` relations
- Both verifications roll up through "Password Authentication" to "Authentication"
- The entire authentication hierarchy is verified through leaf requirement tests
```

### Verification with Multiple Requirements
```markdown
### Integration Test Suite

Comprehensive test suite for authentication flow.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Login Requirement](../Requirements.md#login-requirement)
  * verify: [Session Management](../Requirements.md#session-management)
  * verify: [Logout Requirement](../Requirements.md#logout-requirement)
```

### Using Details for Extended Information
```markdown
### Performance Requirement

System SHALL respond within 100ms for API calls.

#### Details

<details>
<summary>Performance Criteria</summary>

- 95th percentile response time: < 100ms
- 99th percentile response time: < 500ms
- Maximum response time: < 1000ms

Measured under standard load conditions with:
- 100 concurrent users
- 1000 requests per minute

</details>

#### Relations
  * verifiedBy: [Performance Test Suite](../Verifications/Performance.md#performance-test-suite)
```

## Best Practices

1. **Keep requirements focused**: One requirement per element
2. **Use consistent terminology**: Define terms in a glossary if needed
3. **Include rationale**: Explain why a requirement exists in the Details section
4. **Regular validation**: Run validation commands before committing
5. **Version control**: Track requirement changes through Git
6. **Review coverage**: Ensure all requirements have verifications
7. **Link to code**: Use satisfiedBy relations to connect to implementations
