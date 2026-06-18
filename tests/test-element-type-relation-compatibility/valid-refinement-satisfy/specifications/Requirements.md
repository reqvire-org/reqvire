# Elements


### Test Capability Test Element Type Relation Compatibility Valid Refinement Satisfy

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Target Capability

A capability that refinements can define.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-valid-refinement-satisfy)
  * definedBy: [Specification with Satisfy](#specification-with-satisfy)

---

### Target System Requirement

A system requirement that refinements can define.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Capability](#target-capability)
  * definedBy: [Constraint with Satisfy](#constraint-with-satisfy)
  * definedBy: [Behavior with Satisfy](#behavior-with-satisfy)

---

### Constraint with Satisfy

VALID: Constraint (refinement type) can define requirement via definedBy.

#### Metadata
  * type: constraint

---

### Behavior with Satisfy

VALID: Behavior (refinement type) can define requirement via definedBy.

#### Metadata
  * type: behavior

---

### Specification with Satisfy

VALID: Specification (refinement type) can define requirement via definedBy.

#### Metadata
  * type: specification

---
