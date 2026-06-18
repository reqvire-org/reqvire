# Elements

### CSS Style Comment Contract Specification

#### Details
CSS/SCSS traceability comment behavior:
- Uses `/* */` comment wrapper for traceability markers in `.css` and `.scss` files.
- Supports START/END marker blocks in CSS comment format.

Example:
```
/* [reqvire::satisfies: Req1] START */
.button { background-color: blue; }
/* [reqvire::satisfies: Req1] END */
```

#### Metadata
  * type: specification

#### Relations
  * define: [CSS style comment](CodeAlignmentRequirements.md#css-style-comment)
---

### Comment Style Specification

Comment syntax for code traceability markers by file extension.

#### Details
Traceability relation kinds and comment style kinds are defined by the Reqvire code traceability ontology. This specification maps concrete source file extensions to those ontology-defined comment style kinds and gives parser examples.

| Extension | Comment Style | Example |
|-----------|---------------|---------|
| `.bat`, `.cmd` | `REM` | `REM [reqvire::satisfies: Req1] START` |
| `.py`, `.sh`, `.rb`, `.yml` | `#` | `# [reqvire::satisfies: Req1] START` |
| `.sql` | `--` | `-- [reqvire::satisfies: Req1] START` |
| `.css`, `.scss` | `/* */` | `/* [reqvire::satisfies: Req1] START */` |
| `.html`, `.xml`, `.xsl` | `<!-- -->` | `<!-- [reqvire::satisfies: Req1] START -->` |
| `.c`, `.cpp`, `.cs`, `.java`, `.js`, `.ts`, `.rs` | `//` | `// [reqvire::satisfies: Req1] START` |

#### Metadata
  * type: specification

#### Relations
  * define: [Comment Style by File Extension](CodeAlignmentRequirements.md#comment-style-by-file-extension)
---

### Slash Style Comment Contract Specification

#### Details
Slash-style traceability comment behavior:
- Uses `//` single-line comments for supported source files (`.c`, `.cpp`, `.cs`, `.java`, `.js`, `.ts`).
- Supports START/END markers in slash-comment form.

Example:
```
// [reqvire::satisfies: Req1] START
void processSensorData() {
    // Implementation logic
}
// [reqvire::satisfies: Req1] END
```

#### Metadata
  * type: specification

#### Relations
  * define: [Slash style comment](CodeAlignmentRequirements.md#slash-style-comment)
---

### Traceability Format Contract Specification

#### Details
Traceability marker relation kinds are defined by the Reqvire code traceability ontology.

Traceability marker syntax behavior:
- Parses `[reqvire::...]` marker blocks and extracts associated requirement identifiers.
- Uses block markers in the form:
  `[reqvire::<relation_type>: <element identifier>] START ... [reqvire::<relation_type>: <element identifier>] END`
- Restricts `<relation_type>` to ontology-defined traceability relation kind tokens.
- Resolves `<element identifier>` as the traced requirement target.

#### Metadata
  * type: specification

#### Relations
  * define: [Traceability Format](CodeAlignmentRequirements.md#traceability-format)
---

### XML Style Comment Contract Specification

#### Details
XML/HTML traceability comment behavior:
- Uses `<!-- -->` comments for `.html`, `.xml`, and `.xsl` files.
- Supports START/END markers in XML-comment form.

Example:
```
<!-- [reqvire::satisfies: Req1] START -->
<div> UI Component </div>
<!-- [reqvire::satisfies: Req1] END -->
```

#### Metadata
  * type: specification

#### Relations
  * define: [XML style comment](CodeAlignmentRequirements.md#xml-style-comment)
---

