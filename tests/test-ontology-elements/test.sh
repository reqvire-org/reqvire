#!/usr/bin/env bash
set -uo pipefail

assert_invalid_model() {
  local expected="$1"
  local output
  set +e
  output=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  local status=$?
  set -e
  if [ $status -eq 0 ]; then
    echo "FAILED: invalid ontology model should fail validation"
    exit 1
  fi
  if ! echo "$output" | grep -Fq "$expected"; then
    echo "FAILED: expected error containing '${expected}'"
    echo "$output"
    exit 1
  fi
}

write_model() {
  rm -rf "${TEST_DIR}/specifications"
  mkdir -p "${TEST_DIR}/specifications"
  cat > "${TEST_DIR}/specifications/Ontology.md"
}

(
  cd "$TEST_DIR" &&
    git init >/dev/null 2>&1 &&
    git config user.email test@example.com &&
    git config user.name "Test User"
)

write_model << 'EOF'
# Elements

### API Feature

API feature.

#### Metadata
  * type: feature

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

The system shall publish service endpoint contracts.

#### Metadata
  * type: requirement

#### Relations
  * specify: [API Feature](#api-feature)
---
EOF

if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/ontology-elements-valid.out 2>&1); then
  echo "FAILED: valid ontology element should validate"
  cat /tmp/ontology-elements-valid.out
  exit 1
fi

write_model << 'EOF'
# Elements

### Missing Ontology Section

Missing ontology section.

#### Metadata
  * type: ontology
---
EOF
assert_invalid_model "must contain exactly one #### Ontology fenced Turtle block"

write_model << 'EOF'
# Elements

### Duplicate Ontology Section

Duplicate ontology section.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix api: <urn:reqvire:test:api:> .
api:A api:b api:C .
```

#### Ontology
```turtle
@prefix api: <urn:reqvire:test:api:> .
api:D api:e api:F .
```
---
EOF
assert_invalid_model "Duplicate subsection 'Ontology'"

write_model << 'EOF'
# Elements

### Ontology With Shapes

Ontology with shapes.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix api: <urn:reqvire:test:api:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
api:ServiceEndpoint a owl:Class .
```

#### Shapes
```turtle
@prefix api: <urn:reqvire:test:api:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
api:ServiceEndpointShape a sh:NodeShape ; sh:targetClass api:ServiceEndpoint .
```
---
EOF
assert_invalid_model "must not contain a #### Shapes section"

write_model << 'EOF'
# Elements

### Ontology With Attachments

Ontology with attachments.

#### Metadata
  * type: ontology

#### Attachments
  * [Other Ontology](#other-ontology)

#### Ontology
```turtle
@prefix api: <urn:reqvire:test:api:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
api:ServiceEndpoint a owl:Class .
```
---

### Other Ontology

Other ontology.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Ontology With Attachments](#ontology-with-attachments)

#### Ontology
```turtle
@prefix other: <urn:reqvire:test:other:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
other:Thing a owl:Class .
```
---
EOF
assert_invalid_model "cannot have attachments"

exit 0
