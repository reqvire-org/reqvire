# Element

## Metadata
  * type: specification

## Relations
  * define: [Identifiers and Relations](StructureAndParsing.md#identifiers-and-relations)

## IdentifiersAndRelations

# Identifiers and Relations Specification

## Identity and Addressing

An element ID is derived from its globally unique name. A canonical identifier is its workspace-relative source path plus normalized fragment, such as `system-model/Requirements.md#example-requirement`. Moving the element changes its identifier while preserving its name-derived ID.

Targets resolve according to their role and current model membership:
- A model-element identifier selects a registered element. A file-only target may select the sole element of a single-element document.
- An InternalPath selects an eligible workspace-local artifact, such as an implementation or evidence file.
- An ExternalUrl identifies an external resource; Reqvire does not fetch it merely to validate a relation.

Target syntax alone does not determine compatibility. Satisfaction uses implementation or evidence artifacts; ownership and verification select compatible model elements. Contract dependencies select uniquely requirement-owned contracts, and Concept References select native concepts.

## Path and Fragment Normalization

- A leading `/` means relative to the effective Reqvire workspace root, not the filesystem root.
- Other paths are relative to the containing source document; a fragment-only target selects an element in that document.
- Canonical stored paths are workspace-root-relative. Source serialization writes appropriate source-relative destinations.
- Name fragments are lowercased, spaces become hyphens, disallowed anchor characters are removed, and surrounding whitespace is trimmed. Retain exactly one `#` delimiter.
- Existing-element selectors use exact names or canonical identifiers through the shared resolver; literal new names and artifact paths keep their operation-specific domains.

For a source at `project/documents/File1.md` in workspace `project`:

| Authored target | Canonical workspace target |
|---|---|
| `File2.md#Example Requirement` | `documents/File2.md#example-requirement` |
| `subfolder/File3.md#My Element (Draft)` | `documents/subfolder/File3.md#my-element-draft` |
| `../File4.md#Example Requirement` | `File4.md#example-requirement` |
| `/File4.md#Example Requirement` | `File4.md#example-requirement` |
| `#Example Requirement` | `documents/File1.md#example-requirement` |
| `../src/module.rs` | `src/module.rs` |
| `https://example.com/evidence` | `https://example.com/evidence` |

## Structured Relations

Multi-element documents use `#### Relations` within an H3 element. Single-element documents use `## Relations` outside their named body. Entries use two-space bullet indentation and a case-sensitive registered relation token:

```markdown
#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
  * satisfiedBy: [Implementation](../src/module.rs)
```

Canonical relation roles include:
- `derive` / `derivedFrom`: compatible same-family hierarchy and concept-scheme context.
- `specify` / `specifiedBy`: requirement-to-capability realization.
- `define` / `definedBy`: unique requirement ownership of compatible contracts.
- `verifiedBy` / `verify`: requirement-to-concrete-verification coverage.
- `satisfiedBy` / `satisfy`: implementation or evidence satisfaction of requirements and evidence-backed verification types.
- `constrainedBy` / `constrain` and `use` / `usedBy`: semantic-contract application and ontology context.
- `broader` / `narrower`, `related`, `exactMatch`, and `closeMatch`: permitted native-concept taxonomy, association and mapping.

The following snippets illustrate source roles; referenced model elements and local artifacts must exist in the complete model.

```markdown
### Child Requirement

The system SHALL provide the child behavior.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Requirement](#parent-requirement)
  * satisfiedBy: [Implementation](../src/module.rs)
```

```markdown
### Example Specification

Definition of the child behavior.

#### Metadata
  * type: specification

#### Relations
  * define: [Child Requirement](#child-requirement)
```

```markdown
### Example Inspection

Inspect the child behavior against its expected result.

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Child Requirement](#child-requirement)
```

## Dependency and Terminology Declarations

Contract Bindings express shared implementation obligations; Contract References express content dependencies for change-impact review. Only requirements author them, and a requirement uses one dependency section rather than both. Their entries are Markdown links to compatible, uniquely requirement-owned contracts.

```markdown
#### Contract References
  * [Error Response Specification](Contracts.md#error-response-specification)
```

The MCP mutation keywords `bindContract` and `referenceContract` select these subsections. They are not ordinary Relations tokens. Permitted non-ontology, non-semantic-contract elements bind readable terminology through Concept References to native concepts. Authored prose uses readable domain words without model-element hyperlinks.

## Validation

The shared validators enforce registered tokens, source/target compatibility, unique ownership, current target resolution, permitted hierarchy and dependency directions, cycle rules and duplicate declarations. Capabilities receive satisfaction and verification coverage through their requirements. Verification objectives organize concrete verification work without direct verification or satisfaction edges.

Disk loading and complete mutation candidates use the same semantic rules. A rejected mutation preserves source files and accepted state, and returns structured diagnostics with source/target context and repair guidance.
