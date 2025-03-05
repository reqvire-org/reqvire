# ReqFlow Configuration Guide

ReqFlow supports customization through a YAML configuration file. This file allows you to specify folder names, style settings, and other options to customize the tool for your project's specific needs.

## Configuration File Locations

ReqFlow will look for a configuration file in the following locations (in order):

1. The path specified with `--config` or `-c` command line option
2. `reqflow.yml` in the current directory
3. `reqflow.yaml` in the current directory
4. `.reqflow.yml` in the current directory
5. `.reqflow.yaml` in the current directory
6. `.config/reqflow.yml` in the current directory
7. `.config/reqflow.yaml` in the current directory

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
  
  # Folder name for system requirements (relative to specifications_folder)
  system_requirements_folder_name: "SystemRequirements"
  
  # Folder name for design specifications (relative to specifications_folder)
  design_specifications_folder_name: "DesignSpecifications"
  
  # Default output folder
  # Can be overridden via command line by providing the output folder argument
  output_folder: "output"
  
  # Additional external folders that contain system requirements and other files
  # These can be absolute paths or paths relative to the input folder
  # External folders can only contain SystemRequirements, not User or Mission Requirements
  external_folders:
    - "../other-project/specifications"
    - "/mnt/shared/team-requirements"
```

### Style Settings

```yaml
style:
  theme: "default"            # Theme for HTML output (default, dark, light)
  max_width: 1200             # Maximum width for HTML content (in pixels)
  custom_css: null            # Optional path to a custom CSS file
```

## External Folders

The `external_folders` configuration option allows you to include system requirements from other repositories or different directory structures in your ReqFlow processing:

```yaml
paths:
  # Main specifications folder
  specifications_folder: "specifications"
  
  # External folders with additional system requirements
  external_folders:
    - "../other-project/specifications"
    - "/mnt/shared/team-requirements"
```

Key points about external folders:

1. External folders can contain SystemRequirements and other supporting files
2. They CANNOT contain User Requirements or Mission Requirements (these must be in the main specifications folder)
3. Each external folder is expected to have a similar structure to the main specifications folder
4. Paths can be absolute or relative to the input folder
5. System requirements from external folders will be included in diagrams, validation, and relation checking
6. External folders are useful for cross-repository references and distributed requirements management

## Command Line Arguments

With the configuration file in place, the command line arguments for input and output folders become optional:

```
reqflow [INPUT_FOLDER] [OUTPUT_FOLDER] [OPTIONS]
```

If the input folder or output folder is not provided on the command line, ReqFlow will use the values from the configuration file.

## Command Line Override

Command line arguments take precedence over configuration file settings. For example, if you specify `--verbose` on the command line, it will override the `general.verbose` setting in the configuration file.

## Example Configuration

The repository includes an example configuration file at `reqflow.yml.example`. Copy this file to one of the supported locations and modify it as needed.

```bash
# Copy the example configuration to your project
cp /path/to/reqflow.yml.example ./reqflow.yml

# Edit the configuration file
nano reqflow.yml
```

## Using with CI/CD

When using ReqFlow in CI/CD pipelines, you can either:

1. Include a reqflow.yml file in your repository
2. Pass a configuration file explicitly using the `--config` option
3. Override specific settings using command line arguments

This flexibility makes it easy to adapt ReqFlow to various CI/CD environments and workflows.