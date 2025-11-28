# Elements

### Attachment Export Verification

This test verifies that HTML export copies all attachment files to the output directory.

#### Details

##### Acceptance Criteria:
- System shall copy all attachment files referenced by elements
- Attachment files shall preserve their relative paths in output
- Duplicate attachments (same file referenced multiple times) shall be copied only once
- All attachment file types shall be supported (md, pdf, txt, etc.)

##### Test Criteria:
- Create model with elements having attachments
- Run HTML export command
- Verify attachment files exist in output directory at correct relative paths
- Verify attachment files content matches original

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Attachment Export](../WebInterface.md#attachment-export)
  * satisfiedBy: [test.sh](../../../tests/test-attachment-export/test.sh)
---

### Containment Attachment Links Verification

This test verifies that the D3.js containment tree view displays attachments as children of elements.

#### Details

##### Acceptance Criteria:
- Elements with attachments shall show attachments as child nodes in D3.js tree
- Element attachments (refinements) shall use wrench icon (🔧) with type `attachment-element`
- File attachments shall use paperclip icon (📎) with type `attachment-file`
- Element attachments shall be clickable and navigate to the referenced element
- File attachments shall show filename and path for reference

##### Test Criteria:
- Create model with element having attachments
- Run HTML export command
- Verify containment.html contains d3-tree JSON data with attachment nodes
- Verify attachment-element nodes have links to element definitions
- Verify attachment-file nodes show file paths

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Containment View Attachment Links](../WebInterface.md#containment-view-attachment-links)
  * satisfiedBy: [test.sh](../../../tests/test-attachment-export/test.sh)
---

### Diagram Attachment Display Verification

This test verifies that diagrams display attachment links within element boxes.

#### Details

##### Acceptance Criteria:
- Element boxes in diagrams shall include attachment filenames
- Attachments shall be prefixed with paperclip icon (📎)
- Attachments shall appear below element name using line breaks
- Attachment display shall not break diagram rendering

##### Test Criteria:
- Create model with element having attachments
- Generate diagram (format or model command)
- Verify Mermaid output contains multiline labels with attachments
- Verify attachment filenames appear with 📎 prefix
- Verify diagram renders correctly with attachment labels

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Diagram Attachment Display](../WebInterface.md#diagram-attachment-display)
  * satisfiedBy: [test.sh](../../../tests/test-attachment-export/test.sh)
---

### HTML Export Verification

This test verifies that the system exports specifications into HTML format with generated containment view (D3.js tree) and saves them in the designated output location.

#### Details

##### Acceptance Criteria:
- System should export specifications to HTML format
- HTML files should be saved in the designated output location
- HTML output should maintain the structure and content of the original specifications
- System shall generate containment.md in the temporary working directory during HTML export
- containment.md shall be converted to containment.html in the output directory
- containment.html shall contain an interactive D3.js collapsible tree showing containment hierarchy
- containment.html shall display folders, files, and elements in a hierarchical tree
- containment.html shall serve as the primary entry point for HTML documentation
- Links in diagrams and text must be converted to use .html instead of .md
- Paths in HTML files should maintain the original relative structure
- System should work in environments without Git repositories

##### Test Criteria:
- Command exits with success (0) return code
- HTML files are generated at the expected location with .html extension
- Output directory contains containment.html file
- containment.html contains D3.js tree visualization
- containment.html includes d3-tree JSON data block
- HTML content preserves the structure and information from the source files
- Links in HTML files use .html extension instead of .md
- Mermaid click links are properly converted from .md to .html
- Both GitHub-style URLs and direct file paths in mermaid click links are handled correctly
- Paths should not have duplicated folder names (e.g., specifications/specifications)
- The .git directory is not present in export output

#### Metadata
  * type: test-verification

#### Relations
  * verify: [HTML Export](../WebInterface.md#html-export)
  * satisfiedBy: [test.sh](../../../tests/test-html-export/test.sh)
---

### Model View Element Navigation Test

Test verifies that element names in the model-centric view are clickable links.

#### Test Steps
1. Run `reqvire model` command to generate model report
2. Verify output contains element headers as markdown links
3. Verify links follow format `[Element Name](file_path#fragment)`

#### Expected Results
- Element names are rendered as markdown links
- Links point to source file with element fragment

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Model View Element Navigation](../WebInterface.md#model-view-element-navigation)
  * satisfiedBy: [test.sh](../../../tests/test-model-command/test.sh)
---

### Serve Command Verification

This test verifies that the serve command exports HTML to a temporary directory and starts an HTTP server that serves the model documentation.

#### Details

##### Acceptance Criteria:
- System shall export HTML artifacts to a temporary directory
- System shall start HTTP server on specified host and port
- System shall display clickable terminal link to the server URL
- System shall serve containment.html when accessing root URL
- System shall serve all exported HTML files with correct paths
- System shall serve static assets (SVG diagrams, CSS, etc.)
- System shall return 404 for non-existent files
- System shall set correct Content-Type headers for different file types
- System shall run in quiet mode (suppress verbose export output)
- System shall not automatically open browser window
- System shall display instructions for Ctrl-C stop

##### Test Criteria:
- Command starts successfully and displays server URL with instructions
- Server responds to HTTP requests on specified port
- Root URL (/) serves containment.html
- HTML files are served with text/html content type
- SVG files are served with image/svg+xml content type
- Non-existent paths return 404 status
- Export verbose output is suppressed (quiet mode active)

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Serve Command](../WebInterface.md#serve-command)
  * satisfiedBy: [test.sh](../../../tests/test-serve-command/test.sh)
---
