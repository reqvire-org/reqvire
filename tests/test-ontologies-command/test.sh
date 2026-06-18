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

count_occurrences() {
  { grep -oF "$1" <<< "$2" || true; } | wc -l | tr -d ' '
}

if ! grep -q "<https://example.test/ontology#ServiceEndpoint> a <http://www.w3.org/2002/07/owl#Class>" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing ontology class"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "<https://example.test/ontology> a owl:Ontology" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing generated ontology document declaration"
  echo "$TTL_OUTPUT"
  exit 1
fi

if grep -q "<https://example.test/ontology/api-ontology> a owl:Ontology" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output should declare the ontology document at ontology_base, not per element"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:termNamespace \"https://example.test/ontology#\"" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing generated ontology term namespace"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:ontologyPrefix \"testonto\"" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing generated ontology prefix"
  echo "$TTL_OUTPUT"
  exit 1
fi

for prefix in \
  "@prefix owl:" \
  "@prefix rdf:" \
  "@prefix rdfs:" \
  "@prefix reqvire:" \
  "@prefix testonto:" \
  "@prefix ext:" \
  "@prefix sh:" \
  "@prefix xs:" \
  "@prefix xsd:"; do
  PREFIX_COUNT=$(count_occurrences "$prefix" "$TTL_OUTPUT")
  if [ "$PREFIX_COUNT" -gt 1 ]; then
    echo "FAILED: Turtle output contains duplicate prefix declaration for $prefix"
    echo "$TTL_OUTPUT"
    exit 1
  fi
done

ONTOLOGY_DECL_COUNT=$(( \
  $(count_occurrences "<https://example.test/ontology> a owl:Ontology" "$TTL_OUTPUT") + \
  $(count_occurrences "<https://example.test/ontology> a <http://www.w3.org/2002/07/owl#Ontology>" "$TTL_OUTPUT") \
))
if [ "$ONTOLOGY_DECL_COUNT" -ne 1 ]; then
  echo "FAILED: Turtle output should contain exactly one ontology document type declaration"
  echo "$TTL_OUTPUT"
  exit 1
fi

IMPORT_COUNT=$(( \
  $(count_occurrences "owl:imports <https://example.test/imported>" "$TTL_OUTPUT") + \
  $(count_occurrences "<http://www.w3.org/2002/07/owl#imports> <https://example.test/imported>" "$TTL_OUTPUT") \
))
if [ "$IMPORT_COUNT" -ne 1 ]; then
  echo "FAILED: Turtle output should contain the duplicated authored owl:imports statement exactly once"
  echo "$TTL_OUTPUT"
  exit 1
fi

if grep -q "owl:imports <https://example.test/ontology>" <<< "$TTL_OUTPUT" || \
   grep -q "<http://www.w3.org/2002/07/owl#imports> <https://example.test/ontology>" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output must not synthesize ontology imports"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "<http://www.w3.org/2002/07/owl#someValuesFrom> <https://example.test/ontology#Response>" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing ontology restriction construct fixture"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "<http://www.w3.org/ns/shacl#targetClass> <https://example.test/ontology#ServiceEndpoint>" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing SHACL target class"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "<http://www.w3.org/ns/shacl#datatype> <https://example.test/external#ExternalCode>" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing SHACL reference to external custom datatype"
  echo "$TTL_OUTPUT"
  exit 1
fi

for external_source_token in \
  "ExternalResource" \
  "externalCode" \
  "External code datatype"; do
  if grep -qF "$external_source_token" <<< "$TTL_OUTPUT"; then
    echo "FAILED: default Turtle output must not include external ontology source triples: $external_source_token"
    echo "$TTL_OUTPUT"
    exit 1
  fi
done

set +e
EXTERNAL_TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --include-external 2>&1)
EXTERNAL_TTL_EXIT=$?
set -e

if [ $EXTERNAL_TTL_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --include-external command failed"
  echo "$EXTERNAL_TTL_OUTPUT"
  exit 1
fi

for external_source_token in \
  "<https://example.test/external> a <http://www.w3.org/2002/07/owl#Ontology>" \
  "<https://example.test/external#ExternalResource> a <http://www.w3.org/2002/07/owl#Class>" \
  "<https://example.test/external#ExternalCode> a <http://www.w3.org/2000/01/rdf-schema#Datatype>" \
  "<https://example.test/external#externalCode> a <http://www.w3.org/2002/07/owl#DatatypeProperty>"; do
  if ! grep -qF "$external_source_token" <<< "$EXTERNAL_TTL_OUTPUT"; then
    echo "FAILED: --include-external Turtle output missing external source triple: $external_source_token"
    echo "$EXTERNAL_TTL_OUTPUT"
    exit 1
  fi
done

set +e
FULL_EXTERNAL_TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --full --include-external 2>&1)
FULL_EXTERNAL_TTL_EXIT=$?
set -e

if [ $FULL_EXTERNAL_TTL_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --full --include-external command failed"
  echo "$FULL_EXTERNAL_TTL_OUTPUT"
  exit 1
fi

if ! grep -qF "<https://example.test/external#ExternalResource> a <http://www.w3.org/2002/07/owl#Class>" <<< "$FULL_EXTERNAL_TTL_OUTPUT"; then
  echo "FAILED: full external Turtle output missing external source class"
  echo "$FULL_EXTERNAL_TTL_OUTPUT"
  exit 1
fi

if ! grep -qF "reqvire:OntologyProjectionGraph" <<< "$FULL_EXTERNAL_TTL_OUTPUT"; then
  echo "FAILED: full external Turtle output missing ontology projection facts"
  echo "$FULL_EXTERNAL_TTL_OUTPUT"
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

if ! jq -e 'any(.[]; .["@id"] == "https://example.test/ontology" and ((.["http://www.w3.org/1999/02/22-rdf-syntax-ns#type"] // []) | map(.["@id"]) | index("http://www.w3.org/2002/07/owl#Ontology")))' >/dev/null 2>&1 <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: JSON-LD output missing generated ontology document declaration"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

if jq -e 'any(.[]; .["@id"] == "https://example.test/ontology/api-ontology" and ((.["http://www.w3.org/1999/02/22-rdf-syntax-ns#type"] // []) | map(.["@id"]) | index("http://www.w3.org/2002/07/owl#Ontology")))' >/dev/null 2>&1 <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: JSON-LD output should declare the ontology document at ontology_base, not per element"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

if ! grep -qF '"@value":"testonto"' <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: JSON-LD output missing generated ontology prefix"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

JSONLD_IMPORT_COUNT=$(jq '[.[] | select(.["@id"] == "https://example.test/ontology") | .["http://www.w3.org/2002/07/owl#imports"][]? | select(.["@id"] == "https://example.test/imported")] | length' <<< "$JSONLD_OUTPUT")
if [ "$JSONLD_IMPORT_COUNT" -ne 1 ]; then
  echo "FAILED: JSON-LD output should contain the duplicated authored owl:imports statement exactly once"
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

if ! grep -q "reqvire:conceptReference <https://example.test/ontology#ServiceEndpoint>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing concept-reference term edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if grep -q "reqvire:reusesContract <urn:reqvire:element:api-ontology>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output must not contain capability ontology reused_contract_context edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:specifiedBy <urn:reqvire:element:api-endpoint-requirement>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing capability requirement specifiedBy edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:declaresTerm <https://example.test/ontology#ServiceEndpoint>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing ontology term declaration edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:referencesTerm <https://example.test/ontology#ServiceEndpoint>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing semantic-contract reference edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

for relation_fact in \
  "reqvire:constrainedBy <urn:reqvire:element:api-endpoint-shape-contract>" \
  "reqvire:constrain <urn:reqvire:element:api-endpoint-requirement>" \
  "reqvire:use <urn:reqvire:element:api-ontology>" \
  "reqvire:usedBy <urn:reqvire:element:api-endpoint-shape-contract>"; do
  if ! grep -qF "$relation_fact" <<< "$FULL_TTL_OUTPUT"; then
    echo "FAILED: full Turtle output missing semantic-contract relation fact: $relation_fact"
    echo "$FULL_TTL_OUTPUT"
    exit 1
  fi
done

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

set +e
SEARCH_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name "API Endpoint Requirement|API Endpoint Shape Contract|API Ontology" --json 2>&1)
SEARCH_JSON_EXIT=$?
set -e

if [ $SEARCH_JSON_EXIT -ne 0 ]; then
  echo "FAILED: search --json command failed"
  echo "$SEARCH_JSON_OUTPUT"
  exit 1
fi

if ! jq -e '
  any(.files[].elements[];
    .name == "API Endpoint Requirement"
    and any(.relations[]; .relation_type == "constrainedBy" and (.target.target | endswith("SemanticContracts.md#api-endpoint-shape-contract"))))
  and any(.files[].elements[];
    .name == "API Endpoint Shape Contract"
    and (.semantic_contract.shapes.content | contains("sh:NodeShape"))
    and any(.relations[]; .relation_type == "constrain" and (.target.target | endswith("SemanticContracts.md#api-endpoint-requirement")))
    and any(.relations[]; .relation_type == "use" and (.target.target | endswith("SemanticContracts.md#api-ontology"))))
  and any(.files[].elements[];
    .name == "API Ontology"
    and any(.relations[]; .relation_type == "usedBy" and (.target.target | endswith("SemanticContracts.md#api-endpoint-shape-contract"))))
' >/dev/null 2>&1 <<< "$SEARCH_JSON_OUTPUT"; then
  echo "FAILED: search JSON output missing semantic-contract shape artifacts or relation edges"
  echo "$SEARCH_JSON_OUTPUT"
  exit 1
fi

set +e
MODEL_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --from "API Endpoint Requirement" --json 2>&1)
MODEL_JSON_EXIT=$?
set -e

if [ $MODEL_JSON_EXIT -ne 0 ]; then
  echo "FAILED: model --json command failed"
  echo "$MODEL_JSON_OUTPUT"
  exit 1
fi

if ! jq -e '
  any(.. | objects; .relation_type? == "constrainedBy")
  and any(.. | objects; .relation_type? == "use")
' >/dev/null 2>&1 <<< "$MODEL_JSON_OUTPUT"; then
  echo "FAILED: model JSON output missing semantic-contract constrainedBy/use relation chain"
  echo "$MODEL_JSON_OUTPUT"
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
    echo "FAILED: default Turtle output must include document declarations and authored ontology/SHACL only, without projection facts: $forbidden"
    exit 1
  fi
done

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

# Representative OWL reserved vocabulary IRIs must survive serialization without requiring local External Ontology sources.
for reserved_iri in \
  "http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral" \
  "http://www.w3.org/2000/01/rdf-schema#label" \
  "http://www.w3.org/2000/01/rdf-schema#comment" \
  "http://www.w3.org/2000/01/rdf-schema#Literal" \
  "http://www.w3.org/2002/07/owl#rational" \
  "http://www.w3.org/2002/07/owl#real" \
  "http://www.w3.org/2002/07/owl#Thing" \
  "http://www.w3.org/2001/XMLSchema#anyURI" \
  "http://www.w3.org/2001/XMLSchema#string" \
  "http://www.w3.org/2001/XMLSchema#boolean" \
  "http://www.w3.org/2001/XMLSchema#integer"; do
  if ! grep -qF "$reserved_iri" <<< "$TTL_OUTPUT"; then
    echo "FAILED: default Turtle output missing OWL reserved vocabulary IRI: $reserved_iri"
    echo "$TTL_OUTPUT"
    exit 1
  fi
done

if grep -A4 "#### External Ontology" specifications/SemanticContracts.md | grep -Eq "prefix: (rdf|rdfs|owl|xs|xsd)"; then
  echo "FAILED: OWL reserved vocabulary fixture must not use External Ontology declarations"
  exit 1
fi

exit 0
