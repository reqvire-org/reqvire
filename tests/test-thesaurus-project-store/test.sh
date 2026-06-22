#!/usr/bin/env bash
set -euo pipefail

EXPORT_DIR="${TEST_DIR}/exported-site"
STORE_JS="${EXPORT_DIR}/assets/project-store.js"

cd "$TEST_DIR"

"$REQVIRE_BIN" validate > "${TEST_DIR}/validate.log" 2>&1
"$REQVIRE_BIN" export --output "$EXPORT_DIR" > "${TEST_DIR}/export.log" 2>&1

if [ ! -f "$STORE_JS" ]; then
  echo "FAILED: exported Project Store asset is missing"
  cat "${TEST_DIR}/export.log"
  exit 1
fi

node - "$STORE_JS" <<'NODE'
const fs = require("fs");
const vm = require("vm");

const storePath = process.argv[2];
const source = fs.readFileSync(storePath, "utf8");
const context = { window: {} };
vm.runInNewContext(source, context, { filename: storePath });

const store = context.window.reqvireProjectStore;

function assert(condition, message) {
  if (!condition) {
    console.error(`FAILED: ${message}`);
    process.exit(1);
  }
}

assert(store, "Project Store seed was not assigned to window.reqvireProjectStore");
assert(store.thesaurus, "Project Store is missing top-level thesaurus projection");
assert(Array.isArray(store.thesaurus.schemes), "thesaurus.schemes is not an array");
assert(Array.isArray(store.thesaurus.concepts), "thesaurus.concepts is not an array");

const elementById = new Map(store.elements.map((element) => [element.id, element]));
const scheme = store.thesaurus.schemes.find((candidate) => candidate.label === "Store Test Thesaurus");
assert(scheme, "Store Test Thesaurus scheme row is missing");
assert(scheme.id === "https://example.test/thesaurus#StoreTestThesaurus", "scheme SKOS id is not canonical");
assert(
  scheme.element_id === "specifications/Thesaurus.md#store-test-thesaurus",
  "scheme element_id does not point to the native concept-scheme element",
);
assert(
  elementById.get(scheme.element_id)?.element_type === "concept-scheme",
  "scheme element_id does not resolve to a native concept-scheme element",
);
assert(scheme.concept_base === "https://example.test/thesaurus", "scheme concept_base is missing");
assert(scheme.concept_prefix === "concept", "scheme concept_prefix is missing");

const conceptByLabel = new Map(store.thesaurus.concepts.map((concept) => [concept.label, concept]));
const apiSurface = conceptByLabel.get("API Surface");
const endpoint = conceptByLabel.get("Service Endpoint");
const traceability = conceptByLabel.get("Traceability");

assert(apiSurface, "API Surface concept row is missing");
assert(endpoint, "Service Endpoint concept row is missing");
assert(traceability, "Traceability concept row is missing");

for (const concept of [apiSurface, endpoint, traceability]) {
  assert(concept.scheme_id === scheme.id, `${concept.label} scheme_id does not reference the scheme SKOS id`);
  assert(
    concept.scheme_element_id === scheme.element_id,
    `${concept.label} scheme_element_id does not reference the native scheme element`,
  );
  assert(
    elementById.get(concept.element_id)?.element_type === "concept",
    `${concept.label} element_id does not resolve to a native concept element`,
  );
  assert(
    concept.source_href?.includes(concept.element_id),
    `${concept.label} source_href does not target the native concept element`,
  );
}

assert(
  endpoint.element_id === "specifications/Thesaurus.md#service-endpoint",
  "Service Endpoint element_id does not point to the native concept element",
);
assert(endpoint.parent_id === apiSurface.id, "Service Endpoint parent_id does not use the broader API Surface concept id");
assert(
  apiSurface.related_ids.includes(traceability.id) && traceability.related_ids.includes(apiSurface.id),
  "related concept ids are not preserved bidirectionally in the thesaurus projection",
);
assert(endpoint.alt_labels.includes("Endpoint"), "Service Endpoint altLabel is missing");
assert(endpoint.scope_note.includes("addressable API surface"), "Service Endpoint scope note is missing");
assert(
  endpoint.maps_to.some((mapping) => mapping.id === "https://example.test/ontology#ServiceEndpoint"),
  "Service Endpoint is missing ontology mapsToConcept usage evidence",
);

const ontologyGraphConceptNode = store.ontology.graph_data.nodes.find((node) => node.id === endpoint.id);
assert(ontologyGraphConceptNode, "ontology graph does not include the exported SKOS concept node");
assert(
  !endpoint.element_id.includes("Ontology.md"),
  "Thesaurus projection uses ontology provenance as the concept element_id",
);
assert(
  endpoint.element_id !== ontologyGraphConceptNode.sources?.[0]?.source_element_identifier,
  "Thesaurus projection reused ontology graph provenance for native concept navigation",
);
NODE

echo "PASSED: Thesaurus Project Store projection uses native concept identity"
