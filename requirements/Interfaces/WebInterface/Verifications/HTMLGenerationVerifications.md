# Elements

### Component Reuse Verification

Test verifies components are reused across pages without duplication.

#### Details
**Test Procedure:**
1. Analyze generated HTML files
2. Verify navigation component appears identically in all pages
3. Count instances of duplicated code blocks
4. Measure total generated HTML size

**Pass Criteria:**
- Navigation HTML identical across all pages (except nav_prefix)
- Zero duplication of CSS styles across pages (external stylesheet)
- Total HTML size reduced by ≥50% compared to old template system
- Source code organized in component modules

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Component-Based HTML Architecture](../HTMLGeneration.md#component-based-html-architecture)
---

### HTML Validity Verification

Test verifies generated HTML is valid and well-formed.

#### Details
**Test Procedure:**
1. Generate all HTML pages
2. Validate HTML using W3C validator or similar
3. Check for:
   - Proper DOCTYPE declaration
   - Valid HTML5 structure
   - Closed tags
   - Valid attributes
   - No duplicate IDs

**Pass Criteria:**
- All pages pass W3C HTML5 validation
- No HTML parsing errors
- Maud-generated HTML is well-formed

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-generation/test.sh)
  * verify: [Type-Safe HTML Generation](../HTMLGeneration.md#type-safe-html-generation)
---

### Integration Test Verification

Test verifies all 7 generated pages work correctly together.

#### Details
**Test Procedure:**
1. Generate all 7 pages: index.html, model.html, traces.html, traceflow.html, coverage.html, resources.html, + spec file
2. Verify each page contains:
   - Correct title
   - Navigation menu with 6 links
   - Expected content sections
3. Test relative links:
   - From root: nav_prefix = ""
   - From requirements/: nav_prefix = "../"
   - From requirements/System/: nav_prefix = "../../"
4. Verify visualizations load:
   - Mermaid diagrams render
   - D3 Sunburst/Icicle renders
   - D3 Sankey renders
5. Test navigation between pages

**Pass Criteria:**
- All 7 pages generate without errors
- All pages contain required content sections
- Relative links resolve correctly from all nesting levels
- All visualizations load and display properly
- Navigation links functional from all pages

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-generation/test.sh)
  * verify: [Component-Based HTML Architecture](../HTMLGeneration.md#component-based-html-architecture)
  * verify: [Responsive HTML Generation](../HTMLGeneration.md#responsive-html-generation)
---

### Mobile Responsiveness Verification

Test verifies HTML documentation is usable on mobile devices.

#### Details
**Test Procedure:**
1. Generate all 7 HTML pages (index, model, traces, traceflow, coverage, resources, + spec file)
2. Test on Chrome DevTools device emulation:
   - iPhone SE (375px)
   - iPad (768px)
   - Desktop (1920px)
3. Verify navigation menu:
   - Desktop: Horizontal menu visible
   - Mobile: Hamburger menu functional
4. Verify content readability:
   - No horizontal scrolling
   - Text legible without zooming
   - Touch targets ≥ 44px

**Pass Criteria:**
- All pages render without horizontal scroll on all viewport sizes
- Navigation menu adapts correctly (hamburger on mobile, horizontal on desktop)
- All interactive elements accessible via touch on mobile

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-generation/test.sh)
  * verify: [Mobile-Friendly Documentation](../HTMLGeneration.md#mobile-friendly-documentation)
  * verify: [Responsive HTML Generation](../HTMLGeneration.md#responsive-html-generation)
---

### Responsive Design Verification

Test verifies responsive breakpoints and Tailwind CSS integration.

#### Details
**Test Procedure:**
1. Generate all HTML pages
2. Take screenshots at breakpoints: 320px, 640px, 768px, 1024px, 1920px
3. Compare screenshots to expected layouts (visual regression)
4. Verify CSS classes applied correctly:
   - Mobile-only classes (md:hidden)
   - Desktop-only classes (hidden md:flex)
   - Responsive spacing (px-4 md:px-8)

**Pass Criteria:**
- Screenshots match expected layouts at all breakpoints
- No layout breaks or overlapping elements
- Tailwind utility classes present in generated HTML
- Custom Reqvire theme colors applied

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-generation/test.sh)
  * verify: [CSS Framework Integration](../HTMLGeneration.md#css-framework-integration)
  * verify: [Responsive HTML Generation](../HTMLGeneration.md#responsive-html-generation)
---
