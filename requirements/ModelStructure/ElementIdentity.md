# Element

## Metadata
  * type: specification

## Relations
  * refine: [Element Identity Model](StructureAndParsing.md#element-identity-model)

## ElementIdentity

# Element Identity Specification

## Element ID vs Identifier

The system maintains two distinct concepts for element identification:

Element identity and addressing concepts are defined by the Reqvire core element ontology. This design document specifies how the parser and change-detection implementation apply those concepts.

### Element ID

An **Element ID** is a stable unique identifier for element identity:

- **Source**: Derived from the element name (H3 header text)
- **Uniqueness**: Globally unique across the entire model
- **Stability**: Remains unchanged when element is relocated between files or sections
- **Purpose**: Used internally for change detection and tracking element identity across versions
- **Format**: Normalized element name following GitHub fragment identifier rules
- **Visibility**: Internal to the system, not directly visible in markdown documents

**GitHub Fragment Normalization Rules:**
1. Convert all letters to lowercase
2. Replace spaces with hyphens (-)
3. Remove all punctuation characters (except hyphens and underscores)
4. Remove all other whitespace characters (tabs, newlines, etc.)
5. Trim leading and trailing whitespace before processing
6. Keep alphanumeric characters, hyphens, and underscores only

Example transformations:
- `"My Capability Name"` -> `"my-capability-name"`
- `"Version 1.2.3"` -> `"version-123"` (dots removed)
- `"Installation (Windows)"` -> `"installation-windows"` (parentheses removed)
- `"C++ API Reference"` -> `"c-api-reference"` (++ removed)
- `"my_variable_name"` -> `"my_variable_name"` (underscores kept)

### Element Identifier

An **Element Identifier** is a location-based reference for addressing:

- **Source**: Combination of file path and element name fragment
- **Format**: `file_path#element-name-fragment` (e.g., `specifications/File.md#element-name`)
- **Purpose**: Used for relationship targeting in Relations subsections and cross-referencing
- **Usage**: What users write in markdown to reference elements
- **Variability**: Changes when element is relocated (file_path component changes)
- **Resolution**: System resolves identifiers to element IDs during parsing

## Relationship Between ID and Identifier

- Users write relations using **identifiers** (location-based references in markdown)
- System resolves identifiers to **element IDs** during parsing for internal tracking
- One element ID can have different identifiers over time due to relocation
- Identifier changes are detected as relocations, not identity changes

## Implicit Containment Model

Element location is tracked separately from element identity:

Element location is a core ontology concept used by parser and report behavior; it is not an authored relation family.

- **file_path field**: Records which file contains the element (implicit file containment)
- **No explicit relations**: Containment is not expressed as relations in the Relations subsection
- **Location changes**: Detected as relocations when file_path changes
- **Identity preservation**: Element ID remains stable across location changes

## Change Detection Using Element IDs

When comparing model versions:

1. Elements are matched by **Element ID** (not identifier)
2. Change types detected:
   - **Content change**: Same ID, different content hash
   - **Addition**: ID exists only in new version
   - **Removal**: ID exists only in old version
   - **Relocation**: Same ID, different file_path (without content change)
3. Pure relocations do not trigger impact propagation
4. Relocations with content changes propagate based on content change only

## Examples

### Example 1: Element with ID and Identifier

**Markdown in** `specifications/requirements.md`:
```markdown
### User Authentication

The system shall provide secure user authentication.

#### Relations
  * derivedFrom: [Security Requirements](security.md#Security-Requirements)
```

- **Element ID**: `user-authentication` (stable, internal)
- **Element Identifier**: `specifications/requirements.md#user-authentication` (current location)
- **file_path**: `specifications/requirements.md` (implicit containment)

### Example 2: Element Relocation

**Before** - in `specifications/requirements.md`:
```markdown
### User Authentication
Content here.
```
- **Element ID**: `user-authentication`
- **Identifier**: `specifications/requirements.md#user-authentication`

**After** - moved to `specifications/security/auth.md`:
```markdown
### User Authentication
Content here.
```
- **Element ID**: `user-authentication` (unchanged)
- **Identifier**: `specifications/security/auth.md#user-authentication` (changed)
- **Detection**: Change detection recognizes same ID -> relocation, not removal + addition

### Example 3: Writing Relations

**User writes in markdown**:
```markdown
#### Relations
  * derivedFrom: [Parent Req](../parent.md#Parent-Req)
  * verifiedBy: [Test Case](#Test-Case)
```

**System processing**:
1. Parse identifier: `../parent.md#Parent-Req`
2. Resolve to absolute: `specifications/parent.md#parent-req`
3. Resolve to Element ID: `parent-req`
4. Store relation using Element ID internally
5. Change detection compares using Element ID
