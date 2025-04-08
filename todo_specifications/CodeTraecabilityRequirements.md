# Requirements

## Code Traecability
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  e74d8ff4f7["Slash style comment"];
  click e74d8ff4f7 "CodeTraecabilityRequirements.md#slash-style-comment";
  class e74d8ff4f7 requirement;
  9b11c134e5["Comment Style by File Extension"];
  class 9b11c134e5 requirement;
  click 9b11c134e5 "CodeTraecabilityRequirements.md#comment-style-by-file-extension";
  9b11c134e5 --o|contains| e74d8ff4f7;
  26b4eb60f9["Dash style comment"];
  click 26b4eb60f9 "CodeTraecabilityRequirements.md#dash-style-comment";
  class 26b4eb60f9 requirement;
  9b11c134e5 --o|contains| 26b4eb60f9;
  e5b00bf82f["BAT style comment"];
  click e5b00bf82f "CodeTraecabilityRequirements.md#bat-style-comment";
  class e5b00bf82f requirement;
  9b11c134e5 --o|contains| e5b00bf82f;
  bb213f1c85["Traceability Format"];
  click bb213f1c85 "CodeTraecabilityRequirements.md#traceability-format";
  class bb213f1c85 requirement;
  1fc4e44d5f["../specifications/UserRequirements#Code Traceability"];
  class 1fc4e44d5f requirement;
  click 1fc4e44d5f "../specifications/UserRequirements.md#code-traceability";
  bb213f1c85 ==>|refines| 1fc4e44d5f;
  bb213f1c85 -->|relates to| 9b11c134e5;
  1e5a561e29["Validating Traceability Format"];
  class 1e5a561e29 requirement;
  click 1e5a561e29 "CodeTraecabilityRequirements.md#validating-traceability-format";
  bb213f1c85 -->|relates to| 1e5a561e29;
  56c1e18c90["CSS style comment"];
  click 56c1e18c90 "CodeTraecabilityRequirements.md#css-style-comment";
  class 56c1e18c90 requirement;
  9b11c134e5 --o|contains| 56c1e18c90;
  9b11c134e5 ==>|refines| bb213f1c85;
  9b11c134e5 --o|contains| e74d8ff4f7;
  9b11c134e5 --o|contains| 26b4eb60f9;
  9b11c134e5 --o|contains| e5b00bf82f;
  9b11c134e5 --o|contains| 56c1e18c90;
  663b780162["SQL style comment"];
  class 663b780162 requirement;
  click 663b780162 "CodeTraecabilityRequirements.md#sql-style-comment";
  9b11c134e5 --o|contains| 663b780162;
  a5580908b7["XML style comment"];
  class a5580908b7 requirement;
  click a5580908b7 "CodeTraecabilityRequirements.md#xml-style-comment";
  9b11c134e5 --o|contains| a5580908b7;
  1e5a561e29 ==>|refines| bb213f1c85;
  9b11c134e5 --o|contains| 663b780162;
  9b11c134e5 --o|contains| a5580908b7;
```
TODO:
 * check how to related validation requirement
 * this comes with config param to define code folder locations (special case as external foders but for the code) 

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