# Elements

### Contract Bindings and Resource Operation Verification Objective

This objective groups verification that contract_bindings and asset operations parse, validate, render, search, move, remove, and report bound resources correctly.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Bind Contract Command Verification](#bind-contract-command-verification)
  * derive: [Contract Bindings Change Impact Verification](#contract-bindings-change-impact-verification)
  * derive: [Contract Bindings Identifier CRUD Verification](#contract-bindings-identifier-crud-verification)
  * derive: [Contract Bindings Output Rendering Verification](#contract-bindings-output-rendering-verification)
  * derive: [Contract Bindings Scope Constraints Test](#contract-bindings-scope-constraints-test)
  * derive: [Contract Bindings Search Filters Verification](#contract-bindings-search-filters-verification)
  * derive: [Contract Bindings Subsection Parsing Verification](#contract-bindings-subsection-parsing-verification)
  * derive: [Contract Bindings Validation Verification](#contract-bindings-validation-verification)
  * derive: [Contract Bindings Verification](#contract-bindings-verification)
  * derive: [Move Asset Command Verification](#move-asset-command-verification)
  * derive: [Remove Asset Command Verification](#remove-asset-command-verification)
  * derive: [Remove Contract Binding Command Verification](#remove-contract-binding-command-verification)
---

### Bind Contract Command Verification

Verify bindContract command creates Contract Bindings subsection and adds links.

#### Details
Test cases for identifier contract_bindings:
- Create Contract Bindings subsection if missing
- Add link with format `[Name](file.md#id)` or `[Name](#id)` for same-file references
- Idempotent: duplicate binding doesn't create duplicate entry
- Many-to-many: same contract identifier binds to multiple elements
- Dry-run mode makes no changes
- Reuse Contract element by identifier target
- Only Contract types allowed (constraint, behavior, specification, state, input-output)
- Error when bindContract non-Contract element (requirement, verification, etc.)
- Error when identifier target is unresolved
- Error when target is not a valid contract identifier
- Error when target is a file path (identifier-only contract_bindings)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Contract Bindings Commands](../../../Interfaces/CLI/Commands.md#contract-bindings-commands)
---

### Contract Bindings Change Impact Verification

Verify Contract element operations are tracked in change impact analysis.

#### Details
Test cases for Contract element content changes:
- Contract element content change is detected in change-impact report
- Same-file, cross-file, and cross-directory Contract references are validated

Test cases for Contract element mv operations:
- mv to different file: relocation reported, contract_bindings identifiers updated automatically
- mv to different directory: relocation reported, contract_bindings identifiers updated automatically
- Behavior matches relation target relocation handling

Test cases for Contract element rm operations:
- rm of bound Contract element: validation fails with clear error about broken contract_bindings

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-contract-bindings/test.sh)
  * verify: [Change Impact Detection](../../../Processing/ChangeImpact/ChangeImpactRequirements.md#change-impact-detection)
---

### Contract Bindings Identifier CRUD Verification

Verify that moving or renaming Contract elements updates contract_bindings identifiers throughout the model.

#### Details
Test cases for rename operations:
- Renaming `Test Constraint Element` to `Renamed Constraint` updates contract_bindings links in `specifications/Requirements.md` to `#renamed-constraint`
- Renaming `Test Constraint Element` updates contract_bindings links in all referencing files used by the fixture (including `specifications/AdditionalRequirements.md`)
- Model validation succeeds after rename

Test cases for move operations:
- Moving `Test Constraint Element` to `specifications/Contracts.md` updates contract_bindings links in `specifications/Requirements.md` to `Contracts.md#test-constraint-element`
- Moving `Test Constraint Element` updates contract_bindings links in all referencing files used by the fixture (including `specifications/AdditionalRequirements.md`)
- Model validation succeeds after move

Test cases for consistency:
- Contract Bindings identifiers remain resolvable after rename/move operations exercised by the test script

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-contract-elements/test.sh)
  * verify: [Contract Bindings Identifier Updates](../../../ModelStructure/ModelManagement.md#contract-bindings-identifier-updates)
---

### Contract Bindings Output Rendering Verification

Verify contract_bindings render correctly in all output formats.

#### Details
Test cases for identifier contract_bindings:
- Markdown output preserves format
- Explorer content renders clickable links
- JSON includes contract_bindings array
- JSON includes element identifiers in contract_bindings array as strings
- Element identifier format: `"file.md#element-name"`
- Consistent indentation in markdown
- Mixed same-file and cross-file identifier contract_bindings display correctly

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reserved Subsections Support](../../../ModelStructure/StructureAndParsing.md#reserved-subsections-support)
---

### Contract Bindings Scope Constraints Test

Verify that contract_bindings scope constraints (hierarchical independence, define requirement, upstream propagation, and one-direction subgraph flow) are enforced for contract-element identifier contract_bindings.

#### Details
**Test cases for contract hierarchical independence:**
- Model with contract_bindings to contract from same hierarchy causes `validate` to fail
- Error when binding requirement has `definedBy` to the contract
- Error when binding requirement is parent of the defining requirement
- Error when binding requirement is child/grandchild of the defining requirement
- Accept contract_bindings when binding requirement is in a separate branch

**Test cases for upstream contract_bindings propagation:**
- Error when ancestor requirement already has the same contract identifier contract_bindings
- Error when descendant requirement already has the same contract_bindings (suggest move)
- Contract Bindings propagate downstream - descendants cannot re-bind
- Accept contract_bindings when no ancestor or descendant has the same contract_bindings

**Test cases for one-direction subgraph flow:**
- Error when a subgraph tries to bind a contract owned by a subgraph that already binds contracts owned by the first subgraph
- Link command rejects reverse-direction cross-subgraph contract_bindings creation
- Merge command rejects merged contract_bindings that would introduce reverse-direction subgraph flow

**Error message formats:**
- Contract hierarchy: `'<contract>' cannot be bound to '<element>' because it is within the contract's defining hierarchy`
- Ancestor propagation: `'<contract_bindings>' is already bound at '<ancestor>' which is an ancestor. Contract Bindings propagate downstream.`
- Descendant conflict: `'<contract_bindings>' is already bound at '<descendant>' which is a descendant. Move contract_bindings to '<element>' if you want it at higher level.`
- Direction conflict: `'<contract_bindings>' cannot be bound to '<element>' because subgraph '<root>' already receives bound contracts from subgraph '<other-root>'`

**Test cases for define owner requirement:**
- Model with contract_bindings to orphan contract (no define relations) causes `validate` to fail
- Error message indicates contract must define a capability or requirement
- Accept contract_bindings to contract with define relations

**Test cases for bindContract command:**
- `link REQ bindContract CONTRACT` fails when REQ is in same hierarchy
- `link REQ bindContract ORPHAN-CONTRACT` fails when contract has no define
- `link REQ bindContract TARGET` fails when TARGET is not a valid contract identifier
- Error messages are consistent with validate error format

**Test cases for merge command:**
- `merge TARGET SOURCE` fails when SOURCE has contract_bindings that violates hierarchy constraint for TARGET
- `merge TARGET SOURCE` fails when SOURCE has contract_bindings to orphan contract
- Error message indicates which contract_bindings violates the constraint

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-contract-bindings-constraints/test.sh)
  * verify: [Contract Bindings Scope Constraints](../../../ModelStructure/ModelManagement.md#contract-bindings-scope-constraints)
  * verify: [Contract Bindings Scope Validation](../../../Operations/Validation/ValidationRequirements.md#contract-bindings-scope-validation)
---

### Contract Bindings Search Filters Verification

Verify search filters correctly find elements by contract_bindings.

#### Details
Test cases:
- `--has-contract-bindings` finds only elements with contract_bindings
- No false positives or false negatives

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [CLI Search Command](../../../Interfaces/CLI/Commands.md#cli-search-command)
---

### Contract Bindings Subsection Parsing Verification

Verify the system correctly parses Contract Bindings subsections using contract element identifiers.

#### Details
Test cases for element identifiers:
- Parse markdown links to Contract elements (constraint, behavior, specification, state, input-output)
- Normalize element identifiers like relation targets
- Support full identifier format `file.md#element-name`
- Support same-file format `#element-name`
- Handle multiple identifier contract_bindings in single element
- Reject file-path contract_bindings syntax

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reserved Subsections Support](../../../ModelStructure/StructureAndParsing.md#reserved-subsections-support)
  * verify: [Contract Bindings Target Validation](../../../Operations/Validation/ValidationRequirements.md#contract-bindings-target-validation)
---

### Contract Bindings Validation Verification

Verify the system validates contract_bindings targets as contract element identifiers.

#### Details
Test cases for element identifiers:
- Accept Contract element identifiers (constraint, behavior, specification, state, input-output)
- Reject non-Contract element identifiers (capability, requirement, verification)
- Reject file-path contract_bindings syntax
- Error message indicates expected Contract type
- Validation fails for non-existent element identifiers
- Validation occurs in Pass 2

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reserved Subsections Support](../../../ModelStructure/StructureAndParsing.md#reserved-subsections-support)
  * verify: [Contract Bindings Target Validation](../../../Operations/Validation/ValidationRequirements.md#contract-bindings-target-validation)
---

### Contract Bindings Verification

Verify contract_bindings rules for requirement-owned contract elements and rejection of ontology contract_bindings.

#### Details
Test cases:
- Capability contract_bindings to any target fails; capabilities use `#### Concept References` for semantic vocabulary and `specifiedBy` requirements for coverage and specification.
- Requirement contract_bindings to an `ontology` element fails.
- Requirement contract_bindings to a `semantic-contract` fails; requirements use `constrainedBy` instead.
- Requirement contract_bindings to ontology is forbidden because ontology vocabulary dependencies use concept references or semantic-contract `use` relations.
- Requirement contract_bindings to ordinary contracts is limited to requirement-owned contract elements.
- Verification element contract_bindings to any target fails; verification evidence must use `satisfiedBy`, and verified targets must use `verify`.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-capability-contract-bindings/test.sh)
  * verify: [Ontology and Semantic Contract Model](../../../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
  * verify: [Contract Bindings Scope Validation](../../../Operations/Validation/ValidationRequirements.md#contract-bindings-scope-validation)
  * verify: [Contract Bindings Target Validation](../../../Operations/Validation/ValidationRequirements.md#contract-bindings-target-validation)
---

### Move Asset Command Verification

Verify mv-asset moves InternalPath files and updates path-based relations while leaving contract-identifier contract_bindings unchanged.

#### Details
Test cases for contract_bindings behavior:
- Existing contract-identifier contract_bindings remain unchanged after mv-asset
- No contract_bindings entries are rewritten due to path moves

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

Verify rm-asset deletes InternalPath files and removes path-based relations while leaving contract-identifier contract_bindings unchanged.

#### Details
Test cases for contract_bindings behavior:
- Existing contract-identifier contract_bindings remain unchanged after rm-asset
- No contract_bindings entries are removed by rm-asset

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

### Remove Contract Binding Command Verification

Verify remove contract binding command removes identifier links and cleans up empty subsections.

#### Details
Test cases for identifier contract_bindings:
- Remove link from Contract Bindings subsection
- Remove subsection when no contract_bindings remain
- Remove Contract Binding from one element doesn't affect others
- Dry-run mode makes no changes
- Remove Contract Binding Contract element by identifier target
- Works for both same-file and cross-file contract identifiers

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Contract Bindings Commands](../../../Interfaces/CLI/Commands.md#contract-bindings-commands)
---
