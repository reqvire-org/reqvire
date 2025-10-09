# Verification Traceability Report

📂 File: specifications/Verifications/Tests.md

  📖 Section: Authentication Tests

    ## OAuth Flow Test
    **Verification ID:** specifications/Verifications/Tests.md#oauth-flow-test
    **Type:** test-verification

    ### Upward Trace Tree

    ```mermaid
    graph BT
        VER001["OAuth Flow Test<br/>test-verification"]
        SYS001["OAuth Implementation"]:::verified
        SYS002["Session Management"]:::verified
        USER001["User Authentication"]

        SYS001 -.->|verify| VER001
        SYS002 -.->|verify| VER001
        USER001 -.->|deriveReqT| SYS001
        SYS001 -.->|deriveReqT| SYS002

        classDef verified fill:#90EE90,stroke:#5fd75f,stroke-width:2px;
    ```

    ### Requirements Traced
    - ★ [OAuth Implementation](../SystemRequirements.md#oauth-implementation)
    - ★ [Session Management](../SystemRequirements.md#session-management)
    - [User Authentication](../UserRequirements.md#user-authentication) *(root)*

    **Legend:**
    - ★ = Directly verified by this verification (via verify relation)
    - Green highlighted nodes = Directly verified requirements
    - Graph shows upward trace from verification to root requirements

    ---

    ## Session Timeout Test
    **Verification ID:** specifications/Verifications/Tests.md#session-timeout-test
    **Type:** test-verification

    ### Upward Trace Tree

    ```mermaid
    graph BT
        VER002["Session Timeout Test<br/>test-verification"]
        SYS002["Session Management"]:::verified
        SYS001["OAuth Implementation"]
        USER001["User Authentication"]

        SYS002 -.->|verify| VER002
        SYS001 -.->|deriveReqT| SYS002
        USER001 -.->|deriveReqT| SYS001

        classDef verified fill:#90EE90,stroke:#5fd75f,stroke-width:2px;
    ```

    ### Requirements Traced
    - ★ [Session Management](../SystemRequirements.md#session-management)
    - [OAuth Implementation](../SystemRequirements.md#oauth-implementation)
    - [User Authentication](../UserRequirements.md#user-authentication) *(root)*

    **Legend:**
    - ★ = Directly verified by this verification (via verify relation)
    - Green highlighted nodes = Directly verified requirements
    - Graph shows upward trace from verification to root requirements

    ---

  📖 Section: Security Tests

    ## Encryption Coverage Test
    **Verification ID:** specifications/Verifications/Tests.md#encryption-coverage-test
    **Type:** test-verification

    ### Upward Trace Tree

    ```mermaid
    graph BT
        VER003["Encryption Coverage Test<br/>test-verification"]
        SYS003["Encryption Implementation"]:::verified
        USER002["Data Protection"]

        SYS003 -.->|verify| VER003
        USER002 -.->|deriveReqT| SYS003

        classDef verified fill:#90EE90,stroke:#5fd75f,stroke-width:2px;
    ```

    ### Requirements Traced
    - ★ [Encryption Implementation](../SystemRequirements.md#encryption-implementation)
    - [Data Protection](../UserRequirements.md#data-protection) *(root)*

    **Legend:**
    - ★ = Directly verified by this verification (via verify relation)
    - Green highlighted nodes = Directly verified requirements
    - Graph shows upward trace from verification to root requirements

    ---

  📖 Section: Coverage Tests

    ## Coverage Calculation Test
    **Verification ID:** specifications/Verifications/Tests.md#coverage-calculation-test
    **Type:** test-verification

    ### Upward Trace Tree

    ```mermaid
    graph BT
        VER004["Coverage Calculation Test<br/>test-verification"]
        SYS004["Coverage Calculator"]:::verified
        SYS005["Coverage Report Generator"]:::verified
        USER003["Coverage Reports"]

        SYS004 -.->|verify| VER004
        SYS005 -.->|verify| VER004
        USER003 -.->|deriveReqT| SYS004
        SYS004 -.->|deriveReqT| SYS005

        classDef verified fill:#90EE90,stroke:#5fd75f,stroke-width:2px;
    ```

    ### Requirements Traced
    - ★ [Coverage Calculator](../SystemRequirements.md#coverage-calculator)
    - ★ [Coverage Report Generator](../SystemRequirements.md#coverage-report-generator)
    - [Coverage Reports](../UserRequirements.md#coverage-reports) *(root)*

    **Legend:**
    - ★ = Directly verified by this verification (via verify relation)
    - Green highlighted nodes = Directly verified requirements
    - Graph shows upward trace from verification to root requirements

    ---

  📖 Section: Analysis Verifications

    ## Security Analysis
    **Verification ID:** specifications/Verifications/Tests.md#security-analysis
    **Type:** analysis-verification

    ### Upward Trace Tree

    ```mermaid
    graph BT
        VER005["Security Analysis<br/>analysis-verification"]
        USER002["Data Protection"]:::verified

        USER002 -.->|verify| VER005

        classDef verified fill:#90EE90,stroke:#5fd75f,stroke-width:2px;
    ```

    ### Requirements Traced
    - ★ [Data Protection](../UserRequirements.md#data-protection) *(root)*

    **Legend:**
    - ★ = Directly verified by this verification (via verify relation)
    - Green highlighted nodes = Directly verified requirements
    - Graph shows upward trace from verification to root requirements

    ---

  📖 Section: Inspection Verifications

    ## Code Inspection
    **Verification ID:** specifications/Verifications/Tests.md#code-inspection
    **Type:** inspection-verification

    ### Upward Trace Tree

    ```mermaid
    graph BT
        VER006["Code Inspection<br/>inspection-verification"]
        SYS001["OAuth Implementation"]:::verified
        USER001["User Authentication"]

        SYS001 -.->|verify| VER006
        USER001 -.->|deriveReqT| SYS001

        classDef verified fill:#90EE90,stroke:#5fd75f,stroke-width:2px;
    ```

    ### Requirements Traced
    - ★ [OAuth Implementation](../SystemRequirements.md#oauth-implementation)
    - [User Authentication](../UserRequirements.md#user-authentication) *(root)*

    **Legend:**
    - ★ = Directly verified by this verification (via verify relation)
    - Green highlighted nodes = Directly verified requirements
    - Graph shows upward trace from verification to root requirements

    ---
