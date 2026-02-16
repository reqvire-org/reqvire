# Elements

This file should be processed.

### Root User Requirement

Top-level user requirement used as hierarchy parent for system requirements in this fixture.

#### Metadata
  * type: user-requirement

---

### SYS 001 System Requirement

This is a system requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root User Requirement](#root-user-requirement)
  * satisfiedBy: [../DesignSpecifications/ExcludedFile.md](../DesignSpecifications/ExcludedFile.md)
  
---
  
### DM-001 Direct Message Sending

Users must be able to send direct messages to other users.

#### Metadata
* type: requirement
* priority: high

#### Relations
* derivedFrom: [Root User Requirement](#root-user-requirement)
* satisfiedBy: [../DesignSpecifications/DirectMessages.md](../DesignSpecifications/DirectMessages.md)  
