#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

write_valid_model() {
  rm -rf "${TEST_DIR}/specifications"
  mkdir -p "${TEST_DIR}/specifications"
  cat > "${TEST_DIR}/specifications/ConceptReferences.md" << 'EOF'
# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Ontology

API ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: api

#### Ontology
```turtle
@prefix api: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
```
---

### API Concept Scheme

Native concept scheme for API vocabulary.

#### Metadata
  * type: concept-scheme
  * concept_base: https://example.test/concepts
  * concept_prefix: concept
---

### Service Endpoint

Native service endpoint concept.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [API Concept Scheme](#api-concept-scheme)
---

### API Requirement

The system shall publish service endpoint contracts.

#### Metadata
  * type: requirement

#### Concept References
  * [Service Endpoint](#service-endpoint)

#### Relations
  * specify: [API Capability](#api-capability)
---
EOF
}

assert_invalid_model() {
  local expected="$1"
  local output
  set +e
  output=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  local status=$?
  set -e
  if [ $status -eq 0 ]; then
    echo "FAILED: invalid model should fail validation"
    exit 1
  fi
  if ! echo "$output" | grep -Fq "$expected"; then
    echo "FAILED: expected error containing '${expected}'"
    echo "$output"
    exit 1
  fi
}

(
  cd "${TEST_DIR}" &&
    git init >/dev/null 2>&1 &&
    git config user.email test@example.com &&
    git config user.name "Test User"
)

write_valid_model
if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/concept-references-valid.out 2>&1); then
  echo "FAILED: valid concept references should validate"
  cat /tmp/concept-references-valid.out
  exit 1
fi

perl -0pi -e 's/\* \[Service Endpoint\]\(#service-endpoint\)/* Service Endpoint: https:\/\/example.test\/concepts#ServiceEndpoint/' "$TEST_DIR/specifications/ConceptReferences.md"
set +e
LEGACY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
LEGACY_EXIT=$?
set -e
if [ $LEGACY_EXIT -eq 0 ]; then
  echo "FAILED: legacy concept reference syntax should fail validation"
  exit 1
fi
if ! echo "$LEGACY_OUTPUT" | grep -Fq "must use a Markdown link to a native concept element"; then
  echo "FAILED: legacy concept reference syntax should report Markdown link requirement"
  echo "$LEGACY_OUTPUT"
  exit 1
fi
if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" migrate --fix > /tmp/concept-references-migrate.out 2>&1); then
  echo "FAILED: migrate --fix should rewrite legacy concept reference syntax"
  cat /tmp/concept-references-migrate.out
  exit 1
fi
if ! grep -Fq "* [Service Endpoint](#service-endpoint)" "$TEST_DIR/specifications/ConceptReferences.md"; then
  echo "FAILED: migrate --fix should restore canonical concept reference Markdown link"
  cat "$TEST_DIR/specifications/ConceptReferences.md"
  exit 1
fi
if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/concept-references-post-migrate.out 2>&1); then
  echo "FAILED: migrated concept references should validate"
  cat /tmp/concept-references-post-migrate.out
  exit 1
fi

SEARCH_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json)
SUMMARY=$(echo "$SEARCH_JSON" | jq '{
  concept_label: (.files["specifications/ConceptReferences.md"].elements[] | select(.name == "API Requirement") | .concept_references[0].label),
  concept_target: (.files["specifications/ConceptReferences.md"].elements[] | select(.name == "API Requirement") | .concept_references[0].target),
  concept_line_number_valid: (.files["specifications/ConceptReferences.md"].elements[] | select(.name == "API Requirement") | .concept_references[0].line_number > 0)
}')
printf "%s\n" "$SUMMARY" > /tmp/concept-references-summary.json
if ! diff -u "${TEST_SCRIPT_DIR}/expected/search-summary.json" /tmp/concept-references-summary.json; then
  echo "FAILED: concept reference search JSON mismatch"
  exit 1
fi

cp "$TEST_DIR/specifications/ConceptReferences.md" /tmp/concept-references-before-delete.md
set +e
DELETE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "Service Endpoint" 2>&1)
DELETE_EXIT=$?
set -e
if [ $DELETE_EXIT -eq 0 ]; then
  echo "FAILED: deleting native concept should fail while concept references still depend on it"
  echo "$DELETE_OUTPUT"
  exit 1
fi
for marker in \
  "Concept reference not found" \
  "specifications/ConceptReferences.md#api-requirement" \
  "Service Endpoint" \
  "specifications/ConceptReferences.md#service-endpoint" \
  "Removed declaration source: specifications/ConceptReferences.md#service-endpoint"
do
  if ! echo "$DELETE_OUTPUT" | grep -Fq "$marker"; then
    echo "FAILED: delete error missing marker: $marker"
    echo "$DELETE_OUTPUT"
    exit 1
  fi
done
if ! diff -u /tmp/concept-references-before-delete.md "$TEST_DIR/specifications/ConceptReferences.md"; then
  echo "FAILED: failed ontology delete should not persist file changes"
  exit 1
fi

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/ConceptReferences.md" << 'EOF'
# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Ontology

API ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
testonto:ServiceEndpoint a owl:Class .
```
---

### API Requirement

The system shall publish service endpoint contracts.

#### Metadata
  * type: requirement

#### Concept References
  * [Missing Term](#missing-term)

#### Relations
  * specify: [API Capability](#api-capability)
---
EOF
assert_invalid_model "Concept reference not found"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/ConceptReferences.md" << 'EOF'
# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Ontology

API ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: api

#### Ontology
```turtle
@prefix api: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
```
---

### API Concept Scheme

API concept scheme.

#### Metadata
  * type: concept-scheme
  * concept_base: https://example.test/concepts
  * concept_prefix: concept
---

### Service Endpoint

An externally visible service endpoint.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [API Concept Scheme](#api-concept-scheme)
---

### Region

A deployment or infrastructure region.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Service Endpoint](#service-endpoint)
  * broader: [Service Endpoint](#service-endpoint)
---

### API Structural Ontology

API structural ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Relations
  * derivedFrom: [API Ontology](#api-ontology)

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
testonto:ServiceEndpoint a owl:Class .
```
---

### Infra Structural Ontology

Infrastructure ontology terms.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [API Structural Ontology](#api-structural-ontology)

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

testonto:Region a owl:Class .
```
---

### API Requirement

The system shall publish service endpoint contracts.

#### Metadata
  * type: requirement

#### Concept References
  * [Region](#region)

#### Relations
  * specify: [API Capability](#api-capability)
---
EOF
if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/concept-references-global-context-valid.out 2>&1); then
  echo "FAILED: concept reference to a native concept in another element should validate"
  cat /tmp/concept-references-global-context-valid.out
  exit 1
fi

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/ConceptReferences.md" << 'EOF'
# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Ontology

API ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: api

#### Ontology
```turtle
@prefix api: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
```
---

### API Concept Scheme

API concept scheme.

#### Metadata
  * type: concept-scheme
  * concept_base: https://example.test/concepts
  * concept_prefix: concept
---

### Service Endpoint

An externally visible service endpoint.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [API Concept Scheme](#api-concept-scheme)
---

### API Structural Ontology

API structural ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Relations
  * derivedFrom: [API Ontology](#api-ontology)

#### Ontology
```turtle
@prefix api: <https://example.test/ontology#> .
@prefix concept: <https://example.test/concepts#> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
testonto:ServiceEndpoint a owl:Class ;
  reqvire:mapsToConcept concept:ServiceEndpoint .
```
---

### API Requirement

The system shall publish service endpoint contracts.

#### Metadata
  * type: requirement

#### Concept References
  * [Service Endpoint](#api-ontology)

#### Relations
  * specify: [API Capability](#api-capability)
---
EOF
assert_invalid_model "Concept References may target only native concept elements"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/ConceptReferences.md" << 'EOF'
# Elements

### API Ontology

API ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Concept References
  * [Service Endpoint](#api-ontology)

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
testonto:ServiceEndpoint a owl:Class .
```
---
EOF
assert_invalid_model "must not contain a #### Concept References section"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/ConceptReferences.md" << 'EOF'
# Elements

### API Ontology

API ontology terms.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
testonto:ServiceEndpoint a owl:Class .
```
---

### API Shape

API shape.

#### Metadata
  * type: semantic-contract

#### Concept References
  * [Service Endpoint](#api-ontology)

#### Relations
  * use: [API Ontology](#api-ontology)

#### Shapes
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

testonto:EndpointShape
  a sh:NodeShape ;
  sh:targetClass testonto:ServiceEndpoint .
```
---
EOF
assert_invalid_model "Semantic contract element 'API Shape' must not contain a #### Concept References section"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/ConceptReferences.md" << 'EOF'
# Elements

### API Ontology

API ontology terms with an invalid concept bridge.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix api: <https://example.test/ontology#> .
@prefix concept: <https://example.test/concepts#> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
testonto:ServiceEndpoint a owl:Class ;
  reqvire:mapsToConcept concept:MissingNativeConcept .
```
---
EOF
assert_invalid_model "reqvire:mapsToConcept must target a generated native concept resource typed as skos:Concept"

exit 0
