# Elements


### Test Capability Test Lint Command Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Root Requirement

This is the root requirement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-lint-command-specifications-requirements-md)
---

### Parent Requirement

This is a parent requirement derived from root.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #root-requirement

---

### Leaf Requirement

This is a leaf requirement derived from parent.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #parent-requirement
  * verifiedBy: Verifications/Tests.md#test-verification

---

### Alternative Parent Requirement

This is an alternative parent requirement also derived from root.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #root-requirement

---

### Single-Chain Redundant Requirement

This requirement has derivedFrom to both parent and root through a SINGLE chain (safe to auto-remove the root relation).

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #parent-requirement
  * derivedFrom: #root-requirement

---

### Multi-Path Redundant Requirement

This requirement reaches root through MULTIPLE convergent paths (via parent-requirement and alternative-parent-requirement). Since root is reachable through other paths, the direct root relation is redundant and safe to auto-remove.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #parent-requirement
  * derivedFrom: #alternative-parent-requirement
  * derivedFrom: #root-requirement

---

### Standalone Submodel Root

This is an independent root for cross-submodel boundary checks.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-lint-command-specifications-requirements-md)
---

### Standalone Child

This requirement is owned by `Standalone Submodel Root`.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #standalone-submodel-root

---

### Cross-Submodel Linked Requirement

This requirement should only be modeled via contract_bindings, not by hierarchical relation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #parent-requirement
  * derivedFrom: #standalone-child
