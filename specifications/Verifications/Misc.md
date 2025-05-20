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
  click ee855bb5e5d1f05b "Misc.md#html-export-verification";
  class ee855bb5e5d1f05b verification;
  d9686a154fe87b2["UserRequirements.md/Export HTML specifications"];
  class d9686a154fe87b2 requirement;
  click d9686a154fe87b2 "../UserRequirements.md#export-html-specifications";
  ee855bb5e5d1f05b -.->|verifies| d9686a154fe87b2;
  ca710f5a0e5dfc5d["tests/test-html-export/test.sh"];
  class ca710f5a0e5dfc5d default;
  click ca710f5a0e5dfc5d "../../tests/test-html-export/test.sh";
  ca710f5a0e5dfc5d -->|satisfies| ee855bb5e5d1f05b;
  3c6410e3387503c0["AI Agent Context Verification"];
  click 3c6410e3387503c0 "Misc.md#ai-agent-context-verification";
  class 3c6410e3387503c0 verification;
  7fd9156eac77c270["UserRequirements.md/AI Agent Context"];
  class 7fd9156eac77c270 requirement;
  click 7fd9156eac77c270 "../UserRequirements.md#ai-agent-context";
  3c6410e3387503c0 -.->|verifies| 7fd9156eac77c270;
  6dc663bb31c3eb65["tests/test-ai-agent-context/test.sh"];
  class 6dc663bb31c3eb65 default;
  click 6dc663bb31c3eb65 "../../tests/test-ai-agent-context/test.sh";
  6dc663bb31c3eb65 -->|satisfies| 3c6410e3387503c0;
```

---

### HTML Export Verification

This test verifies that the system exports specifications into HTML format and saves them in the designated output location.

#### Metadata
  * type: verification

#### Details

##### Acceptance Criteria:
- System should export specifications to HTML format
- HTML files should be saved in the designated output location
- HTML output should maintain the structure and content of the original specifications
- SpecificationIndex.md should be renamed to index.html in output
- Links in diagrams and text must be converted to use .html instead of .md
- Paths in HTML files should maintain the original relative structure
- System should work in environments without Git repositories

##### Test Criteria:
- Command exits with success (0) return code
- HTML files are generated at the expected location with .html extension
- SpecificationIndex.md is converted to index.html
- HTML content preserves the structure and information from the source files
- Links in HTML files use .html extension instead of .md
- Mermaid click links are properly converted from .md to .html
- Both GitHub-style URLs and direct file paths in mermaid click links are handled correctly
- Paths should not have duplicated folder names (e.g., specifications/specifications)


#### Relations
  * verify: [UserRequirements.md/Export HTML specifications](../UserRequirements.md#export-html-specifications)
  * satisfiedBy: [tests/test-html-export/test.sh](../../tests/test-html-export/test.sh)

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

#### Relations
  * verify: [UserRequirements.md/AI Agent Context](../UserRequirements.md#ai-agent-context)
  * satisfiedBy: [tests/test-ai-agent-context/test.sh](../../tests/test-ai-agent-context/test.sh)

---