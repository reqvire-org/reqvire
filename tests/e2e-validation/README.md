# ReqFlow Validation Tests

This directory contains end-to-end tests for ReqFlow's validation functionality.

## Test Scope

These tests verify that the validation functionality properly identifies various types of issues in the requirements model:

1. **Invalid relation types:** Typos, non-standard relation types
2. **Invalid relation formats:** Non-alphanumeric characters in relation types
3. **Missing relation targets:** Relations pointing to non-existent elements
4. **Duplicate relations:** Same relation type and target defined multiple times

## Running Tests

To run all validation tests:

```bash
./run_all_tests.sh
```

To run a specific test:

```bash
./test_invalid_relations.sh
```

## Test Fixtures

Test fixtures are located in the `/tests/fixtures/test-invalid-relations/` directory. These fixtures contain intentionally invalid relations to verify that validation correctly identifies them.

## Verification Requirements

These tests verify the following requirements:

- [Detailed Error Handling and Logging](../../specifications/SystemRequirements/Requirements.md#detailed-error-handling-and-logging): "The system shall implement detailed error handling and logging throughout the application to facilitate troubleshooting and provide meaningful feedback."