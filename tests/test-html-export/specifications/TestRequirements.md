# Test Requirements

This document contains test requirements for HTML export testing.

## Requirements
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  req1["Test Requirement 1"];
  click req1 "TestRequirements.md#test-requirement-1";
  class req1 requirement;
  
  req2["Test Requirement 2"];
  click req2 "TestRequirements.md#test-requirement-2";
  class req2 requirement;
  
  ver1["TestVerifications.md/Test Verification 1"];
  click ver1 "TestVerifications.md#test-verification-1";
  class ver1 verification;
  
  ver2["TestVerifications.md/Test Verification 2"];
  click ver2 "TestVerifications.md#test-verification-2";
  class ver2 verification;
  
  req1 -.->|verifiedBy| ver1;
  req2 -.->|verifiedBy| ver2;
```

### Test Requirement 1

This is a test requirement with links to [README.md](README.md) and [TestVerifications.md](TestVerifications.md).

#### Relations
  * verifiedBy: [TestVerifications.md/Test Verification 1](TestVerifications.md#test-verification-1)

---

### Test Requirement 2

This is another test requirement.

#### Relations
  * verifiedBy: [TestVerifications.md/Test Verification 2](TestVerifications.md#test-verification-2)

---