# Documents

## Metadata
  * type: specification

## Relations
  * refine: [TraceFlow View Report Generation](../Reporting.md#traceflow-view-report-generation)

## TraceFlowView

# TraceFlow View Specification

## Overview

The TraceFlow view displays the verification traceability flow using an interactive D3.js Sankey diagram visualization. This view shows how capabilities are specified by requirements, how requirements flow to verifications, and how capabilities may be directly verified, providing a clear visual representation of the traceability relationships.

TraceFlow behavior is exposed through the single SPA Explorer export and Project Store trace projection. It is not emitted as a standalone HTML artifact and is not a primary left Explorer view destination.

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

**TraceFlow SPA View:**
- Seed traceability flow data into the browser-local Project Store during export
- Render Sankey visualization behavior from the SPA route/view layer
- Do not generate standalone TraceFlow markdown or HTML artifacts during export

**Integration with Existing Export:**
- Follow existing HTML export styling and structure
- Use Reqvire color scheme for consistency
- Maintain consistent SPA navigation patterns without exposing TraceFlow as a primary left Explorer view
- Support pan/zoom controls like other diagrams

**Requirements:**
- Seeded during `reqvire export` command
- Updates automatically when model changes
- Deterministic output for version control

---

## View Content

The TraceFlow SPA view shall contain:

1. **Sankey Diagram**: The D3.js Sankey visualization
2. **Inspector Evidence**: Selection details in the shared right `Inspector` lane when exposed by the view
3. **Help Guidance**: Brief explanation and interaction guidance through the shared help/inspector surfaces rather than a static first-viewport page title/prose block

---

## Access

The TraceFlow behavior shall remain available through SPA routing/tooling:
- Link text when referenced from reports: "TraceFlow"
- Target: SPA route/view state, not a standalone TraceFlow HTML artifact
- Primary left Explorer pane: no TraceFlow, Coverage, or Resources top-level link
