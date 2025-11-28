# Elements

### Change Impact Detection

The system shall detect, analyze, and report changes to model elements between versions by comparing element content hashes, tracking relocations, and propagating impact through relationships.

#### Details
Change impact detection encompasses:
1. **Detection**: Identify what changed (content, additions, removals, relocations)
2. **Propagation**: Determine how changes flow through model relationships
3. **Reporting**: Present change analysis results to users

**Refinement Element Changes:**
- Refinement element content changes (hash changes) shall propagate change impact through the model via their attachment relationships
- Attachment identifier location changes (moved/renamed Refinement elements) shall be reported but do NOT propagate impact (same behavior as relation relocations)

#### Relations
  * derive: [Requirements Change Propagation](#requirements-change-propagation)
  * derive: [Structural Change Analyzer](#structural-change-analyzer)
  * derivedFrom: [Element Identity Model](../Core/StructureAndParsing.md#element-identity-model)
  * derivedFrom: [Tracing Structural Changes](../Output/Reporting.md#tracing-structural-changes)
  * satisfiedBy: [change_impact.rs](../../../core/src/change_impact.rs)
  * verifiedBy: [Attachments Change Impact Verification](../Core/Verifications/AttachmentsVerifications.md#attachments-change-impact-verification)
---

### Requirements Change Propagation

When a requirement is changed, the system shall propagate the change through related requirements, verification artifacts, and design elements according to relation type definitions and propagation rules.

#### Attachments
  * [ChangePropagation.md](DesignDocuments/ChangePropagation.md)

#### Relations
  * derivedFrom: [Change Impact Detection](#change-impact-detection)
  * satisfiedBy: [change_impact.rs](../../../core/src/change_impact.rs)
  * verifiedBy: [Change Impact Detection Test](Verifications/ChangeImpactVerifications.md#change-impact-detection-test)
  * verifiedBy: [Change Impact Relations Test](Verifications/ChangeImpactVerifications.md#change-impact-relations-test)
  * verifiedBy: [Change Impact Smart Filtering Test](Verifications/ChangeImpactVerifications.md#change-impact-smart-filtering-test)
  * verifiedBy: [Element Content Extraction Test](Verifications/ChangeImpactVerifications.md#element-content-extraction-test)
---

### Structural Change Analyzer

The system shall implement a model change analyzer that identifies structural modifications between model versions, determines affected elements through relationship traversal, and categorizes impacts according to change propagation rules.

#### Relations
  * derivedFrom: [Change Impact Detection](#change-impact-detection)
  * satisfiedBy: [change_impact.rs](../../../core/src/change_impact.rs)
---
