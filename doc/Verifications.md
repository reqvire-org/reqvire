# Verifications in Reqvire

## Conceptual Overview

In Reqvire, **verification** is the process of confirming that a system or its components meet specified **requirements**. Verification ensures that the system has been **built right**, checking whether it conforms to the defined functional and non-functional requirements at each level of system decomposition.

Verification activities are integrated into the **requirements structure**, maintaining clear **traceability** between requirements and their associated **verification methods** and **test artifacts**.

## Verification Element Types

Reqvire supports several verification element types that align with standard systems engineering verification methods:

### Primary Verification Types
- **verification** - Default verification through testing (equivalent to test-verification)
- **test-verification** - Formal testing with documented test procedures and expected outcomes
- **analysis-verification** - Verification through systematic analysis of documentation or code without physical testing
- **inspection-verification** - Verification through formal examination of documentation, code, or physical components
- **demonstration-verification** - Verification through showing functionality in an operational-like environment

### SatisfiedBy Requirements by Type
- **test-verification**: MUST have `satisfiedBy` relations pointing to actual test implementations
- **analysis-verification, inspection-verification, demonstration-verification**: Do NOT require `satisfiedBy` relations (considered satisfied by default)
- **trace relations**: Always allowed for any verification type

## Coverage Philosophy

### Leaf Requirements Focus
Reqvire's verification coverage focuses on **leaf requirements** - requirements that do not have forward relations to other requirements. These represent the actual testable functionality.

**Coverage Rules:**
- **Leaf requirements** MUST be verified - these represent the actual testable functionality
- **Parent/intermediate requirements** MAY be verified but it's not a hard requirement as they might be covered in verification of leaf requirements
- One verification may verify multiple leaf requirements (N:1 relationship)

### Coverage Metrics
The verification coverage system tracks:
- **Verified leaf requirements**: Leaf requirements with `verifiedBy` relations pointing to verification elements
- **Satisfied test-verifications**: Test-verification elements with `satisfiedBy` relations pointing to actual test implementations
- **Unsatisfied test-verifications**: Test-verification elements missing `satisfiedBy` relations (flagged as incomplete)
- **Coverage percentages**: Calculated separately for leaf requirements verification and test-verification satisfaction

## Two-Level Verification System

Reqvire implements a two-level verification approach:

1. **Requirements → Verifications**: Requirements link to verification elements via `verifiedBy` relations
2. **Test-Verifications → Test Implementations**: Test-verification elements link to actual test scripts/artifacts via `satisfiedBy` relations

```markdown
### My Requirement
The system shall process data within 500ms.

#### Relations
  * verifiedBy: [Performance Test](tests/PerformanceTests.md#response-time-test)

---

### Performance Test
This test verifies response time requirements.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [My Requirement](Requirements.md#my-requirement)
  * satisfiedBy: [test.sh](../../tests/test-performance/test.sh)
```

### Bidirectional Traceability
Reqvire automatically maintains bidirectional relations:
- When a requirement has `verifiedBy: [Test A]`, the Test A element shows `verify: [Requirement]`
- This ensures complete traceability in both directions

## Coverage Reporting

Use Reqvire's coverage report command to:
- Identify unverified leaf requirements
- Track test-verification satisfaction status
- Generate coverage metrics and reports
- Support quality assurance and compliance activities

```bash
# Generate coverage report
./target/debug/reqvire coverage-report

# Generate JSON coverage report for programmatic analysis
./target/debug/reqvire coverage-report --json
```

The coverage system will flag:
- ✅ **Satisfied test-verifications**: Those with valid `satisfiedBy` relations to test implementations
- ❌ **Unsatisfied test-verifications**: Those missing `satisfiedBy` relations
- ✅ **Analysis/inspection/demonstration verifications**: Considered satisfied by default (no `satisfiedBy` required)

---