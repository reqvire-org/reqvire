# Elements

### Comment Style Specification

Comment syntax for code traceability markers by file extension.

#### Details
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
  * satisfy: [Comment Style by File Extension](CodeAlignment.md#comment-style-by-file-extension)
---
