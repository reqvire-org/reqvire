# Elements

### Automated Documentation Export on PR Merge Refinement Specification

#### Details
GitHub workflow behavior for docs export:
- Triggers only after pull-request merge to `main`.
- Checks out post-merge `main` state.
- Builds Reqvire from repository source.
- Runs export pipeline with `reqvire export --output docs`.
- Detects added/updated documentation artifacts.
- Commits generated documentation changes with a standard commit message.
- Pushes documentation updates back to `main`.

#### Metadata
  * type: specification

#### Relations
  * refine: [Automated Documentation Export on PR Merge](GitHubIntegration.md#automated-documentation-export-on-pr-merge)
---

### CSS Style Comment Refinement Specification

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
  * refine: [CSS style comment](CodeAlignment.md#css-style-comment)
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
  * refine: [Comment Style by File Extension](CodeAlignment.md#comment-style-by-file-extension)
---

### Pull Request Change Log Workflow Specification

#### Details
Pull request change-log workflow behavior:
- Triggers during pull-request review.
- Summarizes system model and related component changes.
- Produces a pull-request change log that is reviewable by contributors.

#### Metadata
  * type: specification

#### Relations
  * refine: [Generate Change Logs for Pull Requests](GitHubIntegration.md#generate-change-logs-for-pull-requests)
---

### Pull Request Validation Workflow Specification

#### Details
Pull request validation workflow behavior:
- Triggers when a pull request is updated.
- Runs `reqvire validate`.
- Runs `reqvire lint`.
- Reports pull-request validation results as workflow evidence.

#### Metadata
  * type: specification

#### Relations
  * refine: [Automate Pull Request Validations](GitHubIntegration.md#automate-pull-request-validations)
---

### Slash Style Comment Refinement Specification

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
  * refine: [Slash style comment](CodeAlignment.md#slash-style-comment)
---

### Traceability Format Refinement Specification

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
  * refine: [Traceability Format](CodeAlignment.md#traceability-format)
---

### XML Style Comment Refinement Specification

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
  * refine: [XML style comment](CodeAlignment.md#xml-style-comment)
---
