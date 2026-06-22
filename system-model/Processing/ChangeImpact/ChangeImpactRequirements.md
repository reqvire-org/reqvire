# Elements

### Change Impact Detection

The system shall detect, analyze, and report changes to model elements between versions by comparing element content hashes, tracking relocations, and propagating impact through relationships.

#### Details
Change impact detection encompasses:
1. **Detection**: Identify what changed (content, additions, removals, relocations)
2. **Propagation**: Determine how changes flow through model relationships
3. **Reporting**: Present change analysis results to users
4. **Impact Scope**: Compute the minimal set of common parent requirements covering all impacted elements

When generating a change impact report, the system shall compute and display the per-branch lowest common ancestors of all impacted, added, and deleted requirements through the derivedFrom hierarchy, providing reviewers with a summary of affected model areas.

**Contract Element Changes:**
- Contract element content changes (hash changes) shall propagate change impact through the model via their reused_contract_context relationships
- Reused Contract Context identifier location changes (moved/renamed Contract elements) shall be reported but do NOT propagate impact (same behavior as relation relocations)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Impact Scope Computation Specification](Specifications.md#impact-scope-computation-specification)
  * derive: [Requirements Change Propagation](#requirements-change-propagation)
  * derive: [Structural Change Analyzer](#structural-change-analyzer)
  * derivedFrom: [Tracing Structural Changes](../../Reports/ModelReports/ReportingRequirements.md#tracing-structural-changes)
  * satisfiedBy: [change_impact.rs](../../../crates/reqvire-core/src/change_impact.rs)
  * verifiedBy: [Reused Contract Context Change Impact Verification](../../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#reused-contract-context-change-impact-verification)
  * verifiedBy: [Impact Scope Summary Test](../../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#impact-scope-summary-test)
---

### Requirements Change Propagation

When a requirement is changed, the system shall propagate the change through related requirements, verification artifacts, and design elements according to relation type definitions and propagation rules.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [ChangePropagation](ChangePropagation.md#changepropagation)
  * derivedFrom: [Change Impact Detection](#change-impact-detection)
  * satisfiedBy: [change_impact.rs](../../../crates/reqvire-core/src/change_impact.rs)
  * verifiedBy: [Change Impact Detection Test](../../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#change-impact-detection-test)
  * verifiedBy: [Change Impact Relations Test](../../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#change-impact-relations-test)
  * verifiedBy: [Change Impact Smart Filtering Test](../../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#change-impact-smart-filtering-test)
  * verifiedBy: [Element Content Extraction Test](../../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#element-content-extraction-test)
---

### Structural Change Analyzer

The system shall implement a model change analyzer that identifies structural modifications between model versions, determines affected elements through relationship traversal, and categorizes impacts according to change propagation rules.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Change Impact Detection](#change-impact-detection)
  * satisfiedBy: [change_impact.rs](../../../crates/reqvire-core/src/change_impact.rs)
---

