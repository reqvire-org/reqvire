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

for forbidden in \
  "OntologyProjectionGraph" \
  "OntologyConstruct" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "urn:reqvire:ontology-symbol" \
  "projectionDerivationMode"; do
  if grep -qF "$forbidden" <<< "$TTL_OUTPUT"; then
    echo "FAILED: default Turtle output must remain authored ontology/SHACL only: $forbidden"
    exit 1
  fi
done

if grep -qF "$RAW_QUERY_SENTINEL" <<< "$TTL_OUTPUT"; then
  echo "FAILED: default Turtle output must not contain raw semantic-query-contract text"
  exit 1
fi

# Default Turtle output must carry the representative OWL/RDFS constructs.
for construct in \
  "propertyChainAxiom" \
  "inverseOf" \
  "equivalentClass" \
  "equivalentProperty" \
  "sameAs" \
  "domain" \
  "range"; do
  if ! grep -q "$construct" <<< "$TTL_OUTPUT"; then
    echo "FAILED: default Turtle output missing OWL/RDFS construct: $construct"
    exit 1
  fi
done

# xsd:string range must survive serialization (prefixed or full IRI form).
if ! grep -Eq "xsd:string|http://www\.w3\.org/2001/XMLSchema#string" <<< "$TTL_OUTPUT"; then
  echo "FAILED: default Turtle output missing xsd:string range"
  exit 1
fi

exit 0
