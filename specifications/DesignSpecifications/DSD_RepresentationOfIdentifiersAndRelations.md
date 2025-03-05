# Specification Document: Representation of Identifiers and Relations in the System

This document defines how **Identifiers** and **Relations** are to be represented within the system after being parsed from Markdown documents.  
The design ensures consistency, validity, and efficient querying and manipulation of these entities.

## Identifiers

An **identifier** consists of a path following a filename with an extension (e.g., `file.md`) and optionally an **element** name (fragment).  

### Path Resolution Rules

- If an identifier **starts with `/`**, it is considered relative to the **specifications root folder**.
- If an identifier **does not start with `/`**, it is considered **relative** to the path of the document in which it appears.

Each **identifier** must uniquely reference either:
  - A **file**, or
  - An **element within a file**.

### Identifier Types and Examples

#### 1. **Simple Identifiers**
   - Plain file or element references, following the path resolution rules.
   - Examples:
     - File only:  
       ```
       file.md
       ```
     - File with an element fragment:  
       ```
       file.md#element name
       ```
     - Relative path with an element fragment:  
       ```
       relative_path/file.md#element name
       ```
     - Element name fragment only (within the same file):  
       ```
       element name
       ```
     - Element fragment with special characters:  
       ```
       path/file.md#My Element (Draft)
       ```

#### 2. **Git Valid Markdown Link Identifiers**
   - A valid GitHub-style Markdown link to a file or a fragment within a file.
   
   - When converting an element name to a **GitHub-style anchor link**, apply the following transformations:
     - Convert to **lowercase**.
     - Replace **spaces with hyphens (`-`)**.
     - Remove **disallowed characters**.
     - Remove **leading and trailing whitespace**.
   
   - Examples:
     - File link:  
       ```
       [Specification](documents/specification.md)
       ```
     - Fragment link:  
       ```
       [My Element](documents/specification.md#my-element)
       ```

### Path Resolution Examples

Assuming the **specifications root folder** is `/path/to/project`  
And a file exists at `/path/to/project/documents/File1.md`

| Identifier | Resolves to |
|------------|------------|
| `File2.md` | `/path/to/project/documents/File2.md` |
| `subfolder/File3.md` | `/path/to/project/documents/subfolder/File3.md` |
| `/File4.md` | `/path/to/project/File4.md` |
| `../File4.md` | `/path/to/project/File4.md` |

---

## Relations

The `#### Relations` subsection specifies associations between elements, files, or other resources, forming the logical and dependency structure of the model.  

Relations must follow a **structured reference format** to ensure clear traceability.

### Relation Structure

- Relation entries are listed as bullet points (`*`), with **two spaces** (`  *`) of indentation.

#### 1. **Relation Format**
   - Relations follow this format:
     ```
     * relationType: **identifier**
     ```
   - Example:
     ```
     * dependsOn: [Element2](#element2)
     ```

#### 2. **Relation Type**
   - Specifies the type of the relationship.
   - Allowed characters: `[a-zA-Z]`
   - Minimum length: **2 characters**  
   - Maximum length: **80 characters**  
   - Must be one of the predefined, case-sensitive types:
     - containedBy
     - contain
     - derivedFrom
     - derive
     - refine
     - satisfiedBy
     - satisfy
     - verifiedBy
     - verify
     - tracedFrom
     - trace

#### 3. **Target Identifier**
   - Specifies the target of the relation.
   - Must be a valid **Simple Identifier** or **Git Valid Markdown Link Identifier** as defined in this document.

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
  * refines: [subfolder/details.md#refined-section](subfolder/details.html#refined-section)

```
#### 5. **Absolute Path Relations**

If the reference starts with /, it points to a file or element relative to the root specifications folder.

```markdown
#### Relations
  * verifiedBy: [/specifications.md#verification-steps](/specifications.html#verification-steps)

```

## Additional Rules for Relations (Continued)

### 6. **Handling of Special Characters in Identifiers**
   - Identifiers within relations must correctly preserve special characters unless they are part of a Git Valid Markdown Link Identifier, which follows GitHub-style encoding.
   - Simple Identifiers retain the original element name, including spaces and special characters.
   - Git Valid Markdown Link Identifiers encode element names:
     - `My Element (Draft)` → `my-element-draft`
     - `API v2.0` → `api-v20`

   **Examples:**
   ```markdown
   #### Relations
     * refines: [API v2.0](file.md#api-v20) ✅ (Git Valid Markdown Link Identifier)
     * refines: file.md#API v2.0 ✅ (Simple Identifier)
   ```



