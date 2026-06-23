# Elements



### Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---
### Test Capability Test Element Type Relation Compatibility Invalid Verification ContractBindingEntry

Test capability root for verification contract_bindings validation fixtures.

#### Metadata
  * type: capability
---

### Requirement with Verification ContractBindingEntry Target

Requirement owning a contract that a verification must not reuse.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-invalid-verification-contract-bindings)
  * definedBy: [Reusable Verification Criteria](#reusable-verification-criteria)
  * verifiedBy: [Verification With ContractBindingEntry](#verification-with-contract_bindings)
---

### Reusable Verification Criteria

Requirement-owned specification used to prove non-requirement contract_bindings authors are rejected.

#### Metadata
  * type: specification
---

### Verification With ContractBindingEntry

A verification element incorrectly authoring a contract binding. Verification evidence belongs in `satisfiedBy`, and verified targets belong in `verify`.

#### Metadata
  * type: test-verification

#### Contract Bindings
  * [Reusable Verification Criteria](#reusable-verification-criteria)

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Requirement with Verification ContractBindingEntry Target](#requirement-with-verification-contract_bindings-target)
  * satisfiedBy: [test.sh](test.sh)
---
