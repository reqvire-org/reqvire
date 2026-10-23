# Element

## Metadata
  * type: specification

## Relations
  * define: [Structure and Addressing in Markdown Documents](StructureAndParsing.md#structure-and-addressing-in-markdown-documents)

## MarkdownStructure

# Markdown Structure Specification

## Supported Document Forms

The first H1 selects the document grammar. A multi-element document starts with `# Elements`; a single-element document starts with `# Element`. Headings inside fenced code are literal content in either form.

### Multi-Element Documents

- Each element starts with a nonempty `### <Element Name>` heading.
- Its content extends to the next structural heading of the same or higher level, or the end of the document.
- Reserved element subsections use H4 headings, such as `#### Metadata`, `#### Relations`, and `#### Details`, subject to element-type applicability.
- Ordinary H2 page sections organize source text without creating graph elements or changing logical ownership. Core placement indices count elements across the file.
- Formatting preserves page content and document form while normalizing element spacing and separators.

```markdown
# Elements

### Example Model

Ability to describe an example system.

#### Metadata
  * type: capability

---

### Example Requirement

The system SHALL provide the example behavior.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Example Model](#example-model)
```

### Single-Element Documents

- The file contains exactly one named element.
- `## Metadata` supplies its type and other permitted metadata; `## Relations` supplies optional structural declarations.
- A dynamic `## <Actual Element Name>` section supplies the element name and body. H2 sections are structural in this document form.
- The named body may contain arbitrary Markdown headings. They remain content of the single element rather than creating siblings.
- Formatting retains the H2 metadata, relation and named-body layout; it does not convert the element to an H3 collection entry.

```markdown
# Element

## Metadata
  * type: specification

## Relations
  * define: [Example Requirement](Requirements.md#example-requirement)

## Example Behavior Specification

Definition of the example behavior.

### Payload

Detailed payload content remains part of this specification.
```

## Names, Identity and Addressing

- Element names are globally unique across the effective model scope.
- The element ID is derived from its name; the canonical identifier combines its workspace-relative file path and normalized name fragment.
- Relocation changes addressing while retaining the name-derived identity. Source folders and files describe containment rather than logical capability or requirement ownership.
- Empty names, duplicate names, incompatible relations and invalid reserved subsections are rejected according to the shared model rules.

## Element Content and Reserved Subsections

Body text and permitted non-reserved subsections carry readable domain prose. Reserved subsections carry structured metadata, relations, contract dependencies, concept references, ontology, shapes or query content according to the element type.

In a multi-element document, each reserved H4 subsection occurs at most once within an element and ends at the next subsection or structural element boundary. Single-element metadata and relations use the H2 layout above; the named body retains its own Markdown content.

Requirements contain obligations and intent-level clarification; requirement-owned contracts contain precise definitions and interface behavior. Capabilities describe abilities. Concepts define terminology, ontology defines structural meaning, and semantic contracts contain reusable SHACL checks.

Readable prose does not hyperlink to model elements. Declare dependencies on responsible requirements with Contract Bindings or Contract References, and bind terminology through permitted Concept References. Literal code examples retain their authored links and headings as content.
