# Element Type Relation Compatibility Test Matrix

## Test Cases Overview

Based on the Element Type Relation Compatibility specification, the following test cases are needed:

### Legend
- ✅ PASS - Valid combination, should pass validation
- ❌ FAIL - Invalid combination, should fail validation
- N/A - Not applicable (e.g., auto-generated relations)

## derivedFrom Relation Tests

| Source Type | Target Type | Expected | Test Case |
|-------------|-------------|----------|-----------|
| requirement | requirement | ✅ PASS | valid-cases/derivedfrom-req-to-req |
| requirement | user-requirement | ✅ PASS | valid-cases/derivedfrom-req-to-user-req |
| user-requirement | requirement | ✅ PASS | valid-cases/derivedfrom-user-req-to-req |
| user-requirement | user-requirement | ✅ PASS | valid-cases/derivedfrom-user-req-to-user-req |
| test-verification | requirement | ❌ FAIL | invalid-derivedfrom/verification-to-req |
| test-verification | user-requirement | ❌ FAIL | invalid-derivedfrom/verification-to-user-req |
| analysis-verification | requirement | ❌ FAIL | invalid-derivedfrom/analysis-to-req |
| requirement | test-verification | ❌ FAIL | invalid-derivedfrom/req-to-verification |
| other | requirement | ❌ FAIL | invalid-derivedfrom/other-to-req |
| constraint | requirement | ❌ FAIL | invalid-refinement/constraint-derivedfrom |
| behavior | requirement | ❌ FAIL | invalid-refinement/behavior-derivedfrom |
| specification | requirement | ❌ FAIL | invalid-refinement/specification-derivedfrom |

## satisfiedBy Relation Tests

| Source Type | Target Type | Expected | Test Case |
|-------------|-------------|----------|-----------|
| requirement | InternalPath (file) | ✅ PASS | valid-cases/satisfiedby-req-to-file |
| user-requirement | InternalPath (file) | ✅ PASS | valid-cases/satisfiedby-user-req-to-file |
| test-verification | InternalPath (file) | ✅ PASS | valid-cases/satisfiedby-test-verif-to-file |
| requirement | behavior | ✅ PASS | valid-cases/satisfiedby-req-to-behavior |
| requirement | specification | ✅ PASS | valid-cases/satisfiedby-req-to-specification |
| requirement | constraint | ✅ PASS | valid-cases/satisfiedby-req-to-constraint |
| analysis-verification | InternalPath (file) | ❌ FAIL | invalid-satisfiedby/analysis-to-file |
| inspection-verification | InternalPath (file) | ❌ FAIL | invalid-satisfiedby/inspection-to-file |
| demonstration-verification | InternalPath (file) | ❌ FAIL | invalid-satisfiedby/demonstration-to-file |
| requirement | requirement | ❌ FAIL | invalid-satisfiedby/req-to-req |
| test-verification | test-verification | ❌ FAIL | invalid-satisfiedby/verif-to-verif |
| other | InternalPath (file) | ❌ FAIL | invalid-satisfiedby/other-to-file |

## verifiedBy Relation Tests

| Source Type | Target Type | Expected | Test Case |
|-------------|-------------|----------|-----------|
| requirement | test-verification | ✅ PASS | valid-cases/verifiedby-req-to-test-verif |
| requirement | analysis-verification | ✅ PASS | valid-cases/verifiedby-req-to-analysis |
| user-requirement | test-verification | ✅ PASS | valid-cases/verifiedby-user-req-to-verif |
| test-verification | test-verification | ❌ FAIL | invalid-verifiedby/verif-to-verif |
| test-verification | requirement | ❌ FAIL | invalid-verifiedby/verif-to-req |
| other | test-verification | ❌ FAIL | invalid-verifiedby/other-to-verif |

## verify Relation Tests

| Source Type | Target Type | Expected | Test Case |
|-------------|-------------|----------|-----------|
| test-verification | requirement | ✅ PASS | valid-cases/verify-test-verif-to-req |
| analysis-verification | requirement | ✅ PASS | valid-cases/verify-analysis-to-req |
| test-verification | test-verification | ❌ FAIL | (covered by verifiedBy inverse) |
| requirement | requirement | ❌ FAIL | invalid-verifiedby/req-using-verify |

## trace Relation Tests

| Source Type | Target Type | Expected | Test Case |
|-------------|-------------|----------|-----------|
| requirement | requirement | ✅ PASS | valid-trace/req-to-req |
| requirement | test-verification | ✅ PASS | valid-trace/req-to-verif |
| test-verification | requirement | ✅ PASS | valid-trace/verif-to-req |
| test-verification | test-verification | ✅ PASS | valid-trace/verif-to-verif |
| other | requirement | ✅ PASS | valid-trace/other-to-req |
| other | other | ✅ PASS | valid-trace/other-to-other |
| constraint | requirement | ❌ FAIL | invalid-refinement/constraint-trace |
| behavior | requirement | ❌ FAIL | invalid-refinement/behavior-trace |
| specification | requirement | ❌ FAIL | invalid-refinement/specification-trace |

## Refinement Type Tests (Only satisfy Allowed)

### Valid Refinement Relations

| Element Type | Relation | Target Type | Expected | Test Case |
|--------------|----------|-------------|----------|-----------|
| constraint | satisfy | requirement | ✅ PASS | valid-refinement-satisfy/constraint-satisfy |
| behavior | satisfy | requirement | ✅ PASS | valid-refinement-satisfy/behavior-satisfy |
| specification | satisfy | user-requirement | ✅ PASS | valid-refinement-satisfy/specification-satisfy |

### Invalid Refinement Relations

| Element Type | Any Relation | Expected | Test Case |
|--------------|--------------|----------|-----------|
| constraint | derivedFrom | ❌ FAIL | invalid-refinement/constraint-derivedfrom |
| constraint | trace | ❌ FAIL | invalid-refinement/constraint-trace |
| behavior | derivedFrom | ❌ FAIL | invalid-refinement/behavior-derivedfrom |
| behavior | trace | ❌ FAIL | invalid-refinement/behavior-trace |
| specification | derivedFrom | ❌ FAIL | invalid-refinement/specification-derivedfrom |
| specification | trace | ❌ FAIL | invalid-refinement/specification-trace |

## Summary

| Category | Total Tests | Valid (PASS) | Invalid (FAIL) |
|----------|-------------|--------------|----------------|
| derivedFrom | 12 | 4 | 8 |
| satisfiedBy | 12 | 6 | 6 |
| verifiedBy | 6 | 3 | 3 |
| verify | 4 | 2 | 2 |
| trace | 9 | 6 | 3 |
| Refinement | 9 | 3 | 6 |
| **TOTAL** | **52** | **24** | **28** |
