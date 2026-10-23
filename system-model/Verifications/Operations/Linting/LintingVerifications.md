# Elements

### Linting and Model Hygiene Verification Objective

This objective groups verification that Reqvire linting reports model hygiene findings and fixable issues consistently.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Lint Command Verification](#lint-command-verification)
---

### Lint Command Verification

Verify lint inspection and the shared core redundancy policy.

#### Details
**Established fixture evidence**
- Compare default, `--fixable`, `--auditable` and JSON reports against expected fixtures; category filters preserve their contracted separation.
- A verification directly targeting one descendant and its parent retains both edges. The branching verification removes its directly verified ancestor reached from multiple other direct targets, preserving those other targets.
- Redundant direct hierarchical ancestor edges are auto-fixable for both single-chain and multiple-path cases. Compare repaired source files and the post-repair report against their expected fixtures.
- Cross-submodel hierarchical relations remain manual-review findings with source/target roots and identifiers. Their guidance distinguishes reference context from implementation bindings without guessing author intent.
- A separate convergence fixture has two branches to one common ancestor and no direct edge to that ancestor. Compare its manual-review record against the expected JSON and prove repair preserves its complete source file.
- Semantic-contract errors remain validation failures rather than lint findings.

**MCP acceptance still pending**
CLI lint is inspection only and rejects `--fix`. Default MCP inspection has `fix: false`; mutation-enabled `fix: true` applies only safe findings through the complete validated, canonical transaction. Require structured repair results, source diffs, preserved manual-review findings, unchanged rejected candidates and idempotent second repair.

The current legacy CLI repair harness establishes the shared core classifications and repairs. It does not establish the removed CLI adapter boundary, MCP repair discovery or canonical transaction behavior; those executable migrations remain in the MCP lint-fix and canonical-persistence plans.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-lint-command/test.sh)
  * verify: [CLI Lint Command](../../../Interfaces/CLI/Commands.md#cli-lint-command)
  * verify: [Cross-Submodel Hierarchical Relation Detection](../../../Operations/Linting/LintingRequirements.md#cross-submodel-hierarchical-relation-detection)
  * verify: [Lint Auto-fix Capability](../../../Operations/Linting/LintingRequirements.md#lint-auto-fix-capability)
  * verify: [Multi-Branch Convergence Detection](../../../Operations/Linting/LintingRequirements.md#multi-branch-convergence-detection)
  * verify: [Redundant Hierarchical Relations Detection and Auto-Removal](../../../Operations/Linting/LintingRequirements.md#redundant-hierarchical-relations-detection-and-auto-removal)
  * verify: [Redundant Verify Relations Detection](../../../Operations/Linting/LintingRequirements.md#redundant-verify-relations-detection)
---
