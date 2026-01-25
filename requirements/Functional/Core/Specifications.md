# Elements

### Element Type Metadata Specification

Specification for declaring element types in markdown documents through the Metadata subsection.

#### Details
Element types are identified through a reserved `type` metadata property in the Metadata subsection.

**Declaration Format:**
`  * type: <element-type>` within the Metadata subsection.

**Default Type Assignment:**
- When no `type` property is specified, elements default to type `requirement`
- This behavior is location-independent (applies regardless of file location)

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Reserved Subsections Support](StructureAndParsing.md#reserved-subsections-support)
---

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

### Requirements Processing Specification

Specification for how requirements files are discovered and processed.

#### Details
**File Discovery:**
- Parse all .md files from git repository root
- Apply .gitignore and .reqvireignore exclusions
- Reserved files (README.md, LICENSE.md) are excluded

**Processing Pipeline:**
- Pass 1: Element collection and local validation
- Pass 2: Graph construction and relation validation
- GraphRegistry built from ElementRegistry after Pass 1

#### Metadata
  * type: specification

#### Relations
  * satisfy: [GraphRegistry as Primary Registry](Validation.md#graphregistry-as-primary-registry)
---

### Reserved Files Specification

Reserved repository documentation filenames automatically excluded from structured markdown processing.

#### Details
The following filenames are reserved for general repository documentation and are automatically excluded from requirements parsing:

**Reserved Filenames:**
- `README.md` - Project overview and documentation
- `CHANGELOG.md`, `CHANGES.md` - Version history and release notes
- `CONTRIBUTING.md` - Contribution guidelines
- `LICENSE.md` - License information
- `CODE_OF_CONDUCT.md` - Community conduct standards
- `SECURITY.md` - Security policies and vulnerability reporting
- `AUTHORS.md` - Project contributors and credits
- `ROADMAP.md` - Project roadmap and future plans

**Exclusion Rules:**
- Reserved filenames are excluded from structured markdown parsing across the entire repository
- Reserved files can be referenced in file relations to elements (excluded from parsing but linkable)
- Exclusion is combined with .gitignore and .reqvireignore patterns
- Files matching reserved filenames are excluded regardless of ignore file configuration

**Scope:**
- Applies to exact filename matches (case-sensitive on case-sensitive filesystems)
- Applies at all directory levels in the repository
- Takes precedence before ignore pattern evaluation

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Reserved Repository Files Exclusion](Configuration.md#reserved-repository-files-exclusion)
---

### Verification Type Selection Guidelines

Usage guidelines for selecting appropriate verification types.

#### Details
**Default Verification Type:**
- `verification` - Verification through testing (equivalent to `test-verification`)

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
