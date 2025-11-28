# Elements

### Relation Validation Specification

Rules for validating and normalizing relation targets during element creation and manipulation.

#### Details
**Target Format Support:**
- Relative paths from the target file location (e.g., `../UserReqs.md#requirement`)
- Paths relative to git repository root (e.g., `specifications/UserReqs.md#requirement`)
- Same-file references (e.g., `#other-requirement`)

**Normalization Rules:**
- All relation targets must be normalized to git repository root relative format before insertion
- All relation targets must reference existing elements in the model
- External links (http://, https://, etc.) are allowed and not validated

**Validation Behavior:**
- Parse relation targets from the markdown
- Normalize relation targets to be relative to the git repository root
- Validate that each relation target element exists in the model
- Reject the operation if any relation target does not exist
- Provide clear error messages indicating which relation target was not found

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Create Element Operation](ElementManipulation.md#create-element-operation)
---
