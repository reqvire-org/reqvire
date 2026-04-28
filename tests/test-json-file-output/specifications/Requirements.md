# Elements

### Test Requirement Alpha

A root user requirement for JSON file output testing.

#### Metadata
  * type: user-requirement

#### Relations
  * verifiedBy: [Test Verification Alpha](#test-verification-alpha)

---

### Test Requirement Beta

A child requirement derived from alpha for collect testing.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Test Requirement Alpha](#test-requirement-alpha)
  * satisfiedBy: [asset.txt](../docs/asset.txt)

---

### Test Verification Alpha

Verifies the test requirement alpha.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Test Requirement Alpha](#test-requirement-alpha)

---

### Test Verification Beta

Another verification for merge testing.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Test Requirement Alpha](#test-requirement-alpha)

---
