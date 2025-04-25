# Requirements

## Code Traecability
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  a95a4125680208c["CSS style comment"];
  click a95a4125680208c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/CodeTraecabilityRequirements.md#css-style-comment";
  class a95a4125680208c requirement;
  ed3c714e80a3b243["Comment Style by File Extension"];
  class ed3c714e80a3b243 requirement;
  click ed3c714e80a3b243 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/CodeTraecabilityRequirements.md#comment-style-by-file-extension";
  a95a4125680208c --o|contains| ed3c714e80a3b243;
  c3ad800b6ea0175f["Validating Traceability Format"];
  click c3ad800b6ea0175f "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/CodeTraecabilityRequirements.md#validating-traceability-format";
  class c3ad800b6ea0175f requirement;
  d8281700231a102c["Traceability Format"];
  class d8281700231a102c requirement;
  click d8281700231a102c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/CodeTraecabilityRequirements.md#traceability-format";
  c3ad800b6ea0175f -->|refines| d8281700231a102c;
  d208794d54aa32bd["XML style comment"];
  click d208794d54aa32bd "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/CodeTraecabilityRequirements.md#xml-style-comment";
  class d208794d54aa32bd requirement;
  d208794d54aa32bd --o|contains| ed3c714e80a3b243;
  d71502e6a8142057["Dash style comment"];
  click d71502e6a8142057 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/CodeTraecabilityRequirements.md#dash-style-comment";
  class d71502e6a8142057 requirement;
  d71502e6a8142057 --o|contains| ed3c714e80a3b243;
  a4bf10ff95327c04["SQL style comment"];
  click a4bf10ff95327c04 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/CodeTraecabilityRequirements.md#sql-style-comment";
  class a4bf10ff95327c04 requirement;
  a4bf10ff95327c04 --o|contains| ed3c714e80a3b243;
  ed3c714e80a3b243 -->|refines| d8281700231a102c;
  47469d8977723944["../specifications/UserRequirements#Code Traceability"];
  class 47469d8977723944 requirement;
  click 47469d8977723944 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#code-traceability";
  d8281700231a102c -->|refines| 47469d8977723944;
  bb0b29340dfacbdb["BAT style comment"];
  click bb0b29340dfacbdb "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/CodeTraecabilityRequirements.md#bat-style-comment";
  class bb0b29340dfacbdb requirement;
  bb0b29340dfacbdb --o|contains| ed3c714e80a3b243;
  304db90ef17becb6["Slash style comment"];
  click 304db90ef17becb6 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/CodeTraecabilityRequirements.md#slash-style-comment";
  class 304db90ef17becb6 requirement;
  304db90ef17becb6 --o|contains| ed3c714e80a3b243;
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