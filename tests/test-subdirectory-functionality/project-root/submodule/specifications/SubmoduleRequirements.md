# Submodule Requirements

This file contains submodule requirements that SHOULD be processed when run from the submodule directory.

## Submodule Requirements

### Submodule Requirement One

This is a submodule requirement that should be processed when run from the submodule directory.

#### Relations
  * derivedFrom: [Submodule System](#submodule-system)

---

### Submodule System

This is the submodule system requirement.

#### Metadata
  * type: user-requirement

#### Relations
    * derive: [Submodule Requirement One](#submodule-requirement-one)

---

### Cross Reference Test

This requirement references something outside the current directory to test identifier normalization.

#### Relations
  * derivedFrom: [../../specifications/MainRequirements.md#main-system](../../specifications/MainRequirements.md#main-system)

---