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
reqflow -c path/to/custom-config.yaml specifications/ output/
```

### Configuration Options

Here's an example of a ReqFlow configuration file:

```yaml
general:
  html_output: true
  verbose: false
  generate_diagrams: true
  
paths:
  specifications_folder: "specifications"
  system_requirements_folder: "SystemRequirements"
  design_specifications_folder: "DesignSpecifications"
  
validation:
  validate_markdown: true
  validate_relations: true
  validate_cross_components: true
  
linting:
  lint: true
  dry_run: false
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
│   │   └── Requirements.md
│   └── DesignSpecifications/
│       ├── DSD_ComponentA.md
│       └── DSD_ComponentB.md
└── reqflow.yaml
```

### Requirements Format

Requirements are defined in Markdown files following this format:

```markdown
### Requirement Name

The system shall [requirement description].

#### Relations
  * refine: [OtherDocument.md/Other Requirement](OtherDocument.html#other-requirement)
  * verifiedBy: [TestCase.md/Test Case](TestCase.html#test-case)
```

## Validation

ReqFlow provides multiple validation options to ensure your requirements model is consistent.

### Validate Markdown Structure

Ensures your Markdown follows the required format:

```bash
reqflow --validate-markdown specifications/
```

### Validate Relations

Checks that all relations between requirements are valid:

```bash
reqflow --validate-relations specifications/
```

### Validate All

Performs comprehensive validation:

```bash
reqflow --validate-all specifications/
```

## Linting

Linting helps maintain consistent formatting and style.

### Run Linting

Apply automatic fixes to formatting issues:

```bash
reqflow --lint specifications/
```

### Dry Run

Preview linting changes without applying them:

```bash
reqflow --lint --dry-run specifications/
```

## Generating Documentation

ReqFlow can generate HTML documentation from your Markdown files.

### Convert to HTML

```bash
reqflow specifications/ output/ --html
```

This creates HTML files with navigation, properly formatted requirements, and interactive diagrams.

## Traceability

Track relationships between requirements using traceability features.

### Generate Traceability Matrix

```bash
reqflow specifications/ output/ --generate-matrix
```

This generates a traceability matrix showing relationships between requirements.

## Diagrams

ReqFlow can automatically generate diagrams from your requirements model.

### Generate Diagrams

```bash
reqflow specifications/ --generate-diagrams
```

This creates Mermaid diagrams within your requirements files.

## LLM Context Documentation

ReqFlow provides comprehensive documentation for Large Language Models (LLMs) to understand the project structure.

### Generate LLM Context

```bash
reqflow --llm-context
```

This outputs detailed information about ReqFlow methodology, document structure, and syntax conventions, helping LLMs work effectively with your requirements.