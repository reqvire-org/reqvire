# Elements

### Web Interface Navigation Behavior

Navigation behavior in HTML export.

#### Details
**Inter-page Navigation:**
- Element links in Explorer views navigate to `index.html#/elements/<identifier>` and open a scrollable Project Store-backed element modal without leaving the active Explorer context
- Element modals include a secondary source-page link to the generated file HTML anchor when exact source-page browsing is needed
- File links navigate to file page
- External links open in new tab

**Breadcrumbs:**
- Show path: Home > File > Section > Element
- Each segment clickable

**Search:**
- Client-side search in the `index.html#/search` Explorer route backed by Project Store search documents
- Filter by name, type, content

#### Metadata
  * type: behavior
---
