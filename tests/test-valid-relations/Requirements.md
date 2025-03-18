# Test Requirements for Valid Relations

This document contains correctly formatted relations to existing targets, to verify validation passes.

## Requirements

### Requirement with Valid Standard Relations

This requirement has valid relations to existing files, using standard format.

#### Relations
* refine: Requirements.md/Requirement with Valid Markdown Relations
* satisfiedBy: specifications/SystemRequirements/Requirements.md
* satisfiedBy: DesignSpecifications/SampleDSD.md

---

### Requirement with Valid Markdown Relations

This requirement has valid relations to existing files, using markdown link format.

#### Relations
* satisfiedBy: [Requirements.md](Requirements.md)
* verifiedBy: [System Requirements](specifications/SystemRequirements/Requirements.md)
* trace: [Design Specification](DesignSpecifications/SampleDSD.md)

---

### Requirement with DesignSpecifications Reference

This requirement specifically tests validation of relations to files in the DesignSpecifications folder.

#### Metadata
* type: requirement

#### Relations
* derivedFrom: DesignSpecifications/SampleDSD.md/Sample Design Element
* satisfiedBy: [Sample DSD](DesignSpecifications/SampleDSD.md)