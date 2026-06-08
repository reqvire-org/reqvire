# Elements

### Attachment Export Refinement Specification

#### Details
Attachment export behavior during HTML export:
- Collects attachment references from `element.attachments` across the model.
- Resolves each attachment as a refinement element identifier target.
- Skips duplicate identifier processing when the same refinement is referenced by multiple elements.
- Emits progress information for attachment-link processing operations.

This keeps exported documentation complete with navigable refinement attachment links.

#### Metadata
 * type: specification

#### Relations
 * refine: [Attachment Export](Capabilities.md#attachment-export)
---

### CSS Framework Integration Refinement Specification

#### Details
CSS framework integration behavior:
- Uses Tailwind CSS via CDN in current export flow.
- Applies mobile-first utility classes for responsive layout behavior.
- Uses responsive modifiers (`sm`, `md`, `lg`, `xl`) for adaptation.
- Defines Reqvire theme colors including primary, requirement, and verification tones.

Tailwind usage provides:
- Utility-first styling for predictable page composition.
- Built-in responsive modifiers (for example `md:hidden`, `lg:flex`).
- Consistent spacing, color, and typography scales.

#### Metadata
 * type: specification

#### Relations
 * refine: [CSS Framework Integration](HTMLGeneration.md#css-framework-integration)
---

### Component-Based HTML Architecture Refinement Specification

#### Details
The HTML generation system is expected to be organized into reusable components:

**Shared components:**
- Navigation menu (with mobile/desktop variants)
- Page header with metadata
- Footer
- Mobile menu toggle

**Reusable layouts:**
- Base layout for standard pages
- Diagram layout for full-height visualizations

**Page-specific modules:**
- Index/Containment page
- Model view page
- Traces page
- TraceFlow page
- Coverage page
- Resources page
- Individual specification pages

Each component is expected to be defined once and reused across all generated pages to eliminate code duplication.

#### Metadata
 * type: specification
---

### Containment View Attachment Links Refinement Specification

#### Details
Containment view attachment rendering behavior:
- For each element with attachments, renders attachments as child nodes in the D3 tree.
- Uses wrench icon (`🔧`) and type `attachment-element` for element attachments.
- Element-attachment nodes navigate to the referenced element.

#### Metadata
 * type: specification

#### Relations
 * refine: [Containment View Attachment Links](Capabilities.md#containment-view-attachment-links)
---

### D3.js Containment Tree Specification

Specification for the D3.js interactive containment tree visualization.

#### Details
The containment page (containment.html) is expected to display an interactive D3.js collapsible tree showing the containment hierarchy:
1. Root node representing the model root
2. Folder nodes that can be expanded/collapsed
3. File nodes containing element children
4. Element nodes with type-specific icons and colors
5. Attachment nodes as children of elements (refinement element attachments)
6. Clickable elements that navigate to their definitions
7. Expand All / Collapse All buttons for tree control

The containment view serves as the primary entry point for HTML documentation, providing an interactive visual overview of the model structure.

#### Metadata
 * type: specification
---

### Diagram Attachment Display Refinement Specification

#### Details
Diagram attachment rendering behavior in Mermaid output:
- Renders attachment links under the element name inside node labels.
- Prefixes each attachment with paperclip icon (`📎`).
- Displays referenced refinement element names.
- Produces clickable links to the referenced refinement element.
- Uses Mermaid multiline label formatting (`<br/>`).

Example node:
```
elementId["Element Name<br/>📎 Deterministic Output Specification"]
```

#### Metadata
 * type: specification

#### Relations
 * refine: [Diagram Attachment Display](Capabilities.md#diagram-attachment-display)
---

### HTML Branding Specification

Specification for Reqvire branding elements in HTML export.

#### Details
**Logo and Branding:**
- The navigation bar is expected to display the Reqvire logo on the left side before the navigation links
- A favicon is expected to be included for browser tab identification
- Apple touch icons is expected to be included for mobile device support
- All brand assets is expected to be exported to an assets folder during HTML export

**HTML Design:**
The system is expected to design and implement HTML pages with consistent layout, styling, and navigation for browsing the System model.

#### Metadata
 * type: specification
---

### HTML Export Pipeline Specification

Technical specification for HTML export generation pipeline.

#### Details
**Working Directory Setup:**
- Create temporary working directory (e.g., in /tmp)
- Generate markdown files from registry with full relations (user-created and auto-generated inverse relations)
- Copy all related system elements (following satisfiedBy and other relations)
- Resolve and preserve attachment identifier links to refinement elements

**Generation Pipeline (in temporary directory):**
Execute all generation commands treating temporary directory as repository root:
1. Generate all Mermaid diagrams in markdown files
2. Generate index.md (interactive D3.js tree showing containment hierarchy - main entry point)
3. Generate model.md (model-centric visualization with nested relations from model roots)
4. Generate traces.md (verification upward traceability)
5. Generate coverage.md (verification coverage report)
6. Generate ontologies.ttl and ontologies.html from the semantic index

**HTML Conversion:**
- Convert all markdown files to HTML with embedded styles
- Process Mermaid diagrams for web rendering
- Convert internal .md links to .html links
- Preserve directory structure

**Output:**
- Accept optional `--output` option to specify output directory
- When `--output` is not specified, export to a temporary directory and print the path
- When `--output` is specified, create output folder if not existing
- Copy generated HTML and all artifacts from temp directory to output directory
- Clean up temporary working directory (except when output is temp directory)

**Source Protection:**
- Never modify original repository files
- All generation happens in isolated temporary directory

**Git Directory Exclusion:**
- The .git directory is expected to never be exported to the output folder
- This prevents internal git metadata from polluting the exported documentation

**Export Related System Elements:**
- Ensure that any related system elements are also copied into output folder to ensure consistency of exported model

#### Metadata
 * type: specification
---

### Local Linked File Export Refinement Specification

#### Details
Local linked-file export behavior during HTML export:
- Detect local file references in exported markdown content, including standard markdown links and markdown images.
- Preserve relative linked-file paths in the rendered HTML output.
- Copy referenced local non-markdown files into the exported artifact tree so rendered `href` and `img src` targets exist.
- Skip rewriting or copying external URLs, data URLs, anchor-only links, and markdown document links that are exported as HTML pages.

This keeps exported HTML self-contained enough for local linked assets without changing author-written relative paths.

#### Metadata
 * type: specification

#### Relations
 * refine: [Local Linked File Export](Capabilities.md#local-linked-file-export)
---

### HTML Navigation Bar Specification

Specification for the fixed navigation bar in HTML pages.

#### Details
The system is expected to provide a fixed navigation bar in all HTML pages with links to key model artifacts for easy access.

The navigation bar must include (left to right):
- Containment: Link to containment.html (interactive D3.js tree - main entry point)
- Model: Link to model.html (model-centric view with nested relations)
- Traces: Link to traces.html (verification upward traceability report)
- Coverage: Link to coverage.html (verification coverage report)
- Resources: Link to resources.html (referenced files and attachment targets)
- Ontologies: Link to ontologies.html (ontology and SHACL collection)

The navigation bar must be:
- Always visible (fixed position) while scrolling
- Consistent across all HTML pages
- Clearly visible and accessible

#### Metadata
 * type: specification
---

### Ontologies View Generation Refinement Specification

#### Details
Ontologies view generation behavior:
- Uses the semantic index built from graph-registry ontology and semantic-contract elements.
- Displays summary counts for ontology blocks, shape blocks, RDF quads, total blocks, and the `ontologies.ttl` download action as one compact footer row in the ontology viewer sidebar.
- Builds the browser visualization, search index, and inspector construct metadata from `SemanticIndex.ontology_projection` facts; raw quads may support labels, comments, RDF type evidence, SHACL constraint display, and generic low-level links, but shall not be a separate authoritative extraction path for OWL/RDFS construct metadata.
- Does not expose the raw RDF triple graph as the primary user-facing ontology visualization.
- Reuses the same generated ontology construct projection that full semantic export emits so `ontologies.html` is not maintained as a separate HTML-only semantic model.
- Opens directly on the ontology explorer without a separate page header, descriptive preamble, top-level summary-card band, footer, or shared padded content card.
- Uses a dense canvas layout that fills the available viewport below the fixed navigation bar so ontology graph space is prioritized.
- Allocates a right inspector/sidebar wide enough for source links and long identifiers, and confines overflowing inspector content to an internal sidebar scroll area rather than scrolling the whole page.
- Does not render a raw Turtle/source-block list in `ontologies.html`.
- Preserves source element identifier, source name, file path, line number, and block kind as inspector/search evidence.
- Renders source citations in the inspector as links to the exported source HTML page and element fragment.
- Provides the exported `ontologies.ttl` download link only in the compact sidebar footer so the top of the sidebar remains available for search and inspection.
- Colors nodes by semantic role rather than by provenance. Classes, object properties, datatype properties, RDF properties, named individuals, datatypes, restrictions, class expressions, SHACL node shapes, SHACL property shapes, and generic RDF resources each use distinct legend swatches and graph colors.
- Treats a named IRI with `rdf:type` pointing to a declared ontology class as a named individual for node color, search badge, and inspector kind when the node has no stronger semantic role such as class, property, shape, datatype, restriction, or class expression. The `rdf:type` statement remains represented as membership construct evidence rather than as a generic RDF edge.
- Reserves SHACL colors for actual SHACL node shapes and property shapes. SHACL references to ontology terms remain evidence on the referenced term and do not recolor classes, properties, individuals, datatypes, restrictions, or class expressions as SHACL shapes.
- Keeps SHACL references to ontology terms as source/origin/construct evidence without recoloring the referenced ontology class or property as a SHACL shape.
- Treats built-in vocabulary references from XSD, RDF, RDFS, OWL, and SHACL namespaces as external references. External references remain available for datatype/range audit, but they are hidden by default to prevent built-in terms such as `xsd:string` from cluttering the primary ontology map.
- Does not render literal values as primary graph nodes or as a visibility filter layer. Literal object values from datatype properties remain searchable and are shown in the inspector as predicate/value evidence owned by the selected subject node.
- Labels class-expression nodes with their property usage context when the expression is used as a property domain or range, for example `refine range: Capability ∪ Requirement`, so OWL domain/range constraints do not look like authored Reqvire model relations.
- Derives class and property slot facets from SHACL node shapes by combining `sh:targetClass`, `sh:property`, `sh:path`, `sh:datatype`, `sh:class`, `sh:nodeKind`, `sh:minCount`, `sh:maxCount`, `sh:pattern`, and `sh:in`. The target class inspector shall show those slots and facets directly, with source-shape evidence, without requiring users to inspect the shape node first.
- Attaches SHACL-derived slot facets to the named property node as well when the `sh:path` value is a named property already present in the graph. On a property node, those records represent target-class usages of the selected property as a slot; they must be labeled as property usages rather than as duplicate property definitions.

Interaction behavior:
- Provides search over ontology labels, IRIs, semantic kinds, source elements, and SHACL constraint terms.
- Provides node or construct focus, neighbor highlighting, and an inspector for full IRI, semantic kind, RDF type evidence, comments, datatype-property literal values, source citations, domain/range, property characteristics, equivalence membership, inverse relationships, property chains, normalized SHACL-derived slots/facets, and optional raw SHACL evidence.
- Shows raw SHACL evidence only when direct raw constraints are attached to the inspected node. Class and property nodes that only receive normalized SHACL overlays must not show an empty raw-evidence section, and normalized slots/facets remain the primary readable representation of those SHACL overlays.
- Provides filters for grouped semantic node roles, relation edges, construct overlay kinds, and ontology data origin.
- Provides a compact passive color key for detailed semantic node roles and separate filter controls for visibility layers.
- Treats only filter-control entries as selectable filter-in controls in the interactive viewer. All filter controls are checked/active by default; unchecking an entry filters that visual layer out.
- Keeps detailed type-color entries passive so users do not have to treat every color swatch as a visibility control.
- Supports multi-select grouped role filters for ontology terms, properties, SHACL shapes, generic RDF resources, generic relation edges, and external vocabulary references. Detailed semantic types remain visible through color, inspector kind, and search badges.
- Keeps the external-reference role filter available but inactive by default; enabling it shows built-in vocabulary nodes used by datatype constraints, ranges, and other audit-oriented constructs.
- Supports multi-select construct filters for domain/range, subclass, membership, disjointness, equivalence, inverse, property chain, property characteristic, restriction, class-expression, and SHACL-overlay constructs.
- Applies construct filters to construct-specific edges, node badges, and inspector projection-construct rows without necessarily hiding the underlying ontology node when its semantic role remains active.
- Supports multi-select origin filters for authored Reqvire ontology/SHACL content, graph-registry extracted model facts, and generated ontology projection constructs. Generated construct edges shall be tagged with construct origin only, not registry origin, so disabling the construct origin visibly suppresses construct overlay edges even when registry origin remains enabled.
- Combines active filters predictably: filters within the same category are inclusive, while different active categories narrow the visible graph together. Role and origin filters are hard gates for node visibility; construct filters control construct edges, badges, slot/facet sections, and inspector rows only for nodes that remain visible through role and origin filters.
- Keeps filtered-out nodes, edges, badges, and construct rows visually suppressed or hidden without losing the current inspector selection unless the selected node itself is explicitly filtered out.
- Applies directional subclass and membership badges only to the subject side of the construct; superclass and class-object nodes must not display `⊆` or `∈` merely because another node points to them.
- Keeps raw Turtle content available through `ontologies.ttl` instead of duplicating the serialized text in the HTML viewer.

#### Metadata
 * type: specification

#### Relations
 * refine: [Ontologies View Generation](Capabilities.md#ontologies-view-generation)
---

### OWL Semantic Ontology Projection Refinement Specification

#### Details
Semantic projection behavior:
- Classifies resources into semantic node kinds, including OWL/RDFS class, object property, datatype property, RDF property, named individual, SHACL node shape, SHACL property shape, datatype, and generic RDF resource when no stronger kind is known. Literal values are not primary graph nodes; they are subject-owned inspector/search evidence.
- Promotes otherwise-generic named resources to named-individual view nodes when their RDF type evidence references a class declared in the same ontology graph, even when the authored RDF does not explicitly include `owl:NamedIndividual`.
- Materializes direct-authored OWL/RDFS/SHACL constructs as generated ontology projection facts attached to `SemanticIndex` before full semantic export or HTML rendering.
- Preserves full IRI, compact label, RDF types, source element identifiers, source file paths, source line numbers, comments, related SHACL constraints, normalized slot/facet evidence, optional raw SHACL evidence, and projection provenance in the HTML explorer model derived from `SemanticIndex.ontology_projection`.
- Derives SHACL slot/facet records from property-shape blank nodes and attaches them to the target class and named property graph nodes as viewer-facing construct evidence. Target class nodes present those records as slots of the class; named property nodes present those records as class-specific usages of the selected property.
- Uses semantic-query-contract refinements as declarative SPARQL pattern contracts for direct-authored construct extraction; the implementation may execute equivalent native Rust projection over parsed quads until a general query execution layer exists.
- Does not emit semantic-query-contract raw query text through ontology collection or `--full`; generated facts may cite semantic-query-contract IRIs as provenance.
- Separates direct-authored generated facts from inferred facts. Direct-authored facts may drive HTML/export now; inferred facts require a later inference or materialization requirement.
- Suppresses `rdf:type` edges, OWL/RDFS metaclass nodes such as `owl:Class`, `owl:ObjectProperty`, `owl:DatatypeProperty`, and `rdfs:Class`, and RDF list plumbing from the primary graph.
- Suppresses anonymous blank nodes from the primary graph unless the blank node represents a meaningful semantic construct such as a property chain, equivalence group, SHACL shape, or collection member.
- Retains unmodeled RDF statements only as inspector/source evidence, not as graph nodes and edges.

#### Metadata
 * type: specification

#### Relations
 * refine: [OWL Semantic Ontology Projection](Capabilities.md#owl-semantic-ontology-projection)
---

### Direct OWL Construct Projection Query Contract

This query contract defines the direct-authored OWL/RDFS construct patterns that the ontology projection subgraph must materialize for inclusion, membership, disjointness, equivalence, inverse properties, and restrictions.

#### Query
```sparql
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX sh: <http://www.w3.org/ns/shacl#>

SELECT ?constructKind ?subject ?predicate ?object ?property ?symbol WHERE {
  {
    ?subject rdfs:subClassOf ?object .
    BIND("inclusion" AS ?constructKind)
    BIND(rdfs:subClassOf AS ?predicate)
    BIND("U+2286" AS ?symbol)
  }
  UNION {
    ?subject rdf:type ?object .
    FILTER(?object NOT IN (owl:Class, rdfs:Class, owl:ObjectProperty, owl:DatatypeProperty, rdf:Property))
    BIND("membership" AS ?constructKind)
    BIND(rdf:type AS ?predicate)
    BIND("U+2208" AS ?symbol)
  }
  UNION {
    ?subject owl:disjointWith ?object .
    BIND("disjointness" AS ?constructKind)
    BIND(owl:disjointWith AS ?predicate)
    BIND("U+27C2" AS ?symbol)
  }
  UNION {
    VALUES ?predicate { owl:equivalentClass owl:equivalentProperty owl:sameAs }
    ?subject ?predicate ?object .
    BIND("equivalence" AS ?constructKind)
    BIND("U+21D4" AS ?symbol)
  }
  UNION {
    ?subject owl:inverseOf ?object .
    BIND("inverse-property" AS ?constructKind)
    BIND(owl:inverseOf AS ?predicate)
    BIND("U+27F2" AS ?symbol)
  }
  UNION {
    ?subject a owl:Restriction ;
      owl:onProperty ?property ;
      owl:allValuesFrom ?object .
    BIND("universal-restriction" AS ?constructKind)
    BIND(owl:allValuesFrom AS ?predicate)
    BIND("U+2200" AS ?symbol)
  }
  UNION {
    ?subject a owl:Restriction ;
      owl:onProperty ?property ;
      owl:someValuesFrom ?object .
    BIND("existential-restriction" AS ?constructKind)
    BIND(owl:someValuesFrom AS ?predicate)
    BIND("U+2203" AS ?symbol)
  }
}
```

#### Metadata
 * type: semantic-query-contract

#### Relations
 * refine: [OWL Semantic Ontology Projection](Capabilities.md#owl-semantic-ontology-projection)
---

### RDF List OWL Construct Projection Query Contract

This query contract defines authored RDF-list constructs that the ontology projection subgraph must materialize for intersections, unions, and property chains. The native projector shall preserve list order when creating generated projection facts.

#### Query
```sparql
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX owl: <http://www.w3.org/2002/07/owl#>

SELECT ?constructKind ?subject ?predicate ?list ?member ?symbol WHERE {
  {
    ?subject owl:intersectionOf ?list .
    ?list rdf:rest*/rdf:first ?member .
    BIND("intersection" AS ?constructKind)
    BIND(owl:intersectionOf AS ?predicate)
    BIND("U+2229" AS ?symbol)
  }
  UNION {
    ?subject owl:unionOf ?list .
    ?list rdf:rest*/rdf:first ?member .
    BIND("union" AS ?constructKind)
    BIND(owl:unionOf AS ?predicate)
    BIND("U+222A" AS ?symbol)
  }
  UNION {
    ?subject owl:propertyChainAxiom ?list .
    ?list rdf:rest*/rdf:first ?member .
    BIND("property-chain" AS ?constructKind)
    BIND(owl:propertyChainAxiom AS ?predicate)
    BIND("U+21D2" AS ?symbol)
  }
}
```

#### Metadata
 * type: semantic-query-contract

#### Relations
 * refine: [OWL Semantic Ontology Projection](Capabilities.md#owl-semantic-ontology-projection)
---

### OWL Property Metadata Projection Query Contract

This query contract defines property-centric authored patterns that the ontology projection subgraph must materialize for domain, range, and property characteristic badges.

#### Query
```sparql
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX owl: <http://www.w3.org/2002/07/owl#>

SELECT ?constructKind ?subject ?predicate ?object ?symbol WHERE {
  {
    ?subject rdfs:domain ?object .
    BIND("property-domain" AS ?constructKind)
    BIND(rdfs:domain AS ?predicate)
    BIND("" AS ?symbol)
  }
  UNION {
    ?subject rdfs:range ?object .
    BIND("property-range" AS ?constructKind)
    BIND(rdfs:range AS ?predicate)
    BIND("" AS ?symbol)
  }
  UNION {
    VALUES (?object ?constructKind ?symbol) {
      (owl:FunctionalProperty "functional-property" "U+2192")
      (owl:InverseFunctionalProperty "inverse-functional-property" "U+2190")
      (owl:SymmetricProperty "symmetric-property" "U+2194")
      (owl:AsymmetricProperty "asymmetric-property" "U+21AE")
      (owl:ReflexiveProperty "reflexive-property" "U+25CB")
      (owl:IrreflexiveProperty "irreflexive-property" "U+2209")
      (owl:TransitiveProperty "transitive-property" "U+25B3")
    }
    ?subject rdf:type ?object .
    BIND(rdf:type AS ?predicate)
  }
}
```

#### Metadata
 * type: semantic-query-contract

#### Relations
 * refine: [Ontology Property-Centric Visualization](Capabilities.md#ontology-property-centric-visualization)
---

### Ontology Property-Centric Visualization Refinement Specification

#### Details
Property-centric visualization behavior:
- Renders object properties and datatype properties as first-class property nodes or property cards.
- Aggregates all `rdfs:domain` classes and all `rdfs:range` classes or datatypes for each property.
- Shows domain and range as internal property compartments or ports by default so that one property with many domains or ranges does not become many repeated arcs.
- May provide an expanded layout that shows domain and range as external nodes, but the compact property-centered layout remains the default.
- Distinguishes object-property ranges that point to classes from datatype-property ranges that point to datatypes or literal constraints.
- Shows property characteristics as compact badges or inspector attributes instead of overloading the primary label by default.
- Shows SHACL-derived slot facets on target classes and named properties, including datatype/class range constraints, node kind, min/max cardinality, pattern constraints, allowed values, and source shape.
- Recognizes OWL property characteristic types including functional, inverse functional, transitive, symmetric, asymmetric, reflexive, and irreflexive properties.

#### Metadata
 * type: specification

#### Relations
 * refine: [Ontology Property-Centric Visualization](Capabilities.md#ontology-property-centric-visualization)
---

### Ontology Construct Grouping Refinement Specification

#### Details
OWL construct grouping behavior:
- Computes equivalence groups for `owl:equivalentClass`, `owl:equivalentProperty`, and `owl:sameAs` using deterministic connected components over the equivalent resources.
- Assigns each equivalence group a stable identifier derived from a canonical sorted member list rather than a random UUID.
- Renders equivalence groups as collapsible group nodes or grouped regions so users can inspect group membership without requiring pairwise equivalence edges to dominate the graph.
- Represents `owl:inverseOf` as an inverse relationship between property nodes with clear visual treatment and inspector evidence.
- Parses `owl:propertyChainAxiom` RDF lists into ordered chain members and attaches the ordered chain to the defining object property.
- Renders property chains as collapsible ordered chain constructs, preserving the member order from the RDF list.
- Allows an object property to participate in multiple property chains without duplicating the property node.
- Presents SHACL node shapes and property shapes as an overlay on referenced ontology terms and as inspector constraints, not as raw blank-node plumbing mixed with the ontology model.
- Presents anonymous OWL class-expression blank nodes such as `owl:unionOf`, `owl:intersectionOf`, and `owl:complementOf` as structural inspector constructs with expression kind, ordered members, and usage context. Raw blank-node identifiers are available only in collapsible raw details.

#### Metadata
 * type: specification

#### Relations
 * refine: [Ontology Construct Grouping](Capabilities.md#ontology-construct-grouping)
---

### Ontology Symbol and Badge Vocabulary Refinement Specification

#### Details
The ontology viewer symbol vocabulary defines canonical rendered symbols for OWL and set-theoretic concepts. The viewer shall render the `Rendered Unicode Character` value for visual badges and shall keep the `Raw Unicode` code point available in source/specification evidence.

Symbols shall:
- Appear only in approved viewer locations listed in the table.
- Have tooltip text and inspector text that names the semantic concept.
- Have accessible labels when rendered in interactive controls, badges, edges, or group headers.
- Supplement normal labels; labels such as property names and class names shall remain searchable without symbol prefixes.
- Keep raw Unicode code points in source/specification evidence and semantic data; do not render raw code points as visible badge text in the ontology inspector.
- Prefer domain labels such as `Subclass` over low-level set-theory labels such as `Subset or equal` when rendering visible ontology badges.
- Use font fallbacks that can render mathematical and arrow symbols consistently.

| Concept | Raw Unicode | Rendered Unicode Character | Viewer Usage | Tooltip / Accessible Label |
|---------|-------------|----------------------------|--------------|----------------------------|
| Disjointness | U+27C2 | ⟂ | Class expression badge, construct group | Disjointness |
| Logical AND / Set Intersection | U+2229 | ∩ | Class expression badge, construct group | Intersection |
| Logical OR / Set Union | U+222A | ∪ | Class expression badge, construct group | Union |
| Logical Implication | U+21D2 | ⇒ | Rule or implication edge label | Implies |
| Logical Equivalence | U+21D4 | ⇔ | Equivalence edge label, equivalence group header | Equivalent |
| Universal Quantifier | U+2200 | ∀ | Restriction badge, inspector field | Universal restriction |
| Existential Quantifier | U+2203 | ∃ | Restriction badge, inspector field | Existential restriction |
| Set Membership | U+2208 | ∈ | Inspector field, class membership edge label | Member of |
| Set Non-Membership | U+2209 | ∉ | Inspector field, class exclusion edge label | Not member of |
| Set Inclusion | U+2286 | ⊆ | Subclass or inclusion badge | Subset or equal |
| Proper Subset | U+2282 | ⊂ | Strict inclusion badge | Proper subset |
| Set Difference | U+2216 | ∖ | Class expression badge | Set difference |
| Symmetric Relation | U+2194 | ↔ | Property characteristic badge | Symmetric property |
| Inverse Relation | U+27F2 | ⟲ | Inverse property edge label, property badge | Inverse property |
| Reflexive Relation | U+25CB | ○ | Property characteristic badge | Reflexive property |
| Transitive Relation | U+25B3 | △ | Property characteristic badge | Transitive property |
| Antisymmetric / Asymmetric Relation | U+21AE | ↮ | Property characteristic badge | Asymmetric property |
| Inverse Functional | U+2190 | ← | Property characteristic badge | Inverse functional property |
| Functional | U+2192 | → | Property characteristic badge | Functional property |

#### Metadata
 * type: specification

#### Relations
 * refine: [Ontology Symbol and Badge Vocabulary](Capabilities.md#ontology-symbol-and-badge-vocabulary)
---

### Model View Element Navigation Refinement Specification

#### Details
Model-view element navigation behavior:
- Element-name headers render as hyperlinks.
- Links target element source file plus fragment identifier.
- Link format: `[Element Name](file_path#element-fragment)`.
- Navigation enables direct jump from model view to definition.

#### Metadata
 * type: specification

#### Relations
 * refine: [Model View Element Navigation](Capabilities.md#model-view-element-navigation)
---

### Model-Centric View Generation Refinement Specification

#### Details
Model-centric view generation behavior:
- Uses model roots selected by default traversal rules as top-level entries.
- Expands relations recursively with full target element details.
- Includes summary metadata for element and relation counts.
- Generates Mermaid diagrams for nested relation structures.
- Produces markdown output that is later rendered as `model.html`.

#### Metadata
 * type: specification

#### Relations
 * refine: [Model-Centric View Generation](Capabilities.md#model-centric-view-generation)
---

### Responsive HTML Generation Refinement Specification

#### Details
Responsive HTML behavior:
- Supports viewport widths from 320px (mobile) through 1920px+ (desktop).
- Uses mobile-first CSS with progressive enhancement.
- Provides hamburger navigation for viewports under 768px.
- Scales typography and spacing by responsive breakpoints.

Breakpoints:
- `sm`: 640px and up.
- `md`: 768px and up.
- `lg`: 1024px and up.
- `xl`: 1280px and up.

#### Metadata
 * type: specification

#### Relations
 * refine: [Responsive HTML Generation](HTMLGeneration.md#responsive-html-generation)
---

### Serve Command Refinement Specification

#### Details
Serve command behavior:
- Accept `--host <HOST>` option to specify the bind address (default: localhost)
- Accept `--port <PORT>` option to specify the server port (default: 8080)
- Use a random temporary directory for HTML export
- Run HTML Export to generate complete documentation in the temporary directory
- Start an HTTP server serving static files from the temporary directory
- Display clickable server URL for user to open in browser
- Display instructions to press Ctrl-C to stop server
- Continue serving until terminated by the user (Ctrl-C)

#### Metadata
 * type: specification

#### Relations
 * refine: [Serve Command](Capabilities.md#serve-command)
---

### Type-Safe HTML Generation Refinement Specification

#### Details
Type-safe HTML generation behavior:
- Uses `maud` macros for compile-time HTML generation.
- Relies on Rust type checks to validate structure during compilation.
- Prevents malformed tags, unclosed nodes, and invalid nesting.
- Produces well-formed HTML5 output for generated pages.

This shifts most structural HTML errors to compile time instead of runtime.

#### Metadata
 * type: specification

#### Relations
 * refine: [Type-Safe HTML Generation](HTMLGeneration.md#type-safe-html-generation)
---

### Web Interface Refinement Specification

#### Details
The browse interface allows users to:
- View HTML-rendered specifications and requirements
- Navigate through diagrams and visualizations
- Access verification traces and coverage reports
- Explore the complete model structure through an integrated web interface

This capability enables both human users (via browser) and AI agents (via MCP server) to efficiently explore and understand the System model without manually navigating file structures.

All generated HTML content is expected to produce deterministic output with consistent ordering to enable reliable version control and reproducible builds.

The system is expected to ensure deterministic HTML output by:
- Sorting elements by identifier before rendering
- Sorting relations by type and target identifier
- Maintaining consistent navigation and page ordering
- Generating stable diagram node and relation ordering

This determinism ensures that:
- Running HTML generation multiple times produces byte-identical output
- Version control diffs reflect actual content changes
- Continuous integration pipelines produce reproducible results

#### Metadata
 * type: specification
---

### Web Interface Style Specification

Styling conventions for HTML export web interface.

#### Details
**Page Layout:**
- Navigation bar at top with links to views
- Content area with responsive width
- Sidebar for element tree (optional)

**Typography:**
- System font stack for readability
- Monospace for code and identifiers
- Heading hierarchy matches markdown levels

**Color Palette (MONO Theme):**
The web interface uses a monochrome grayscale theme for consistent, professional appearance.

| Usage | Color | Notes |
|-------|-------|-------|
| Navigation background | #1c1c1c | Dark gray for main nav bar |
| Navigation hover | #2a2a2a | Subtle highlight on hover |
| Primary/Buttons | #333333 | Lighter gray for interactive elements |
| Primary hover | #4a4a4a | Button hover state |
| Page background | #FAFAFA | Light gray page background |
| Content background | #FFFFFF | White content cards |
| Text primary | #212121 | Dark gray for headings |
| Text secondary | #424242 | Medium gray for body text |
| Text muted | #757575 | Light gray for secondary info |
| Links | #4a4a4a | Grayscale links |
| Borders | #EEEEEE | Light borders |
| Highlight | #d0d0d0 | Selection/highlight color |

**Element Cards:**
| Element Type | Border Color | Background |
|--------------|--------------|------------|
| Requirement | #0066FF | #D0E0FF |
| User-requirement | #0066FF | #D0E0FF |
| Verification | #CC9900 | #FFF7B3 |
| Behavior | #9900CC | #E0D0FF |
| Specification | #009900 | #DFFFD0 |
| Constraint | #CC0000 | #FFD0D0 |

**Navigation:**
- Breadcrumb trail for element hierarchy
- Clickable relation links
- Collapsible sections for long content

#### Metadata
 * type: specification
---
