# Documents

## Metadata
  * type: specification

## Relations
  * refine: [TraceFlow View Report Generation](../Reporting.md#traceflow-view-report-generation)

## TraceFlowView

# TraceFlow View Specification

## Overview

The TraceFlow view displays the verification traceability flow using an interactive D3.js Sankey diagram visualization. This view shows how capabilities are specified by requirements, how requirements flow to verifications, and how capabilities may be directly verified, providing a clear visual representation of the traceability relationships.

The TraceFlow page is accessible via the "TraceFlow" link in the navigation bar, positioned after "Traces".

## Visualization

### Sankey Diagram

The TraceFlow visualization displays the traceability hierarchy as a Sankey flow diagram:

**Structure:**
- Left-to-right flow showing requirement hierarchy
- Nodes represent elements (capabilities, requirements, verifications)
- Links represent relations (specifiedBy, derivedFrom, verifiedBy), including direct capability verification links
- Link width proportional to the number of connections
- Color-coded by element type

**Node Types:**
- Capabilities (blue)
- System requirements (deep purple)
- Verifications (green)
- Test verifications (green)

**Interactive Capabilities:**
- Pan and zoom support with mouse wheel and buttons
- Touch pinch-zoom for mobile devices
- Hover shows element details
- Click on nodes navigates to element definition
- Reset button to restore initial view

---

## Color Scheme

The visualization uses consistent colors matching other diagrams:

| Type | Color | Description |
|------|-------|-------------|
| capability | #BBDEFB | Blue for product/capability capabilities |
| requirement | #673AB7 | Deep purple for requirements |
| verification | #4CAF50 | Green for verifications |
| test-verification | #4CAF50 | Green for test verifications |

---

## HTML Export Integration

HTML export integration must:

**TraceFlow Page:**
- Generate as `traceflow.md` containing the Sankey visualization
- Convert to `traceflow.html` during export
- Include in navigation bar after "Traces" link

**Integration with Existing Export:**
- Follow existing HTML export styling and structure
- Use Reqvire color scheme for consistency
- Maintain consistent navigation patterns
- Support pan/zoom controls like other diagrams

**Requirements:**
- Generated during `reqvire export` command
- Updates automatically when model changes
- Deterministic output for version control

---

## Page Content

The TraceFlow page shall contain:

1. **Page Title**: "TraceFlow - Verification Traceability"
2. **Description**: Brief explanation of what the view shows
3. **Instructions**: How to interact with the diagram
4. **Sankey Diagram**: The D3.js Sankey visualization

---

## Navigation

The TraceFlow link shall be added to the navigation bar:
- Position: After "Traces", before "Coverage"
- Link text: "TraceFlow"
- Target: `traceflow.html`
