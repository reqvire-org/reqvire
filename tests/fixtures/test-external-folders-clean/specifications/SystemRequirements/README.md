# Test for External Folders Feature

This test verifies that ReqFlow can process requirements from external folders specified in the configuration.

## Structure

- `specifications/` - Main specifications folder
  - `UserRequirements.md` - Contains user requirements
  - `SystemRequirements/Requirements.md` - Contains system requirements
- `external-project/` - External folder
  - `SystemRequirements/Requirements.md` - Contains system requirements that reference the main system requirements

## Test Case

To run this test, use the following command from the ReqFlow root directory:

```
cargo run -- test-fixtures/test-external-folders/ test-output/test-external-folders/ --config test-fixtures/test-external-folders/reqflow.yaml
```

## Expected Results

1. ReqFlow should process requirements from both the main specifications folder and the external folder
2. It should validate relations between requirements across both folders
3. Generated diagrams should show connections between requirements in both folders
4. No validation errors for the external folder structure should occur