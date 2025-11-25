# Elements

### Attachment Identifier CRUD Update Behavior

When a Refinement element is moved or renamed through CRUD operations, all attachment identifiers referencing that element must be updated using the same update mechanism as relation target updates.

#### Details
The update process follows these steps:

1. **Identify affected attachments**: Find all elements that have attachment identifiers pointing to the affected Refinement element
2. **Update identifier paths**: For each affected attachment:
   - **On move**: Update the file path portion of the identifier to reflect the new location
   - **On rename**: Update the element name portion of the identifier (fragment) to reflect the new name
3. **Preserve link text**: The display text of the markdown link is preserved
4. **File persistence**: Modified files are written back to disk with updated attachments

This behavior mirrors the existing relation target update behavior used when moving or renaming elements, ensuring consistency across the model.

#### Metadata
  * type: behavior
---
