# ReqFlow User Guide

This user guide provides detailed instructions on how to use ReqFlow effectively.

## Table of Contents

- [Basic Commands](#basic-commands)
- [Configuration](#configuration)
- [Working with Requirements](#working-with-requirements)
- [Validation](#validation)
- [Linting](#linting)
- [Generating Documentation](#generating-documentation)
- [Traceability](#traceability)
- [Diagrams](#diagrams)
- [LLM Context Documentation](#llm-context-documentation)

## Basic Commands

ReqFlow offers several core commands for managing your requirements:

### Help

View the available commands and options:

```bash
reqflow --help
```

### Version

Display the current version:

```bash
reqflow --version
```

## Configuration

ReqFlow uses a YAML configuration file to customize its behavior.

### Default Configuration File

By default, ReqFlow looks for configuration in the following files (in order):
- `reqflow.yaml`
- `reqflow.yml`
- `.reqflow.yaml`
- `.reqflow.yml`

### Custom Configuration

To use a custom configuration file:

```bash
reqflow -c path/to/custom-config.yaml
```

### Configuration Options

Here's an example of a ReqFlow configuration file:

```yaml
  # Path to the specifications folder
  specifications_folder: "specifications"
  
  # Default output folder for exported html specifications
  output_folder: "output"
  
  # Additional external folders that contain system requirements and other files
  # These can be absolute paths or paths relative to the input folder
  # All markdown files in these folders are considered requirements (except those matching exclusion patterns)
  external_folders:
    - todo_specifications
    - tests
    - core
    - cli
      
  # Glob patterns to exclude from requirements processing
  # These are patterns that shouldn't be considered requirements even if they're in specifications or external folders
  excluded_filename_patterns:
    - "Usecases.md"
    - "**/Logical*.md"
    - "**/Physical*.md"
    - "**/TODO.md"
    - "**/DesignSpecifications/**"        
    - "**/tests/**"    
    - "**/core/**"    
    - "**/cli/**"            

style:
  # Theme for HTML output (default, dark, light)
  theme: "default"
  
  # Maximum width for HTML output
  max_width: 1200
  
  # Optional path to custom CSS file
  # custom_css: "path/to/custom.css"
  
  # Diagram direction (TD for top-down, LR for left-to-right)
  diagram_direction: "LR"
  
```

## Working with Requirements

ReqFlow is designed to work with a structured requirements hierarchy in Markdown files.

### Folder Structure

A typical ReqFlow project structure looks like this:

```
project/
├── specifications/
│   ├── UserRequirements.md
│   ├── MissionRequirements.md
│   ├── SystemRequirements/
│       └── Requirements.md
└── reqflow.yaml
```

### Requirements and general Markdown files format

Read specifications in [../specifications/SpecificationsRequirements.md](../specifications/SpecificationsRequirements.md)


## Validation

Performs comprehensive validation:

```bash
reqflow --validate
```

## Linting

Linting helps maintain consistent formatting and style.

### Run Linting

Sometimes it is requred to run linting several times to converged to clean document format.

Apply automatic fixes to formatting issues:

```bash
reqflow --lint
```

### Dry Run

Preview linting changes without applying them:

```bash
reqflow --lint --dry-run
```

## Generating Documentation

ReqFlow can generate HTML documentation from your Markdown files.

### Convert to HTML

```bash
reqflow --html
```

This creates HTML files with navigation, properly formatted requirements, and interactive diagrams.

## Traceability

Track relationships between requirements using traceability features.

### Generate Traceability Matrix

```bash
reqflow --traces
```

This generates a traceability matrix showing relationships between requirements.

## Diagrams

ReqFlow can automatically generate diagrams from your requirements model.

### Generate Diagrams

```bash
reqflow --generate-diagrams
```

This creates Mermaid diagrams within your requirements files.

## LLM Context Documentation

ReqFlow provides comprehensive documentation for Large Language Models (LLMs) to understand the project structure.

### Generate LLM Context

```bash
reqflow --llm-context
```

This outputs detailed information about ReqFlow methodology, document structure, and syntax conventions, helping LLMs work effectively with your requirements.

### TODO: add change impact and other important commands 
