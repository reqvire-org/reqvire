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
  specifications_folder: "specifications"
  
  # Folder name for system requirements (relative to specifications_folder)
  system_requirements_folder_name: "SystemRequirements"
  
  # Folder name for design specifications (relative to specifications_folder)
  design_specifications_folder_name: "DesignSpecifications"
  
  # Default output folder
  output_folder: "output"
```

### Style Settings

```yaml
style:
  theme: "default"            # Theme for HTML output (default, dark, light)
  max_width: 1200             # Maximum width for HTML content (in pixels)
  custom_css: null            # Optional path to a custom CSS file
```

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