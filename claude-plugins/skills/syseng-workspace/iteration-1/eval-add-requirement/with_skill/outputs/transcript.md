# Eval: add-requirement (with new skill)

## Task
"I need to add a new feature for 'Batch Export' - the system should support exporting multiple specification files at once into a single archive. Add the feature, add a requirement that specifies it, and add a test verification for that requirement. Use the MBSE workflow. Put features in requirements/Features.md and requirements in requirements/Functional/Operations/Specifications.md. Don't actually run the commands, just show me the exact reqvire commands you would run."

## Approach
- Read the skill at `/mnt/Radni/reqvire/claude-plugins/skills/syseng/SKILL.md`
- Read the AddFeature reference at `/mnt/Radni/reqvire/claude-plugins/skills/syseng/reference/AddFeature.md`
- Analyzed existing model structure by reading:
  - `requirements/Features.md` (existing feature model)
  - `requirements/Interfaces/WebInterface/Features.md` (existing export-related system requirements)
  - `requirements/Interfaces/Interfaces.md` (parent hierarchy)
  - `requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.md` (existing verification format)

**Note:** The actual project organizes features in `requirements/Features.md` and requirements by functional area.

## Commands

### Command 1 -- Add Feature

```bash
reqvire add requirements/Features.md --content '### Batch Export

As a **System Engineer**, I want to export multiple specification files at once into a single archive, so that I can efficiently share and distribute complete sets of system documentation.

#### Metadata
  * type: feature
'
```

### Command 2 -- Add Requirement

```bash
reqvire add requirements/Functional/Operations/Specifications.md --content '### Batch Specification Export

When the user requests a batch export, the system shall collect all selected specification files and produce a single archive containing the exported artifacts.

#### Details
The batch export shall:
1. Accept a list of specification file paths or a glob pattern
2. Process each specification file through the existing export pipeline
3. Bundle all exported outputs into a single archive file
4. Report any files that failed to export without aborting the entire batch

#### Metadata
  * type: requirement

#### Relations
  * specify: [Batch Export](../../Features.md#batch-export)
'
```

### Command 3 -- Add Test Verification

```bash
reqvire add requirements/Functional/Operations/Verifications/BatchExportVerifications.md --content '### Batch Specification Export Test

Test verifies that batch export correctly bundles multiple specification files into a single archive.

#### Details

##### Acceptance Criteria:
- System shall accept multiple specification file paths for batch export
- System shall produce a single archive containing all exported artifacts
- System shall include all selected files in the output archive
- System shall report errors for individual files without aborting the batch

##### Test Criteria:
- Create a model with at least three specification files
- Run batch export command with all files selected
- Verify the output archive exists and contains exported content for each file
- Run batch export with one invalid file path and verify partial success with error report

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Batch Specification Export](../Specifications.md#batch-specification-export)
'
```

### Command 4 -- Validate

```bash
reqvire validate
reqvire lint --fix
reqvire coverage
reqvire format --fix
```

### Traceability Chain

```
Batch Export (feature)
    |
    +-- derive --> Batch Specification Export (requirement)
                       |
                       +-- verifiedBy --> Batch Specification Export Test (test-verification)
```

### Key Design Decisions

1. **EARS pattern**: The system requirement uses the event-driven pattern ("When the user requests...the system shall...").
2. **No `satisfiedBy`** on the feature (forbidden per skill constraints) or on the system requirement (no implementation exists yet).
3. **Bottom roll-up verification**: The test verification is attached only to the leaf system requirement; parent coverage is inherited.
4. **`derivedFrom` inline**: The derive link from user to system requirement is embedded in the system requirement's Relations section, so a separate `reqvire link` command is not needed.
