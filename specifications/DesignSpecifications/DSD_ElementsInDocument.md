# Design Specification Document: Structure and Addressing in Markdown Documents

This document defines the structure, rules, and usage of **Elements**, **Relations**, and **Identifiers** in Markdown (`.md`) documents. 

- **Elements** represent distinct, identifiable sections within a document and are the foundation of the model's structure and relationships.  
- The **Relations** subsection within elements specifies associations between elements, files, or other resources, forming the logical and dependency structure of the model.  
- **Identifiers** are relative URIs derived from a defined root folder, providing unambiguous references to files or specific fragments (elements) within files, serving as a consistent addressing mechanism for the model.  


## Elements in Markdown Documents

An **Element** is a uniquely identifiable section within a Markdown document. It starts with a `###` header and includes all content under that header until the next header of the same or higher hierarchy.

### Structure of an Element

1. **Element Header**
  - The `###` header defines the start of an element.
  - The text of the `###` header serves as the **element name**.
  - The element name must be unique within the same document to ensure unambiguous references.

2. **Element Content**
  - The element includes all content under the `###` header until:
    - The next `###` header, or
    - A higher-level header (`##`, `#`), or
    - The end of the document.
  - The content can include:
    - Text
    - Subheaders (e.g., `####`)
    - Bullet points, code blocks, tables, etc.


## Rules for Elements

1. **Header Format**:
   - An element must start with a `###` header.
   - The `###` header text must not be empty.

2. **Uniqueness**:
   - Element names must be unique within the same file.
 
3. **Nested Subheaders**:
   - Subheaders within an element (e.g., `####`) are part of the same element and do not create new elements.

4. **No Overlapping Content**:
   - Content in an element belongs exclusively to that element and cannot overlap with another.


### Examples of Elements

Single Element:
```markdown
### My Element

This is the content of My Element.

#### Subsection
Additional details about My Element.
```

Multiple Elements:
```
### Element One

This is the content of Element One.

### Element Two

This is the content of Element Two.
```

Nested Subheaders:
```
### Main Element
This is the main element content.

#### Subsection
Details about the subsection.

#### Another Subsection
More details about another subsection.
```

### Invalid Cases

Element headers not empty:
```
###
```

Headers not unique within the same document:
```
### Duplicate
Content of the first duplicate.

### Duplicate
Content of the second duplicate.
```

## Identifiers in Markdown Documents

The **identifier** is represented as a path following a filename with extension (eg file.md) and optionally an **element** name.
Identifier path if starting with '/' is considered starting from a documentation 'root' folder.
If path doesn't not start with the '/', it is considered **relative** to a path of the document itself.


Each **identifier** must uniquely reference either:
  * A file, or
  * An element within a file.


Examples of Identifiers:
  - When documentation **root** folder is : `/path/to/project`
  - And a file path: `/path/to/project/documents/File1.md`
  - Identifiers in the document would resolve to:
    - 'File2.md' -> path/to/project/documents/File2.md'
    - 'subfolder/File3.md' -> path/to/project/documents/subfolder/File3.md'    
    - '/File4.md' -> '/path/to/project/File4.md'
    - '../File4.md' -> '/path/to/project/File4.md'    

Other Examples of Identifiers:
 * Relative path with element fragment: 'relative_path/file.md/element name'.
 * File with element fragment: 'file.md/element name'.
 * File only: 'file.md'.
 * Element name fragment only: 'element name'.
 * Element fragment with special characters: 'path/file.md/My Element (Draft)'



##  Relations in Markdown Documents

The `#### Relations` subsection:
- Is a dedicated part of an **element** section in Markdown document.
- Starts with the `#### Relations` header.
- Contains a list of relations in a specific format.


The `#### Relations` subsection must be located within an element chunk.
Each element chunk can have at most one `#### Relations` subsection.

The `#### Relations` header marks the beginning of the subsection.

The `#### Relations` subsection must appear directly within an element  chunk.
It must follow the `###` header of the parent element and any preceding content.

Relation entries are listed as bullet points (`*`) with 2 spaces '  *' indentation from the beginning of the line, and follow this format:
 * relationType: **identifier**

**relationType**:
 - Specifies the type of the relationship.
 - Allowed characters are [a-zA-z].
 - Min 2 characters and Max 80 characters.

**identifier**:
 - Specifies the target of the relation.
 - For the details see **identifier** specification.

Duplicate relation entries within the same `#### Relations` subsection are not allowed.


### Examples of `#### Relations`

Simple Relations:
```markdown
### My Element
This is the content of My Element.

#### Relations
  * dependsOn: Element2
  * relatedTo: path/to/anotherFile.md/Section3
  * uses: file.md
```

Relations with Special Characters:
```
### Complex Element
This is the content of a complex element.

#### Relations
* dependsOn: Element (Alpha & Beta)
* relatedTo: path/to/special File.md/Section (Draft)
```

Invalid Entries:
```
### Invalid Relations Example
This element contains invalid relation entries.

#### Relations
* dependsOn: Element2
* InvalidEntry
* : MissingRelationType
* relatedTo: path/to/file.md
```



## Identifier Usage in Relations

Identifiers are used in relations to reference files or specific elements within files. Examples:

1. **Relation to a File**:
   ```markdown
   #### Relations
     * satisfiedBy: documents/specification.md  
   ```
    
2. **Relation to an Element**:
   ```markdown
   #### Relations
     * dependsOn: documents/specification.md/section one      
   ```


