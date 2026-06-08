# Element

## Metadata
  * type: specification

## Relations
  * refine: [Traces View Generation](../Capabilities.md#traces-view-generation)

## TraceFlowView

# TraceFlow View Specification

## Overview

The TraceFlow view displays the verification traceability flow using an interactive D3.js Sankey diagram visualization. This view shows how capabilities are specified by requirements, how requirements flow to verifications, and how capabilities may be directly verified, providing a clear visual representation of the traceability relationships.

TraceFlow behavior is exposed through the served SPA Explorer and Project Store trace projection. It is served as route state and is not a primary left Explorer view destination.

## Visualization

### Sankey Diagram

The TraceFlow visualization displays the traceability hierarchy as a Sankey flow diagram:

**Structure:**
- Left-to-right flow showing requirement hierarchy
- Nodes represent elements (capabilities, requirements, verifications)
- Links represent relations (specifiedBy, derivedFrom, verifiedBy), including direct capability verification links
- Link width proportional to the number of connections
- Encoded by semantic role tokens and type glyphs

**Node Types:**
- Capabilities use the capability role token and glyph
- System requirements use the requirement role token and glyph
- Verifications use the verification role token and glyph
- Test verifications use the verification role token plus verification subtype label

**Interactive Capabilities:**
- Pan and zoom support with mouse wheel and buttons
- Touch pinch-zoom for mobile devices
- Hover shows element details
- Click on nodes navigates to element definition
- Reset button to restore initial view

---

## Visual Semantics

The visualization uses the shared Explorer design-system role palette:

| Role | Visual contract |
|------|-----------------|
| file | Source-file role token and compact source-file glyph |
| capability | Capability role token and capability glyph |
| requirement | Requirement role token and requirement glyph |
| verification | Verification role token and verification glyph |

Concrete color values are owned by the Explorer design-system tokens. The Trace flow document only defines which semantic roles appear in the trace visualization.

---

## Explorer Integration

Explorer integration must:

**TraceFlow SPA View:**
- Seed traceability flow data into the browser-local Project Store during serve runtime generation
- Render Sankey visualization behavior from the SPA route/view layer
- Do not generate separate TraceFlow route files during serve runtime generation

**Integration with Existing Explorer:**
- Follow existing Explorer styling and structure
- Use the shared Explorer design-system role palette and surface tokens for consistency
- Maintain consistent SPA navigation patterns without exposing TraceFlow as a primary left Explorer view
- Support pan/zoom controls like other diagrams

**Requirements:**
- Seeded during Explorer serve runtime generation
- Updates automatically when model changes
- Deterministic output for version control

---

## View Content

The TraceFlow SPA view shall contain:

1. **Sankey Diagram**: The D3.js Sankey visualization
2. **Selection Evidence**: Selection details through the shared left-pane selected-item link and modal detail pattern when exposed by the view
3. **Help Guidance**: Brief explanation and interaction guidance through the shared help surfaces rather than a static first-viewport page title/prose block

---

## Access

The TraceFlow behavior shall remain available through SPA routing/tooling:
- Link text when referenced from reports: "TraceFlow"
- Target: SPA route/view state backed by Project Store trace data
- Primary left Explorer pane: no TraceFlow, Coverage, or Resources top-level link
