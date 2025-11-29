# Elements

### Git Repository Scope Specification

Path resolution and scope validation rules for Git repository-based project management.

#### Details
**Git Root Detection:**
- Git root is detected via `git rev-parse --show-toplevel`
- All internal paths are normalized to git-root-relative format for storage

**Path Resolution Rules:**
- All paths are resolved relative to the current working directory
- When run from the git repository root: paths are relative to the git root
- When run from a subdirectory: paths are relative to that subdirectory

**Processing Scope:**
- When run from git root: all files in the repository are processed
- When run from a subdirectory: processing is limited to files within that subdirectory scope

**Scope Boundary Validation:**
- Relations referencing elements outside the subdirectory scope report missing relation target errors
- References using relative paths (e.g., `../ParentFile.md#element`) that escape the subdirectory result in missing relation target errors
- Absolute paths pointing outside the subdirectory scope generate missing relation target errors
- Missing relation target errors clearly identify the unreachable reference due to subdirectory scope limitations

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Git Repository as Project Root](ModelManagement.md#git-repository-as-project-root)
---

### Ignore Files Specification

Rules for processing .gitignore and .reqvireignore exclusion patterns.

#### Details
**Pattern Sources:**
- `.gitignore` - Version control exclusions (files not tracked by Git)
- `.reqvireignore` - Reqvire-specific exclusions (files tracked by Git but excluded from requirements processing)

**Processing Rules:**
- ONLY the root .gitignore file shall be used (not nested .gitignore files in subdirectories)
- ONLY the root .reqvireignore file shall be used (not nested .reqvireignore files in subdirectories)
- .reqvireignore shall use the same format and syntax as .gitignore
- Patterns from .gitignore and .reqvireignore shall be combined
- Files matching ANY exclusion pattern shall be excluded from parsing as requirements

**Exclusion Behavior Differences:**
- Files excluded by `.gitignore`: completely excluded - cannot be parsed as structured markdown AND cannot be referenced in file relations
- Files excluded by `.reqvireignore`: excluded from parsing BUT can still be referenced in file relations (useful for design documents, diagrams)

**Fallback Behavior:**
- If .reqvireignore does not exist, process normally using only .gitignore patterns
- If .gitignore does not exist, process normally using only .reqvireignore patterns

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Ignore Files Integration](Configuration.md#ignore-files-integration)
---

### Supported Element Types Specification

Element types supported by the system for classification and behavior determination.

#### Details
Element types are identified through a reserved `type` metadata property in the `#### Metadata` subsection.

**Requirement Types:**

| Type | Description |
|------|-------------|
| `requirement` | System requirement (default type if not specified) |
| `user-requirement` | User requirement representing stakeholder needs |

**Verification Types:**

| Type | Description |
|------|-------------|
| `verification` | Verification through testing (equivalent to `test-verification`) |
| `test-verification` | Explicit verification through testing with documented test procedures |
| `analysis-verification` | Verification through formal analysis of documentation or code |
| `inspection-verification` | Verification through formal inspection or review |
| `demonstration-verification` | Verification through demonstration in a realistic environment |

**Refinement Types:**

| Type | Description | Constraints |
|------|-------------|-------------|
| `constraint` | Documents constraints that limit or bound the system | Only `satisfy` relations allowed |
| `behavior` | Documents behavior details and operational specifications | Only `satisfy` relations allowed |
| `specification` | Documents detailed specifications and technical descriptions | Only `satisfy` relations allowed |

**Other Types:**

| Type | Description |
|------|-------------|
| `other` | Custom element types defined by users |

**Usage Example:**
To specify an element type, add a Metadata subsection with the type property:
`* type: user-requirement`

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Supported Element Types](ModelManagement.md#supported-element-types)
---

### Verification Type Categories Specification

Supported verification types and their usage guidelines.

#### Details
**Default Verification Type:**
- `verification` - Verification through testing (equivalent to `test-verification`)

**Specific Verification Types:**

| Type | Description | Use When |
|------|-------------|----------|
| `test-verification` | Explicit verification through testing with documented test procedures | Formal test procedures with expected outcomes are required |
| `analysis-verification` | Verification through formal analysis of documentation or code | Requirements can be verified through analysis of documentation or code |
| `inspection-verification` | Verification through formal inspection or review | Requirements can be verified through review of artifacts |
| `demonstration-verification` | Verification through demonstration in a realistic environment | Requirements can be verified by demonstrating functionality |

**Type Selection Guidelines:**
- **Test-verification**: Quantitative requirements, functional behavior, performance criteria
- **Analysis-verification**: Design constraints, architectural requirements, compliance with standards
- **Inspection-verification**: Documentation requirements, labeling, configuration settings
- **Demonstration-verification**: User-facing features, workflow requirements, integration scenarios

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Verification Type Categories](ModelManagement.md#verification-type-categories)
---
