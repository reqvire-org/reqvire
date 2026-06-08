# Elements


### Test Capability Test Element Type Relation Compatibility Invalid Verification Attachment

Test capability root for verification attachment validation fixtures.

#### Metadata
  * type: capability
---

### Requirement with Verification Attachment Target

Requirement owning a refinement that a verification must not attach.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-element-type-relation-compatibility-invalid-verification-attachment)
  * refinedBy: [Reusable Verification Criteria](#reusable-verification-criteria)
  * verifiedBy: [Verification With Attachment](#verification-with-attachment)
---

### Reusable Verification Criteria

Requirement-owned specification used to prove non-requirement attachment authors are rejected.

#### Metadata
  * type: specification
---

### Verification With Attachment

A verification element incorrectly authoring an attachment. Verification evidence belongs in `satisfiedBy`, and verified targets belong in `verify`.

#### Metadata
  * type: test-verification

#### Attachments
  * [Reusable Verification Criteria](#reusable-verification-criteria)

#### Relations
  * verify: [Requirement with Verification Attachment Target](#requirement-with-verification-attachment-target)
  * satisfiedBy: [test.sh](test.sh)
---
