# Design Specification Document: Structure and Addressing in Markdown Documents

This document defines the structure, rules, and usage of **Elements**, **Subsections** including **Relations**, and **Details**, as well as **Identifiers** in Markdown (`.md`) documents. 


## Elements

An **Element** is a uniquely identifiable section within a Markdown document and form the foundation of the model’s structure and relationships.
An **Element** is equivalent to a **node** in the internal representation of the model structure.

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

See more in design specification document [RepresentationOfIdentifiersAndRelations.md](RepresentationOfIdentifiersAndRelations.md).

##  Subsections

An element may contain different **Subsections**, some of which are strictly defined, while others allow free-form content.
- **Reserved Subsections**: These subsections follow a predefined structure.
- **Other Subsections**: These allow additional descriptive or supporting information.

Subsections starts with the `#### Subsection Name` and ends either with new element or next subsection.
Subsection must be located **within an element chunk**.
The `#### ` header marks the beginning of the subsection.
It must appear directly within an element chunk, **following** the `###` header of the parent element and any preceding content, including previous subsections.
Each element chunk can have **at most one** `#### SubsectionName` subsection where 'SubsectionName' is a unique name of the subsection within an element.

The reserved subsections are:
 * Relations
 * Details
 * Properties
 * Metadata
 
Those have defines structure that must be followed.



### Details Subsection

Must be defined with a level 4 header: `#### Details`.

The **#### Details** subsection within an element provides additional information directly related to the main requirement text.

- Content within the **Details** subsection is considered an **extension of the requirement text**.
- Any statements in the **Details** subsection hold the same validity as the main requirement text.



###  Relations Subsection

Must be defined with a level 4 header: `#### Relations`.

Duplicate relation entries within the same `#### Relations` subsection are not allowed.

See more in design specification document [IdentifiersAndRelations.md](IdentifiersAndRelations.md).



### Metadata Subsection

Must be defined with a level 4 header: `#### Metadata`.

The metadata section of an element follows these rules:
1. Contains properties in list format: `* property_name: property_value`
2. Property entries are listed as bullet points (`*`), with **two spaces** (`  *`) of indentation followed by property_name + ': ' + property_value.
3. May include any custom properties, not just `type`

#### Reserved Properties

The following properties have special meaning:

- `type`: Defines the element type (requirement, verification, etc.)
- Additional reserved properties may be defined in future releases

#### Example Metadata Section

```markdown


### My Element

This is a verification element.

#### Metadata
  * type: verification
  * priority: high
  * owner: team-a

#### Relations
* verifies: [Some Requirement](#some-requirement)
```

```markdown


### My Element

This is a verification element.

#### Details

Some details.

#### Metadata
  * type: verification
  * priority: high
  * owner: team-a

#### Relations
  * verifies: [Some Requirement](#some-requirement)
```

#### Metadata Parsing

When parsing elements from Markdown:
1. The parser identifies `#### Metadata` sections within elements
2. Properties are extracted from list items following the metadata header
3. The element type is set based on the metadata "type" property if present otherwise defaults to a **requirement** type.
4. All metadata is stored for potential future use.


Elements types are defined in a **#### Metadata**  **subsection** within an **element** as a **type** property and value, see more for structure of elements in  [ElementsInDocument.md](ElementsInDocument.md).


### Element Types in Metadata Section

#### Supported Element Types

Element types are identified through a reserved "type" metadata property. The following types are supported:

1. **requirement**: The default element type when no type is specified
2. **verification**: For verification tests and validation procedures
3. **other**: Custom element types defined by users

#### Type Determination

The type of an element is determined through the following process:

1. If a `#### Metadata` subsection exists and includes a `type` property, use that value
2. If no type is specified, default to `Requirement`
3. Future versions may add more built-in types as needed


### Element algebraic data type representation


Relation ADT:
```
pub enum RelationDirection {
    Forward,   // From source to target (e.g., "refine")
    Backward,  // From target to source (e.g., "refinedBy")
    Neutral    // Non-directional (e.g., "trace")
}

pub struct RelationType {
    pub name: &'static str,          // Name of the relation type
    pub direction: RelationDirection, // Direction of relation
    pub opposite: Option<&'static str>, // The opposite relation name, if any
    pub description: &'static str,    // Description of what the relation means
}

pub struct Relation {
    /// Type of the relation (e.g., dependsOn, verifiedBy)
    pub relation_type: RelationType,
    
    /// Target identifier of the relation (normalized)
    pub target: String,
    
    /// raw URL/Link part of the target identifier if was a markdown link representation or identifer in raw form otherwise
    pub name: String,
}
```

Element ADT:
```

pub enum ElementType {
    Requirement,
    Verification,
    File,
    Other(String),
}

pub struct Element {
    /// Name of the element (from the level 3 header)
    pub name: String,
    
    /// Content of the element (excluding relations)
    pub content: String,
    
    /// Relations defined within this element
    pub relations: Vec<Relation>,

    /// Normalized identifier
    pub identifier: String,
        
    /// Normalized file path
    pub file_path: String,
    
    /// Type of element (requirement, verification, etc.)
    pub element_type: ElementType,
    
    /// Additional metadata properties for the element
    pub metadata: HashMap<String, String>,
}
```

Files ADT (a separate system representation is kept with all relations that were pointing to files and not elements in documents):
```
pub struct File {
    /// Filename of the file with extension
    pub name: String,
    
    /// Normalized identifier
    pub identifier: String,
        
    /// Normalized file path
    pub file_path: String,
}
```

See specifications for normalizing identifiers and file_path in [IdentifiersAndRelations.md](IdentifiersAndRelations.md).
