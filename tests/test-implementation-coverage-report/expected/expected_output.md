### Requirement Implementation Coverage

- **Total Requirements in Scope:** 8
- **Covered Requirements:** 5 (62.5%)
- **Uncovered Requirements:** 3

#### Coverage Sources

- direct_satisfied: 3
- contract_satisfied_via_contract_bindings: 1
- contract_satisfied_via_child: 1

## Covered Requirements

### [specifications/Requirements.md](specifications/Requirements.md)

- ✅ **[Contract Consumer Implemented](specifications/Requirements.md#contract-consumer-implemented)** (direct_satisfied)
  - Evidence:
    - [specifications/src/contract_consumer.rs](specifications/src/contract_consumer.rs)
- ✅ **[Contract Owner](specifications/Requirements.md#contract-owner)** (contract_satisfied_via_contract_bindings)
  - Evidence:
    - [specifications/Requirements.md#contract-consumer-implemented](specifications/Requirements.md#contract-consumer-implemented)
- ✅ **[Derived Child Implemented](specifications/Requirements.md#derived-child-implemented)** (direct_satisfied)
  - Evidence:
    - [specifications/src/derived_child.rs](specifications/src/derived_child.rs)
- ✅ **[Derived Parent](specifications/Requirements.md#derived-parent)** (contract_satisfied_via_child)
  - Evidence:
    - [specifications/Requirements.md#derived-child-implemented](specifications/Requirements.md#derived-child-implemented)
- ✅ **[Direct Implemented](specifications/Requirements.md#direct-implemented)** (direct_satisfied)
  - Evidence:
    - [specifications/src/direct.rs](specifications/src/direct.rs)

## Uncovered Requirements

### [specifications/Requirements.md](specifications/Requirements.md)

- ❌ **[Derived Intermediate](specifications/Requirements.md#derived-intermediate)**
- ❌ **[Root Requirement](specifications/Requirements.md#root-requirement)**
- ❌ **[Uncovered Requirement](specifications/Requirements.md#uncovered-requirement)**

## Capability Coverage

- **[Test Capability Test Implementation Coverage Report Specifications Requirements Md](specifications/Requirements.md#test-capability-test-implementation-coverage-report-specifications-requirements-md)**: partial verification 0.0% (0/4 leaf), implementation 62.5% (5/8 requirements)
