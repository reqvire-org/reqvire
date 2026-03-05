# Eval: add-requirement (with new skill)

## Task
"I need to add a new user requirement for 'Batch Export' - the system should support exporting multiple specification files at once into a single archive. Derive a system requirement from it and add a test verification. Use the MBSE workflow. Put user requirements in requirements/UserRequirements.md and system requirements in requirements/SystemRequirements/Requirements.md. Don't actually run the commands, just show me the exact reqvire commands you would run."

## Approach
- Read the skill at `/mnt/Radni/reqvire/claude-plugins/skills/syseng/SKILL.md`
- Read the AddFeature reference at `/mnt/Radni/reqvire/claude-plugins/skills/syseng/reference/AddFeature.md`
- Analyzed existing model structure by reading:
  - `requirements/UserStories.md` (existing user requirements format)
  - `requirements/Interfaces/WebInterface/Features.md` (existing export-related system requirements)
  - `requirements/Interfaces/Interfaces.md` (parent hierarchy)
  - `requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.md` (existing verification format)

**Note:** The files `requirements/UserRequirements.md` and `requirements/SystemRequirements/Requirements.md` do not currently exist. In the actual project, user requirements are in `UserStories.md` and system requirements are organized by functional area.

## Commands

### Command 1 -- Add User Requirement

```bash
reqvire add requirements/UserRequirements.md --content '### Batch Export

As a **System Engineer**, I want to export multiple specification files at once into a single archive, so that I can efficiently share and distribute complete sets of system documentation.

#### Metadata
  * type: user-requirement
'
```

### Command 2 -- Add System Requirement (derived)

```bash
reqvire add requirements/SystemRequirements/Requirements.md --content '### Batch Specification Export

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
  * derivedFrom: [Batch Export](../UserRequirements.md#batch-export)
'
```

### Command 3 -- Add Test Verification

```bash
reqvire add requirements/SystemRequirements/Verifications/BatchExportVerifications.md --content '### Batch Specification Export Test

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
  * verify: [Batch Specification Export](../Requirements.md#batch-specification-export)
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
Batch Export (user-requirement)
    |
    +-- derive --> Batch Specification Export (requirement)
                       |
                       +-- verifiedBy --> Batch Specification Export Test (test-verification)
```

### Key Design Decisions

1. **EARS pattern**: The system requirement uses the event-driven pattern ("When the user requests...the system shall...").
2. **No `satisfiedBy`** on the user-requirement (forbidden per skill constraints) or on the system requirement (no implementation exists yet).
3. **Bottom roll-up verification**: The test verification is attached only to the leaf system requirement; parent coverage is inherited.
4. **`derivedFrom` inline**: The derive link from user to system requirement is embedded in the system requirement's Relations section, so a separate `reqvire link` command is not needed.
