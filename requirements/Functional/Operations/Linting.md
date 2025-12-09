# Elements

### Model Linting

The system shall provide model linting capabilities to analyze model quality and detect issues in requirements relations.

#### Details
When linting the model, the system shall identify issues with model relations that may need attention.

When categorizing lint issues, the system shall distinguish between issues that can be automatically fixed and those requiring human judgment.

When reporting lint findings, the system shall provide clear categorization to help users understand what actions are needed.

When running the lint command, the system shall support both reporting mode and automatic fixing mode.

When filtering lint results, the system shall allow focusing on specific categories of issues.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Lint Auto-fix Capability](#lint-auto-fix-capability)
  * derive: [Multi-Branch Convergence Detection](#multi-branch-convergence-detection)
  * derive: [Redundant Hierarchical Attachment Detection](#redundant-hierarchical-attachment-detection)
  * derive: [Redundant Hierarchical Relations Detection and Auto-Removal](#redundant-hierarchical-relations-detection-and-auto-removal)
  * derive: [Redundant Verify Relations Detection](#redundant-verify-relations-detection)
  * derivedFrom: [Linting Model Quality](../../UserStories.md#linting-model-quality)
  * satisfiedBy: [Lint Output Specification](Specifications.md#lint-output-specification)
---

### Lint Auto-fix Capability

The system shall provide automatic fixing capability for auto-fixable lint issues, applying changes directly to model files when the `--fix` flag is used.

#### Details
Auto-fix shall:
- Only apply fixes for issues categorized as auto-fixable
- Modify the affected markdown files directly
- Remove redundant verify relations from verification elements
- Remove redundant hierarchical attachments from child elements
- Preserve all other content and formatting in the files
- Report all changes made (files modified, relations removed)
- Skip issues categorized as needing manual review

#### Metadata
  * type: requirement

#### Attachments
  * [Diff Output Format Specification](../Output/Specifications.md#diff-output-format-specification)

#### Relations
  * derivedFrom: [Model Linting](#model-linting)
  * verifiedBy: [Lint Command Verification](Verifications/LintingVerifications.md#lint-command-verification)
---

### Multi-Branch Convergence Detection

The system shall detect when an element reaches a common ancestor through multiple distinct branch paths without a direct relation, reporting these cases as needing manual review to determine if both branches are semantically necessary or if one represents a modeling error.

#### Details
A multi-branch convergence occurs when:
- An element reaches a common ancestor through two or more distinct derivedFrom branch paths
- There is NO direct derivedFrom relation from the element to the ancestor
- Each branch represents a potentially different semantic relationship
- The convergence may be intentional (element truly derives from ancestor through multiple contexts) OR may represent redundant modeling

**Key Distinction from Redundant Hierarchical Relations:**
- **Redundant Hierarchical Relations**: Element has a DIRECT relation to ancestor PLUS alternate paths → auto-fixable (remove direct relation)
- **Multi-Branch Convergence**: Element reaches ancestor through MULTIPLE branches with NO direct relation → needs manual review (determine if branches are semantically distinct)

**Example:**
```
Authorization (root)
  → Management API
    → API Specification
  → Public API
    → API Specification
```
API Specification reaches Authorization through two branches (Management API and Public API). Both branches might be semantically valid (spec derives from auth in context of both APIs), OR one might be a modeling error that should be removed.

Detection shall:
- Use the trace tree building logic to identify elements that reach common ancestors through multiple distinct branch paths
- Exclude cases where a direct relation exists (those are handled by Redundant Hierarchical Relations Detection)
- Report the element, the common ancestor, and all distinct branch paths
- Categorize as **needs manual review** since determining semantic necessity requires human judgment
- Explain that the user must decide whether all branches represent valid semantic relationships or if one is redundant

This enables the model author to review and decide:
- Are both branches semantically necessary? (keep both)
- Is one branch a modeling error? (remove that branch's intermediate relations)
- Should there be a direct relation instead? (restructure the model)

#### Metadata
  * type: requirement

#### Attachments
  * [Verification Trace Tree Construction](../Processing/Specifications.md#verification-trace-tree-construction)

#### Relations
  * derivedFrom: [Model Linting](#model-linting)
  * satisfiedBy: [lint.rs](../../../core/src/lint.rs)
  * verifiedBy: [Lint Command Verification](Verifications/LintingVerifications.md#lint-command-verification)
---

### Redundant Hierarchical Attachment Detection

The system shall detect attachments that are duplicated between child elements and their ancestors in the derivedFrom hierarchy, flagging the child's attachment as redundant and auto-fixable.

#### Details
An attachment is redundant when:
- A child element has an attachment (file or refinement element)
- An ancestor element (via derivedFrom chain) has the same attachment
- The attachment is already accessible through the ancestor

Detection shall:
- Traverse the derivedFrom hierarchy for each element with attachments
- Check if any ancestor has the same attachment
- Report the first (topmost) ancestor with the attachment
- Categorize as **auto-fixable** since the child's duplicate adds no value

Auto-fix behavior:
- Remove the redundant attachment from the child element
- Preserve the attachment on the ancestor (topmost stays)
- Report all removals for transparency

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Linting](#model-linting)
  * satisfiedBy: [lint.rs](../../../core/src/lint.rs)
  * verifiedBy: [Redundant Hierarchical Attachment Test](Verifications/LintingVerifications.md#redundant-hierarchical-attachment-test)
---

### Redundant Hierarchical Relations Detection and Auto-Removal

The system shall detect and auto-remove redundant derivedFrom relations where an element has direct derivedFrom relations to both a requirement and its ancestor in the requirement hierarchy.

#### Metadata
  * type: requirement

#### Attachments
  * [Verification Trace Tree Construction](../Processing/Specifications.md#verification-trace-tree-construction)

#### Relations
  * derivedFrom: [Model Linting](#model-linting)
  * satisfiedBy: [lint.rs](../../../core/src/lint.rs)
  * satisfiedBy: [Redundant Hierarchical Relations Specification](Specifications.md#redundant-hierarchical-relations-specification)
  * verifiedBy: [Lint Command Verification](Verifications/LintingVerifications.md#lint-command-verification)
---

### Redundant Verify Relations Detection

The system shall detect redundant verify relations where a verification directly verifies both a child requirement and its ancestor, leveraging the existing verification trace tree logic from the Verification Trace Builder.

#### Details
A verify relation is redundant when:
- A verification directly verifies both a leaf requirement AND its parent/ancestor in the hierarchy
- The verification trace tree shows that an ancestor requirement is also directly verified
- Since verification traces roll up automatically through derivedFrom relations, verifying the leaf is sufficient

Detection shall:
- Reuse the trace tree building logic from [Verification Trace Builder](../Processing/VerificationTraces.md#verification-trace-builder)
- Identify ancestor requirements in each verification's trace tree that are also directly verified
- Report these as redundant relations that add noise to the model
- Categorize as **auto-fixable** since removing them is safe and mechanical

#### Metadata
  * type: requirement

#### Attachments
  * [Verification Trace Tree Construction](../Processing/Specifications.md#verification-trace-tree-construction)

#### Relations
  * derivedFrom: [Model Linting](#model-linting)
  * verifiedBy: [Lint Command Verification](Verifications/LintingVerifications.md#lint-command-verification)
---
