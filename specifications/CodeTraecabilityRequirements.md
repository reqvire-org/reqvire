# Requirements

## Code Traecability
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  594ced3b454de491["Dash style comment"];
  click 594ced3b454de491 "CodeTraecabilityRequirements.md#dash-style-comment";
  class 594ced3b454de491 requirement;
  441d1cdce06ad3a2["Comment Style by File Extension"];
  class 441d1cdce06ad3a2 requirement;
  click 441d1cdce06ad3a2 "CodeTraecabilityRequirements.md#comment-style-by-file-extension";
  594ced3b454de491 --o|contains| 441d1cdce06ad3a2;
  7b316e0013b74149["Traceability Format"];
  class 7b316e0013b74149 requirement;
  click 7b316e0013b74149 "CodeTraecabilityRequirements.md#traceability-format";
  441d1cdce06ad3a2 -->|refines| 7b316e0013b74149;
  65c6dfea7db1c92f["CSS style comment"];
  click 65c6dfea7db1c92f "CodeTraecabilityRequirements.md#css-style-comment";
  class 65c6dfea7db1c92f requirement;
  65c6dfea7db1c92f --o|contains| 441d1cdce06ad3a2;
  1fc4e44d5fd988a6["../specifications/UserRequirements#Code Traceability"];
  class 1fc4e44d5fd988a6 requirement;
  click 1fc4e44d5fd988a6 "UserRequirements.md#code-traceability";
  7b316e0013b74149 -->|refines| 1fc4e44d5fd988a6;
  d443939ad4f72d1a["XML style comment"];
  click d443939ad4f72d1a "CodeTraecabilityRequirements.md#xml-style-comment";
  class d443939ad4f72d1a requirement;
  d443939ad4f72d1a --o|contains| 441d1cdce06ad3a2;
  f8509ca65ccd9ff8["SQL style comment"];
  click f8509ca65ccd9ff8 "CodeTraecabilityRequirements.md#sql-style-comment";
  class f8509ca65ccd9ff8 requirement;
  f8509ca65ccd9ff8 --o|contains| 441d1cdce06ad3a2;
  388e93e521a38370["Validating Traceability Format"];
  click 388e93e521a38370 "CodeTraecabilityRequirements.md#validating-traceability-format";
  class 388e93e521a38370 requirement;
  388e93e521a38370 -->|refines| 7b316e0013b74149;
  642b1b8120da29c6["Slash style comment"];
  click 642b1b8120da29c6 "CodeTraecabilityRequirements.md#slash-style-comment";
  class 642b1b8120da29c6 requirement;
  642b1b8120da29c6 --o|contains| 441d1cdce06ad3a2;
  79d357d6f00a0427["BAT style comment"];
  click 79d357d6f00a0427 "CodeTraecabilityRequirements.md#bat-style-comment";
  class 79d357d6f00a0427 requirement;
  79d357d6f00a0427 --o|contains| 441d1cdce06ad3a2;
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