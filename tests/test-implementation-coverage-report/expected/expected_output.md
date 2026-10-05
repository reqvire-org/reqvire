### Requirement Implementation Coverage

- **Total Requirements in Scope:** 8
- **Covered Requirements:** 6
- **Uncovered Requirements:** 2
- **Total Terminal Requirements:** 4
- **Covered Terminal Requirements:** 3 (75.0%)
- **Uncovered Terminal Requirements:** 1

#### Coverage Sources

- direct_satisfied: 3
- requirement_rollup: 2
- contract_consumer_rollup: 1
- combined_rollup: 0

## Covered Requirements

### [specifications/Requirements.md](specifications/Requirements.md)

- ✅ **[Contract Consumer Implemented](specifications/Requirements.md#contract-consumer-implemented)** (direct_satisfied)
  - Direct evidence:
    - [specifications/src/contract_consumer.rs](specifications/src/contract_consumer.rs)
  - Supporting evidence:
    - [specifications/src/contract_consumer.rs](specifications/src/contract_consumer.rs)
- ✅ **[Contract Owner](specifications/Requirements.md#contract-owner)** (contract_consumer_rollup)
  - Supporting evidence:
    - [specifications/src/contract_consumer.rs](specifications/src/contract_consumer.rs)
  - Contributing requirements:
    - [specifications/Requirements.md#contract-consumer-implemented](specifications/Requirements.md#contract-consumer-implemented)
- ✅ **[Derived Child Implemented](specifications/Requirements.md#derived-child-implemented)** (direct_satisfied)
  - Direct evidence:
    - [specifications/src/derived_child.rs](specifications/src/derived_child.rs)
  - Supporting evidence:
    - [specifications/src/derived_child.rs](specifications/src/derived_child.rs)
- ✅ **[Derived Intermediate](specifications/Requirements.md#derived-intermediate)** (requirement_rollup)
  - Supporting evidence:
    - [specifications/src/derived_child.rs](specifications/src/derived_child.rs)
  - Contributing requirements:
    - [specifications/Requirements.md#derived-child-implemented](specifications/Requirements.md#derived-child-implemented)
- ✅ **[Derived Parent](specifications/Requirements.md#derived-parent)** (requirement_rollup)
  - Supporting evidence:
    - [specifications/src/derived_child.rs](specifications/src/derived_child.rs)
  - Contributing requirements:
    - [specifications/Requirements.md#derived-intermediate](specifications/Requirements.md#derived-intermediate)
- ✅ **[Direct Implemented](specifications/Requirements.md#direct-implemented)** (direct_satisfied)
  - Direct evidence:
    - [specifications/src/direct.rs](specifications/src/direct.rs)
  - Supporting evidence:
    - [specifications/src/direct.rs](specifications/src/direct.rs)

## Uncovered Requirements

### [specifications/Requirements.md](specifications/Requirements.md)

- ❌ **[Root Requirement](specifications/Requirements.md#root-requirement)** (uncovered)
  - Supporting evidence:
    - [specifications/src/contract_consumer.rs](specifications/src/contract_consumer.rs)
    - [specifications/src/derived_child.rs](specifications/src/derived_child.rs)
    - [specifications/src/direct.rs](specifications/src/direct.rs)
  - Contributing requirements:
    - [specifications/Requirements.md#contract-consumer-implemented](specifications/Requirements.md#contract-consumer-implemented)
    - [specifications/Requirements.md#contract-owner](specifications/Requirements.md#contract-owner)
    - [specifications/Requirements.md#derived-parent](specifications/Requirements.md#derived-parent)
    - [specifications/Requirements.md#direct-implemented](specifications/Requirements.md#direct-implemented)
    - [specifications/Requirements.md#uncovered-requirement](specifications/Requirements.md#uncovered-requirement)
  - Blocking requirements:
    - [specifications/Requirements.md#uncovered-requirement](specifications/Requirements.md#uncovered-requirement)
- ❌ **[Uncovered Requirement](specifications/Requirements.md#uncovered-requirement)** (uncovered)
  - Blocking requirements:
    - [specifications/Requirements.md#uncovered-requirement](specifications/Requirements.md#uncovered-requirement)

## Capability Coverage

- **[Test Capability Test Implementation Coverage Report Specifications Requirements Md](specifications/Requirements.md#test-capability-test-implementation-coverage-report-specifications-requirements-md)**: partial verification 0.0% (0/5 leaf), implementation 75.0% (3/4 terminal requirements), implementation incomplete
