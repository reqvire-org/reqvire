# Elements


### Test Feature Test Excluded Patterns Specifications Systemrequirements Validfile Md

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature
---

This file should be processed.

### Root Feature

Top-level feature used as hierarchy parent for system requirements in this fixture.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature-test-excluded-patterns-specifications-systemrequirements-validfile-md)
---

### SYS 001 System Requirement

This is a system requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Feature](#root-feature)
  * satisfiedBy: [../DesignSpecifications/ExcludedFile.md](../DesignSpecifications/ExcludedFile.md)
  
---
  
### DM-001 Direct Message Sending

Users must be able to send direct messages to other users.

#### Metadata
* type: requirement
* priority: high

#### Relations
* derivedFrom: [Root Feature](#root-feature)
* satisfiedBy: [../DesignSpecifications/DirectMessages.md](../DesignSpecifications/DirectMessages.md)  
