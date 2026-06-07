#!/bin/bash
set -euo pipefail

WORKSPACE="${TEST_DIR}/sqc-workspace"
MODEL_FILE="${WORKSPACE}/specifications/Model.md"

fail() {
  echo "FAILED: $1"
  if [ $# -gt 1 ] && [ -f "$2" ]; then
    cat "$2"
  fi
  exit 1
}

prepare_workspace() {
  rm -rf "$WORKSPACE"
  mkdir -p "${WORKSPACE}/specifications"
}

write_model() {
  prepare_workspace
  cat > "$MODEL_FILE"
}

run_reqvire() {
  (cd "$TEST_DIR" && "$REQVIRE_BIN" --workspace "$WORKSPACE" "$@")
}

expect_invalid() {
  local case_name="$1"
  local expected_pattern="$2"
  local output_file="${WORKSPACE}/output-${case_name// /-}.txt"

  write_model

  set +e
  run_reqvire validate > "$output_file" 2>&1
  local status=$?
  set -e

  if [ $status -eq 0 ]; then
    fail "${case_name} should fail validation" "$output_file"
  fi

  if ! grep -Eiq "$expected_pattern" "$output_file"; then
    echo "FAILED: ${case_name} should report '${expected_pattern}'"
    cat "$output_file"
    exit 1
  fi
}

write_model << 'EOF'
# Elements

### API Capability

API capability with reachable ontology context.

#### Metadata
  * type: capability

#### Attachments
  * [API Ontology](#api-ontology)

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Ontology

API ontology.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix api: <urn:reqvire:test:api:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

api:ServiceEndpoint a owl:Class .
```
---

### API Requirement

The system shall expose API endpoint semantics.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Capability](#api-capability)
  * refinedBy: [Endpoint Verification Query](#endpoint-verification-query)
---

### Endpoint Verification Query

Requirement-owned semantic query contract.

#### Metadata
  * type: semantic-query-contract

#### Relations
  * refine: [API Requirement](#api-requirement)

#### Query
```sparql
PREFIX reqvire: <https://www.reqvire.org/ontology#>

SELECT ?element ?marker
WHERE {
  ?element a reqvire:Requirement .
  BIND("SEMANTIC_QUERY_CONTRACT_MARKER" AS ?marker)
}
```
---
EOF

if ! run_reqvire validate > "${WORKSPACE}/valid-validate.txt" 2>&1; then
  fail "valid requirement-owned semantic-query-contract should validate" "${WORKSPACE}/valid-validate.txt"
fi

set +e
SEARCH_JSON=$(run_reqvire search --filter-type=semantic-query-contract --json 2>&1)
SEARCH_STATUS=$?
set -e

if [ $SEARCH_STATUS -ne 0 ]; then
  echo "$SEARCH_JSON"
  fail "search --filter-type=semantic-query-contract --json should succeed"
fi

if ! echo "$SEARCH_JSON" | jq . >/dev/null 2>&1; then
  echo "$SEARCH_JSON"
  fail "search output should be valid JSON"
fi

if ! echo "$SEARCH_JSON" | jq -e '
  [.files | to_entries[] | .value.elements[] | select(.name == "Endpoint Verification Query")] as $matches |
  ($matches | length) == 1 and
  ($matches[0].semantic_query_contract.iri == "urn:reqvire:semantic-query-contract:endpoint-verification-query") and
  ($matches[0].semantic_query_contract.query.language == "sparql") and
  ($matches[0].semantic_query_contract.query.content | contains("SEMANTIC_QUERY_CONTRACT_MARKER")) and
  ($matches[0].semantic_query_contract.query.line_number > 0) and
  ($matches[0] | has("semantic_contract") | not) and
  ($matches[0] | has("ontology") | not)
' >/dev/null; then
  echo "$SEARCH_JSON"
  fail "search JSON should expose semantic_query_contract IRI and Query block details"
fi

set +e
ONTOLOGIES_OUTPUT=$(run_reqvire ontologies 2>&1)
ONTOLOGIES_STATUS=$?
FULL_OUTPUT=$(run_reqvire ontologies --full 2>&1)
FULL_STATUS=$?
set -e

if [ $ONTOLOGIES_STATUS -ne 0 ]; then
  echo "$ONTOLOGIES_OUTPUT"
  fail "ontologies should succeed"
fi

if [ $FULL_STATUS -ne 0 ]; then
  echo "$FULL_OUTPUT"
  fail "ontologies --full should succeed"
fi

if echo "$ONTOLOGIES_OUTPUT" "$FULL_OUTPUT" | grep -Eq "SemanticQueryContract|semantic-query-contract|endpoint-verification-query|SEMANTIC_QUERY_CONTRACT_MARKER"; then
  echo "$ONTOLOGIES_OUTPUT"
  echo "$FULL_OUTPUT"
  fail "ontology semantic output should not emit query contracts or Query content"
fi

expect_invalid "capability-owned query contract" "requirement-owned only" << 'EOF'
# Elements

### API Capability

Capability with invalid query contract ownership.

#### Metadata
  * type: capability

#### Relations
  * refinedBy: [Capability Query Contract](#capability-query-contract)
---

### Capability Query Contract

Invalid capability-owned semantic query contract.

#### Metadata
  * type: semantic-query-contract

#### Relations
  * refine: [API Capability](#api-capability)

#### Query
```sparql
SELECT ?s WHERE { ?s ?p ?o . }
```
---
EOF

expect_invalid "query with ontology" "must not contain a #### Ontology section" << 'EOF'
# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Requirement

API requirement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Capability](#api-capability)
  * refinedBy: [Query Contract With Ontology](#query-contract-with-ontology)
---

### Query Contract With Ontology

Invalid semantic query contract with ontology content.

#### Metadata
  * type: semantic-query-contract

#### Relations
  * refine: [API Requirement](#api-requirement)

#### Query
```sparql
SELECT ?s WHERE { ?s ?p ?o . }
```

#### Ontology
```turtle
@prefix api: <urn:reqvire:test:api:> .
api:Term api:predicate api:Object .
```
---
EOF

expect_invalid "query with shapes" "must not contain a #### Shapes section" << 'EOF'
# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Requirement

API requirement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Capability](#api-capability)
  * refinedBy: [Query Contract With Shapes](#query-contract-with-shapes)
---

### Query Contract With Shapes

Invalid semantic query contract with shapes content.

#### Metadata
  * type: semantic-query-contract

#### Relations
  * refine: [API Requirement](#api-requirement)

#### Query
```sparql
SELECT ?s WHERE { ?s ?p ?o . }
```

#### Shapes
```turtle
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix api: <urn:reqvire:test:api:> .
api:Shape a sh:NodeShape .
```
---
EOF

expect_invalid "missing query" "must contain exactly one #### Query fenced SPARQL block" << 'EOF'
# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Requirement

API requirement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Capability](#api-capability)
  * refinedBy: [Missing Query Contract](#missing-query-contract)
---

### Missing Query Contract

Invalid semantic query contract without Query.

#### Metadata
  * type: semantic-query-contract

#### Relations
  * refine: [API Requirement](#api-requirement)
---
EOF

expect_invalid "duplicate query" "Duplicate subsection 'Query'|at most one #### Query" << 'EOF'
# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Requirement

API requirement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Capability](#api-capability)
  * refinedBy: [Duplicate Query Contract](#duplicate-query-contract)
---

### Duplicate Query Contract

Invalid semantic query contract with duplicate Query blocks.

#### Metadata
  * type: semantic-query-contract

#### Relations
  * refine: [API Requirement](#api-requirement)

#### Query
```sparql
SELECT ?s WHERE { ?s ?p ?o . }
```

#### Query
```sparql
SELECT ?o WHERE { ?s ?p ?o . }
```
---
EOF

expect_invalid "unsupported fence language" "language tag 'sparql'" << 'EOF'
# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Requirement

API requirement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Capability](#api-capability)
  * refinedBy: [SQL Query Contract](#sql-query-contract)
---

### SQL Query Contract

Invalid semantic query contract using an unsupported fence language.

#### Metadata
  * type: semantic-query-contract

#### Relations
  * refine: [API Requirement](#api-requirement)

#### Query
```sql
SELECT * FROM requirements;
```
---
EOF

expect_invalid "non-query element with query" "must not contain a #### Query section" << 'EOF'
# Elements

### API Capability

API capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [API Requirement](#api-requirement)
---

### API Requirement

Requirement incorrectly using reserved Query subsection.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Capability](#api-capability)

#### Query
```sparql
SELECT ?s WHERE { ?s ?p ?o . }
```
---
EOF

exit 0
