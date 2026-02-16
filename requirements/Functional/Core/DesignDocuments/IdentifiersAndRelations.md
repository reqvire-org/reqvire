# Documents

## Metadata
  * type: specification

## Relations
  * refine: [Identifiers and Relations](../StructureAndParsing.md#identifiers-and-relations)

## IdentifiersAndRelations

# Identifiers and Relations Specification

## Identifiers in Markdown Documents

An **identifier** is a location-based reference for addressing elements in markdown documents.
It consists of a file path and optionally an element name (fragment).

**Distinction from Element ID**:
- **Identifier**: Location-based address for targeting elements in relations (`file.md#element-name`)
- **Element ID**: Stable identity independent of location (derived from element name)

Identifiers are used for writing relations in markdown and cross-referencing between elements. The system resolves identifiers to Element IDs during parsing for internal tracking and change detection. When an element is relocated to a different file or section, its identifier changes but its Element ID remains stable.

## Identifier in markdown document can be of several types

-**Identifier**
  - An internal system element reference with fragment, pointing to specific elements within markdown documents.
  - Used for element-to-element relations (e.g., `derivedFrom`, `verifiedBy`, `verify`)
  - **Example**: `"file.md#element-name"`
- **ExternalUrl**
  - An external URL represented as a string.
  - Used for references to external resources
  - **Example**: `"https://example.com"`
- **InternalPath**
  - An internal filesystem file path without fragment, pointing to implementation files.
  - Used for satisfaction and traceability relations (e.g., `satisfiedBy`, `satisfy`, `trace`)
  - **Example**: `"../../core/src/diagrams.rs"`


### Identifier Path Resolution Rules

- If an identifier **starts with `/`**, it is considered relative to the **git repository root folder**.
- If an identifier **does not start with `/`**, it is considered **relative** to the path of the document in which it appears.

Each **identifier** must uniquely reference either:
  - A **file**, or
  - An **element within a file**.

#### Identifier Path Resolution Examples

Assuming the **<git repository root> folder** is `project` and a file exists at `/path/to/project/documents/File1.md`:

| Identifier | Resolves to | Type  |
|------------|------------|-----------|
| `File2.md` | `project/documents/File2.md` | InternalPath |
| `subfolder/File3.md` | `project/documents/subfolder/File3.md` | InternalPath |
| `../File4.md` | `project/File4.md` | InternalPath |
| `/project/File4.md` | `project/File4.md` | InternalPath |
| `https://example.com` | `https://example.com` | ExternalUrl |
| `../Requirments#element-name` | `project/Requirments#element-name` | Identifier |

---

### Identifier Form Variations and Examples

System recognises 2 kinds of identifier that may appear in documents and relations:
 * Simple identifiers
 * GitHub-style Markdown Link Identifiers


Both Simple identifier and link part of GitHub-style markdown identifier can be etiher internal internal paths or external links (eg. starting with known scheme eg. https://)


When parsing identifiers, both styles are nomarlized into the same form used internally by the system.

As part of normailization process, element names are converted to **GitHub-style anchor link** fragments which are internal identifer representations:
  - Convert to **lowercase**.
  - Replace **spaces with hyphens (`-`)**.
  - Remove **disallowed characters**.
  - Remove **leading and trailing whitespace**.


#### 1. Simple Identifiers

Plain file or element references, following the path resolution rules.


Examples:

- File only identifier found in the document '<git repository root>/path/to/document.md'
```
file.md
```
  - Normalized to '<git repository root>/path/to/file.md'

- File with an element fragment in the document '<git repository root>/path/to/document.md':
```
file.md#element name
```
  - Normalized to '<git repository root>/path/to/file.md#element-name'

- Relative path with an element fragment in the document '<git repository root>/path/to/document.md':
```
../relative_path/file.md#element name
```
  - Normalized to '<git repository root>/path/file.md#element-name'

- Element name fragment only (within the same file) in the document '<git repository root>/path/to/document.md':
```
#element name
```
  - Normalized to '<git repository root>/path/to/document.md#element-name'


- Relative path with the element fragment with special characters in the document '<git repository root>/path/to/document.md':
```
path/file.md#My Element (Draft)
```
  - Normalized to '<git repository root>/path/to/path/file.md##my-element-draft'


- Absolute path with the element fragment in any document:
```
/path/file.md#Elements
```
  - Normalized to '<git repository root>/path/file.md#elements'


#### 2. GitHub-style Markdown Link Identifiers

A valid GitHub-style Markdown link to a file or a fragment within a file.
Identifier is considered the **link** part of the markdown link: everything inside '(identifier)'.

Once link part is obtained from GitHub-style Markdown link, it is following same rules for normalization as **simple identifiers**.

Examples:

- File link in '<git repository root>/path/to/document.md':
```
[Specification](documents/specification.md)
```
  - Normalized to '<git repository root>/path/to/path/documents/specification.md'

- Fragment link in '<git repository root>/path/to/document.md':  :
```
[My Element](documents/specification.md#my-element)
```
  - Normalized to '<git repository root>/path/to/path/documents/specification.md#my-element'

---


##  Relations in Markdown Documents

The `#### Relations` subsection specifies associations between elements, files, or other resources, forming the logical and dependency structure of the model.

The `#### Relations` subsection:
- Is a dedicated part of an **element** section in Markdown document.
- Starts with the `#### Relations` header.
- Contains a list of relations in a specific format.


The `#### Relations` subsection must be located within an element chunk.
Each element chunk can have at most one `#### Relations` subsection.

The `#### Relations` header marks the beginning of the subsection.

The `#### Relations` subsection must appear directly within an element  chunk.
It must follow the `###` header of the parent element and any preceding content.


### Relation Structure

- Relation entries are listed as bullet points (`*`), with **two spaces** (`  *`) of indentation.

####  **Relation Format**
   - Relations follow this format:
     ```
     * relationType: **identifier**
     ```
   - Example:
     ```
     * dependsOn: [Element2](#element2)
     ```

####  **Relation Type**
   - Specifies the type of the relationship.
   - Allowed characters: `[a-zA-Z]`
   - Minimum length: **2 characters**
   - Maximum length: **80 characters**
   - Must be one of the predefined, case-sensitive types


####  **Target Identifier**
   - Specifies the target of the relation.
   - Must be a valid **Simple Identifier** or **Git Valid Markdown Link Identifier** as defined in this document.



---

### Examples of `#### Relations`

#### 1. **Simple Relations**
```markdown


### My Element
This is the content of My Element.

#### Relations
  * dependsOn: [Element2](#element2)
  * relatedTo: [path/to/anotherFile.md/Section3](path/to/anotherFile.html#section3)
  * uses: [file.md](file.html)
```

#### 2. **Relations with Special Characters**
```markdown


### API v2.0
Details about API version 2.0.

#### Relations
  * satisfies: [documents/specification.md#API: v2.0](documents/specification.html#api-v20)

```

#### 3. **Relation to an Element in the Same File**
If the referenced element exists within the same file, the identifier can be a fragment only.

```markdown

#### Relations
  * extends: [Another Section](#another-section)
```

#### 4. **Relative Path Relations**
If the referenced file is located in a subfolder relative to the current document, use a relative path.
```markdown

#### Relations
  * derive: [subfolder/details.md#refined-section](subfolder/details.html#refined-section)

```

#### 5. **Absolute Path Relations**

If the reference starts with /, it points to a file or element relative to the git repository root folder.

```markdown

#### Relations
  * verifiedBy: [/specifications.md#verification-steps](/specifications.html#verification-steps)

```

#### 6. **Invalid Relations Example**

```markdown

This element contains invalid relation entries.

#### Relations
  * derivedFrom: [Element2](#element2)
* InvalidEntry
* : MissingRelationType
  * trace: [path/to/file.md](path/to/file.html)
```

## Identifier Usage in Relations

Identifiers are used in relations to reference files or specific elements within files. Examples:

1. **Relation to a File**:
   ```markdown
   #### Relations
     * satisfiedBy: [documents/specification.md](documents/specification.html)
   ```

2. **Relation to an Element**:
   ```markdown
   #### Relations
     * derivedFrom: [documents/specification.md/section one](documents/specification.html#section-one)
   ```


## Validation rules

The system must validate relation usage according to these rules:
- Only the relation types defined in this registry are allowed
- Relations should connect elements of appropriate types
- Circular dependencies should be detected and reported
- Duplicate relation entries of same type and target are not allowed
- **Identifier** targets (with fragments) must reference existing elements in markdown documents
- **InternalPath** targets (without fragments) must reference existing files in the filesystem
- **ExternalUrl** targets are not validated for existence
