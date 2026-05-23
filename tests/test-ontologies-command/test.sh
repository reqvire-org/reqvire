#!/bin/bash
set -uo pipefail

set +e
TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies 2>&1)
TTL_EXIT=$?
set -e

if [ $TTL_EXIT -ne 0 ]; then
  echo "FAILED: ontologies command failed"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! echo "$TTL_OUTPUT" | grep -q "api:ServiceEndpoint a owl:Class"; then
  echo "FAILED: Turtle output missing ontology class"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! echo "$TTL_OUTPUT" | grep -q "sh:targetClass api:ServiceEndpoint"; then
  echo "FAILED: Turtle output missing SHACL target class"
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

if ! echo "$JSONLD_OUTPUT" | jq . >/dev/null 2>&1; then
  echo "FAILED: JSON-LD output should be valid JSON"
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

if ! echo "$FULL_TTL_OUTPUT" | grep -q "urn:reqvire:element:api-feature"; then
  echo "FAILED: full Turtle output missing feature element IRI"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! echo "$FULL_TTL_OUTPUT" | grep -q "reqvire:attaches <urn:reqvire:element:api-ontology>"; then
  echo "FAILED: full Turtle output missing feature ontology attachment edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! echo "$FULL_TTL_OUTPUT" | grep -q "reqvire:specifiedBy <urn:reqvire:element:api-endpoint-requirement>"; then
  echo "FAILED: full Turtle output missing feature requirement specifiedBy edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! echo "$FULL_TTL_OUTPUT" | grep -q "reqvire:declaresTerm <urn:reqvire:test:api:ServiceEndpoint>"; then
  echo "FAILED: full Turtle output missing ontology declaration edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! echo "$FULL_TTL_OUTPUT" | grep -q "reqvire:referencesTerm <urn:reqvire:test:api:ServiceEndpoint>"; then
  echo "FAILED: full Turtle output missing semantic-contract reference edge"
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

if ! echo "$FULL_JSONLD_OUTPUT" | jq . >/dev/null 2>&1; then
  echo "FAILED: full JSON-LD output should be valid JSON"
  echo "$FULL_JSONLD_OUTPUT"
  exit 1
fi

if ! echo "$FULL_JSONLD_OUTPUT" | grep -q "urn:reqvire:element:api-feature"; then
  echo "FAILED: full JSON-LD output missing model context element"
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

exit 0
