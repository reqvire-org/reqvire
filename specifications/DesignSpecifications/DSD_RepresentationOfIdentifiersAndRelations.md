# Specification Document: Representation of Identifiers and Relations in the System


This document defines how **Identifiers** and **Relations** are to be represented within the system after being parsed from Markdown documents. 
The design ensures consistency, validity, and efficient querying and manipulation of these entities.

## Identifiers

Identifiers represent files or specific fragments (elements) within files. They must adhere to strict rules to ensure uniqueness and validity.

### Definition

1. **File Identifier**:
   - Represents an entire file.
   - Example:
     ```
     documents/specification.md
     ```

2. **Fragment Identifier**:
   - Represents a specific element within a file.
   - The fragment is derived from the element's name, URL-encoded to ensure validity.

   - Example:
     ```
     documents/specification.md#my%20element
     ```

### Rules for Identifiers

1. **URL Encoding**:
   - Fragments must be URL-encoded according to [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986).
     - Spaces (` `) → `%20`
     - Open parenthesis (`(`) → `%28`
     - Close parenthesis (`)`) → `%29`
   - This ensures that special characters in fragment names are safely represented.

2. **Uniqueness**:
   - Each identifier must uniquely reference:
     - A file, or
     - An element within a file.

3. **No Absolute Paths**:
   - Identifiers must always use relative paths from the defined root folder.

## Relations

Relations define associations between elements, files, or other resources. 

### Relation Structure

1. **Relation Type**:
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
  
2. **Target Identifier**:
   - Specifies the target of the relation.
   - Follows the rules for identifiers defined in this document.

### Rules for Relations

1. **Valid Relation Types**:
   - Relation types must be one of defined types.
   - Invalid or undefined types should be rejected during parsing.
   - Duplicated relations should be rejected during a parsing.

2. **Target Validation**:
   - The target identifier must be a valid file or fragment identifier.



