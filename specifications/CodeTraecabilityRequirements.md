# Requirements

## Code Traecability
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  642b1b8120["Slash style comment"];
  click 642b1b8120 "CodeTraecabilityRequirements.md#slash-style-comment";
  class 642b1b8120 requirement;
  441d1cdce0["Comment Style by File Extension"];
  class 441d1cdce0 requirement;
  click 441d1cdce0 "CodeTraecabilityRequirements.md#comment-style-by-file-extension";
  441d1cdce0 --o|contains| 642b1b8120;
  7b316e0013["Traceability Format"];
  class 7b316e0013 requirement;
  click 7b316e0013 "CodeTraecabilityRequirements.md#traceability-format";
  441d1cdce0 ==>|refines| 7b316e0013;
  441d1cdce0 --o|contains| 642b1b8120;
  d443939ad4["XML style comment"];
  class d443939ad4 requirement;
  click d443939ad4 "CodeTraecabilityRequirements.md#xml-style-comment";
  441d1cdce0 --o|contains| d443939ad4;
  65c6dfea7d["CSS style comment"];
  class 65c6dfea7d requirement;
  click 65c6dfea7d "CodeTraecabilityRequirements.md#css-style-comment";
  441d1cdce0 --o|contains| 65c6dfea7d;
  f8509ca65c["SQL style comment"];
  class f8509ca65c requirement;
  click f8509ca65c "CodeTraecabilityRequirements.md#sql-style-comment";
  441d1cdce0 --o|contains| f8509ca65c;
  79d357d6f0["BAT style comment"];
  class 79d357d6f0 requirement;
  click 79d357d6f0 "CodeTraecabilityRequirements.md#bat-style-comment";
  441d1cdce0 --o|contains| 79d357d6f0;
  594ced3b45["Dash style comment"];
  class 594ced3b45 requirement;
  click 594ced3b45 "CodeTraecabilityRequirements.md#dash-style-comment";
  441d1cdce0 --o|contains| 594ced3b45;
  388e93e521["Validating Traceability Format"];
  click 388e93e521 "CodeTraecabilityRequirements.md#validating-traceability-format";
  class 388e93e521 requirement;
  388e93e521 ==>|refines| 7b316e0013;
  441d1cdce0 --o|contains| d443939ad4;
  441d1cdce0 --o|contains| 65c6dfea7d;
  1fc4e44d5f["../specifications/UserRequirements#Code Traceability"];
  class 1fc4e44d5f requirement;
  click 1fc4e44d5f "UserRequirements.md#code-traceability";
  7b316e0013 ==>|refines| 1fc4e44d5f;
  7b316e0013 -->|relates to| 441d1cdce0;
  7b316e0013 -->|relates to| 388e93e521;
  441d1cdce0 --o|contains| f8509ca65c;
  441d1cdce0 --o|contains| 79d357d6f0;
  441d1cdce0 --o|contains| 594ced3b45;
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