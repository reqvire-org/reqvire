# Design Specification: External Folders and Requirements Processing

## Introduction

This design specification document describes the implementation of external folders support and the new flexible requirements file detection logic in ReqFlow. The goal is to improve the flexibility of the system by allowing requirements to be stored in multiple repositories and to provide configurable filtering of files that shouldn't be processed as requirements.

## Requirements Satisfied

* [SystemRequirements/Requirements.md/External Folders Support](../SystemRequirements/Requirements.html#external-folders-support)
* [SystemRequirements/Requirements.md/Configurable Filename Exclusion Patterns](../SystemRequirements/Requirements.html#configurable-filename-exclusion-patterns)
* [SystemRequirements/Requirements.md/Unified System Requirements Processing](../SystemRequirements/Requirements.html#unified-system-requirements-processing)

## Implementation Details

### Configuration Changes

1. The `PathsConfig` struct will be modified to:
   - Remove the `system_requirements_folder` field which is no longer needed
   - Add an `external_folders: Vec<String>` field to store paths to external folders
   - Add an `excluded_filename_patterns: Vec<String>` field for glob patterns to exclude

```rust
pub struct PathsConfig {
    pub specifications_folder: String,
    pub design_specifications_folder: String,
    pub output_folder: String,
    pub external_folders: Vec<String>,
    pub excluded_filename_patterns: Vec<String>,
}
```

2. The default configuration will include common exclusion patterns:

```rust
excluded_filename_patterns: vec!["**/README*.md".to_string(), "**/index.md".to_string()],
```

### Requirements File Detection

The `is_requirements_file_only` function will be updated with the following logic:

1. Check if the file is a markdown file (extension .md)
2. Check if the file matches any excluded filename patterns using the glob crate
3. Skip files in design specifications folders
4. If the file is in any external folder, treat it as a requirements file
5. If the file is in the specifications folder or a subfolder (excluding design specifications), treat it as a requirements file

This logic replaces the previous approach that relied on a specific SystemRequirements folder and filename pattern matching.

### Path Handling

To support the new functionality:

1. The `is_requirements_file_by_path` function will be updated to check external folders
2. Path comparisons will be made case-insensitive for better cross-platform compatibility
3. All paths will be normalized before comparison

## Performance Considerations

* The use of glob patterns for exclusion adds a small overhead but provides significant flexibility
* Caching file contents will minimize any performance impact from additional pattern matching
* The system will process files in parallel when possible to maintain performance

## Security Considerations

* External folder paths will be validated to ensure they point to valid directories
* Absolute paths will be checked against the base path to prevent access to unauthorized locations
* Error handling will ensure clear messaging when an external folder cannot be accessed

## Migration Path

Projects using the previous `system_requirements_folder` approach can migrate by:

1. Updating their configuration to use the new structure
2. Using external_folders for any additional requirement sources
3. Adding exclusion patterns for files that should not be processed

The configuration documentation will be updated to guide users through this migration.