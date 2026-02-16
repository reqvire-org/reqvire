# Documents

## Metadata
  * type: specification

## Relations
  * refine: [Structure and Addressing in Markdown Documents](../StructureAndParsing.md#structure-and-addressing-in-markdown-documents)

## MarkdownStructure

# Markdown Structure Specification

## Elements in Markdown Documents

An **Element** is a uniquely identifiable system element within a Markdown document. It starts with a `###` header and includes all content under that header until the next header of the same or higher hierarchy.

### Structure of an Element

1. **Element Header**
  - The `###` header defines the start of an element.
  - The text of the `###` header serves as the **element name**.
  - The element name must be globally unique to ensure unambiguous references.

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

2. **Global Uniqueness**:
   - Element names must be globally unique across all files in the model.
   - Element names serve as stable IDs for element identity independent of file location.
   - File location is the only containment property tracked by the system.

3. **Nested Subheaders**:
   - Subheaders within an element defined with `####` header are part of the same element and do not create new elements.

4. **No Overlapping Content**:
   - Content in an element belongs exclusively to that element and cannot overlap with another.

## Section Headers (H2) - Not Tracked

Section headers (`## Header`) may exist in markdown documents for visual organization but are **not tracked** by the system:
- The parser ignores `##` headers during model construction
- Elements are indexed by their position within the file, not within sections
- When writing/formatting files, existing `##` headers are preserved but not managed
- The `--index` parameter in CLI commands refers to element position in the entire file

### Examples of Elements

Single Element:
```markdown
### My Element

This is the content of My Element.

#### Subsection
Additional details about My Element.
```

Multiple Elements:
```markdown
### Element One

This is the content of Element One.

### Element Two

This is the content of Element Two.
```

Nested Subheaders:
```markdown
### Main Element
This is the main element content.

#### Subsection
Details about the subsection.

#### Another Subsection
More details about another subsection.
```

### Invalid Cases

Element headers empty:
```
###
```

Headers not unique within the model:
```markdown
### Duplicate
Content of the first duplicate.

### Duplicate
Content of the second duplicate.
```

## Subsections in Markdown documents

An element may contain different **Subsections**, some of which are strictly defined, while others allow free-form content.
- **Reserved Subsections**: These subsections follow a predefined structure and behavior.
- **Other Subsections**: These allow additional descriptive or supporting information.

Subsections starts with the `#### Subsection Name` and ends either with new element or next subsection.
Subsection must be located **within an element chunk**.

The `#### ` header marks the beginning of the subsection.
It must appear directly within an element chunk, **following** the `###` header of the parent element and any preceding content, including previous subsections.
Each element chunk can have **at most one** `#### SubsectionName` subsection where 'SubsectionName' is a unique name of the subsection within an element.

Some subsections are **reserved** with predefined structure and behavior, while others allow free-form content. The specific reserved subsections are defined separately as capabilities of the system.
