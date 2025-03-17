# TODO


* test that non markdown files can be in relations as file reference (any extension)

Create technical specification for rust project structure, eg:
```
src/
 ├── element.rs        # Element struct and parser
 ├── subsection.rs     # Handles subsections like Metadata, Relations, etc.
 ├── identifier.rs     # Normalization of file paths and element IDs
 ├── relation.rs       # Handling relations between elements
 ├── parser.rs         # Core parsing logic for Markdown elements
 ├── main.rs           # Example usage
 ├── utils.rs          # Helper functions
 ├── Cargo.toml        # Dependencies
```
