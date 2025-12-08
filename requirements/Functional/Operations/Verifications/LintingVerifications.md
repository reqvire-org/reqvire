# Elements

### Lint Command Verification

This test verifies that the lint command analyzes model quality, detects issues in requirements relations, and provides categorized output distinguishing between auto-fixable issues and those requiring manual review.

#### Details

##### Acceptance Criteria:
**Basic Command Functionality:**
- System shall provide lint command that analyzes model quality
- System shall default to dry-run mode (report issues without applying fixes)
- System shall support --fixable flag to show only auto-fixable issues
- System shall support --auditable flag to show only issues requiring manual review
- System shall support --fix flag to automatically apply fixes for auto-fixable issues
- System shall support --json flag for structured JSON output
- System shall show ALL issues when no filter flags are provided
- System shall exit with code 0 when no issues are found or fixes are successfully applied
- System shall exit with non-zero code on errors

**Redundant Verify Relations Detection:**
- System shall detect when a verification verifies both a leaf requirement and its ancestor
- System shall reuse verification trace tree logic for detection
- System shall report these as auto-fixable issues
- System shall show affected verification elements with file paths
- System shall list specific redundant verify relations
- System shall explain why relations are redundant

**Maybe-Redundant Hierarchical Relations Detection:**
- System shall detect when an element has derivedFrom to both a requirement and its ancestor
- System shall use virtual verification connected to all leaf requirements for detection
- System shall reuse trace tree building logic for consistency
- System shall report these as needs manual review issues
- System shall show affected elements with their derivedFrom relations
- System shall not suggest which relations to remove

**Output Formatting:**
- System shall categorize output into "Auto-fixable Issues" and "Needs Manual Review" sections
- Each issue shall include element identifier, file path, specific relations, and rationale
- JSON output shall include issue categorization, type, affected elements, relation details, and rationale
- System shall not suggest what to remove in output

**Auto-fix Capability:**
- System shall apply fixes only for auto-fixable issues when --fix is used
- System shall remove redundant verify relations from verification elements
- System shall remove safe redundant hierarchical derivation relations following single-chain constraint
- System shall NOT auto-remove hierarchical relations with multiple converging paths (requires manual review)
- System shall preserve all other content and formatting in files
- System shall report all changes made (files modified, relations removed)
- System shall not modify issues categorized as needs manual review

##### Test Criteria:
1. **Command execution**
   - Lint command runs successfully with default behavior
   - --fixable flag shows only auto-fixable issues
   - --auditable flag shows only manual review issues
   - --fix flag applies fixes for auto-fixable issues
   - --json flag produces structured JSON output

2. **JSON output filtering**
   - --fixable --json shows only auto_fixable items (needs_manual_review is empty array)
   - --auditable --json shows only needs_manual_review items (auto_fixable is empty array)
   - --json without filters shows both auto_fixable and needs_manual_review items
   - JSON output correctly filters based on flags before serialization

3. **Redundant verify detection**
   - Correctly identifies redundant verify relations using trace tree logic
   - Reports verification elements with redundant relations
   - Categorizes as auto-fixable

4. **Hierarchical redundancy detection**
   - Creates virtual verification connected to leaf requirements
   - Correctly identifies potentially redundant derivedFrom relations
   - Categorizes as needs manual review

5. **Output quality**
   - Clear categorization into auto-fixable and manual review sections
   - Complete information for each issue
   - JSON output has correct structure
   - No suggestions on what to remove

6. **Auto-fix functionality**
   - Removes redundant verify relations from markdown files
   - Removes safe redundant hierarchical derivation relations (single-chain only)
   - Does NOT auto-remove hierarchical relations with multiple converging paths
   - Preserves file content and formatting
   - Reports changes accurately
   - Does not modify manual review issues

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-lint-command/test.sh)
  * verify: [Lint Auto-fix Capability](../Linting.md#lint-auto-fix-capability)
  * verify: [Model Linting](../Linting.md#model-linting)
  * verify: [Multi-Branch Convergence Detection](../Linting.md#multi-branch-convergence-detection)
  * verify: [Redundant Verify Relations Detection](../Linting.md#redundant-verify-relations-detection)
  * verify: [Safe Redundant Hierarchical Relations Auto-Removal](../Linting.md#safe-redundant-hierarchical-relations-auto-removal)
---

### Redundant Hierarchical Attachment Test

This test verifies that the lint command detects redundant attachments that exist on both child elements and their ancestors in the derivedFrom hierarchy, and can auto-fix by removing duplicates from children.

#### Details

##### Acceptance Criteria:
- System shall detect when a child element has the same attachment as an ancestor
- System shall report which attachment is redundant and on which ancestor it exists
- System shall categorize as auto-fixable
- System shall remove the redundant attachment from the child when --fix is used
- System shall preserve the attachment on the ancestor

##### Test Criteria:
1. Create parent requirement with attachment
2. Create child requirement derived from parent with same attachment
3. Run lint --fixable to detect redundant attachment
4. Verify issue is reported with correct element, attachment, and ancestor info
5. Run lint --fix to apply fix
6. Verify child's attachment is removed
7. Verify parent's attachment is preserved
8. Verify no more issues after fix

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-lint-redundant-attachments/test.sh)
  * verify: [Redundant Hierarchical Attachment Detection](../Linting.md#redundant-hierarchical-attachment-detection)
---
