# Elements

### CLI interface

The system shall provide command line interface (CLI) to faciliate model management.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [CLI Interface Structure](CLI/Commands.md#cli-interface-structure)
  * derivedFrom: [System Model Interfaces](../UserStories.md#system-model-interfaces)
---

### Web Interface

The system SHALL provide a web-based interface to browse the System model documentation, including all generated artifacts such as diagrams, reports, and verification traces.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [HTML Export](WebInterface/Features.md#html-export)
  * derive: [Serve Command](WebInterface/Features.md#serve-command)
  * derivedFrom: [System Model Interfaces](../UserStories.md#system-model-interfaces)
  * refinedBy: [Web Interface Refinement Specification](WebInterface/Specifications.md#web-interface-refinement-specification)
---
