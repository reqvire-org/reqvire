# Link and Unlink Elements

Use `definedBy` for contract ownership, `bindContract` for shared implementation obligations, and `referenceContract` for dependencies on contract content. The decision rules are in [Choosing Contract Dependencies](../SKILL.md#choosing-contract-dependencies).

## Link

Inspect the source requirement, the target contract, its owner, and the existing dependency section before editing. Use exact element names or normalized identifiers. Both contract dependency kinds target compatible, uniquely requirement-owned `source`, `constraint`, `behavior`, `specification`, `state`, or `input-output` elements; file paths and URLs are not contract dependency targets.

Preview a reference, then apply it:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "API Documentation" referenceContract "Error Response Specification" --dry-run
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "API Documentation" referenceContract "Error Response Specification"
```

A requirement implementing the shared error response obligation uses a binding:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "API Endpoint" bindContract "Error Response Specification"
```

The command writes a Markdown link under the corresponding `#### Contract References` or `#### Contract Bindings` section and computes the relative target path. These are subsection operations, not tokens to author under `#### Relations`.

### Supported ordinary relations

| Relation | Purpose |
|---|---|
| `derivedFrom` / `derive` | Child/parent hierarchy within a compatible family |
| `specify` / `specifiedBy` | Requirement/capability association |
| `verifiedBy` / `verify` | Requirement/concrete verification association |
| `satisfiedBy` / `satisfy` | Requirement or evidence-backed verification implementation/evidence |
| `definedBy` / `define` | Requirement ownership of an ordinary contract |
| `constrainedBy` / `constrain` | Requirement/semantic-contract application |
| `use` / `usedBy` | Semantic-contract ontology context |

Ordinary relation targets depend on the relation. For example, implementation evidence can target a file, while `definedBy` requires a contract element. Capabilities and verification objectives do not use `satisfiedBy`, and capabilities are not directly verified. Generated inverse relations do not constitute a second authored edge.

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Password Login" derivedFrom "Authentication"
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" link "Password Login" satisfiedBy "src/auth/login.rs"
```

### Rejected candidates

- A requirement cannot contain both Contract Bindings and Contract References, even for different targets.
- Duplicate normalized reference targets and targets repeated in Relations are invalid.
- References cannot point to their source's own contract or form cycles through referenced/bound contract owners and requirement parents. Acyclic references within one hierarchy are permitted.
- Bindings retain hierarchy-independence, subgraph-direction, and combined fulfillment-cycle constraints.
- Link, relink, create, override, hierarchy/ownership edits, and merge must validate the candidate before persisting it. Rejected edits leave authored files unchanged; dry runs do not persist edits.

## Unlink

`unlink` identifies the existing authored relation, binding, or reference to the target. It removes the entry and cleans up an empty subsection.

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "API Documentation" "Error Response Specification" --dry-run
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" unlink "API Documentation" "Error Response Specification"
```

## Convert existing dependencies

1. Read every dependency on the source requirement and decide whether it allocates implementation work or preserves content/review context.
2. For a wholly context-dependent requirement, replace the whole Contract Bindings section with Contract References, preserving target identifiers. Review the complete candidate before applying it. For one target, unlink the binding and link the reference; a requirement with multiple bindings must have all bindings removed before any reference can be added.
3. If the requirement has genuinely different responsibilities, separate those obligations into meaningful requirements and place each dependency accordingly. Do not reclassify an implementation obligation merely to avoid the mutually exclusive section rule.
4. Preserve contract ownership and satisfaction evidence. Run validation and compare coverage: a contract owner that loses its last binding consumer may now need direct implementation evidence. A reference consumer does not satisfy that owner.
5. Review collection and change impact to ensure dependency context remains reachable.

## Search and verification

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --has-contract-bindings --filter-contract-bindings="*#error-response-specification" --json
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --has-contract-references --filter-contract-references="*#error-response-specification" --json
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" collect "API Documentation" --json
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json
```

Target filters use globs over normalized identifiers and combine with other filters using AND. Ordinary relation filters do not substitute for binding/reference filters. Both dependency kinds are included in collection and change-impact analysis; only bindings create fulfillment obligations.
