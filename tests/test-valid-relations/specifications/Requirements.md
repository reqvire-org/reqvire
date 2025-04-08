# Test Requirements for Valid Relations

This document contains correctly formatted relations to existing targets, to verify validation passes.

## Requirements

### Requirement with Valid Standard Relations

This requirement has valid relations to existing files, using standard format.

#### Relations
* refine: Requirements.md#Requirement with Valid Markdown Relations
* satisfiedBy: DesignSpecifications/SampleDSD.md

---

### Requirement with Valid Markdown Relations

This requirement has valid relations to existing files, using markdown link format.

#### Relations
* satisfiedBy: [./DesignSpecifications/SampleDSD.md](./DesignSpecifications/SampleDSD.md)
* trace: [Design Specification](DesignSpecifications/SampleDSD.md)

---

### Requirement with DesignSpecifications Reference

This requirement specifically tests validation of relations to files in the DesignSpecifications folder.

#### Metadata
* type: requirement

#### Relations
* derivedFrom: #Requirement with Valid Standard Relations
* satisfiedBy: [Sample DSD](DesignSpecifications/SampleDSD.md)

---

### Requirement with Many Subsections

This requirement specifically tests validation of 'Other' subsections


#### Subsection 1

Some text of subsection 1

#### Subsection 2

Some text of subsection 2

---
