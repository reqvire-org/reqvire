# Elements


### Test Capability Test Element Type Relation Compatibility Valid Contract Satisfy

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Target Capability

A capability that contracts can define.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-valid-contract-satisfy)
  * definedBy: [Specification with Satisfy](#specification-with-satisfy)

---

### Target System Requirement

A system requirement that contracts can define.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Capability](#target-capability)
  * definedBy: [Constraint with Satisfy](#constraint-with-satisfy)
  * definedBy: [Behavior with Satisfy](#behavior-with-satisfy)

---

### Constraint with Satisfy

VALID: Constraint (contract type) can define requirement via definedBy.

#### Metadata
  * type: constraint

---

### Behavior with Satisfy

VALID: Behavior (contract type) can define requirement via definedBy.

#### Metadata
  * type: behavior

---

### Specification with Satisfy

VALID: Specification (contract type) can define requirement via definedBy.

#### Metadata
  * type: specification

---
