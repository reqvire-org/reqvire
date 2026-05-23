#!/bin/bash

set -uo pipefail

write_valid_model() {
  rm -rf "${TEST_DIR}/specifications"
  mkdir -p "${TEST_DIR}/specifications"
  cat > "${TEST_DIR}/specifications/SemanticContracts.md" << 'EOF'
# Elements

### Billing Feature

Billing feature.

#### Metadata
  * type: feature

#### Attachments
  * [Billing Ontology](#billing-ontology)
  * [Shared Tax Ontology](#shared-tax-ontology)

#### Relations
  * refinedBy: [Billing Source](#billing-source)
  * specifiedBy: [Billing Requirement](#billing-requirement)
---

### Billing Source

External billing vocabulary source.

#### Metadata
  * type: source

#### Relations
  * refine: [Billing Feature](#billing-feature)
---

### Billing Ontology

Billing vocabulary.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

billing:CorrectiveInvoice a owl:Class .
billing:Invoice a owl:Class .
```
---

### Billing Requirement

The system shall produce billing invoices.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Feature](#billing-feature)
  * refinedBy: [Billing Invoice Shape Contract](#billing-invoice-shape-contract)
---

### Billing Invoice Shape Contract

Billing invoice shape contract.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Billing Requirement](#billing-requirement)

#### Shapes
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

billing:InvoiceShape
  a sh:NodeShape ;
  sh:targetClass billing:Invoice .
```
---

### Shared Tax Ontology

Shared tax vocabulary.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Billing Ontology](#billing-ontology)

#### Ontology
```turtle
@prefix tax: <urn:reqvire:test:tax:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

tax:TaxInvoice a owl:Class .
```
---
EOF
}

write_valid_model
if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/feature-refinements-valid.out 2>&1); then
  echo "FAILED: valid ontology attachment and requirement-owned semantic contract should validate"
  cat /tmp/feature-refinements-valid.out
  exit 1
fi

VALID_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json)
ONTOLOGY_JSON=$(echo "$VALID_JSON" | jq '.files | to_entries[] | .value.elements[] | select(.name == "Billing Ontology")')
if [ -z "$ONTOLOGY_JSON" ]; then
  echo "FAILED: ontology should be present in search JSON"
  exit 1
fi

if [ "$(echo "$ONTOLOGY_JSON" | jq -r '.ontology.iri')" != "urn:reqvire:ontology:billing-ontology" ]; then
  echo "FAILED: ontology JSON should expose derived IRI"
  echo "$ONTOLOGY_JSON"
  exit 1
fi

if [ "$(echo "$ONTOLOGY_JSON" | jq -r '.ontology.ontology.language')" != "turtle" ]; then
  echo "FAILED: ontology JSON should expose Ontology language"
  echo "$ONTOLOGY_JSON"
  exit 1
fi

if [ "$(echo "$ONTOLOGY_JSON" | jq '.ontology.ontology.line_number')" -le 0 ]; then
  echo "FAILED: ontology JSON should expose Ontology fenced block line number"
  echo "$ONTOLOGY_JSON"
  exit 1
fi

SHAPE_CONTRACT_JSON=$(echo "$VALID_JSON" | jq '.files | to_entries[] | .value.elements[] | select(.name == "Billing Invoice Shape Contract")')
if [ "$(echo "$SHAPE_CONTRACT_JSON" | jq -r '.semantic_contract.iri')" != "urn:reqvire:semantic-contract:billing-invoice-shape-contract" ]; then
  echo "FAILED: semantic contract JSON should expose derived IRI"
  echo "$SHAPE_CONTRACT_JSON"
  exit 1
fi

if [ "$(echo "$SHAPE_CONTRACT_JSON" | jq -r '.semantic_contract.shapes.language')" != "turtle" ]; then
  echo "FAILED: semantic contract JSON should expose Shapes language"
  echo "$SHAPE_CONTRACT_JSON"
  exit 1
fi

if [ "$(echo "$SHAPE_CONTRACT_JSON" | jq '.semantic_contract.shapes.line_number')" -le 0 ]; then
  echo "FAILED: semantic contract JSON should expose Shapes fenced block line number"
  echo "$SHAPE_CONTRACT_JSON"
  exit 1
fi

assert_invalid_model() {
  local expected="$1"
  local output
  output=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  local status=$?
  if [ $status -eq 0 ]; then
    echo "FAILED: invalid model should fail validation"
    exit 1
  fi
  if ! echo "$output" | grep -qi "$expected"; then
    echo "FAILED: expected error containing '${expected}'"
    echo "$output"
    exit 1
  fi
}

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/RequirementOwnedOntologySection.md" << 'EOF'
# Elements

### Billing Feature

Feature.

#### Metadata
  * type: feature

#### Attachments
  * [Billing Ontology](#billing-ontology)

#### Relations
  * specifiedBy: [Billing Requirement](#billing-requirement)
---

### Billing Ontology

Billing vocabulary.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
billing:Invoice a owl:Class .
```
---

### Billing Requirement

The system shall bill customers.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Feature](#billing-feature)
  * refinedBy: [Bad Semantic Contract](#bad-semantic-contract)
---

### Bad Semantic Contract

Bad ownership.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Billing Requirement](#billing-requirement)

#### Ontology
```turtle
@prefix ex: <urn:reqvire:test:bad-requirement-owned:> .
ex:A ex:b ex:C .
```

#### Shapes
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
billing:InvoiceShape a sh:NodeShape ; sh:targetClass billing:Invoice .
```
---
EOF
assert_invalid_model "must not contain"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/MissingShapes.md" << 'EOF'
# Elements

### Billing Feature

Feature.

#### Metadata
  * type: feature

#### Relations
  * specifiedBy: [Billing Requirement](#billing-requirement)
---

### Billing Requirement

The system shall bill customers.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Feature](#billing-feature)
  * refinedBy: [Missing Shapes Contract](#missing-shapes-contract)
---

### Missing Shapes Contract

Missing shapes.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Billing Requirement](#billing-requirement)
---
EOF
assert_invalid_model "Shapes"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/MissingOntology.md" << 'EOF'
# Elements

### Billing Ontology

Missing ontology.

#### Metadata
  * type: ontology
---
EOF
assert_invalid_model "Ontology"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/FeatureOwnedSemanticContract.md" << 'EOF'
# Elements

### Billing Feature

Feature.

#### Metadata
  * type: feature

#### Relations
  * refinedBy: [Feature Semantic Contract](#feature-semantic-contract)
---

### Feature Semantic Contract

Feature-owned semantic contract is invalid in the ontology split model.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Billing Feature](#billing-feature)

#### Shapes
```turtle
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix billing: <urn:reqvire:test:billing:> .
billing:InvoiceShape a sh:NodeShape ; sh:targetClass billing:Invoice .
```
---
EOF
assert_invalid_model "semantic-contract"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/BadTurtle.md" << 'EOF'
# Elements

### Bad Turtle Ontology

Bad turtle.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix ex: <urn:reqvire:test:bad-turtle:> .
ex:A ex:b .
```
---
EOF
assert_invalid_model "Turtle"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/BadLanguage.md" << 'EOF'
# Elements

### Bad Language Ontology

Bad language.

#### Metadata
  * type: ontology

#### Ontology
```json
@prefix ex: <urn:reqvire:test:bad-language:> .
ex:A ex:b ex:C .
```
---
EOF
assert_invalid_model "language tag"

exit 0
