# Verification Traces Report

## File: specifications/Verifications/Tests.md

### Section: Authentication Tests

#### OAuth Flow Test

- **Type**: test-verification
- **Directly Verified**: 2 requirements
- **Total in Tree**: 3 requirements

```mermaid
graph TD
  classDef verified fill:#90EE90,stroke:#000,stroke-width:2px;
  classDef requirement fill:#87CEEB,stroke:#000,stroke-width:1px;
  classDef verification fill:#FFD700,stroke:#000,stroke-width:2px;

  bfd26e943c93f2b8["OAuth Flow Test<br>(test-verification)"]:::verification
  d5a439e4972ea591["OAuth Implementation<br>(requirement)"]:::verified
  bfd26e943c93f2b8 -.->|verifies| d5a439e4972ea591;
  2934ea21a1d898a3["User Authentication<br>(user-requirement)"]:::requirement
  d5a439e4972ea591 -.->|derivedFrom| 2934ea21a1d898a3;
  5a755bb51a740c10["Session Management<br>(requirement)"]:::verified
  bfd26e943c93f2b8 -.->|verifies| 5a755bb51a740c10;
```


#### Session Timeout Test

- **Type**: test-verification
- **Directly Verified**: 1 requirements
- **Total in Tree**: 3 requirements

```mermaid
graph TD
  classDef verified fill:#90EE90,stroke:#000,stroke-width:2px;
  classDef requirement fill:#87CEEB,stroke:#000,stroke-width:1px;
  classDef verification fill:#FFD700,stroke:#000,stroke-width:2px;

  473f7ef449cf9463["Session Timeout Test<br>(test-verification)"]:::verification
  5a755bb51a740c10["Session Management<br>(requirement)"]:::verified
  473f7ef449cf9463 -.->|verifies| 5a755bb51a740c10;
  d5a439e4972ea591["OAuth Implementation<br>(requirement)"]:::requirement
  5a755bb51a740c10 -.->|derivedFrom| d5a439e4972ea591;
  2934ea21a1d898a3["User Authentication<br>(user-requirement)"]:::requirement
  d5a439e4972ea591 -.->|derivedFrom| 2934ea21a1d898a3;
```


### Section: Security Tests

#### Encryption Coverage Test

- **Type**: test-verification
- **Directly Verified**: 1 requirements
- **Total in Tree**: 2 requirements

```mermaid
graph TD
  classDef verified fill:#90EE90,stroke:#000,stroke-width:2px;
  classDef requirement fill:#87CEEB,stroke:#000,stroke-width:1px;
  classDef verification fill:#FFD700,stroke:#000,stroke-width:2px;

  30572046e8226c8d["Encryption Coverage Test<br>(test-verification)"]:::verification
  de36a6e809a20bc2["Encryption Implementation<br>(requirement)"]:::verified
  30572046e8226c8d -.->|verifies| de36a6e809a20bc2;
  8ddbf0ce66bc6c18["Data Protection<br>(user-requirement)"]:::requirement
  de36a6e809a20bc2 -.->|derivedFrom| 8ddbf0ce66bc6c18;
```


### Section: Coverage Tests

#### Coverage Calculation Test

- **Type**: test-verification
- **Directly Verified**: 2 requirements
- **Total in Tree**: 3 requirements

```mermaid
graph TD
  classDef verified fill:#90EE90,stroke:#000,stroke-width:2px;
  classDef requirement fill:#87CEEB,stroke:#000,stroke-width:1px;
  classDef verification fill:#FFD700,stroke:#000,stroke-width:2px;

  28383c90c121c4a5["Coverage Calculation Test<br>(test-verification)"]:::verification
  4dc56d6383b230df["Coverage Calculator<br>(requirement)"]:::verified
  28383c90c121c4a5 -.->|verifies| 4dc56d6383b230df;
  a1b39797065ef491["Coverage Reports<br>(user-requirement)"]:::requirement
  4dc56d6383b230df -.->|derivedFrom| a1b39797065ef491;
  21afc68bc3823e3b["Coverage Report Generator<br>(requirement)"]:::verified
  28383c90c121c4a5 -.->|verifies| 21afc68bc3823e3b;
```


### Section: Analysis Verifications

#### Security Analysis

- **Type**: analysis-verification
- **Directly Verified**: 1 requirements
- **Total in Tree**: 1 requirements

```mermaid
graph TD
  classDef verified fill:#90EE90,stroke:#000,stroke-width:2px;
  classDef requirement fill:#87CEEB,stroke:#000,stroke-width:1px;
  classDef verification fill:#FFD700,stroke:#000,stroke-width:2px;

  ee61d9a254ace553["Security Analysis<br>(analysis-verification)"]:::verification
  8ddbf0ce66bc6c18["Data Protection<br>(user-requirement)"]:::verified
  ee61d9a254ace553 -.->|verifies| 8ddbf0ce66bc6c18;
```


### Section: Inspection Verifications

#### Code Inspection

- **Type**: inspection-verification
- **Directly Verified**: 1 requirements
- **Total in Tree**: 2 requirements

```mermaid
graph TD
  classDef verified fill:#90EE90,stroke:#000,stroke-width:2px;
  classDef requirement fill:#87CEEB,stroke:#000,stroke-width:1px;
  classDef verification fill:#FFD700,stroke:#000,stroke-width:2px;

  cd126a6fddc2bd83["Code Inspection<br>(inspection-verification)"]:::verification
  d5a439e4972ea591["OAuth Implementation<br>(requirement)"]:::verified
  cd126a6fddc2bd83 -.->|verifies| d5a439e4972ea591;
  2934ea21a1d898a3["User Authentication<br>(user-requirement)"]:::requirement
  d5a439e4972ea591 -.->|derivedFrom| 2934ea21a1d898a3;
```
