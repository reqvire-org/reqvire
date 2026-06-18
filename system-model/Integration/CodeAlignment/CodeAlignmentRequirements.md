# Elements

### Code Traceability

The system shall support code traceability by using structured comments to link code implementations to corresponding requirements in the System model.

#### Metadata
  * type: requirement

#### Relations
  * constrainedBy: [Source Marker Traceability Shape](../../Ontologies/Integration.md#source-marker-traceability-shape)
  * derive: [Traceability Format](#traceability-format)
  * specify: [Aligning Design with Code](../IntegrationFeature.md#aligning-design-with-code)
---

### Traceability Format

When parsing a source file for traceability, the system shall identify and extract all `[reqvire::...]` markers along with their associated requirement element identifiers.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Traceability Format Contract Specification](Specifications.md#traceability-format-contract-specification)
  * derive: [Comment Style by File Extension](#comment-style-by-file-extension)
  * derive: [Validating Traceability Format](#validating-traceability-format)
  * derivedFrom: [Code Traceability](#code-traceability)
---

### Comment Style by File Extension

The system shall use different comment style based of file extension of the code source file.

#### Details
Comment style kinds and traceability relation kinds are defined by the Reqvire code traceability ontology. The implementation requirements define which source file extensions use each comment style.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Comment Style Specification](Specifications.md#comment-style-specification)
  * derive: [BAT style comment](#bat-style-comment)
  * derive: [CSS style comment](#css-style-comment)
  * derive: [Dash style comment](#dash-style-comment)
  * derive: [Slash style comment](#slash-style-comment)
  * derive: [SQL style comment](#sql-style-comment)
  * derive: [XML style comment](#xml-style-comment)
  * derivedFrom: [Traceability Format](#traceability-format)
---

### BAT style comment

When a source file has a `.bat` or `.cmd` extension, the system shall use `REM` for comments.

#### Details
```
REM [reqvire::satisfies: Req1] START

echo Hello, World!

REM [reqvire::satisfies: Req1] END

```

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Comment Style by File Extension](#comment-style-by-file-extension)
---

### CSS style comment

When a source file has a `.css` or `.scss` extension, the system shall use `/* */` for comments.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [CSS Style Comment Contract Specification](Specifications.md#css-style-comment-contract-specification)
  * derivedFrom: [Comment Style by File Extension](#comment-style-by-file-extension)
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

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Comment Style by File Extension](#comment-style-by-file-extension)
---

### SQL style comment

When a source file has a `.sql` extension, the system shall use `--` for single-line comments.

```
-- [reqvire::satisfies: Req1] START
SELECT * FROM users;
-- [reqvire::satisfies: Req1] END
```

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Comment Style by File Extension](#comment-style-by-file-extension)
---

### Slash style comment

When a source file has a `.c`, `.cpp`, `.cs`, `.java`, `.js`, or `.ts` extension, the system shall use `//` for single-line comments.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Slash Style Comment Contract Specification](Specifications.md#slash-style-comment-contract-specification)
  * derivedFrom: [Comment Style by File Extension](#comment-style-by-file-extension)
---

### XML style comment

When a source file has a `.html`, `.xml`, or `.xsl` extension, the system shall use `<!-- -->` for comments.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [XML Style Comment Contract Specification](Specifications.md#xml-style-comment-contract-specification)
  * derivedFrom: [Comment Style by File Extension](#comment-style-by-file-extension)
---

### Validating Traceability Format

While processing traceability in code, the system shall ensure that each `[reqvire::...] START` tag has a corresponding `[reqvire::...] END` tag.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Traceability Format](#traceability-format)
---

