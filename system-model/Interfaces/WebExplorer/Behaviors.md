# Elements

### Web Interface Navigation Behavior

Navigation behavior in the served Explorer.

#### Details
**Inter-page Navigation:**
- Element links in Explorer views navigate to `index.html#/elements/<identifier>` and open a scrollable Project Store-backed element modal without leaving the active Explorer context
- Element modals include a secondary source-page link to the Explorer content route and source anchor when exact source browsing is needed
- Element modals render authored concept references as inline links on matching content text and open the referenced native concept element modal when activated
- File links navigate to file page
- External links open in new tab

**Breadcrumbs:**
- Show the active workspace branch and workspace-root-relative folder/file focus using the shared Model or Resources navigation. Parsed elements belong directly to their modeled source file.
- Folder and file segments navigate to their corresponding workspace focus; element activation uses the shared detail modal with source browsing as a secondary action.
- Document headings and reserved subsections are content anchors rather than hierarchy nodes.

**Search:**
- Client-side ranked search in the `index.html#/search` Explorer route backed by Project Store search documents
- Build the ranked search index in a browser worker after the initial Explorer shell renders so Model, Ontologies, Traces, and Graph remain usable while indexing runs
- Rank matches with BM25-style scoring, boosting element/file titles first, normalized path tokens second, result kind third, and body/content text last
- Support prefix and fuzzy matching so partial terms and small spelling mistakes still find matching elements, files, resources, and ontology terms
- Filter ranked results by result kind without rebuilding the index

#### Metadata
  * type: behavior
---
