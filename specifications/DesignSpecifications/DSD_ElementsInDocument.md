# Design Specification Document: Structure and Addressing in Markdown Documents

This document defines the structure, rules, and usage of **Elements**, **Subsections** including **Relations**, and **Details**, as well as **Identifiers** in Markdown (`.md`) documents. 


## Elements

An **Element** is a uniquely identifiable section within a Markdown document and form the foundation of the model’s structure and relationships:

Elements may contain structured **Subsections**, each serving a specific purpose.

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
    - Subheaders (e.g., `####`) representing a start of subsection.
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

5. **Allowed Characters**
   - **Letters**: `a-z`, `A-Z`
   - **Numbers**: `0-9`
   - **Spaces**: ` `
   - **Special characters allowed**: `-`, `_`, `.` (hyphen, underscore, and period)


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

## Identifiers

See more in design specification document [DSD_RepresentationOfIdentifiersAndRelations.md](DSD_RepresentationOfIdentifiersAndRelations.md).

##  Subsections

An element may contain different **Subsections**, some of which are strictly defined, while others allow free-form content.
- **Reserved Subsections**: These subsections follow a predefined structure.
- **Other Subsections**: These allow additional descriptive or supporting information.

Subsections starts with the `#### Subsection Name` and ends either with new element or next subsection.

The reserved subsections are:
 * Relations
 * Details
 * Properties
 * Metadata
 
Those have defines structure that must be followed.

### Details Subsection

The **#### Details** subsection within an element provides additional information directly related to the main requirement text.

- Content within the **Details** subsection is considered an **extension of the requirement text**.
- Any statements in the **Details** subsection hold the same validity as the main requirement text.

###  Relations Subsection

- The `#### Relations` subsection must be located **within an element chunk**.
- Each element chunk can have **at most one** `#### Relations` subsection.
- The `#### Relations` header marks the beginning of the subsection.
- It must appear directly within an element chunk, **following** the `###` header of the parent element and any preceding content, including previous subsections.
- Duplicate relation entries within the same `#### Relations` subsection are not allowed.

See more in design specification document [DSD_RepresentationOfIdentifiersAndRelations.md](DSD_RepresentationOfIdentifiersAndRelations.md).

