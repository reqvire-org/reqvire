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

  ee855bb5e5d1f05b["HTML Export Verification"];
  click ee855bb5e5d1f05b "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/Verifications/Misc.md#html-export-verification";
  class ee855bb5e5d1f05b verification;
  d9686a154fe87b2["UserRequirements.md/Export HTML specifications"];
  class d9686a154fe87b2 requirement;
  click d9686a154fe87b2 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/UserRequirements.md#export-html-specifications";
  ee855bb5e5d1f05b -.->|verifies| d9686a154fe87b2;
  ca710f5a0e5dfc5d["tests/test-html-export/test.sh"];
  class ca710f5a0e5dfc5d default;
  click ca710f5a0e5dfc5d "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/tests/test-html-export/test.sh";
  ee855bb5e5d1f05b -.->|trace| ca710f5a0e5dfc5d;
  3c6410e3387503c0["AI Agent Context Verification"];
  click 3c6410e3387503c0 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/Verifications/Misc.md#ai-agent-context-verification";
  class 3c6410e3387503c0 verification;
  7fd9156eac77c270["UserRequirements.md/AI Agent Context"];
  class 7fd9156eac77c270 requirement;
  click 7fd9156eac77c270 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/specifications/UserRequirements.md#ai-agent-context";
  3c6410e3387503c0 -.->|verifies| 7fd9156eac77c270;
  6dc663bb31c3eb65["tests/test-ai-agent-context/test.sh"];
  class 6dc663bb31c3eb65 default;
  click 6dc663bb31c3eb65 "https://github.com/Reqvire/reqvire/blob/f18e52f9f88c64f67a79abc4e93eb74b3ec22615/tests/test-ai-agent-context/test.sh";
  3c6410e3387503c0 -.->|trace| 6dc663bb31c3eb65;
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