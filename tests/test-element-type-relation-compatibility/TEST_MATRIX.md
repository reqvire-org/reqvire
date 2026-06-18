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
| requirement | capability | ✅ PASS | valid-cases/derivedfrom-req-to-user-req |
| capability | requirement | ✅ PASS | valid-cases/derivedfrom-user-req-to-req |
| capability | capability | ✅ PASS | valid-cases/derivedfrom-user-req-to-user-req |
| test-verification | requirement | ❌ FAIL | invalid-derivedfrom/verification-to-req |
| test-verification | capability | ❌ FAIL | invalid-derivedfrom/verification-to-user-req |
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
| capability | InternalPath (file) | ❌ FAIL | invalid-satisfiedby/user-req-to-file |
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
| capability | verification | ✅ PASS | valid-cases/verifiedby-capability-to-verif |
| test-verification | test-verification | ❌ FAIL | invalid-verifiedby/verif-to-verif |
| test-verification | requirement | ❌ FAIL | invalid-verifiedby/verif-to-req |
| other | test-verification | ❌ FAIL | invalid-verifiedby/other-to-verif |

## verify Relation Tests

| Source Type | Target Type | Expected | Test Case |
|-------------|-------------|----------|-----------|
| test-verification | requirement | ✅ PASS | valid-cases/verify-test-verif-to-req |
| analysis-verification | requirement | ✅ PASS | valid-cases/verify-analysis-to-req |
| analysis-verification | capability | ✅ PASS | valid-cases/verify-analysis-to-capability |
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

## Attachment Author Tests

| Source Type | Target Type | Expected | Test Case |
|-------------|-------------|----------|-----------|
| refinement | requirement-owned refinement | ❌ FAIL | invalid-refinement-attachment |
| test-verification | requirement-owned refinement | ❌ FAIL | invalid-verification-attachment |

## Ordinary Refinement Type Tests (Only define Allowed)

### Valid Refinement Relations

| Element Type | Relation | Target Type | Expected | Test Case |
|--------------|----------|-------------|----------|-----------|
| source | define | requirement | ✅ PASS | valid-refinement/source-define-requirement |
| ontology | derivedFrom | ontology | ✅ PASS | valid-hierarchy/ontology-derived-from-ontology |
| constraint | define | requirement | ✅ PASS | valid-refinement/constraint-define-requirement |
| behavior | define | requirement | ✅ PASS | valid-refinement/behavior-define-requirement |
| specification | define | requirement | ✅ PASS | valid-refinement/specification-define-requirement |
| state | define | requirement | ✅ PASS | valid-refinement/state-define-requirement |
| input-output | define | requirement | ✅ PASS | valid-refinement/input-output-define-requirement |

## Semantic Contract Relation Tests

| Source Type | Relation | Target Type | Expected | Test Case |
|-------------|----------|-------------|----------|-----------|
| requirement | constrainedBy | semantic-contract | ✅ PASS | valid-cases/requirement-constrainedby-semantic-contract |
| semantic-contract | constrain | requirement | ✅ PASS | valid-cases/semantic-contract-constrain-requirement |
| semantic-contract | use | ontology | ✅ PASS | valid-cases/semantic-contract-use-ontology |
| ontology | usedBy | semantic-contract | ✅ PASS | valid-cases/ontology-usedby-semantic-contract |
| semantic-contract | define | requirement | ❌ FAIL | invalid-refinement/semantic-contract-define-requirement |
| requirement | definedBy | semantic-contract | ❌ FAIL | invalid-refinement/requirement-refinedby-semantic-contract |

### Invalid Refinement Relations

| Element Type | Any Relation | Expected | Test Case |
|--------------|--------------|----------|-----------|
| constraint | derivedFrom | ❌ FAIL | invalid-refinement/constraint-derivedfrom |
| constraint | trace | ❌ FAIL | invalid-refinement/constraint-trace |
| behavior | derivedFrom | ❌ FAIL | invalid-refinement/behavior-derivedfrom |
| behavior | trace | ❌ FAIL | invalid-refinement/behavior-trace |
| specification | derivedFrom | ❌ FAIL | invalid-refinement/specification-derivedfrom |
| specification | trace | ❌ FAIL | invalid-refinement/specification-trace |
| source | define capability | ❌ FAIL | invalid-capability-refinements/source-define-capability |
| semantic-contract | define requirement | ❌ FAIL | invalid-refinement/semantic-contract-define-requirement |
| constraint | define capability | ❌ FAIL | invalid-capability-refinements/constraint-define-capability |
| behavior | define capability | ❌ FAIL | invalid-capability-refinements/behavior-define-capability |
| specification | define capability | ❌ FAIL | invalid-capability-refinements/specification-define-capability |
| state | define capability | ❌ FAIL | invalid-capability-refinements/state-define-capability |
| input-output | define capability | ❌ FAIL | invalid-capability-refinements/input-output-define-capability |

## Summary

| Category | Total Tests | Valid (PASS) | Invalid (FAIL) |
|----------|-------------|--------------|----------------|
| derivedFrom | 12 | 4 | 8 |
| satisfiedBy | 12 | 5 | 7 |
| verifiedBy | 6 | 3 | 3 |
| verify | 4 | 2 | 2 |
| trace | 9 | 6 | 3 |
| attachments | 2 | 0 | 2 |
| Refinement | 9 | 3 | 6 |
| **TOTAL** | **54** | **23** | **31** |
