#!/bin/bash
set -uo pipefail

RAW_QUERY_SENTINEL="REQVIRE_ONTOLOGY_EXPORT_RAW_QUERY_SENTINEL"

set +e
TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies 2>&1)
TTL_EXIT=$?
set -e

if [ $TTL_EXIT -ne 0 ]; then
  echo "FAILED: ontologies command failed"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "api:ServiceEndpoint a owl:Class" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing ontology class"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "owl:someValuesFrom api:Response" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing ontology restriction construct fixture"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "sh:targetClass api:ServiceEndpoint" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing SHACL target class"
  echo "$TTL_OUTPUT"
  exit 1
fi

for forbidden in \
  "reqvire:OntologyProjectionGraph" \
  "reqvire:OntologyConstruct" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "urn:reqvire:ontology-symbol" \
  "reqvire:projectionDerivationMode"; do
  if grep -qF "$forbidden" <<< "$TTL_OUTPUT"; then
    echo "FAILED: default Turtle output must not contain generated ontology projection marker: $forbidden"
    echo "$TTL_OUTPUT"
    exit 1
  fi
done

if grep -qF "$RAW_QUERY_SENTINEL" <<< "$TTL_OUTPUT"; then
  echo "FAILED: default Turtle output must not contain raw semantic-query-contract text"
  echo "$TTL_OUTPUT"
  exit 1
fi

set +e
JSONLD_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --jsonld 2>&1)
JSONLD_EXIT=$?
set -e

if [ $JSONLD_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --jsonld command failed"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

if ! jq . >/dev/null 2>&1 <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: JSON-LD output should be valid JSON"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

for forbidden in \
  "OntologyProjectionGraph" \
  "OntologyConstruct" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "urn:reqvire:ontology-symbol" \
  "projectionDerivationMode"; do
  if grep -qF "$forbidden" <<< "$JSONLD_OUTPUT"; then
    echo "FAILED: default JSON-LD output must not contain generated ontology projection marker: $forbidden"
    echo "$JSONLD_OUTPUT"
    exit 1
  fi
done

if grep -qF "$RAW_QUERY_SENTINEL" <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: default JSON-LD output must not contain raw semantic-query-contract text"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

set +e
FULL_TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --full 2>&1)
FULL_TTL_EXIT=$?
set -e

if [ $FULL_TTL_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --full command failed"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "urn:reqvire:element:api-capability" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing capability element IRI"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:attaches <urn:reqvire:element:api-ontology>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing capability ontology attachment edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:specifiedBy <urn:reqvire:element:api-endpoint-requirement>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing capability requirement specifiedBy edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:declaresTerm <urn:reqvire:test:api:ServiceEndpoint>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing ontology declaration edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:referencesTerm <urn:reqvire:test:api:ServiceEndpoint>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing semantic-contract reference edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

for token in \
  "reqvire:OntologyProjectionGraph" \
  "reqvire:OntologyConstructProjection" \
  "reqvire:OntologyConstruct" \
  "reqvire:OntologySymbol" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "urn:reqvire:ontology-symbol" \
  "reqvire:projectionDerivationMode \"direct-authored\"" \
  "reqvire:constructFamily \"property-domain-range\"" \
  "reqvire:constructKind \"property-chain\"" \
  "reqvire:constructKind \"restriction\"" \
  "reqvire:restrictionKind \"existential\"" \
  "reqvire:constructSubject" \
  "reqvire:constructPredicate" \
  "reqvire:constructObject" \
  "reqvire:constructSourceBlock" \
  "reqvire:constructProvenance" \
  "reqvire:constructMember" \
  "reqvire:constructSequenceIndex"; do
  if ! grep -qF "$token" <<< "$FULL_TTL_OUTPUT"; then
    echo "FAILED: full Turtle output missing ontology projection fact: $token"
    echo "$FULL_TTL_OUTPUT"
    exit 1
  fi
done

if grep -qF "$RAW_QUERY_SENTINEL" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output must not contain raw semantic-query-contract text"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

set +e
FULL_JSONLD_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --full --jsonld 2>&1)
FULL_JSONLD_EXIT=$?
set -e

if [ $FULL_JSONLD_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --full --jsonld command failed"
  echo "$FULL_JSONLD_OUTPUT"
  exit 1
fi

if ! jq . >/dev/null 2>&1 <<< "$FULL_JSONLD_OUTPUT"; then
  echo "FAILED: full JSON-LD output should be valid JSON"
  echo "$FULL_JSONLD_OUTPUT"
  exit 1
fi

if ! grep -q "urn:reqvire:element:api-capability" <<< "$FULL_JSONLD_OUTPUT"; then
  echo "FAILED: full JSON-LD output missing model context element"
  echo "$FULL_JSONLD_OUTPUT"
  exit 1
fi

for token in \
  "OntologyProjectionGraph" \
  "OntologyConstruct" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "urn:reqvire:ontology-symbol" \
  "projectionDerivationMode"; do
  if ! grep -qF "$token" <<< "$FULL_JSONLD_OUTPUT"; then
    echo "FAILED: full JSON-LD output missing generated ontology projection marker: $token"
    echo "$FULL_JSONLD_OUTPUT"
    exit 1
  fi
done

if grep -qF "$RAW_QUERY_SENTINEL" <<< "$FULL_JSONLD_OUTPUT"; then
  echo "FAILED: full JSON-LD output must not contain raw semantic-query-contract text"
  echo "$FULL_JSONLD_OUTPUT"
  exit 1
fi

set +e
EXPORT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" export --output out 2>&1)
EXPORT_EXIT=$?
set -e

if [ $EXPORT_EXIT -ne 0 ]; then
  echo "FAILED: export command failed"
  echo "$EXPORT_OUTPUT"
  exit 1
fi

if [ ! -f "$TEST_DIR/out/ontologies.ttl" ]; then
  echo "FAILED: export did not generate ontologies.ttl"
  exit 1
fi

if [ ! -f "$TEST_DIR/out/ontologies.html" ]; then
  echo "FAILED: export did not generate ontologies.html"
  exit 1
fi

if ! grep -q "api:ServiceEndpoint" "$TEST_DIR/out/ontologies.ttl"; then
  echo "FAILED: exported ontologies.ttl missing ontology content"
  exit 1
fi

for forbidden in \
  "OntologyProjectionGraph" \
  "OntologyConstruct" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "urn:reqvire:ontology-symbol" \
  "projectionDerivationMode"; do
  if grep -qF "$forbidden" "$TEST_DIR/out/ontologies.ttl"; then
    echo "FAILED: exported ontologies.ttl must remain authored ontology/SHACL only: $forbidden"
    exit 1
  fi
done

if grep -qF "$RAW_QUERY_SENTINEL" "$TEST_DIR/out/ontologies.ttl"; then
  echo "FAILED: exported ontologies.ttl must not contain raw semantic-query-contract text"
  exit 1
fi

# Exported ontologies.ttl must carry the representative OWL/RDFS constructs.
TTL_FILE="$TEST_DIR/out/ontologies.ttl"
for construct in \
  "propertyChainAxiom" \
  "inverseOf" \
  "equivalentClass" \
  "equivalentProperty" \
  "sameAs" \
  "domain" \
  "range"; do
  if ! grep -q "$construct" "$TTL_FILE"; then
    echo "FAILED: exported ontologies.ttl missing OWL/RDFS construct: $construct"
    exit 1
  fi
done

# xsd:string range must survive serialization (prefixed or full IRI form).
if ! grep -Eq "xsd:string|http://www\.w3\.org/2001/XMLSchema#string" "$TTL_FILE"; then
  echo "FAILED: exported ontologies.ttl missing xsd:string range"
  exit 1
fi

# Exported ontologies.html must expose the semantic view-model / UI evidence.
HTML_FILE="$TEST_DIR/out/ontologies.html"
for token in \
  "ontology-sidebar-summary" \
  "ontology-summary-entry" \
  "ontology-summary-entry ontology-footer-download" \
  "ontology-legend-grid" \
  "ontology-color-key" \
  "ontology-legend-key-item" \
  "ontology-legend-symbol" \
  "graphology@0.25.4" \
  "sigma@3.0.0" \
  "graphology-layout-forceatlas2" \
  "@sigma/edge-curve@3.1.0" \
  "createDrawCurvedEdgeLabel" \
  "createEdgeCurveProgram" \
  "indexParallelEdgesIndex" \
  "new Graph({ type: 'directed', multi: true, allowSelfLoops: true })" \
  "new Sigma(graph, container" \
  "zIndex: true" \
  "edgeProgramClasses" \
  "curvedArrow: createEdgeCurveProgram" \
  "setOperatorArrow: createEdgeCurveProgram" \
  "lengthToThicknessRatio: 3.6" \
  "drawLabel: renderOntologySigmaEdgeLabel" \
  "renderOntologySigmaEdgeLabel" \
  "ontologyEdgeLabelText" \
  "ontologyEdgeProgramType" \
  "isSetOperatorEdge" \
  "drawSigmaCurvedEdgeLabel" \
  "if (isSetOperatorEdge(edgeData)) return ''" \
  "focusSigmaEdgeColor" \
  "renderWebVowlConnectorOverlay" \
  "installConstructGlyphLayer" \
  "drawConstructGlyphLayer" \
  "ontologyConstructGlyphs" \
  "installFocusOverlayLayer" \
  "drawFocusOverlay" \
  "ontologyFocusEdges" \
  "ontologyFocusNodes" \
  "overlayFocusNodeIds" \
  "drawFocusNodeOverlay" \
  "drawOpenArrowHead" \
  "drawWhiteDiamondMarker" \
  "drawSetOperatorNodeSymbol" \
  "drawConstructNodeSymbol" \
  "drawRestrictionConstructSymbol" \
  "isConstructGlyphNode" \
  "nodeConstructClasses" \
  "nodeHasConstructClass" \
  "setOperatorSymbol" \
  "nodeHasConstructClass(nodeData, 'restriction')" \
  "nodeHasConstructClass(nodeData, 'class-expression')" \
  "return ''" \
  "quadraticPoint" \
  "quadraticTangentAngle" \
  "context.setLineDash(lineStyle === 'dotted' ? [1, 7] : [8, 7])" \
  "result.zIndex = result.focused ? 100 : result.inFocusNeighborhood ? 80 : 0" \
  "isGenericShapeOverlayLabel" \
  "edgeHasConstruct(edgeData, 'shape-overlay') && isGenericShapeOverlayLabel(raw)" \
  "edgeLabelPalette" \
  "curvedEdgeLabelPoint" \
  "const offset = length * curvature" \
  "Subclass of" \
  "forceAtlas2.assign" \
  "sigmaNodeLabel" \
  "fullSigmaNodeLabel" \
  "fullLabel: fullSigmaNodeLabel(nodeData)" \
  "result.label = attributes.fullLabel || attributes.label ||" \
  "inspectorConstructDetails" \
  "inspectorSlotFacets" \
  "nodeReducer" \
  "edgeReducer" \
  "result.hidden = true" \
  "result.forceLabel = false" \
  "drawFocusSolidConnector" \
  "drawConstructGlyphNode" \
  "assignInitialSigmaPositions" \
  "applyOntologyLayout" \
  "nodeShapeType" \
  "class-anchor" \
  "connectionCounts" \
  "Math.sqrt(connectionCount)" \
  "sigmaNodeSize" \
  "sigmaEdgeColor" \
  "sigmaEdgeCurvature" \
  "applySigmaParallelEdgeCurvature" \
  "indexParallelEdgesIndex(graph)" \
  "parallelIndex" \
  "parallelMaxIndex" \
  "animatedReset" \
  "#8fb6e8" \
  "#9fbde3" \
  "#8bbd62" \
  "#d6a43f" \
  "#d8d8d4" \
  "Constructs" \
  "Notation" \
  "data-filter-category=\"role\"" \
  "checked data-webvowl-filter=\"datatype-properties\"" \
  "checked data-webvowl-filter=\"object-properties\"" \
  "checked data-webvowl-filter=\"class-membership\"" \
  "checked data-webvowl-filter=\"shacl-overlays\"" \
  "data-webvowl-filter=\"datatype-properties\"" \
  "data-webvowl-filter=\"object-properties\"" \
  "data-webvowl-filter=\"class-membership\"" \
  "data-webvowl-filter=\"class-disjointness\"" \
  "data-webvowl-filter=\"restrictions\"" \
  "data-webvowl-filter=\"class-expressions\"" \
  "data-webvowl-filter=\"shacl-overlays\"" \
  "data-filter-value=\"ontology-term\"" \
  "data-filter-value=\"shacl-shape\"" \
  "data-filter-value=\"resource\"" \
  "data-filter-value=\"shacl-shape\" aria-pressed=\"false\"" \
  "data-filter-value=\"resource\" aria-pressed=\"false\"" \
  "data-filter-value=\"external-reference\"" \
  "data-filter-value=\"external-reference\" aria-pressed=\"false\"" \
  "ontology-dot-class" \
  "ontology-dot-named-individual" \
  "ontology-dot-datatype" \
  "ontology-dot-restriction" \
  "ontology-dot-class-expression" \
  "ontology-dot-node-shape" \
  "ontology-dot-property-shape" \
  "ontology-dot-external-reference" \
  "ontology-edge-sample" \
  "Domain/range" \
  "Membership" \
  "Property char." \
  "SHACL overlay" \
  "isConstructOnlyNode" \
  "return hasConstructRole" \
  "label.includes('shape overlay')" \
  "label.endsWith('restriction')" \
  "['intersection', 'union', 'complement', 'class-expression']" \
  "constructs.add('restriction')" \
  "constructs.add('class-expression')" \
  "const webVowlFilterState = new Set([" \
  "!webVowlFilterState.has('datatype-properties')" \
  "!webVowlFilterState.has('object-properties')" \
  "!webVowlFilterState.has('shacl-overlays')" \
  "hasAuthoredSource" \
  "Download .ttl" \
  "ontology-source-name" \
  "SemanticContracts.html#api-ontology" \
  "body:has(.ontology-page) > div.w-full" \
  "height: calc(100vh - 50px)" \
  "body:has(.ontology-page) footer" \
  "display: none" \
  '"domain"' \
  '"range"' \
  '"slot_facets"' \
  "Slots / Facets" \
  "Used As Slot / Facets" \
  "Literal Values" \
  "ontology-slot-facet" \
  '"literal_values"' \
  "Projection Constructs" \
  "Raw SHACL Evidence" \
  "urn:reqvire:ontology-construct" \
  "Subclass" \
  "Property Chains" \
  "Inverse Properties" \
  "Equivalence Group" \
  "U+21D4" \
  "U+27F2" \
  "U+2192"; do
  if ! grep -qF "$token" "$HTML_FILE"; then
    echo "FAILED: exported ontologies.html missing UI evidence: $token"
    exit 1
  fi
done

for forbidden in \
  "<h1>Ontologies" \
  "Collected ontology element" \
  "ontology-code-block" \
  "class=\"ontology-block\"" \
  "ontology-source-list" \
  "ontology-source-block" \
  "ontology-sidebar-actions" \
  "ontology-badge-code" \
  "SHACL / Association Specs" \
  "No structural validation rules mapped." \
  "data-webvowl-filter=\"solitary-subclasses\"" \
  "data-webvowl-filter=\"equivalence\"" \
  "data-webvowl-filter=\"inverse-properties\"" \
  "data-webvowl-filter=\"property-chains\"" \
  "data-webvowl-filter=\"property-characteristics\"" \
  "ontology-degree-filter" \
  "ontology-degree-control" \
  "setOntologyMinDegree" \
  "resetOntologyDegreeFilter" \
  "nodePassesDegreeFilter" \
  "Solitary subclasses" \
  "isSolitarySubclassNode" \
  "selectedNodeId && !visibleNodeIds.has(selectedNodeId)" \
  "if (webVowlFilterState.has('datatype-properties') && edgeData.property_kind" \
  "if (webVowlFilterState.has('object-properties') && edgeData.rendered_kind" \
  "if (webVowlFilterState.has('shacl-overlays') && edgeHasConstruct" \
  "filterOntologyBlocks" \
  "ontology-graph-expand" \
  "toggleOntologyFullscreen" \
  "data-filter-category=\"origin\"" \
  "data-filter-value=\"authored\"" \
  "data-filter-value=\"registry\"" \
  "data-filter-value=\"construct\"" \
  "data-filter-category=\"construct\"" \
  "ontology-legend-origin-grid" \
  "ontology-origin-dot" \
  "const origins = new Set();" \
  "edgeHasAuthoredEndpointPair" \
  "origins.add('construct')" \
  "origins.add('registry')" \
  "data-filter-value=\"relation\"" \
  "edgeRoleValues" \
  "relationVisible" \
  "data-filter-value=\"class\"" \
  "data-filter-value=\"object-property\"" \
  "data-filter-value=\"datatype-property\"" \
  "data-filter-value=\"rdf-property\"" \
  "data-filter-value=\"named-individual\"" \
  "data-filter-value=\"datatype\"" \
  "data-filter-value=\"node-shape\"" \
  "data-filter-value=\"property-shape\"" \
  "data-filter-value=\"literal\"" \
  "ontology-dot-literal" \
  "https://d3js.org/d3.v7.min.js" \
  "forceSimulation" \
  "ontology-graph-svg" \
  "defaultEdgeType: 'arrow'" \
  "type: 'arrow'," \
  "defaultDrawNodeLabel:" \
  "renderOntologySigmaLabel"; do
  if grep -qF "$forbidden" "$HTML_FILE"; then
    echo "FAILED: exported ontologies.html contains obsolete explorer chrome/source block evidence: $forbidden"
    exit 1
  fi
done

# Datatype range evidence in the HTML view-model (prefixed or full IRI form).
if ! grep -Eq "xsd:string|http://www\.w3\.org/2001/XMLSchema#string" "$HTML_FILE"; then
  echo "FAILED: exported ontologies.html missing xsd:string datatype evidence"
  exit 1
fi

if grep -qF "$RAW_QUERY_SENTINEL" "$HTML_FILE"; then
  echo "FAILED: exported ontologies.html must not contain raw semantic-query-contract text"
  exit 1
fi

HTML_FILE="$HTML_FILE" node - <<'NODE'
const fs = require('fs');
const html = fs.readFileSync(process.env.HTML_FILE, 'utf8');
const match = html.match(/const ontologyGraphData = (.*?);<\/script>/s);
if (!match) {
  console.error('FAILED: exported ontologies.html missing ontologyGraphData JSON');
  process.exit(1);
}
const graph = JSON.parse(match[1]);
if (graph.nodes.some(node => node.semantic_type === 'literal' || node.type === 'literal')) {
  console.error('FAILED: literal values should not render as primary ontology graph nodes');
  process.exit(1);
}
if (graph.nodes.some(node => node.id === 'http://www.w3.org/2001/XMLSchema#string')) {
  console.error('FAILED: xsd:string datatype constraints should remain evidence, not primary ontology graph nodes');
  process.exit(1);
}
const service = graph.nodes.find(node => node.id === 'urn:reqvire:test:api:ServiceEndpoint');
if (!service || service.type !== 'owl' || service.semantic_type !== 'class') {
  console.error('FAILED: SHACL-targeted ontology class should remain class-colored, not SHACL-colored');
  process.exit(1);
}
const serviceSlot = (service.slot_facets || []).find(slot => slot.slot_iri === 'urn:reqvire:test:api:identifier');
if (!serviceSlot) {
  console.error('FAILED: SHACL property shape should materialize identifier slot facet on target class');
  process.exit(1);
}
const serviceFacetText = (serviceSlot.facets || []).map(facet => `${facet.name}=${facet.value}`).join('|');
if (!serviceFacetText.includes('datatype=string') || !serviceFacetText.includes('minCount=1')) {
  console.error('FAILED: target class slot facet should include datatype and minCount evidence');
  process.exit(1);
}
if (serviceSlot.source_shape_iri !== 'urn:reqvire:test:api:ServiceEndpointShape') {
  console.error('FAILED: target class slot facet should cite source shape');
  process.exit(1);
}
const identifier = graph.nodes.find(node => node.id === 'urn:reqvire:test:api:identifier');
if (!identifier || identifier.semantic_type !== 'datatype-property') {
  console.error('FAILED: named SHACL path property should remain available as datatype property node');
  process.exit(1);
}
const identifierUsage = (identifier.slot_facets || []).find(slot =>
  slot.target_class_iri === 'urn:reqvire:test:api:ServiceEndpoint'
    && slot.source_shape_iri === 'urn:reqvire:test:api:ServiceEndpointShape'
);
if (!identifierUsage) {
  console.error('FAILED: named property node should expose target-class slot usage evidence');
  process.exit(1);
}
const shape = graph.nodes.find(node => node.id === 'urn:reqvire:test:api:ServiceEndpointShape');
if (!shape || shape.type !== 'shacl' || shape.semantic_type !== 'node-shape') {
  console.error('FAILED: SHACL node shape should retain SHACL visual type');
  process.exit(1);
}
const secondary = graph.nodes.find(node => node.id === 'urn:reqvire:test:api:SecondaryEndpoint');
if (!secondary || secondary.type !== 'owl' || secondary.semantic_type !== 'named-individual') {
  console.error('FAILED: named IRI typed by a declared ontology class should render as a named individual');
  process.exit(1);
}
const secondaryIdentifier = (secondary.literal_values || []).find(item =>
  String(item.predicate || '').endsWith('identifier') && item.value === 'secondary'
);
if (!secondaryIdentifier) {
  console.error('FAILED: datatype-property literal values should be owned inspector/search evidence on the subject node');
  process.exit(1);
}
const secondaryMembership = (secondary.constructs || []).find(construct =>
  construct.kind === 'membership'
    && construct.subject === 'urn:reqvire:test:api:SecondaryEndpoint'
    && construct.object === 'urn:reqvire:test:api:ServiceEndpoint'
);
if (!secondaryMembership) {
  console.error('FAILED: typed named individual should retain membership construct evidence');
  process.exit(1);
}
const endpointRangeExpression = graph.nodes.find(node =>
  node.semantic_type === 'class-expression'
    && (node.constructs || []).some(construct =>
      construct.kind === 'property-range'
        && construct.subject === 'urn:reqvire:test:api:exposes'
        && construct.object === node.id
    )
    && (node.constructs || []).some(construct =>
      construct.kind === 'class-expression'
        && (construct.members || []).some(member => String(member).endsWith('ServiceEndpoint'))
    )
);
if (!endpointRangeExpression) {
  console.error('FAILED: class expression nodes should retain property domain/range usage evidence for display labels');
  process.exit(1);
}
const existentialRestriction = graph.nodes.find(node =>
  node.semantic_type === 'restriction'
    && (node.constructs || []).some(construct =>
      construct.kind === 'restriction'
        && String(construct.label || '').toLowerCase().includes('existential')
        && String(construct.property || '').endsWith('produces')
        && String(construct.object || '').endsWith('Response')
    )
);
if (!existentialRestriction) {
  console.error('FAILED: restriction nodes should retain existential someValuesFrom construct evidence for glyph rendering');
  process.exit(1);
}
NODE
if [ $? -ne 0 ]; then
  exit 1
fi

exit 0
