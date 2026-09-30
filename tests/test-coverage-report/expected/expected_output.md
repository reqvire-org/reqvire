## Summary

### Leaf Requirements

- **Total Leaf Requirements:** 5
- **Verified Leaf Requirements:** 4 (80.0%)
- **Unverified Leaf Requirements:** 1

### Test Verifications

- **Total Test Verifications:** 4
- **Satisfied Test Verifications:** 2 (50.0%)
- **Unsatisfied Test Verifications:** 2

### Orphaned Verifications

- **Total Verifications:** 7
- **Orphaned Verifications:** 3 (42.9%)

### Verification Types

- Test: 2
- Formal Proof: 2
- Analysis: 1
- Inspection: 1
- Demonstration: 1

## Verified Leaf Requirements

### [specifications/Requirements.md](specifications/Requirements.md)

- ✅ **[Another Leaf Requirement Verified](specifications/Requirements.md#another-leaf-requirement-verified)**
  - Verified by:
    - [specifications/Requirements.md#test-verification-unsatisfied](specifications/Requirements.md#test-verification-unsatisfied)
- ✅ **[Leaf Requirement Formal Proof Unsatisfied](specifications/Requirements.md#leaf-requirement-formal-proof-unsatisfied)**
  - Verified by:
    - [specifications/Requirements.md#formal-proof-verification-unsatisfied](specifications/Requirements.md#formal-proof-verification-unsatisfied)
- ✅ **[Leaf Requirement Verified](specifications/Requirements.md#leaf-requirement-verified)**
  - Verified by:
    - [specifications/Requirements.md#test-verification-satisfied](specifications/Requirements.md#test-verification-satisfied)
- ✅ **[Leaf Requirement Verified By Formal Proof](specifications/Requirements.md#leaf-requirement-verified-by-formal-proof)**
  - Verified by:
    - [specifications/Requirements.md#formal-proof-verification-satisfied](specifications/Requirements.md#formal-proof-verification-satisfied)

## Unverified Leaf Requirements

### [specifications/Requirements.md](specifications/Requirements.md)

- ❌ **[Leaf Requirement Unverified](specifications/Requirements.md#leaf-requirement-unverified)**

## Satisfied Test Verifications

### [specifications/Requirements.md](specifications/Requirements.md)

- ✅ **[Formal Proof Verification Satisfied](specifications/Requirements.md#formal-proof-verification-satisfied)** (formal-proof-verification)
  - Satisfied by:
    - [specifications/proof-satisfied.txt](specifications/proof-satisfied.txt)
- ✅ **[Test Verification Satisfied](specifications/Requirements.md#test-verification-satisfied)** (test-verification)
  - Satisfied by:
    - [specifications/test-satisfied.sh](specifications/test-satisfied.sh)

## Unsatisfied Test Verifications

### [specifications/Requirements.md](specifications/Requirements.md)

- ❌ **[Formal Proof Verification Unsatisfied](specifications/Requirements.md#formal-proof-verification-unsatisfied)** (formal-proof-verification)
- ❌ **[Test Verification Unsatisfied](specifications/Requirements.md#test-verification-unsatisfied)** (test-verification)

## Orphaned Verifications

### [specifications/Requirements.md](specifications/Requirements.md)

- ⚠️  **[Analysis Verification Test](specifications/Requirements.md#analysis-verification-test)** (analysis-verification)
- ⚠️  **[Demonstration Verification Test](specifications/Requirements.md#demonstration-verification-test)** (demonstration-verification)
- ⚠️  **[Inspection Verification Test](specifications/Requirements.md#inspection-verification-test)** (inspection-verification)

### Requirement Implementation Coverage

- **Total Requirements in Scope:** 6
- **Covered Requirements:** 0
- **Uncovered Requirements:** 6
- **Total Terminal Requirements:** 5
- **Covered Terminal Requirements:** 0 (0.0%)
- **Uncovered Terminal Requirements:** 5

#### Coverage Sources

- direct_satisfied: 0
- requirement_rollup: 0
- contract_consumer_rollup: 0
- combined_rollup: 0

## Uncovered Requirements

### [specifications/Requirements.md](specifications/Requirements.md)

- ❌ **[Another Leaf Requirement Verified](specifications/Requirements.md#another-leaf-requirement-verified)** (uncovered)
  - Blocking requirements:
    - [specifications/Requirements.md#another-leaf-requirement-verified](specifications/Requirements.md#another-leaf-requirement-verified)
- ❌ **[Leaf Requirement Formal Proof Unsatisfied](specifications/Requirements.md#leaf-requirement-formal-proof-unsatisfied)** (uncovered)
  - Blocking requirements:
    - [specifications/Requirements.md#leaf-requirement-formal-proof-unsatisfied](specifications/Requirements.md#leaf-requirement-formal-proof-unsatisfied)
- ❌ **[Leaf Requirement Unverified](specifications/Requirements.md#leaf-requirement-unverified)** (uncovered)
  - Blocking requirements:
    - [specifications/Requirements.md#leaf-requirement-unverified](specifications/Requirements.md#leaf-requirement-unverified)
- ❌ **[Leaf Requirement Verified](specifications/Requirements.md#leaf-requirement-verified)** (uncovered)
  - Blocking requirements:
    - [specifications/Requirements.md#leaf-requirement-verified](specifications/Requirements.md#leaf-requirement-verified)
- ❌ **[Leaf Requirement Verified By Formal Proof](specifications/Requirements.md#leaf-requirement-verified-by-formal-proof)** (uncovered)
  - Blocking requirements:
    - [specifications/Requirements.md#leaf-requirement-verified-by-formal-proof](specifications/Requirements.md#leaf-requirement-verified-by-formal-proof)
- ❌ **[Parent Requirement](specifications/Requirements.md#parent-requirement)** (uncovered)
  - Contributing requirements:
    - [specifications/Requirements.md#leaf-requirement-unverified](specifications/Requirements.md#leaf-requirement-unverified)
    - [specifications/Requirements.md#leaf-requirement-verified](specifications/Requirements.md#leaf-requirement-verified)
  - Blocking requirements:
    - [specifications/Requirements.md#leaf-requirement-unverified](specifications/Requirements.md#leaf-requirement-unverified)
    - [specifications/Requirements.md#leaf-requirement-verified](specifications/Requirements.md#leaf-requirement-verified)

## Capability Coverage

- **[Coverage Capability](specifications/Requirements.md#coverage-capability)**: partial verification 80.0% (4/5 leaf), implementation 0.0% (0/5 terminal requirements), implementation incomplete
