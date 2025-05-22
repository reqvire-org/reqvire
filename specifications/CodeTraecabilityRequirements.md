# Requirements

## Code Traecability
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  7fdde6a23819450b["BAT style comment"];
  click 7fdde6a23819450b "specifications/CodeTraecabilityRequirements.md#bat-style-comment#bat-style-comment";
  class 7fdde6a23819450b requirement;
  f4c09ad7359812ff["Comment Style by File Extension"];
  class f4c09ad7359812ff requirement;
  click f4c09ad7359812ff "specifications/CodeTraecabilityRequirements.md#comment-style-by-file-extension#comment-style-by-file-extension";
  7fdde6a23819450b --o|contains| f4c09ad7359812ff;
  9314736a59776478["Traceability Format"];
  click 9314736a59776478 "specifications/CodeTraecabilityRequirements.md#traceability-format#traceability-format";
  class 9314736a59776478 requirement;
  26ca72b617aff229["../specifications/UserRequirements#Code Traceability"];
  class 26ca72b617aff229 requirement;
  click 26ca72b617aff229 "specifications/UserRequirements.md#code-traceability#code-traceability";
  9314736a59776478 -->|refines| 26ca72b617aff229;
  3db41d3530e56596["Validating Traceability Format"];
  click 3db41d3530e56596 "specifications/CodeTraecabilityRequirements.md#validating-traceability-format#validating-traceability-format";
  class 3db41d3530e56596 requirement;
  3db41d3530e56596 -->|refines| 9314736a59776478;
  f4c09ad7359812ff -->|refines| 9314736a59776478;
  39fc946195c12956["SQL style comment"];
  click 39fc946195c12956 "specifications/CodeTraecabilityRequirements.md#sql-style-comment#sql-style-comment";
  class 39fc946195c12956 requirement;
  39fc946195c12956 --o|contains| f4c09ad7359812ff;
  474c97b7d0029622["XML style comment"];
  click 474c97b7d0029622 "specifications/CodeTraecabilityRequirements.md#xml-style-comment#xml-style-comment";
  class 474c97b7d0029622 requirement;
  474c97b7d0029622 --o|contains| f4c09ad7359812ff;
  a91fc474f292f1c9["CSS style comment"];
  click a91fc474f292f1c9 "specifications/CodeTraecabilityRequirements.md#css-style-comment#css-style-comment";
  class a91fc474f292f1c9 requirement;
  a91fc474f292f1c9 --o|contains| f4c09ad7359812ff;
  be41c3d9e39abe6f["Dash style comment"];
  click be41c3d9e39abe6f "specifications/CodeTraecabilityRequirements.md#dash-style-comment#dash-style-comment";
  class be41c3d9e39abe6f requirement;
  be41c3d9e39abe6f --o|contains| f4c09ad7359812ff;
  99c4cc438042d855["Slash style comment"];
  click 99c4cc438042d855 "specifications/CodeTraecabilityRequirements.md#slash-style-comment#slash-style-comment";
  class 99c4cc438042d855 requirement;
  99c4cc438042d855 --o|contains| f4c09ad7359812ff;
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