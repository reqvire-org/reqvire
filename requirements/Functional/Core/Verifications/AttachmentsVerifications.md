# Elements

### Attach Command Verification

Verify attach command creates Attachments subsection and adds links.

#### Details
Test cases for identifier attachments:
- Create Attachments subsection if missing
- Add link with format `[Name](file.md#id)` or `[Name](#id)` for same-file references
- Idempotent: duplicate attach doesn't create duplicate entry
- Many-to-many: same refinement identifier attaches to multiple elements
- Dry-run mode makes no changes
- Attach Refinement element by identifier target
- Only Refinement types allowed (constraint, behavior, specification, state, input-output)
- Error when attaching non-Refinement element (requirement, verification, etc.)
- Error when identifier target is unresolved
- Error when target is not a valid refinement identifier
- Error when target is a file path (identifier-only attachments)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Attachment Commands](../../../Interfaces/CLI/Commands.md#attachment-commands)
---

### Attachment Identifier CRUD Verification

Verify that moving or renaming Refinement elements updates attachment identifiers throughout the model.

#### Details
Test cases for rename operations:
- Renaming `Test Constraint Element` to `Renamed Constraint` updates attachment links in `specifications/Requirements.md` to `#renamed-constraint`
- Renaming `Test Constraint Element` updates attachment links in all referencing files used by the fixture (including `specifications/AdditionalRequirements.md`)
- Model validation succeeds after rename

Test cases for move operations:
- Moving `Test Constraint Element` to `specifications/Refinements.md` updates attachment links in `specifications/Requirements.md` to `Refinements.md#test-constraint-element`
- Moving `Test Constraint Element` updates attachment links in all referencing files used by the fixture (including `specifications/AdditionalRequirements.md`)
- Model validation succeeds after move

Test cases for consistency:
- Attachment identifiers remain resolvable after rename/move operations exercised by the test script

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-refinement-elements/test.sh)
  * verify: [Attachment Identifier Updates](../ModelManagement.md#attachment-identifier-updates)
---

### Attachment Output Rendering Verification

Verify attachments render correctly in all output formats.

#### Details
Test cases for identifier attachments:
- Markdown output preserves format
- HTML export renders clickable links
- JSON includes attachments array
- JSON includes element identifiers in attachments array as strings
- Element identifier format: `"file.md#element-name"`
- Consistent indentation in markdown
- Mixed same-file and cross-file identifier attachments display correctly

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-attachment-export/test.sh)
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
---

### Ontology and Contract Attachment Verification

Verify attachment rules for ontology elements and requirement-owned semantic-contract elements.

#### Details
Test cases:
- Feature attachment to an `ontology` element validates.
- Feature attachment to a requirement-owned `semantic-contract` fails.
- Feature attachment to a requirement-detail refinement such as `input-output` fails.
- Requirement attachment to an `ontology` element fails.
- Requirement attachment to a requirement-owned `semantic-contract` validates.
- Requirement attachment to ontology is forbidden because ontology context is inherited from the owning feature path.
- Requirement attachment to semantic contracts is limited to requirement-owned semantic contracts.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-feature-attachments/test.sh)
  * verify: [Attachment Target Validation](../Validation.md#attachment-target-validation)
  * verify: [Attachment Scope Validation](../Validation.md#attachment-scope-validation)
  * verify: [Ontology and Semantic Contract Model](../ModelManagement.md#ontology-and-semantic-contract-model)
---

### Attachment Scope Constraints Test

Verify that attachment scope constraints (hierarchical independence, refine requirement, upstream propagation, and one-direction subgraph flow) are enforced for refinement-element identifier attachments.

#### Details
**Test cases for refinement hierarchical independence:**
- Model with attachment to refinement from same hierarchy causes `validate` to fail
- Error when attaching requirement has `refinedBy` to the refinement
- Error when attaching requirement is parent of the defining requirement
- Error when attaching requirement is child/grandchild of the defining requirement
- Accept attachment when attaching requirement is in a separate branch

**Test cases for upstream attachment propagation:**
- Error when ancestor requirement already has the same refinement identifier attachment
- Error when descendant requirement already has the same attachment (suggest move)
- Attachments propagate downstream - descendants cannot re-attach
- Accept attachment when no ancestor or descendant has the same attachment

**Test cases for one-direction subgraph flow:**
- Error when a subgraph tries to attach a refinement owned by a subgraph that already attaches refinements owned by the first subgraph
- Link command rejects reverse-direction cross-subgraph attachment creation
- Merge command rejects merged attachments that would introduce reverse-direction subgraph flow

**Error message formats:**
- Refinement hierarchy: `'<refinement>' cannot be attached to '<element>' because it is within the refinement's defining hierarchy`
- Ancestor propagation: `'<attachment>' is already attached at '<ancestor>' which is an ancestor. Attachments propagate downstream.`
- Descendant conflict: `'<attachment>' is already attached at '<descendant>' which is a descendant. Move attachment to '<element>' if you want it at higher level.`
- Direction conflict: `'<attachment>' cannot be attached to '<element>' because subgraph '<root>' already receives attachment contracts from subgraph '<other-root>'`

**Test cases for refine owner requirement:**
- Model with attachment to orphan refinement (no refine relations) causes `validate` to fail
- Error message indicates refinement must refine a feature or requirement
- Accept attachment to refinement with refine relations

**Test cases for attach command:**
- `link REQ attaching REFINEMENT` fails when REQ is in same hierarchy
- `link REQ attaching ORPHAN-REFINEMENT` fails when refinement has no refine
- `link REQ attaching TARGET` fails when TARGET is not a valid refinement identifier
- Error messages are consistent with validate error format

**Test cases for merge command:**
- `merge TARGET SOURCE` fails when SOURCE has attachment that violates hierarchy constraint for TARGET
- `merge TARGET SOURCE` fails when SOURCE has attachment to orphan refinement
- Error message indicates which attachment violates the constraint

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-attachment-constraints/test.sh)
  * verify: [Attachment Scope Constraints](../ModelManagement.md#attachment-scope-constraints)
  * verify: [Attachment Scope Validation](../Validation.md#attachment-scope-validation)
---

### Attachment Search Filters Verification

Verify search filters correctly find elements by attachments.

#### Details
Test cases:
- `--has-attachments` finds only elements with attachments
- No false positives or false negatives

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [CLI Search Command](../../../Interfaces/CLI/Commands.md#cli-search-command)
---

### Attachments Change Impact Verification

Verify Refinement element operations are tracked in change impact analysis.

#### Details
Test cases for Refinement element content changes:
- Refinement element content change is detected in change-impact report
- Same-file, cross-file, and cross-directory Refinement references are validated

Test cases for Refinement element mv operations:
- mv to different file: relocation reported, attachment identifiers updated automatically
- mv to different directory: relocation reported, attachment identifiers updated automatically
- Behavior matches relation target relocation handling

Test cases for Refinement element rm operations:
- rm of attached Refinement element: validation fails with clear error about broken attachment

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-attachments/test.sh)
  * verify: [Change Impact Detection](../../Processing/ChangeImpact.md#change-impact-detection)
---

### Attachments Subsection Parsing Verification

Verify the system correctly parses Attachments subsections using refinement element identifiers.

#### Details
Test cases for element identifiers:
- Parse markdown links to Refinement elements (constraint, behavior, specification, state, input-output)
- Normalize element identifiers like relation targets
- Support full identifier format `file.md#element-name`
- Support same-file format `#element-name`
- Handle multiple identifier attachments in single element
- Reject file-path attachment syntax

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
  * verify: [Attachment Target Validation](../Validation.md#attachment-target-validation)
---

### Attachments Validation Verification

Verify the system validates attachment targets as refinement element identifiers.

#### Details
Test cases for element identifiers:
- Accept Refinement element identifiers (constraint, behavior, specification, state, input-output)
- Reject non-Refinement element identifiers (feature, requirement, verification)
- Reject file-path attachment syntax
- Error message indicates expected Refinement type
- Validation fails for non-existent element identifiers
- Validation occurs in Pass 2

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
  * verify: [Attachment Target Validation](../Validation.md#attachment-target-validation)
---

### Detach Command Verification

Verify detach command removes identifier links and cleans up empty subsections.

#### Details
Test cases for identifier attachments:
- Remove link from Attachments subsection
- Remove subsection when no attachments remain
- Detach from one element doesn't affect others
- Dry-run mode makes no changes
- Detach Refinement element by identifier target
- Works for both same-file and cross-file refinement identifiers

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Attachment Commands](../../../Interfaces/CLI/Commands.md#attachment-commands)
---

### Move Asset Command Verification

Verify mv-asset moves InternalPath files and updates path-based relations while leaving refinement-identifier attachments unchanged.

#### Details
Test cases for attachment behavior:
- Existing refinement-identifier attachments remain unchanged after mv-asset
- No attachment entries are rewritten due to path moves

Test cases for Relations updates:
- Find all satisfiedBy relations with InternalPath matching old path
- Find all satisfy relations with InternalPath matching old path
- Find all trace relations with InternalPath matching old path
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

Verify rm-asset deletes InternalPath files and removes path-based relations while leaving refinement-identifier attachments unchanged.

#### Details
Test cases for attachment behavior:
- Existing refinement-identifier attachments remain unchanged after rm-asset
- No attachment entries are removed by rm-asset

Test cases for Relations updates:
- Find all satisfiedBy relations with InternalPath matching path
- Find all satisfy relations with InternalPath matching path
- Find all trace relations with InternalPath matching path
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
