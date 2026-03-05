# Eval: add-requirement (with old skill)

## Task
"I need to add a new user requirement for 'Batch Export'..."

## Approach
- Read skill-snapshot/SKILL.md (842 lines)
- Read AddFeature reference
- Analyzed existing model: UserStories.md, globbed requirements/**/*.md, grepped for export
- Noted files requested by user don't exist (project uses UserStories.md and Functional/*/)

## Commands Produced

### Step 1: User Requirement
```bash
reqvire add requirements/UserRequirements.md <<'EOF'
### Batch Export

As a **System Engineer**, I want to export multiple specification files at once into a single archive, so that I can efficiently share and distribute complete model snapshots.

#### Metadata
  * type: user-requirement
EOF
```

### Step 2: System Requirement
```bash
reqvire add requirements/SystemRequirements/Requirements.md <<'EOF'
### Batch Specification Export

The system shall support exporting multiple specification files into a single archive file in a single operation.

#### Details

When the user requests a batch export, the system shall:
1. Accept a set of specification file paths or a filter pattern
2. Collect all matching specification files
3. Package the collected files into a single archive
4. Output the archive to the specified destination

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Batch Export](../UserRequirements.md#batch-export)
EOF
```

### Step 3: Test Verification
```bash
reqvire add requirements/SystemRequirements/Verifications/BatchExportVerifications.md <<'EOF'
### Batch Export Verification

Test verifies batch export functionality:
1. Multiple specification files can be selected for export
2. Selected files are packaged into a single archive
3. The archive contains all expected specification files with correct content
4. Export fails gracefully when no files match the selection criteria
5. Export fails gracefully when the output path is invalid

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Batch Specification Export](../Requirements.md#batch-specification-export)
EOF
```

### Step 4: Validate
```bash
reqvire validate
reqvire lint
reqvire coverage
reqvire format --fix
```

## Key Observations
- Used heredoc syntax (<<'EOF') correctly
- MBSE order maintained: user-req -> system-req -> verification
- EARS pattern used ("The system shall...")
- derivedFrom and verify relations correct
- Noted discrepancy between user-specified paths and project conventions
