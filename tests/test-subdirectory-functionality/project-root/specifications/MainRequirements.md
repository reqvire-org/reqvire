# Elements


### Test Capability Test Subdirectory Functionality Project Root Specifications Mainrequirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

This file contains main requirements that should NOT be processed when run from a subdirectory.


### Main Requirement One

This is a main requirement that should be ignored when processing only the submodule.

#### Relations
  * derivedFrom: [Main System](#main-system)

---

### Main System

This is the main system requirement.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-subdirectory-functionality-project-root-specifications-mainrequirements-md)
    * derive: [Main Requirement One](#main-requirement-one)

---