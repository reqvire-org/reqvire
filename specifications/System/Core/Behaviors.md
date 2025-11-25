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

### Attachment Input Auto-Detection Behavior

When attaching or detaching via CLI commands, the system shall auto-detect whether the input refers to a file path or a Refinement element name.

#### Details
The detection follows this priority order:

1. **File Path Check (Priority)**: Check if the input exists as a file on the filesystem
   - Check relative to current working directory
   - Check relative to git root directory
   - If file exists, treat as file path attachment

2. **Element Name Lookup (Fallback)**: If no file exists, attempt to resolve as element name
   - Search for element by display name in the model
   - Element must be a Refinement type (constraint, behavior, specification)
   - Convert element name to identifier format for storage

3. **Error Handling**: If neither file nor element found, report clear error message indicating what was attempted

This behavior ensures backward compatibility (existing file attachments work unchanged) while enabling element attachments without requiring explicit flags.

#### Metadata
  * type: behavior
---
