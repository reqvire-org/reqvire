#!/usr/bin/env bash
set -uo pipefail

write_model() {
  rm -rf "${TEST_DIR}/specifications"
  mkdir -p "${TEST_DIR}/specifications"
  cat > "${TEST_DIR}/specifications/Ontologies.md"
}

assert_invalid_model() {
  local expected="$1"
  local output
  set +e
  output=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  local status=$?
  set -e
  if [ $status -eq 0 ]; then
    echo "FAILED: disconnected ontology graph should fail validation"
    exit 1
  fi
  if ! echo "$output" | grep -Fq "$expected"; then
    echo "FAILED: expected error containing '${expected}'"
    echo "$output"
    exit 1
  fi
}

(
  cd "$TEST_DIR" &&
    git init >/dev/null 2>&1 &&
    git config user.email test@example.com &&
    git config user.name "Test User"
)

write_model << 'EOF'
# Elements

### Core Ontology

Core ontology.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix core: <urn:reqvire:test:core:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
core:Element a owl:Class .
```
---

### API Ontology

API ontology.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Core Ontology](#core-ontology)

#### Ontology
```turtle
@prefix api: <urn:reqvire:test:api:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
api:ServiceEndpoint a owl:Class .
```
---
EOF

if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/ontology-single-root-valid.out 2>&1); then
  echo "FAILED: connected ontology graph should validate"
  cat /tmp/ontology-single-root-valid.out
  exit 1
fi

write_model << 'EOF'
# Elements

### Core Ontology

Core ontology.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix core: <urn:reqvire:test:core:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
core:Element a owl:Class .
```
---

### Disconnected Ontology

Disconnected ontology.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix other: <urn:reqvire:test:other:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
other:Thing a owl:Class .
```
---
EOF

assert_invalid_model "Disconnected ontology graph"

exit 0
