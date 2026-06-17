# Elements

### Web Interface Navigation Behavior

Navigation behavior in the served Explorer.

#### Details
**Inter-page Navigation:**
- Element links in Explorer views navigate to `index.html#/elements/<identifier>` and open a scrollable Project Store-backed element modal without leaving the active Explorer context
- Element modals include a secondary source-page link to the Explorer content route and source anchor when exact source browsing is needed
- File links navigate to file page
- External links open in new tab

**Breadcrumbs:**
- Show path: Home > File > Section > Element
- Each segment clickable

**Search:**
- Client-side ranked search in the `index.html#/search` Explorer route backed by Project Store search documents
- Build the ranked search index in a browser worker after the initial Explorer shell renders so Model, Ontologies, Traces, and Graph remain usable while indexing runs
- Rank matches with BM25-style scoring, boosting element/file titles first, normalized path tokens second, result kind third, and body/content text last
- Support prefix and fuzzy matching so partial terms and small spelling mistakes still find matching elements, files, resources, and ontology terms
- Filter ranked results by result kind without rebuilding the index

#### Metadata
  * type: behavior
---
