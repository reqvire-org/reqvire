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
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<https://example.test/ontology> a owl:Ontology .
testonto:Element a owl:Class .
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
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
testonto:ServiceEndpoint a owl:Class .
```
---
EOF

if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/ontology-single-root-valid.out 2>&1); then
  echo "FAILED: connected ontology graph should validate"
  cat /tmp/ontology-single-root-valid.out
  exit 1
fi

VALID_EXPORT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies 2>&1)
if ! grep -q "<https://example.test/ontology> a owl:Ontology" <<< "$VALID_EXPORT"; then
  echo "FAILED: ontology export should declare the root ontology_base as the OWL document"
  echo "$VALID_EXPORT"
  exit 1
fi
if ! grep -q "reqvire:ontologyElement" <<< "$VALID_EXPORT" ||
   ! grep -q "<urn:reqvire:element:core-ontology>" <<< "$VALID_EXPORT" ||
   ! grep -q "<urn:reqvire:element:api-ontology>" <<< "$VALID_EXPORT"; then
  echo "FAILED: ontology export should show same-base child ontology elements contributing to the root document"
  echo "$VALID_EXPORT"
  exit 1
fi
if grep -q "<https://example.test/ontology/api-ontology> a owl:Ontology" <<< "$VALID_EXPORT"; then
  echo "FAILED: same-base child ontology element must not become a separate OWL document"
  echo "$VALID_EXPORT"
  exit 1
fi

write_model << 'EOF'
# Elements

### Missing Boundary Declaration

Missing boundary declaration.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
testonto:Element a owl:Class .
```
---
EOF

assert_invalid_model "must explicitly declare <https://example.test/ontology> a owl:Ontology"

write_model << 'EOF'
# Elements

### Core Ontology

Core ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<https://example.test/ontology> a owl:Ontology .
testonto:Element a owl:Class .
```
---

### Child Contribution

Child contribution.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Core Ontology](#core-ontology)

#### Ontology
```turtle
@prefix owl: <http://www.w3.org/2002/07/owl#> .
testonto:ChildElement a owl:Class .
```
---
EOF

assert_invalid_model "must explicitly declare prefix 'testonto' as 'https://example.test/ontology#'"

write_model << 'EOF'
# Elements

### Platform Ontology

Platform ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/platform
  * ontology_prefix: platform

#### Ontology
```turtle
@prefix platform: <https://example.test/platform#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<https://example.test/platform> a owl:Ontology .
platform:PlatformElement a owl:Class .
```
---

### API Ontology

API ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/api
  * ontology_prefix: api

#### Relations
  * derivedFrom: [Platform Ontology](#platform-ontology)

#### Ontology
```turtle
@prefix api: <https://example.test/api#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<https://example.test/api> a owl:Ontology .
api:ApiElement a owl:Class .
```
---
EOF

assert_invalid_model "must explicitly declare <https://example.test/api> owl:imports <https://example.test/platform>"

write_model << 'EOF'
# Elements

### Platform Ontology

Platform ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/platform
  * ontology_prefix: platform

#### Ontology
```turtle
@prefix platform: <https://example.test/platform#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<https://example.test/platform> a owl:Ontology .
platform:PlatformElement a owl:Class .
```
---

### API Ontology

API ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/api
  * ontology_prefix: api

#### Relations
  * derivedFrom: [Platform Ontology](#platform-ontology)

#### Ontology
```turtle
@prefix api: <https://example.test/api#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<https://example.test/api>
  a owl:Ontology ;
  owl:imports <https://example.test/platform> .
api:ApiElement a owl:Class .
```
---
EOF

if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/ontology-cross-boundary-valid.out 2>&1); then
  echo "FAILED: explicit cross-boundary owl:imports should validate"
  cat /tmp/ontology-cross-boundary-valid.out
  exit 1
fi

write_model << 'EOF'
# Elements

### Core Ontology

Core ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

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
  * ontology_base: https://example.test/other-ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix other: <urn:reqvire:test:other:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
other:Thing a owl:Class .
```
---
EOF

assert_invalid_model "Disconnected ontology graph"

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
@prefix testonto: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
testonto:ServiceEndpoint a owl:Class .
```
---
EOF

assert_invalid_model "Top parent ontology element 'Core Ontology' must define non-empty ontology_base metadata"

write_model << 'EOF'
# Elements

### Core Ontology

Core ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology

#### Ontology
```turtle
@prefix core: <https://example.test/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
core:Element a owl:Class .
```
---
EOF

assert_invalid_model "Top parent ontology element 'Core Ontology' must define non-empty ontology_prefix metadata"

write_model << 'EOF'
# Elements

### Core Ontology

Core ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: core

#### Ontology
```turtle
@prefix core: <urn:reqvire:test:core:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
core:Element a owl:Class .
```
---
EOF

assert_invalid_model "Ontology Turtle prefix 'core' maps to 'urn:reqvire:test:core:', but inherited ontology metadata requires 'https://example.test/ontology#'"

exit 0
