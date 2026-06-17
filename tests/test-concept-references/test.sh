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

#### Attachments
  * [API Ontology](#api-ontology)

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
  * Service Endpoint: https://example.test/ontology#ServiceEndpoint

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

SEARCH_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json)
SUMMARY=$(echo "$SEARCH_JSON" | jq '{
  concept_label: (.files["specifications/ConceptReferences.md"].elements[] | select(.name == "API Requirement") | .concept_references[0].label),
  concept_iri: (.files["specifications/ConceptReferences.md"].elements[] | select(.name == "API Requirement") | .concept_references[0].iri),
  concept_line_number_valid: (.files["specifications/ConceptReferences.md"].elements[] | select(.name == "API Requirement") | .concept_references[0].line_number > 0)
}')
printf "%s\n" "$SUMMARY" > /tmp/concept-references-summary.json
if ! diff -u "${TEST_SCRIPT_DIR}/expected/search-summary.json" /tmp/concept-references-summary.json; then
  echo "FAILED: concept reference search JSON mismatch"
  exit 1
fi

cp "$TEST_DIR/specifications/ConceptReferences.md" /tmp/concept-references-before-delete.md
set +e
DELETE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "API Ontology" 2>&1)
DELETE_EXIT=$?
set -e
if [ $DELETE_EXIT -eq 0 ]; then
  echo "FAILED: deleting ontology should fail while concept references still depend on it"
  echo "$DELETE_OUTPUT"
  exit 1
fi
for marker in \
  "Concept reference not found" \
  "specifications/ConceptReferences.md#api-requirement" \
  "Service Endpoint" \
  "https://example.test/ontology#ServiceEndpoint" \
  "Removed declaration source: specifications/ConceptReferences.md#api-ontology"
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

#### Attachments
  * [API Ontology](#api-ontology)

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
  * Missing Term: https://example.test/ontology#MissingTerm

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

#### Attachments
  * [API Ontology](#api-ontology)

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

### Infra Ontology

Infrastructure ontology terms.

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
testonto:Region a owl:Class .
```
---

### API Requirement

The system shall publish service endpoint contracts.

#### Metadata
  * type: requirement

#### Concept References
  * Region: https://example.test/ontology#Region

#### Relations
  * specify: [API Capability](#api-capability)
---
EOF
assert_invalid_model "Concept reference outside context"

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
  * Service Endpoint: https://example.test/ontology#ServiceEndpoint

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

exit 0
