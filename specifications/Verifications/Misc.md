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

  14ef985b9a43174e["HTML Export Verification"];
  class 14ef985b9a43174e verification;
  click 14ef985b9a43174e "Misc.md#html-export-verification";
  a4c40962cac85d0c["UserRequirements.md/Export HTML specifications"];
  class a4c40962cac85d0c requirement;
  click a4c40962cac85d0c "../UserRequirements.md#export-html-specifications";
  14ef985b9a43174e -.->|verifies| a4c40962cac85d0c;
  8899a52ea9866d8b["tests/test-html-export/test.sh"];
  class 8899a52ea9866d8b default;
  click 8899a52ea9866d8b "../../tests/test-html-export/test.sh";
  8899a52ea9866d8b -->|satisfies| 14ef985b9a43174e;
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