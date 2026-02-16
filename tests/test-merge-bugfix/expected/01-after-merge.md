# Elements

### Test Parent

Parent element for testing.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [CLI Interface Structure](#cli-interface-structure)
---

### CLI Interface Structure

The CLI interface shall implement the clear `[OPTIONS] <COMMAND> [COMMAND OPTIONS]` structure.

#### Details
The CLI must display all commands and options and command's options flattened in the main help output which must also be a default command:
```
Reqvire requirements & traceability management tool

Usage: reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]

Commands:
  search            Search and filter model elements
  coverage          Generate verification and implementation coverage report
  help              Print help for commands

Options:
  -h, --help               Print help
  -V, --version            Print version
```
The system shall provide a `search` command to query and filter model elements with flexible output formats.

#### Merged Details (CLI Search Command)
The `search` command shall:
- Support searching by element name, type, content, or relationships
- Support `--filter-name`, `--filter-type`, `--filter-content` options
- Support `--json` flag for structured JSON output
- Support `--short` flag for abbreviated output
- Default to formatted text reports
- Exit with code 0 on success, non-zero on error
- Command syntax: `reqvire search [OPTIONS]`

The output shall include element identifiers, names, types, and relationships.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Test Parent](#test-parent)
---

