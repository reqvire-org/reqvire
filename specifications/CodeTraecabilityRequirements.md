# Requirements

## Code Traecability
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  befb3119a3411cc9["Traceability Format"];
  click befb3119a3411cc9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/CodeTraecabilityRequirements.md#traceability-format";
  class befb3119a3411cc9 requirement;
  ee212ffe5248cf1a["../specifications/UserRequirements#Code Traceability"];
  class ee212ffe5248cf1a requirement;
  click ee212ffe5248cf1a "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#code-traceability";
  befb3119a3411cc9 -->|refines| ee212ffe5248cf1a;
  8ea0bce64c0e4478["Dash style comment"];
  click 8ea0bce64c0e4478 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/CodeTraecabilityRequirements.md#dash-style-comment";
  class 8ea0bce64c0e4478 requirement;
  b721f3b78f158cf2["Comment Style by File Extension"];
  class b721f3b78f158cf2 requirement;
  click b721f3b78f158cf2 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/CodeTraecabilityRequirements.md#comment-style-by-file-extension";
  8ea0bce64c0e4478 --o|contains| b721f3b78f158cf2;
  f8c8018fe76465fc["CSS style comment"];
  click f8c8018fe76465fc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/CodeTraecabilityRequirements.md#css-style-comment";
  class f8c8018fe76465fc requirement;
  f8c8018fe76465fc --o|contains| b721f3b78f158cf2;
  bd4e0dd56ca955a8["Validating Traceability Format"];
  click bd4e0dd56ca955a8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/CodeTraecabilityRequirements.md#validating-traceability-format";
  class bd4e0dd56ca955a8 requirement;
  bd4e0dd56ca955a8 -->|refines| befb3119a3411cc9;
  6a9d8aa0bfd66e44["Slash style comment"];
  click 6a9d8aa0bfd66e44 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/CodeTraecabilityRequirements.md#slash-style-comment";
  class 6a9d8aa0bfd66e44 requirement;
  6a9d8aa0bfd66e44 --o|contains| b721f3b78f158cf2;
  b721f3b78f158cf2 -->|refines| befb3119a3411cc9;
  51141d55bd221f9d["SQL style comment"];
  click 51141d55bd221f9d "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/CodeTraecabilityRequirements.md#sql-style-comment";
  class 51141d55bd221f9d requirement;
  51141d55bd221f9d --o|contains| b721f3b78f158cf2;
  de1f37415b6b9a2d["XML style comment"];
  click de1f37415b6b9a2d "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/CodeTraecabilityRequirements.md#xml-style-comment";
  class de1f37415b6b9a2d requirement;
  de1f37415b6b9a2d --o|contains| b721f3b78f158cf2;
  a8ddfb7fb289138b["BAT style comment"];
  click a8ddfb7fb289138b "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/CodeTraecabilityRequirements.md#bat-style-comment";
  class a8ddfb7fb289138b requirement;
  a8ddfb7fb289138b --o|contains| b721f3b78f158cf2;
```
TODO:
 * this comes with config param to define code folder locations (special case as external foders but for the code) 
 * this is not implemented yet !!

---

### Traceability Format

When parsing a source file for traceability, the system shall identify and extract all `[reqflow::...]` markers along with their associated requirement element identifiers.

#### Details

Syntax used for `[reqflow::...]` markers:

```
[reqflow::<relation_type>: <element identifier>] START [reqflow::<relation_type>: <element idetifier>] END

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


While processing traceability in code, the system shall ensure that each `[reqflow::...] START` tag has a corresponding `[reqflow::...] END` tag.

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
// [reqflow::satisfies: Req1] START
void processSensorData() {
    // Implementation logic
}
// [reqflow::satisfies: Req1] END
```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---

### Dash style comment

When a source file has a `.py`, `.sh`, `.rb`, or `.yml` extension, the system shall use `#` for single-line comments.

#### Details

```
# [reqflow::satisfies: Req1] START
def process_sensor_data():
    pass  # Implementation logic
# [reqflow::satisfies: Req1] END
```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---

### XML style comment

When a source file has a `.html`, `.xml`, or `.xsl` extension, the system shall use `<!-- -->` for comments.

#### Details

```
<!-- [reqflow::satisfies: Req1] START -->
<div> UI Component </div>
<!-- [reqflow::satisfies: Req1] END -->

```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---

### CSS style comment

When a source file has a `.css` or `.scss` extension, the system shall use `/* */` for comments.

#### Details

```
/* [reqflow::satisfies: Req1] START */
.button { background-color: blue; }
/* [reqflow::satisfies: Req1] END */
```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---

### SQL style comment

When a source file has a `.sql` extension, the system shall use `--` for single-line comments.

```
-- [reqflow::satisfies: Req1] START
SELECT * FROM users;
-- [reqflow::satisfies: Req1] END
```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---

### BAT style comment

When a source file has a `.bat` or `.cmd` extension, the system shall use `REM` for comments.

#### Details

```
REM [reqflow::satisfies: Req1] START

echo Hello, World!

REM [reqflow::satisfies: Req1] END

```

#### Relations
  * containedBy: [Comment Style by File Extension](#comment-style-by-file-extension)

---