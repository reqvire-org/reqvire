# Elements

### Reused Contract Context and Resource Operation Verification Objective

This objective groups verification that reused_contract_context and asset operations parse, validate, render, search, move, remove, and report reused resources correctly.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Reuse Command Verification](#reuse-command-verification)
  * derive: [Reused Contract Context Identifier CRUD Verification](#reused-contract-context-identifier-crud-verification)
  * derive: [Reused Contract Context Output Rendering Verification](#reused-contract-context-output-rendering-verification)
  * derive: [Reused Contract Context Scope Constraints Test](#reused-contract-context-scope-constraints-test)
  * derive: [Reused Contract Context Search Filters Verification](#reused-contract-context-search-filters-verification)
  * derive: [Reused Contract Context Change Impact Verification](#reused-contract-context-change-impact-verification)
  * derive: [Reused Contract Context Subsection Parsing Verification](#reused-contract-context-subsection-parsing-verification)
  * derive: [Reused Contract Context Validation Verification](#reused-contract-context-validation-verification)
  * derive: [Contract Reused Contract Context Verification](#contract-reused-contract-context-verification)
  * derive: [Remove Reused Context Command Verification](#remove reused context-command-verification)
  * derive: [Move Asset Command Verification](#move-asset-command-verification)
  * derive: [Remove Asset Command Verification](#remove-asset-command-verification)
---

### Reuse Command Verification

Verify reuse command creates Reused Contract Context subsection and adds links.

#### Details
Test cases for identifier reused_contract_context:
- Create Reused Contract Context subsection if missing
- Add link with format `[Name](file.md#id)` or `[Name](#id)` for same-file references
- Idempotent: duplicate reuse doesn't create duplicate entry
- Many-to-many: same contract identifier reuses to multiple elements
- Dry-run mode makes no changes
- Reuse Contract element by identifier target
- Only Contract types allowed (constraint, behavior, specification, state, input-output)
- Error when reusesContract non-Contract element (requirement, verification, etc.)
- Error when identifier target is unresolved
- Error when target is not a valid contract identifier
- Error when target is a file path (identifier-only reused_contract_context)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reused Contract Context Commands](../../../Interfaces/CLI/Commands.md#reused-contract-context-commands)
---

### Reused Contract Context Identifier CRUD Verification

Verify that moving or renaming Contract elements updates reused_contract_context identifiers throughout the model.

#### Details
Test cases for rename operations:
- Renaming `Test Constraint Element` to `Renamed Constraint` updates reused_contract_context links in `specifications/Requirements.md` to `#renamed-constraint`
- Renaming `Test Constraint Element` updates reused_contract_context links in all referencing files used by the fixture (including `specifications/AdditionalRequirements.md`)
- Model validation succeeds after rename

Test cases for move operations:
- Moving `Test Constraint Element` to `specifications/Contracts.md` updates reused_contract_context links in `specifications/Requirements.md` to `Contracts.md#test-constraint-element`
- Moving `Test Constraint Element` updates reused_contract_context links in all referencing files used by the fixture (including `specifications/AdditionalRequirements.md`)
- Model validation succeeds after move

Test cases for consistency:
- Reused Contract Context identifiers remain resolvable after rename/move operations exercised by the test script

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-contract-elements/test.sh)
  * verify: [Reused Contract Context Identifier Updates](../../../ModelStructure/ModelManagement.md#reused-contract-context-identifier-updates)
---

### Reused Contract Context Output Rendering Verification

Verify reused_contract_context render correctly in all output formats.

#### Details
Test cases for identifier reused_contract_context:
- Markdown output preserves format
- Explorer content renders clickable links
- JSON includes reused_contract_context array
- JSON includes element identifiers in reused_contract_context array as strings
- Element identifier format: `"file.md#element-name"`
- Consistent indentation in markdown
- Mixed same-file and cross-file identifier reused_contract_context display correctly

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reserved Subsections Support](../../../ModelStructure/StructureAndParsing.md#reserved-subsections-support)
---

### Reused Contract Context Scope Constraints Test

Verify that reused_contract_context scope constraints (hierarchical independence, define requirement, upstream propagation, and one-direction subgraph flow) are enforced for contract-element identifier reused_contract_context.

#### Details
**Test cases for contract hierarchical independence:**
- Model with reused_contract_context to contract from same hierarchy causes `validate` to fail
- Error when reusesContract requirement has `definedBy` to the contract
- Error when reusesContract requirement is parent of the defining requirement
- Error when reusesContract requirement is child/grandchild of the defining requirement
- Accept reused_contract_context when reusesContract requirement is in a separate branch

**Test cases for upstream reused_contract_context propagation:**
- Error when ancestor requirement already has the same contract identifier reused_contract_context
- Error when descendant requirement already has the same reused_contract_context (suggest move)
- Reused Contract Context propagate downstream - descendants cannot re-reuse
- Accept reused_contract_context when no ancestor or descendant has the same reused_contract_context

**Test cases for one-direction subgraph flow:**
- Error when a subgraph tries to reuse a contract owned by a subgraph that already reuses contracts owned by the first subgraph
- Link command rejects reverse-direction cross-subgraph reused_contract_context creation
- Merge command rejects merged reused_contract_context that would introduce reverse-direction subgraph flow

**Error message formats:**
- Contract hierarchy: `'<contract>' cannot be reused to '<element>' because it is within the contract's defining hierarchy`
- Ancestor propagation: `'<reused_contract_context>' is already reused at '<ancestor>' which is an ancestor. Reused Contract Context propagate downstream.`
- Descendant conflict: `'<reused_contract_context>' is already reused at '<descendant>' which is a descendant. Move reused_contract_context to '<element>' if you want it at higher level.`
- Direction conflict: `'<reused_contract_context>' cannot be reused to '<element>' because subgraph '<root>' already receives reused_contract_context contracts from subgraph '<other-root>'`

**Test cases for define owner requirement:**
- Model with reused_contract_context to orphan contract (no define relations) causes `validate` to fail
- Error message indicates contract must define a capability or requirement
- Accept reused_contract_context to contract with define relations

**Test cases for reuse command:**
- `link REQ reusesContract CONTRACT` fails when REQ is in same hierarchy
- `link REQ reusesContract ORPHAN-CONTRACT` fails when contract has no define
- `link REQ reusesContract TARGET` fails when TARGET is not a valid contract identifier
- Error messages are consistent with validate error format

**Test cases for merge command:**
- `merge TARGET SOURCE` fails when SOURCE has reused_contract_context that violates hierarchy constraint for TARGET
- `merge TARGET SOURCE` fails when SOURCE has reused_contract_context to orphan contract
- Error message indicates which reused_contract_context violates the constraint

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-reused-contract-context-constraints/test.sh)
  * verify: [Reused Contract Context Scope Constraints](../../../ModelStructure/ModelManagement.md#reused-contract-context-scope-constraints)
  * verify: [Reused Contract Context Scope Validation](../../../Operations/Validation/ValidationRequirements.md#reused-contract-context-scope-validation)
---

### Reused Contract Context Search Filters Verification

Verify search filters correctly find elements by reused_contract_context.

#### Details
Test cases:
- `--has-reused-contract-context` finds only elements with reused_contract_context
- No false positives or false negatives

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [CLI Search Command](../../../Interfaces/CLI/Commands.md#cli-search-command)
---

### Reused Contract Context Change Impact Verification

Verify Contract element operations are tracked in change impact analysis.

#### Details
Test cases for Contract element content changes:
- Contract element content change is detected in change-impact report
- Same-file, cross-file, and cross-directory Contract references are validated

Test cases for Contract element mv operations:
- mv to different file: relocation reported, reused_contract_context identifiers updated automatically
- mv to different directory: relocation reported, reused_contract_context identifiers updated automatically
- Behavior matches relation target relocation handling

Test cases for Contract element rm operations:
- rm of reused Contract element: validation fails with clear error about broken reused_contract_context

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-reused-contract-context/test.sh)
  * verify: [Change Impact Detection](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#change-impact-detection)
---

### Reused Contract Context Subsection Parsing Verification

Verify the system correctly parses Reused Contract Context subsections using contract element identifiers.

#### Details
Test cases for element identifiers:
- Parse markdown links to Contract elements (constraint, behavior, specification, state, input-output)
- Normalize element identifiers like relation targets
- Support full identifier format `file.md#element-name`
- Support same-file format `#element-name`
- Handle multiple identifier reused_contract_context in single element
- Reject file-path reused_contract_context syntax

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reserved Subsections Support](../../../ModelStructure/StructureAndParsing.md#reserved-subsections-support)
  * verify: [Reused Contract Context Target Validation](../../../Operations/Validation/ValidationRequirements.md#reused-contract-context-target-validation)
---

### Reused Contract Context Validation Verification

Verify the system validates reused_contract_context targets as contract element identifiers.

#### Details
Test cases for element identifiers:
- Accept Contract element identifiers (constraint, behavior, specification, state, input-output)
- Reject non-Contract element identifiers (capability, requirement, verification)
- Reject file-path reused_contract_context syntax
- Error message indicates expected Contract type
- Validation fails for non-existent element identifiers
- Validation occurs in Pass 2

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reserved Subsections Support](../../../ModelStructure/StructureAndParsing.md#reserved-subsections-support)
  * verify: [Reused Contract Context Target Validation](../../../Operations/Validation/ValidationRequirements.md#reused-contract-context-target-validation)
---

### Contract Reused Contract Context Verification

Verify reused_contract_context rules for requirement-owned contract elements and rejection of ontology reused_contract_context.

#### Details
Test cases:
- Capability reused_contract_context to any target fails; capabilities use `#### Concept References` for semantic vocabulary and relations for verification/specification.
- Requirement reused_contract_context to an `ontology` element fails.
- Requirement reused_contract_context to a `semantic-contract` fails; requirements use `constrainedBy` instead.
- Requirement reused_contract_context to ontology is forbidden because ontology vocabulary dependencies use concept references or semantic-contract `use` relations.
- Requirement reused_contract_context to ordinary contracts is limited to requirement-owned contract elements.
- Verification element reused_contract_context to any target fails; verification evidence must use `satisfiedBy`, and verified targets must use `verify`.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-capability-reused-contract-context/test.sh)
  * verify: [Ontology and Semantic Contract Model](../../../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
  * verify: [Reused Contract Context Scope Validation](../../../Operations/Validation/ValidationRequirements.md#reused-contract-context-scope-validation)
  * verify: [Reused Contract Context Target Validation](../../../Operations/Validation/ValidationRequirements.md#reused-contract-context-target-validation)
---

### Remove Reused Context Command Verification

Verify remove reused context command removes identifier links and cleans up empty subsections.

#### Details
Test cases for identifier reused_contract_context:
- Remove link from Reused Contract Context subsection
- Remove subsection when no reused_contract_context remain
- Remove Reused Context from one element doesn't affect others
- Dry-run mode makes no changes
- Remove Reused Context Contract element by identifier target
- Works for both same-file and cross-file contract identifiers

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reused Contract Context Commands](../../../Interfaces/CLI/Commands.md#reused-contract-context-commands)
---

### Move Asset Command Verification

Verify mv-asset moves InternalPath files and updates path-based relations while leaving contract-identifier reused_contract_context unchanged.

#### Details
Test cases for reused_contract_context behavior:
- Existing contract-identifier reused_contract_context remain unchanged after mv-asset
- No reused_contract_context entries are rewritten due to path moves

Test cases for Relations updates:
- Find all satisfiedBy relations with InternalPath matching old path
- Find all satisfy relations with InternalPath matching old path
- Find all relation targets with InternalPath matching old path
- Update relation target to new path
- Handle relative path resolution from element's file location

Test cases for filesystem and reporting:
- Move/rename physical file on filesystem
- Report count of affected Relations
- Report list of modified specification files
- Dry-run mode: show changes without applying (file not moved)
- JSON mode: emit valid structured CRUD result output
- JSON file mode: write valid structured CRUD result output to `--output <FILE>`
- Validation passes after move

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [CLI Move Asset Command](../../../Interfaces/CLI/Commands.md#cli-move-asset-command)
---

### Remove Asset Command Verification

Verify rm-asset deletes InternalPath files and removes path-based relations while leaving contract-identifier reused_contract_context unchanged.

#### Details
Test cases for reused_contract_context behavior:
- Existing contract-identifier reused_contract_context remain unchanged after rm-asset
- No reused_contract_context entries are removed by rm-asset

Test cases for Relations updates:
- Find all satisfiedBy relations with InternalPath matching path
- Find all satisfy relations with InternalPath matching path
- Find all relation targets with InternalPath matching path
- Remove entire relation line from element

Test cases for filesystem and reporting:
- Delete physical file from filesystem
- Report count of removed Relations
- Report list of modified specification files
- Dry-run mode: show changes without applying (file not deleted)
- JSON mode: emit valid structured CRUD result output
- JSON file mode: write valid structured CRUD result output to `--output <FILE>`
- Validation passes after removal

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [CLI Remove Asset Command](../../../Interfaces/CLI/Commands.md#cli-remove-asset-command)
---
