# Elements



### Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---
### Test Capability Test Element Type Relation Compatibility Invalid Verification ReusedContractContextEntry

Test capability root for verification reused_contract_context validation fixtures.

#### Metadata
  * type: capability
---

### Requirement with Verification ReusedContractContextEntry Target

Requirement owning a contract that a verification must not reuse.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-invalid-verification-reused-context)
  * definedBy: [Reusable Verification Criteria](#reusable-verification-criteria)
  * verifiedBy: [Verification With ReusedContractContextEntry](#verification-with-reused_contract_context)
---

### Reusable Verification Criteria

Requirement-owned specification used to prove non-requirement reused_contract_context authors are rejected.

#### Metadata
  * type: specification
---

### Verification With ReusedContractContextEntry

A verification element incorrectly authoring an reused_contract_context. Verification evidence belongs in `satisfiedBy`, and verified targets belong in `verify`.

#### Metadata
  * type: test-verification

#### Reused Contract Context
  * [Reusable Verification Criteria](#reusable-verification-criteria)

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Requirement with Verification ReusedContractContextEntry Target](#requirement-with-verification-reused_contract_context-target)
  * satisfiedBy: [test.sh](test.sh)
---
