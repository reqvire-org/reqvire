#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

mkdir -p "${TEST_DIR}/specifications"
cp "${TEST_SCRIPT_DIR}/fixtures/SemanticContracts.md.txt" "${TEST_DIR}/specifications/SemanticContracts.md"

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: search command failed"
  echo "$OUTPUT"
  exit 1
fi

if ! echo "$OUTPUT" | jq . >/dev/null 2>&1; then
  echo "FAILED: search output should be valid JSON"
  echo "$OUTPUT"
  exit 1
fi

SUMMARY=$(echo "$OUTPUT" | jq '{
  ontology_name: (.files | to_entries[] | .value.elements[] | select(.name == "API Ontology") | .name),
  ontology_iri: (.files | to_entries[] | .value.elements[] | select(.name == "API Ontology") | .ontology.iri),
  ontology_language: (.files | to_entries[] | .value.elements[] | select(.name == "API Ontology") | .ontology.ontology.language),
  ontology_contains_class: (.files | to_entries[] | .value.elements[] | select(.name == "API Ontology") | .ontology.ontology.content | contains("api:ServiceEndpoint")),
  ontology_has_line_number: (.files | to_entries[] | .value.elements[] | select(.name == "API Ontology") | .ontology.ontology.line_number > 0),
  semantic_contract_name: (.files | to_entries[] | .value.elements[] | select(.name == "API Endpoint Shape Contract") | .name),
  semantic_contract_iri: (.files | to_entries[] | .value.elements[] | select(.name == "API Endpoint Shape Contract") | .semantic_contract.iri),
  shapes_language: (.files | to_entries[] | .value.elements[] | select(.name == "API Endpoint Shape Contract") | .semantic_contract.shapes.language),
  shapes_contains_node_shape: (.files | to_entries[] | .value.elements[] | select(.name == "API Endpoint Shape Contract") | .semantic_contract.shapes.content | contains("sh:NodeShape")),
  semantic_contract_has_no_ontology: (.files | to_entries[] | .value.elements[] | select(.name == "API Endpoint Shape Contract") | has("ontology") | not)
}')

printf "%s\n" "$SUMMARY" > /tmp/semantic-contract-summary.json

if ! diff -u "${TEST_SCRIPT_DIR}/expected/semantic-contract-summary.json" /tmp/semantic-contract-summary.json; then
  echo "FAILED: semantic-contract JSON summary mismatch"
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/semantic-contract-summary.json"
  exit 1
fi

exit 0
