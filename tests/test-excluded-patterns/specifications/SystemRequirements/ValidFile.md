# Elements


### Test Capability Test Excluded Patterns Specifications Systemrequirements Validfile Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

This file should be processed.

### Root Capability

Top-level capability used as hierarchy parent for system requirements in this fixture.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-excluded-patterns-specifications-systemrequirements-validfile-md)
---

### SYS 001 System Requirement

This is a system requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Capability](#root-capability)
  * satisfiedBy: [../DesignSpecifications/ExcludedFile.md](../DesignSpecifications/ExcludedFile.md)
  
---
  
### DM-001 Direct Message Sending

Users must be able to send direct messages to other users.

#### Metadata
* type: requirement
* priority: high

#### Relations
* derivedFrom: [Root Capability](#root-capability)
* satisfiedBy: [../DesignSpecifications/DirectMessages.md](../DesignSpecifications/DirectMessages.md)  
