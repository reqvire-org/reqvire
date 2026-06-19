# Elements

### Ontology Rendering Details

#### Details
This document describes how the Ontologies Explorer turns authored RDF/Turtle, SHACL, and generated Reqvire ontology projection facts into the rendered graph.

The renderer is intentionally not a raw RDF triple viewer. It is an ontology-diagram projection:
- Classes, named individuals, SHACL shapes, restrictions, class expressions, and generic RDF resources are graph nodes when they carry meaningful ontology semantics.
- Object, datatype, and RDF properties are not graph boxes. They are projected into labeled relationship semantics, modal evidence, domain/range rows, SHACL slot rows, badges, and search text.
- Literal values and datatype constraints remain evidence. They do not become primary graph nodes.
- Built-in vocabulary terms such as `xsd:string`, `owl:Class`, and SHACL/RDF/RDFS metamodel resources remain supporting evidence unless explicitly enabled as external references.

#### Data Flow
Rendering starts from the `SemanticIndex`:
1. Authored ontology and SHACL blocks provide parsed RDF quads.
2. The semantic contract projection materializes direct OWL/RDFS/SHACL constructs such as domain/range, subclass, membership, equivalence, inverse properties, property chains, property characteristics, restrictions, class expressions, and SHACL overlays.
3. `build_graph_data` builds an `ontologyGraphData` JSON model containing semantic nodes, construct edges, property metadata, SHACL slot/facet records, badges, source links, ontology-node detail evidence, and explicit graph layer/source-kind metadata.
4. The browser module imports Graphology, Sigma, ForceAtlas2, `@sigma/edge-curve`, and `@sigma/node-image`.
5. The browser creates a Sigma graph from the projected nodes and rendered links, applies layout, and declares typed Sigma node/edge programs for construct glyphs and ontology connectors.

The generated `ontologyGraphData` is the contract between Rust projection logic and browser rendering. Filters and layout may hide graph items on the canvas, but they must not remove evidence from the selected ontology node modal.

#### Graph Layers
Ontology graph data uses explicit layers:
- `authored` contains authored ontology and SHACL semantics plus projection facts derived from that authored content.
- `reqvire-context` contains generated semantic context for model-to-term provenance only: model elements that `declaresTerm` or `referencesTerm` ontology terms.
- `external-source` is reserved for imported external ontology vocabulary triples in the used external subset.

The Ontologies view treats `authored` as the implicit base graph, not as an optional layer control. The left pane must not render an Authored row because disabling or selecting authored ontology content would remove the point of the ontology view itself. Semantic Context and External Sources are the only visible overlay filters, letting users inspect model-to-term provenance or used external subset vocabulary without changing the primary authored ontology view.

#### Semantic Projection
The graph projection applies these rules before Sigma sees the graph:
- Property nodes are filtered out of the primary rendered node set.
- Property domain/range facts are re-routed into property edges between rendered domain/range terms.
- Duplicate rendered links are deduplicated by source, target, label, rendered kind, and property identity.
- Literal nodes are suppressed.
- Datatype IRIs such as `http://www.w3.org/2001/XMLSchema#string` are suppressed as primary nodes.
- Anonymous blank nodes render only when they represent meaningful constructs, currently SHACL shapes, OWL restrictions, or OWL class expressions.
- RDF list plumbing is suppressed as graph nodes and raw `rdf:first`/`rdf:rest` edges; list members are preserved as readable construct evidence, such as class-expression members in the ontology node modal and property-chain member rows.
- Property details remain available in the ontology node modal for selected classes, individuals, and terms.

Construct nodes must win over generic class hints. For example, an anonymous node can first appear as the object of `rdfs:subClassOf` and later be recognized as `owl:Restriction`. The semantic type rank gives `restriction` and `class-expression` higher priority than generic `class` so these nodes survive primary graph filtering and render with construct notation.

#### Construct Classification
The browser uses one construct-class contract for render decisions:
- `nodeConstructClasses(nodeData)` derives construct classes from `semantic_type` and from projection `constructs`.
- `nodeHasConstructClass(nodeData, "restriction")` identifies OWL restriction nodes.
- `nodeHasConstructClass(nodeData, "class-expression")` identifies OWL class-expression nodes.

This same classification drives:
- Glyph-only construct rendering.
- Relation visibility controls.
- Construct-only node gating.
- Focus-neighborhood behavior.
- Suppression of forced Sigma labels on glyph-only nodes.
- Sigma image-node assignment for glyph-only construct nodes.

The renderer must not use separate ad hoc tests for the same concept. If a new construct family becomes visual, it should be added to the construct-class contract first, then consumed by filters and rendering.

#### Rendered Node Kinds
Rendered node roles are:
- `class`: normal OWL/RDFS class-like ontology terms.
- `named-individual`: authored or inferred named instances.
- `datatype`: authored datatype terms when meaningful, not built-in datatype evidence.
- `restriction`: OWL anonymous restriction constructs.
- `class-expression`: OWL anonymous set/operator expressions such as union, intersection, and complement.
- `node-shape`: SHACL node shapes.
- `property-shape`: SHACL property shapes when they are meaningful standalone shapes.
- `resource`: generic RDF resources that are not otherwise classified.

Property semantic kinds are still visible, but through edge label palettes and modal badges rather than node fill colors.

#### Rendered Edge Kinds
Rendered edge categories are derived from construct evidence:
- Domain/range property links: solid labeled Sigma arrows between domain and range anchors, with compact property label badges rendered on the edge.
- Subclass links: dedicated Sigma/WebGL dashed connectors with `Subclass of` labels and a hollow triangle marker at the superclass target side, using the same stroke cadence and strength as class-expression construct links.
- Membership links: class-instance links labeled `member`.
- SHACL overlays: visible overlay lines from SHACL shape context to ontology terms; generic overlay labels are omitted unless the edge has a more specific label.
- Inverse/equivalence/disjoint/property-chain/property-characteristic links: construct-specific edges governed by relation visibility controls when they have a direct canvas-visible control; otherwise they remain passive notation and modal evidence.
- Restriction links: separate Sigma-native construct connectors from restriction glyph nodes to their `on property` and filler/target evidence; they are not rendered as normal domain/range property edges.
- Class-expression links: dedicated Sigma/WebGL dashed membership connectors with an open diamond marker at the anonymous construct/source side.

Graph relationship edges are hidden in the default full-graph view. Edges become visible for the hovered focus tree, selected focus tree, or active rollover tree, subject to role and relation visibility controls.

#### WebVOWL-Aligned Constructs
The renderer follows the WebVOWL visual grammar where it improves ontology readability:
- OWL class expressions and restrictions render as compact glyph-only circles, not text-heavy boxes.
- Union, intersection, and complement use glyphs such as `U`, `∩`, and `¬` inside the construct circle.
- Restrictions use quantifier/cardinality glyphs such as `∀`, `∃`, `≥`, `≤`, `=`, or compact `R` when no more specific symbol applies.
- Class-expression and subclass connectors use dedicated Sigma edge program types so connector strokes are rendered by Sigma, not by an edge-label canvas overlay.
- Class-expression member links use a dedicated Sigma/WebGL edge program that draws a dashed curved connector with an open diamond marker at the anonymous construct/source side and an arrowhead at the member target side. Other WebVOWL marker variants should be added only through Sigma/WebGL edge programs, not through duplicate canvas connector drawing.
- Subclass links use a dedicated Sigma/WebGL edge program that draws the same dashed connector style used by class-expression construct links and a hollow triangle marker at the superclass target side.
- Restriction links remain construct notation around restriction glyph nodes, with `on property` and restriction-kind evidence in the edge label/modal detail instead of pretending the restriction is an ordinary property relationship.

Construct circles and set-operator edge styles should appear only for actual OWL anonymous construct nodes:
- `owl:unionOf`
- `owl:intersectionOf`
- `owl:complementOf`
- `owl:Restriction`, including `owl:someValuesFrom`, `owl:allValuesFrom`, and cardinality restrictions

They should not appear on ordinary named classes such as `RdfProjection` unless that named class is itself represented by an anonymous construct node in the projection.

#### Sigma Rendering Layers
The browser renderer uses Sigma 3 with Graphology:
- The base graph is a directed multi-graph.
- `@sigma/edge-curve` supplies curved arrow edge programs and parallel-edge indexing.
- ForceAtlas2 assigns the main layout.
- Sigma default node-label and hover rendering handles ordinary node labels.
- Normal node labels may be truncated for density; selected or hovered ordinary node labels switch to the full term label.
- Construct nodes use Sigma labels for their construct kind, such as `Union`, `Intersection`, `Complement`, or `Restriction`, while drawing the compact symbol through Sigma `nodeProgramClasses` with `@sigma/node-image`; no custom construct-node hover or overlay canvas is used.
- Construct glyph nodes use Sigma's node color as the circular image-node background, and their image is an inline SVG pictogram data URI containing only a bold semantic glyph. The renderer clips and centers the SVG through the Sigma image-node program instead of using PNG/raster sprites, SVG border detail, or transparent square image backgrounds. Out-of-focus construct nodes use the same Sigma reducer path as ordinary nodes, including muted node color and muted pictogram images.

Focused nodes use Sigma's `highlighted` node path and z-index handling. Focused relationship edges use Sigma's native `edgeReducer` path and the declared `@sigma/edge-curve` edge programs; there is no separate focused-edge canvas overlay. The renderer assigns explicit Sigma z-index bands so selected nodes render above focused neighbor nodes, focused neighbor nodes render above focused edges, and focused edges render above unrelated or muted graph items.

#### Filtering Semantics
The filter controls are canvas visibility controls, not data filters.

Role controls are coarse hard gates:
- Ontology terms
- SHACL shapes
- Resources
- External references

The `Show` controls expose canvas visibility toggles in one button group:
- Datatype property links
- Object property links
- Class disjointness
- Restrictions
- Class expressions
- SHACL shapes
- Resources
- External references

Ontology terms and class-membership context are always available because hiding them removes the meaningful ontology graph backbone. Checked means shown. The default state shows the remaining visibility controls so the ontology map opens as a complete authored graph; relationship density is managed through hover/selection focus rather than by hiding the core graph by default.

The `Overlays` controls expose only optional non-authored graph additions. Authored ontology content is implicit and is not shown as a layer row. Semantic Context and External Sources are the only checkable overlay rows. Semantic Context must not appear in the `Types` legend because it is provenance for model-to-term declaration/reference facts, not an ontology semantic node kind.

The single `SHACL shapes` role filter controls both SHACL shape nodes and their SHACL overlay relations. The renderer must not expose a second SHACL slot-overlay checkbox that can hide overlay relations while leaving SHACL shape nodes visible, or vice versa.

When no node is pinned, hovering a node computes its focused neighborhood from currently visible relations and fades unrelated visible nodes. When a node is selected, the selected node and its focused neighborhood become the pinned selection tree; nodes outside that tree are hidden from the canvas. Rolling over a visible node inside the pinned selection tree opens that node's own focused neighborhood as a temporary rollover tree. Nodes from the pinned selection tree that are outside the rollover tree stay visible but use the low-strength dimmed treatment; nodes outside both trees remain hidden. Disabling a relation filter removes neighbors that were reachable only through that relation from the selected or rollover focus tree, while ontology terms remain available as the graph backbone.

If the focused neighborhood reaches a visible construct-only node, the renderer expands through that construct node to include its currently visible member/filler links. This keeps OWL unions, intersections, complements, and restrictions readable from the selected class or property context without expanding through ordinary neighbor nodes.

Equivalence, inverse-property, property-chain, and property-characteristic constructs are currently passive `Notation` legend and modal evidence. They should not be exposed as active filters until they have a direct canvas-visible effect.

Changing any filter must not reset the camera. This keeps before/after visual comparison possible.

#### Modal Detail Semantics
The ontology node modal is evidence-oriented and intentionally independent from canvas filters:
- It shows the selected node's kind, RDF type evidence, URI or blank-node identifier, description, properties, domain/range, slots/facets, inverse/equivalence/chain evidence, projection constructs, source links, and raw SHACL evidence when directly present.
- Property usage rows are deduplicated by the underlying semantic facts rather than by repeated visual edges.
- SHACL slot/facet rows are normalized from target-class and property-shape evidence.
- Filtered-out badges, constructs, or SHACL evidence must remain available in the modal when they belong to the selected node.

This rule prevents graph decluttering from becoming accidental semantic data loss.

#### Reset, Drag, Hover, And Selection
The viewer distinguishes view operations:
- `Reset` reruns the layout and then fits the current graph view.
- Dragging a node updates its in-memory Graphology coordinates and refreshes the renderer, letting users uncover relation lines or labels hidden behind nodes or labels.
- Hover sets a temporary focus node and reveals its incident eligible edges.
- Click selection persists focus, centers the selected node, and opens a selected-node link in the left pane that launches the ontology node modal.
- Hover and selection can coexist through rollover refinement: selection pins the selected focus tree, and hovering a visible node inside that tree temporarily makes the hovered node's focus tree active while dimming selected-tree context outside the rollover tree.

Focused graph items use Sigma z-index, highlighted-node rendering, and native edge reducers rather than separate focus overlay canvases.

#### Testing Contract
The ontology command e2e test validates both projection and rendering contracts:
- Default Turtle and JSON-LD omit generated projection facts.
- Full Turtle and JSON-LD include generated projection facts.
- Served `ontologies.ttl` includes generated ontology document declarations plus authored ontology/SHACL content, and omits generated projection facts.
- The served Ontologies SPA route includes the Sigma, Graphology, ForceAtlas2, `@sigma/edge-curve`, and `@sigma/node-image` renderer evidence.
- Source tokens assert the unified construct-class helpers and Sigma image-node construct glyph path exist.
- Source tokens assert construct image nodes use Sigma's native circular image-node background with inline SVG pictogram data URIs.
- Graph JSON assertions ensure literals and built-in datatype nodes do not render as primary graph nodes.
- Fixture ontology includes an existential `owl:someValuesFrom` restriction so restriction construct nodes are proven to survive graph projection and retain construct evidence for glyph rendering.

#### Change Guidance
When changing ontology rendering:
- Update the semantic projection first if the graph is missing a meaningful ontology fact.
- Update `nodeConstructClasses` before adding special-case renderer/filter logic for a construct.
- Keep properties as edges/modal evidence unless there is a strong semantic reason to reintroduce them as nodes.
- Keep filters canvas-only and preserve modal evidence.
- Prefer Sigma and `@sigma/edge-curve` renderer hooks for edge geometry and labels.
- Update specifications, verifications, and the ontology e2e fixture/test in the same change.

#### Metadata
  * type: specification

#### Relations
  * define: [Ontologies View Generation](Capabilities.md#ontologies-view-generation)
---
