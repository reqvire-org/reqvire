# Requirements

## Code Traecability
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  d6b76d3e1a3056ce["BAT style comment"];
  click d6b76d3e1a3056ce "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/CodeTraecabilityRequirements.md#bat-style-comment";
  class d6b76d3e1a3056ce requirement;
  c332a9a7fee51b73["Comment Style by File Extension"];
  class c332a9a7fee51b73 requirement;
  click c332a9a7fee51b73 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/CodeTraecabilityRequirements.md#comment-style-by-file-extension";
  d6b76d3e1a3056ce --o|contains| c332a9a7fee51b73;
  e7b2735ad78db7f7["Slash style comment"];
  click e7b2735ad78db7f7 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/CodeTraecabilityRequirements.md#slash-style-comment";
  class e7b2735ad78db7f7 requirement;
  e7b2735ad78db7f7 --o|contains| c332a9a7fee51b73;
  8086d70961a4c09["Traceability Format"];
  click 8086d70961a4c09 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/CodeTraecabilityRequirements.md#traceability-format";
  class 8086d70961a4c09 requirement;
  8c02a4fb740392b["../specifications/UserRequirements#Code Traceability"];
  class 8c02a4fb740392b requirement;
  click 8c02a4fb740392b "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/UserRequirements.md#code-traceability";
  8c02a4fb740392b -->|refines| 8086d70961a4c09;
  cb7a50ba5073bd8f["SQL style comment"];
  click cb7a50ba5073bd8f "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/CodeTraecabilityRequirements.md#sql-style-comment";
  class cb7a50ba5073bd8f requirement;
  cb7a50ba5073bd8f --o|contains| c332a9a7fee51b73;
  f02fa9023214d641["XML style comment"];
  click f02fa9023214d641 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/CodeTraecabilityRequirements.md#xml-style-comment";
  class f02fa9023214d641 requirement;
  f02fa9023214d641 --o|contains| c332a9a7fee51b73;
  e7305180ad512acd["Dash style comment"];
  click e7305180ad512acd "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/CodeTraecabilityRequirements.md#dash-style-comment";
  class e7305180ad512acd requirement;
  e7305180ad512acd --o|contains| c332a9a7fee51b73;
  8086d70961a4c09 -->|refines| c332a9a7fee51b73;
  1e4723b5a9d499c3["Validating Traceability Format"];
  click 1e4723b5a9d499c3 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/CodeTraecabilityRequirements.md#validating-traceability-format";
  class 1e4723b5a9d499c3 requirement;
  8086d70961a4c09 -->|refines| 1e4723b5a9d499c3;
  240322b67055a395["CSS style comment"];
  click 240322b67055a395 "https://github.com/Reqvire/reqvire/blob/667635d4628fed400323f7df3689af2dbc2ad666/specifications/CodeTraecabilityRequirements.md#css-style-comment";
  class 240322b67055a395 requirement;
  240322b67055a395 --o|contains| c332a9a7fee51b73;
```
TODO:
 * this comes with config param to define code folder locations (special case as external foders but for the code) 
 * this is not implemented yet !!

---

### Traceability Format

When parsing a source file for traceability, the system shall identify and extract all `[reqvire::...]` markers along with their associated requirement element identifiers.

#### Details

Syntax used for `[reqvire::...]` markers:

```
[reqvire::<relation_type>: <element identifier>] START [reqvire::<relation_type>: <element idetifier>] END

```

Where:
- `<relation_type>` specifies the type of relation with only 2 types allowed:
  * satisfies
  * trace
- `<element identifier>` is the identifier of the requirement being traced.

#### Relations
  * refine: [../specifications/UserRequirements#Code Traceability](../specifications/UserRequirements.md#code-traceability)

---

### Validating Traceability Format


While processing traceability in code, the system shall ensure that each `[reqvire::...] START` tag has a corresponding `[reqvire::...] END` tag.

#### Relations
  * refine: [Traceability Format](#traceability-format)

---

### Comment Style by File Extension




The system shall use different comment style based of file extension of the code source file.

#### Relations
  * refine: [Traceability Format](#traceability-format)

---

### Slash style comment

When a source file has a `.c`, `.cpp`, `.cs`, `.java`, `.js`, or `.ts` extension, the system shall use `//` for single-line comments.

#### Details

```
// [reqvire::satisfies: Req1] START
void processSensorData() {
    // Implementation logic
}
// [reqvire::satisfies: Req1] END
```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---

### Dash style comment

When a source file has a `.py`, `.sh`, `.rb`, or `.yml` extension, the system shall use `#` for single-line comments.

#### Details

```
# [reqvire::satisfies: Req1] START
def process_sensor_data():
    pass  # Implementation logic
# [reqvire::satisfies: Req1] END
```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---

### XML style comment

When a source file has a `.html`, `.xml`, or `.xsl` extension, the system shall use `<!-- -->` for comments.

#### Details

```
<!-- [reqvire::satisfies: Req1] START -->
<div> UI Component </div>
<!-- [reqvire::satisfies: Req1] END -->

```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---

### CSS style comment

When a source file has a `.css` or `.scss` extension, the system shall use `/* */` for comments.

#### Details

```
/* [reqvire::satisfies: Req1] START */
.button { background-color: blue; }
/* [reqvire::satisfies: Req1] END */
```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---

### SQL style comment

When a source file has a `.sql` extension, the system shall use `--` for single-line comments.

```
-- [reqvire::satisfies: Req1] START
SELECT * FROM users;
-- [reqvire::satisfies: Req1] END
```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---

### BAT style comment

When a source file has a `.bat` or `.cmd` extension, the system shall use `REM` for comments.

#### Details

```
REM [reqvire::satisfies: Req1] START

echo Hello, World!

REM [reqvire::satisfies: Req1] END

```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---