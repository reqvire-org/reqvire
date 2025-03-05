# Test Requirements for Missing Targets

This document contains intentional errors for testing validation of missing targets, as well as valid targets to ensure both cases are handled correctly.

## Requirements

### Requirement with Missing Target Reference

This requirement has a relation to a non-existent element, using a non-markdown link format.

#### Relations
* refine: UserRequirements.md/MissingRequirement
* satisfiedBy: NonExistentFile.md

---

### Requirement with Markdown Link to Missing Target

This requirement has a relation to a non-existent element, using markdown link format where the display text and URL don't match.

#### Relations
* satisfiedBy: [DesignSpecifications/API.md](DesignSpecifications/API2.md)
* verifiedBy: [Existing File](specifications/SystemRequirements.md)

---

### Requirement with Valid Relations

This requirement has valid relations to existing files in the model, to verify that validation correctly handles both valid and invalid targets.

#### Relations
* refine: Requirements.md/Requirement with Missing Target Reference
* trace: [This document](Requirements.md)