#!/bin/bash

set -uo pipefail

write_valid_model() {
  rm -rf "${TEST_DIR}/specifications"
  mkdir -p "${TEST_DIR}/specifications"
  cat > "${TEST_DIR}/specifications/SemanticContracts.md" << 'EOF'
# Elements

### Billing Capability

Billing capability.

#### Metadata
  * type: capability

#### Concept References
  * Invoice: urn:reqvire:test:billing:Invoice
  * Tax Invoice: urn:reqvire:test:tax:TaxInvoice

#### Relations
  * specifiedBy: [Billing Requirement](#billing-requirement)
---

### Billing Source

External billing vocabulary source.

#### Metadata
  * type: source

#### Relations
  * define: [Billing Requirement](#billing-requirement)
---

### Billing Ontology

Billing vocabulary.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix billing: <urn:reqvire:test:billing:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
billing:CorrectiveInvoice a owl:Class .
billing:Invoice a owl:Class .
```
---

### Billing Requirement

The system shall produce billing invoices.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Capability](#billing-capability)
  * definedBy: [Billing Source](#billing-source)
  * constrainedBy: [Billing Invoice Shape Contract](#billing-invoice-shape-contract)
---

### Billing Invoice Shape Contract

Billing invoice shape contract.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Billing Requirement](#billing-requirement)
  * use: [Billing Ontology](#billing-ontology)

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
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Relations
  * derivedFrom: [Billing Ontology](#billing-ontology)

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix tax: <urn:reqvire:test:tax:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
tax:TaxInvoice a owl:Class .
```
---
EOF
}

write_valid_model
if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/capability-contracts-valid.out 2>&1); then
  echo "FAILED: valid concept references and requirement-owned semantic contract should validate"
  cat /tmp/capability-contracts-valid.out
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

assert_invalid_requirement_owned_only() {
  assert_invalid_model "non-semantic contract element to a requirement"
}

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/RequirementOwnedOntologySection.md" << 'EOF'
# Elements

### Billing Capability

Capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Billing Requirement](#billing-requirement)
---

### Billing Ontology

Billing vocabulary.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix billing: <urn:reqvire:test:billing:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
billing:Invoice a owl:Class .
```
---

### Billing Requirement

The system shall bill customers.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Capability](#billing-capability)
  * constrainedBy: [Bad Semantic Contract](#bad-semantic-contract)
---

### Bad Semantic Contract

Bad ownership.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Billing Requirement](#billing-requirement)
  * use: [Billing Ontology](#billing-ontology)

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

### Billing Capability

Capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Billing Requirement](#billing-requirement)
---

### Billing Requirement

The system shall bill customers.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Capability](#billing-capability)
  * constrainedBy: [Missing Shapes Contract](#missing-shapes-contract)
---

### Missing Shapes Contract

Missing shapes.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Billing Requirement](#billing-requirement)
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
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto
---
EOF
assert_invalid_model "Ontology"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/CapabilityOwnedSemanticContract.md" << 'EOF'
# Elements

### Billing Capability

Capability.

#### Metadata
  * type: capability

#### Relations
  * constrainedBy: [Capability Semantic Contract](#capability-semantic-contract)
---

### Capability Semantic Contract

Capability-owned semantic contract is invalid in the ontology split model.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Billing Capability](#billing-capability)

#### Shapes
```turtle
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix billing: <urn:reqvire:test:billing:> .
billing:InvoiceShape a sh:NodeShape ; sh:targetClass billing:Invoice .
```
---
EOF
assert_invalid_model "semantic-contract element to a requirement"

assert_capability_owned_contract_invalid() {
  local contract_type="$1"
  local element_name="$2"
  local fragment="${element_name,,}"
  fragment="${fragment// /-}"

  rm -rf "${TEST_DIR}/specifications"
  mkdir -p "${TEST_DIR}/specifications"
  cat > "${TEST_DIR}/specifications/CapabilityOwned${element_name// /}.md" << EOF
# Elements

### Billing Capability

Capability with invalid contract ownership.

#### Metadata
  * type: capability

#### Relations
  * definedBy: [${element_name}](#${fragment})
---

### ${element_name}

Capability-owned ${contract_type} is invalid.

#### Metadata
  * type: ${contract_type}

#### Relations
  * define: [Billing Capability](#billing-capability)
---
EOF
  assert_invalid_requirement_owned_only
}

assert_capability_owned_contract_invalid "source" "Capability Source"
assert_capability_owned_contract_invalid "constraint" "Capability Constraint"
assert_capability_owned_contract_invalid "behavior" "Capability Behavior"
assert_capability_owned_contract_invalid "specification" "Capability Specification"
assert_capability_owned_contract_invalid "state" "Capability State"
assert_capability_owned_contract_invalid "input-output" "Capability Input Output"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cat > "${TEST_DIR}/specifications/BadTurtle.md" << 'EOF'
# Elements

### Bad Turtle Ontology

Bad turtle.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

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
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```json
@prefix ex: <urn:reqvire:test:bad-language:> .
ex:A ex:b ex:C .
```
---
EOF
assert_invalid_model "language tag"

exit 0
