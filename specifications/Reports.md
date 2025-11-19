# Provide Reports

## Requirements

### Model Reports

When requested the system shall provide human readable MBSE model reports.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Provide Reports](UserStories.md#provide-reports)
---

### Model Structure and Summaries

When requested the system shall generate reports summarizing the structure and relationships in the MBSE model, including counts and types of connections also supporting json and cypher output.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Deterministic Output for All Generated Content](ReqvireTool/ValidationAndReporting/Reports.md#deterministic-output-for-all-generated-content)
---

### Structural Change Reports

The system shall generate detailed reports summarizing the impact of structural changes, including affected relationships and components.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)
---

### Provide Validation Reports

The system shall generate detailed validation reports, highlighting any inconsistencies or errors in the MBSE model structure.

#### Details
Validation shall be performed automatically when any command requires the parsed model, eliminating the need for a separate validation command. Commands that operate on raw files shall skip validation to allow operation on potentially invalid documents.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Align with Industry Standards](Mission.md#align-with-industry-standards)
  * derivedFrom: [Deterministic Output for All Generated Content](ReqvireTool/ValidationAndReporting/Reports.md#deterministic-output-for-all-generated-content)
---

### Verification Coverage Report

The system shall generate verification coverage reports focusing on leaf requirements (requirements that do not have forward relations to any other requirement), showing the percentage and details of verified and unverified leaf requirements, including breakdowns by file, section, and verification type.

#### Details
The verification coverage report shall provide:
- Total count of leaf requirements with breakdown by requirement type
- Count and percentage of verified leaf requirements (those with verifiedBy relations pointing to existing verification artifacts)
- Count and percentage of unverified leaf requirements
- Total count of verification artifacts with breakdown by verification type (test, analysis, inspection, demonstration)
- Count and percentage of satisfied test-verification artifacts (those with satisfiedBy relations pointing to existing test implementations)
- Count and percentage of orphaned verification artifacts (those without any verify relations to requirements)
- Detailed list of verified leaf requirements grouped by file and section
- Detailed list of unverified leaf requirements with impact analysis
- Detailed list of orphaned verifications (flagged for attention as they may be redundant or incorrectly configured)
- Output in both human-readable text and machine-readable JSON formats

The report helps track verification completeness and identify gaps in requirement verification coverage, supporting quality assurance and compliance activities.

**Coverage Philosophy:**
- **Leaf requirements** (requirements that don't derive other requirements) MUST be verified
- **Parent/intermediate requirements** MAY be verified but it's not a hard requirement as they might be covered in verification of leaf requirements
- One verification may verify multiple leaf requirements (N:1 relationship)
- The change impact analysis system propagates changes from parent requirements down to leaf requirements and their verifications
- System engineers/architects are responsible for ensuring verification scopes are broad enough to cover parent requirements when there's no dedicated parent verification
- AI systems can help create comprehensive verification scopes and prevent verification overlap

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Deterministic Output for All Generated Content](ReqvireTool/ValidationAndReporting/Reports.md#deterministic-output-for-all-generated-content)
---

### Change Impact Analysis

When requested the system shall generate change impact report, in Markdown format by default and also supporting json output.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Requirements Change Propagation](TraceChanges.md#requirements-change-propagation)
  * derivedFrom: [Deterministic Output for All Generated Content](ReqvireTool/ValidationAndReporting/Reports.md#deterministic-output-for-all-generated-content)
---

### Verification Upward Traceability

The system shall provide upward traceability visualization from verifications to root requirements, showing the complete requirement hierarchy and indicating which requirements are directly verified.

#### Details
**Used for identifying redundant verifications**: When a verification directly verifies both a leaf requirement and its parent requirement, this creates a redundant relation that adds noise into the model and may be removed from the parent - verifying the leaf requirement is sufficient since it traces upward to the parent. This keeps verification placement at the most specific level.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Verification Coverage Report](#verification-coverage-report)
---

