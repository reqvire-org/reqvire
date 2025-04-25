# Reqvire Configuration Guide

Reqvire supports customization through a YAML configuration file. This file allows you to specify folder names, style settings, and other options to customize the tool for your project's specific needs.

## Configuration File Locations

Reqvire will look for a configuration file in the following locations (in order):

1. The path specified with `--config` or `-c` command line option
2. `reqvire.yml` in the current directory
3. `reqvire.yaml` in the current directory
4. `.reqvire.yml` in the current directory
5. `.reqvire.yaml` in the current directory
6. `.config/reqvire.yml` in the current directory
7. `.config/reqvire.yaml` in the current directory

If no configuration file is found, default settings will be used.

## Configuration File Format

The configuration file is a YAML document with several sections:

```yaml
# General options
general:
  verbose: false

# Path settings
paths:
  specifications_folder: "specifications"
  system_requirements_folder_name: "SystemRequirements"
  design_specifications_folder_name: "DesignSpecifications"
  output_folder: "output"
  requirements_filename_match: "Requirements"

# Style settings for HTML output
style:
  theme: "default"
  max_width: 1200
  custom_css: null
```

## Configuration Sections

### General Options

```yaml
general:
  verbose: false      # Enable verbose output (same as --verbose flag)
```

### Path Settings

```yaml
paths:
  # Main folder containing specifications (relative to execution directory)
  # Can be overridden via command line by providing the input folder argument
  specifications_folder: "specifications"
  
  
  # Folder name for design specifications (relative to specifications_folder)
  design_specifications_folder_name: "DesignSpecifications"
  
  # Default output folder
  # Can be overridden via command line by providing the output folder argument
  output_folder: "output"
  
  # Additional external folders that contain system requirements and other files
  # These can be absolute paths or paths relative to the input folder
  # All markdown files in these folders are considered requirements (except excluded patterns)
  external_folders:
    - "../other-project/specifications"
    - "/mnt/shared/team-requirements"
    
  # Glob patterns to exclude from requirements processing
  # These are patterns that shouldn't be considered requirements even if they're in 
  # specifications or external folders
  excluded_filename_patterns:
    - "**/README*.md"
    - "**/index.md"
```

### Style Settings

```yaml
style:
  theme: "default"            # Theme for HTML output (default, dark, light)
  max_width: 1200             # Maximum width for HTML content (in pixels)
  custom_css: null            # Optional path to a custom CSS file
```

## External Folders

The `external_folders` configuration option allows you to include system requirements from other repositories or different directory structures in your Reqvire processing:

```yaml
paths:
  # Main specifications folder
  specifications_folder: "specifications"
  
  # External folders with additional system requirements
  external_folders:
    - "../other-project/specifications"
    - "/mnt/shared/team-requirements"
    
  # Glob patterns to exclude from requirements processing
  # These are patterns that shouldn't be considered requirements even if they're in specifications or external folders
  excluded_filename_patterns:
    - "**/README*.md"
    - "**/index.md"
```

Key points about external folders:

1. External folders can contain requirements and other supporting files
2. They CANNOT contain User Requirements or Mission Requirements (these must be in the main specifications folder)
3. All markdown files in external folders are treated as system requirements (except those matching excluded patterns)
4. Paths can be absolute or relative to the input folder
5. Requirements from external folders will be included in diagrams, validation, and relation checking
6. External folders are useful for cross-repository references and distributed requirements management

Key points about excluded filename patterns:

1. These glob patterns determine which files should be skipped during requirements processing
2. Patterns follow glob syntax (e.g., `**/README*.md` matches any README.md file in any directory)
3. Files matching these patterns won't be processed even if they're in specifications or external folders
4. This is useful for excluding documentation files like README.md, index.md, etc. that aren't requirements

## Command Line Arguments

With the configuration file in place, the command line arguments for input and output folders become optional:

```
reqvire [INPUT_FOLDER] [OUTPUT_FOLDER] [OPTIONS]
```

If the input folder or output folder is not provided on the command line, Reqvire will use the values from the configuration file.

## Command Line Override

Command line arguments take precedence over configuration file settings. For example, if you specify `--verbose` on the command line, it will override the `general.verbose` setting in the configuration file.

## Example Configuration

The repository includes an example configuration file at `reqvire.yml.example`. Copy this file to one of the supported locations and modify it as needed.

```bash
# Copy the example configuration to your project
cp /path/to/reqvire.yml.example ./reqvire.yml

# Edit the configuration file
nano reqvire.yml
```

## Using with CI/CD

When using Reqvire in CI/CD pipelines, you can either:

1. Include a reqvire.yml file in your repository
2. Pass a configuration file explicitly using the `--config` option
3. Override specific settings using command line arguments

This flexibility makes it easy to adapt Reqvire to various CI/CD environments and workflows.