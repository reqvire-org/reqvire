# Miscellaneous Verifications

This document contains miscellaneous verification tests that don't fit into the other verification categories.

## Export and Context Verifications
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  8d1839785fa3e85f["AI Agent Context Verification"];
  click 8d1839785fa3e85f "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/Misc.md#ai-agent-context-verification";
  class 8d1839785fa3e85f verification;
  c5fffc3cd9f22134["UserRequirements.md/AI Agent Context"];
  class c5fffc3cd9f22134 requirement;
  click c5fffc3cd9f22134 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#ai-agent-context";
  8d1839785fa3e85f -.->|verifies| c5fffc3cd9f22134;
  79f1e8bcdc9255fb["tests/test-ai-agent-context/test.sh"];
  class 79f1e8bcdc9255fb default;
  click 79f1e8bcdc9255fb "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-ai-agent-context/test.sh";
  8d1839785fa3e85f -.->|trace| 79f1e8bcdc9255fb;
  bc50e797577ffd51["HTML Export Verification"];
  click bc50e797577ffd51 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/Verifications/Misc.md#html-export-verification";
  class bc50e797577ffd51 verification;
  6424b4fd0b132482["UserRequirements.md/Export HTML specifications"];
  class 6424b4fd0b132482 requirement;
  click 6424b4fd0b132482 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#export-html-specifications";
  bc50e797577ffd51 -.->|verifies| 6424b4fd0b132482;
  82700fdf287e5702["tests/test-html-export/test.sh"];
  class 82700fdf287e5702 default;
  click 82700fdf287e5702 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/tests/test-html-export/test.sh";
  bc50e797577ffd51 -.->|trace| 82700fdf287e5702;
```

---

### HTML Export Verification

This test verifies that the system exports specifications into HTML format and saves them in the designated output location.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should export specifications to HTML format
- HTML files should be saved in the designated output location
- HTML output should maintain the structure and content of the original specifications

##### Test Criteria
- Command exits with success (0) return code
- HTML files are generated at the expected location
- HTML content preserves the structure and information from the source files

##### Test Procedure
1. Prepare test fixtures with Markdown files containing links to other documents
2. Ensure there's a README.md file to test conversion to index.html
3. Run Reqvire with the --html flag
4. Verify that HTML files are generated for all Markdown files
5. Verify that README.md is converted to index.html
6. Verify that links use .html extension instead of .md
7. Verify that structure and content are preserved in the HTML files

#### Relations
  * verify: [UserRequirements.md/Export HTML specifications](../UserRequirements.md#export-html-specifications)
  * trace: [tests/test-html-export/test.sh](../../tests/test-html-export/test.sh)

---

### AI Agent Context Verification

This test verifies that the system provides necessary context for AI agents to understand how to use Reqvire and its methodology.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria
- System should provide comprehensive context for AI agents
- Context should include information about Reqvire usage and methodology
- Context should be accessible via a dedicated command

##### Test Criteria
- Command exits with success (0) return code
- Context information is comprehensive and usable by AI agents
- Command output is properly formatted for AI consumption

##### Test Procedure
1. Run Reqvire with the `--llm-context` flag to generate the AI context
2. Verify that the output contains essential Reqvire concepts (Requirements, Relations, Verification, Metadata)
3. Verify that the output is properly formatted in markdown with headers
4. Verify that the command returns a successful exit code

#### Relations
  * verify: [UserRequirements.md/AI Agent Context](../UserRequirements.md#ai-agent-context)
  * trace: [tests/test-ai-agent-context/test.sh](../../tests/test-ai-agent-context/test.sh)

---