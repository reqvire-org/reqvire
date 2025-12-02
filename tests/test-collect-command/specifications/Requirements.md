# Elements

### Root Requirement

The root requirement for testing content collection.

#### Details
This is the top-level requirement that has no derivedFrom relations.

#### Metadata
  * type: user-requirement
---

### Mid-Level Requirement

The mid-level requirement derives from the root.

#### Details
This requirement sits in the middle of the hierarchy.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Leaf Requirement

The leaf requirement at the bottom of the hierarchy.

#### Details
This is the leaf requirement that derives from the mid-level.

#### Attachments
  * [DesignDoc.md](DesignDoc.md)

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Mid-Level Requirement](#mid-level-requirement)
---

### Test Verification

A verification element to test error handling.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Leaf Requirement](#leaf-requirement)
---
