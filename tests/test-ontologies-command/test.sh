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

ONTOLOGIES_ENTRY="$TEST_DIR/out/ontologies"'.html'
if [ -f "$ONTOLOGIES_ENTRY" ]; then
  echo "FAILED: export must not generate standalone Ontologies page"
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

# Exported index.html must expose the ontology semantic view-model through the
# single SPA Project Store seed.
INDEX_FILE="$TEST_DIR/out/index.html" node - <<'NODE'
const fs = require("fs");
const html = fs.readFileSync(process.env.INDEX_FILE, "utf8");
const match = html.match(/(?:const|let|var)\s+reqvireProjectStore\s*=\s*(\{[\s\S]*?\});\s*<\/script>/);
if (!match) {
  console.error("FAILED: exported index.html missing Project Store seed");
  process.exit(1);
}
const store = JSON.parse(match[1]);
if (!store.ontology || store.ontology.ttl_href !== "ontologies.ttl") {
  console.error("FAILED: Project Store ontology projection missing ontologies.ttl link");
  process.exit(1);
}
if (!store.ontology.graph_renderer || !store.ontology.graph_data) {
  console.error("FAILED: Project Store ontology projection missing graph renderer/data");
  process.exit(1);
}
const graph = store.ontology.graph_data;
if (graph.nodes.some((node) => node.semantic_type === "literal" || node.type === "literal")) {
  console.error("FAILED: literal values should not render as primary ontology graph nodes");
  process.exit(1);
}
if (graph.nodes.some((node) => node.id === "http://www.w3.org/2001/XMLSchema#string")) {
  console.error("FAILED: xsd:string datatype constraints should remain evidence, not primary ontology graph nodes");
  process.exit(1);
}
const service = graph.nodes.find((node) => node.id === "urn:reqvire:test:api:ServiceEndpoint");
if (!service || service.type !== "owl" || service.semantic_type !== "class") {
  console.error("FAILED: declared OWL class should be a class graph node");
  process.exit(1);
}
const serviceSlot = (service.slot_facets || []).find((slot) =>
  slot.slot_iri === "urn:reqvire:test:api:identifier"
    && slot.source_shape_iri === "urn:reqvire:test:api:ServiceEndpointShape"
);
if (!serviceSlot) {
  console.error("FAILED: target class should retain SHACL slot facet evidence");
  process.exit(1);
}
const identifier = graph.nodes.find((node) => node.id === "urn:reqvire:test:api:identifier");
if (!identifier || identifier.semantic_type !== "datatype-property") {
  console.error("FAILED: named SHACL path property should remain available as datatype property node");
  process.exit(1);
}
const shape = graph.nodes.find((node) => node.id === "urn:reqvire:test:api:ServiceEndpointShape");
if (!shape || shape.type !== "shacl" || shape.semantic_type !== "node-shape") {
  console.error("FAILED: SHACL node shape should retain SHACL visual type");
  process.exit(1);
}
const secondary = graph.nodes.find((node) => node.id === "urn:reqvire:test:api:SecondaryEndpoint");
if (!secondary || secondary.type !== "owl" || secondary.semantic_type !== "named-individual") {
  console.error("FAILED: named IRI typed by a declared ontology class should render as a named individual");
  process.exit(1);
}
const secondaryIdentifier = (secondary.literal_values || []).find((item) =>
  String(item.predicate || "").endsWith("identifier") && item.value === "secondary"
);
if (!secondaryIdentifier) {
  console.error("FAILED: datatype-property literal values should be owned inspector/search evidence on the subject node");
  process.exit(1);
}
const secondaryMembership = (secondary.constructs || []).find((construct) =>
  construct.kind === "membership"
    && construct.subject === "urn:reqvire:test:api:SecondaryEndpoint"
    && construct.object === "urn:reqvire:test:api:ServiceEndpoint"
);
if (!secondaryMembership) {
  console.error("FAILED: typed named individual should retain membership construct evidence");
  process.exit(1);
}
const endpointRangeExpression = graph.nodes.find((node) =>
  node.semantic_type === "class-expression"
    && (node.constructs || []).some((construct) =>
      construct.kind === "property-range"
        && construct.subject === "urn:reqvire:test:api:exposes"
        && construct.object === node.id
    )
    && (node.constructs || []).some((construct) =>
      construct.kind === "class-expression"
        && (construct.members || []).some((member) => String(member).endsWith("ServiceEndpoint"))
    )
);
if (!endpointRangeExpression) {
  console.error("FAILED: class expression nodes should retain property domain/range usage evidence for display labels");
  process.exit(1);
}
const existentialRestriction = graph.nodes.find((node) =>
  node.semantic_type === "restriction"
    && (node.constructs || []).some((construct) =>
      construct.kind === "restriction"
        && String(construct.label || "").toLowerCase().includes("existential")
        && String(construct.property || "").endsWith("produces")
        && String(construct.object || "").endsWith("Response")
    )
);
if (!existentialRestriction) {
  console.error("FAILED: restriction nodes should retain existential someValuesFrom construct evidence for glyph rendering");
  process.exit(1);
}
NODE

exit 0
