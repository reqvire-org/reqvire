# Elements

### Implementation Traceability Verification Objective

This objective groups verification of requirement implementation evidence assessment, shared-contract obligations, and capability implementation rollup.

#### Metadata
  * type: verification-objective
---

### Implementation Coverage Rollup Test

This test verifies recursive implementation coverage and its evidence explanations through the shared coverage report.

#### Details
- Exercise terminal requirements with and without direct evidence, including owners of unused contracts.
- Exercise complete and incomplete multi-level requirement hierarchies without owned contracts. Direct parent evidence cannot hide a missing child.
- Exercise multiple contract consumers, consumers binding several owned contracts, and bindings on parents with complete and incomplete children. Every required consumer must be covered.
- Exercise requirements with both children and contract consumers; either kind of gap prevents coverage. Preserve direct and recursively collected evidence on uncovered parents and identify their uncovered contributions.
- Deduplicate shared descendants and consumers. Reordering declarations or relation lists must not change classifications or evidence.
- Count terminal requirements for implementation percentages while preserving classification counts for all requirements. Count childless contract owners as verification leaves even when they are nonterminal for implementation coverage.
- Check capability rollup, an empty capability, and a capability containing only a covered contract owner whose terminal consumers are external. Scope selection preserves classifications and evidence without importing external subjects into counts.
- Check per-requirement aggregate verification-leaf and implementation-terminal counts for complete and incomplete parents, shared descendants, and binding consumers outside the selected scope. Preserve aggregate metrics across scopes and update them after evidence removal.
- Compare JSON and text reports, the protocol-neutral coverage tool used by MCP, and the Explorer store projection. Check that removing implementation evidence invalidates dependent coverage on the next read.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Implementation Traceability Verification Objective](#implementation-traceability-verification-objective)
  * satisfiedBy: [implementation_coverage.rs](../../../crates/reqvire-core/tests/implementation_coverage.rs)
  * satisfiedBy: [test.sh](../../../tests/test-implementation-rollup/test.sh)
  * verify: [Implementation Coverage Evaluation](../../Implementation/Traceability/ImplementationTraceabilityRequirements.md#implementation-coverage-evaluation)
  * verify: [Requirement Implementation Coverage Report](../../Reports/ModelReports/ReportingRequirements.md#requirement-implementation-coverage-report)
---

### Implementation Coverage Semantics Analysis

This analysis verifies that the implementation coverage contract, ontology, semantic shape, and concept definitions express one consistent assessment model.

#### Details
Review the following cases against the owned logic specification and the ontology reached through the requirement's constraining semantic contract:

| Case | Expected implementation classification |
|------|----------------------------------------|
| No children or contract consumers, with direct evidence | Covered terminal requirement |
| No children or contract consumers, without direct evidence | Uncovered terminal requirement |
| Owns an unused contract | Same terminal rule; ownership alone supplies no coverage |
| All immediate children covered recursively | Covered through requirement rollup |
| One child uncovered, even with direct parent evidence | Uncovered |
| All binding requirements covered recursively | Covered through contract consumer rollup, without requiring direct owner evidence |
| One binding requirement uncovered | Uncovered, even when another consumer or the owner has direct evidence |
| Both children and consumers | Covered only when every contribution in both sets is covered |
| Binding on a parent | Parent's complete recursive coverage applies; one implemented descendant is insufficient |
| One consumer binds several owned contracts | One required consumer contribution |

Confirm that capabilities receive aggregate coverage without direct satisfaction, contracts provide neither evidence nor coverage units, and direct evidence remains visible on uncovered parents.

Confirm that terminal percentage units are distinct from all-requirement classifications. External contributors retain their coverage and evidence without entering selected scope counts, and a verification leaf may be nonterminal for implementation coverage.

Inspect the SHACL constraints for valid covered and uncovered terminal records, required nonterminal contributions, rejection of covered records with uncovered contributions, and rejection of direct capability evidence. Check that every shape term is reachable through explicit ontology use and that incomplete implementation remains a coverage gap rather than an invalid authored requirement.

Confirm that the concept bridges, semantic-contract constraint, owned specification, and consuming requirement bindings provide explicit traceability. This analysis checks the modeled semantics; executable evaluation correctness requires separate test verification.

Review artifacts: [implementation_coverage.rs](../../../crates/reqvire-core/tests/implementation_coverage.rs), [test.sh](../../../tests/test-implementation-coverage-report/test.sh).

#### Concept References
  * [Implementation Coverage](../../Thesaurus/Thesaurus.md#implementation-coverage)
  * [Terminal Requirement](../../Thesaurus/Thesaurus.md#terminal-requirement)

#### Metadata
  * type: analysis-verification

#### Relations
  * derivedFrom: [Implementation Traceability Verification Objective](#implementation-traceability-verification-objective)
  * verify: [Implementation Coverage Evaluation](../../Implementation/Traceability/ImplementationTraceabilityRequirements.md#implementation-coverage-evaluation)
---
