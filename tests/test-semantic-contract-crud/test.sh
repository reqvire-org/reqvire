#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

assert_diff() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: ${description}"
    exit 1
  fi
}

set +e
VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATE_EXIT=$?
set -e
if [ $VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: initial semantic CRUD fixture should validate"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

cp "$TEST_DIR/specifications/SemanticContracts.md" /tmp/semantic-contract-crud-before-delete.md

BAD_CONTRACT='### Bad Shape Contract

Bad semantic contract with a dangling reference.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Billing Requirement](#billing-requirement)

#### Shapes
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

billing:BadPayloadShape
  a sh:NodeShape ;
  sh:targetClass billing:BillingPayload ;
  sh:property [
    sh:path billing:MissingAddedTerm ;
    sh:minCount 1 ;
  ] .
```'

set +e
ADD_OUTPUT=$(cd "$TEST_DIR" && printf "%s\n" "$BAD_CONTRACT" | "$REQVIRE_BIN" add specifications/SemanticContracts.md 2>&1)
ADD_EXIT=$?
set -e

if [ $ADD_EXIT -eq 0 ]; then
  echo "FAILED: adding Bad Shape Contract should fail because it creates a dangling semantic reference"
  echo "$ADD_OUTPUT"
  exit 1
fi

for marker in \
  "Semantic reference not found" \
  "specifications/SemanticContracts.md#bad-shape-contract" \
  "sh:path" \
  "urn:reqvire:test:billing:MissingAddedTerm" \
  "Update or remove the SHACL reference before deleting or editing the declaring ontology"
do
  if ! echo "$ADD_OUTPUT" | grep -Fq "$marker"; then
    echo "FAILED: add error missing marker: $marker"
    echo "$ADD_OUTPUT"
    exit 1
  fi
done

if grep -Fq "### Bad Shape Contract" "$TEST_DIR/specifications/SemanticContracts.md"; then
  echo "FAILED: failed semantic add should not persist Bad Shape Contract"
  exit 1
fi

OUTSIDE_CONTEXT_CONTRACT='### Outside Context Shape Contract

Bad semantic contract with a reference declared outside the reachable feature-root context.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Billing Requirement](#billing-requirement)

#### Shapes
```turtle
@prefix billing: <urn:reqvire:test:billing:> .
@prefix customer: <urn:reqvire:test:customer:> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

billing:OutsideContextPayloadShape
  a sh:NodeShape ;
  sh:targetClass billing:BillingPayload ;
  sh:property [
    sh:path customer:customerId ;
    sh:minCount 1 ;
  ] .
```'

set +e
OUTSIDE_ADD_OUTPUT=$(cd "$TEST_DIR" && printf "%s\n" "$OUTSIDE_CONTEXT_CONTRACT" | "$REQVIRE_BIN" add specifications/SemanticContracts.md 2>&1)
OUTSIDE_ADD_EXIT=$?
set -e

if [ $OUTSIDE_ADD_EXIT -eq 0 ]; then
  echo "FAILED: adding Outside Context Shape Contract should fail because it bypasses semantic attachment context"
  echo "$OUTSIDE_ADD_OUTPUT"
  exit 1
fi

for marker in \
  "Semantic reference outside context" \
  "specifications/SemanticContracts.md#outside-context-shape-contract" \
  "sh:path" \
  "urn:reqvire:test:customer:customerId" \
  "specifications/SemanticContracts.md#customer-ontology" \
  "owning requirement 'specifications/SemanticContracts.md#billing-requirement'" \
  "Attach the declaring ontology"
do
  if ! echo "$OUTSIDE_ADD_OUTPUT" | grep -Fq "$marker"; then
    echo "FAILED: outside-context add error missing marker: $marker"
    echo "$OUTSIDE_ADD_OUTPUT"
    exit 1
  fi
done

if grep -Fq "### Outside Context Shape Contract" "$TEST_DIR/specifications/SemanticContracts.md"; then
  echo "FAILED: failed outside-context semantic add should not persist Outside Context Shape Contract"
  exit 1
fi

assert_diff \
  /tmp/semantic-contract-crud-before-delete.md \
  "$TEST_DIR/specifications/SemanticContracts.md" \
  "failed outside-context semantic add should not persist file changes"

set +e
UNLINK_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Billing Feature" "Tax Ontology" 2>&1)
UNLINK_EXIT=$?
set -e

if [ $UNLINK_EXIT -eq 0 ]; then
  echo "FAILED: unlinking Tax Ontology attachment should fail because it makes the VAT reference outside-context"
  echo "$UNLINK_OUTPUT"
  exit 1
fi

for marker in \
  "Semantic reference outside context" \
  "specifications/SemanticContracts.md#billing-shape-contract" \
  "sh:path" \
  "urn:reqvire:test:tax:VatRate" \
  "specifications/SemanticContracts.md#tax-ontology" \
  "owning requirement 'specifications/SemanticContracts.md#billing-requirement'" \
  "Attach the declaring ontology"
do
  if ! echo "$UNLINK_OUTPUT" | grep -Fq "$marker"; then
    echo "FAILED: unlink error missing marker: $marker"
    echo "$UNLINK_OUTPUT"
    exit 1
  fi
done

assert_diff \
  /tmp/semantic-contract-crud-before-delete.md \
  "$TEST_DIR/specifications/SemanticContracts.md" \
  "failed semantic unlink should not persist file changes"

set +e
DELETE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "Tax Ontology" 2>&1)
DELETE_EXIT=$?
set -e

if [ $DELETE_EXIT -eq 0 ]; then
  echo "FAILED: deleting Tax Ontology should fail because it leaves a dangling semantic reference"
  echo "$DELETE_OUTPUT"
  exit 1
fi

for marker in \
  "Semantic reference not found" \
  "specifications/SemanticContracts.md#billing-shape-contract" \
  "sh:path" \
  "urn:reqvire:test:tax:VatRate" \
  "Removed declaration source: specifications/SemanticContracts.md#tax-ontology" \
  "Update or remove the SHACL reference before deleting or editing the declaring ontology"
do
  if ! echo "$DELETE_OUTPUT" | grep -Fq "$marker"; then
    echo "FAILED: delete error missing marker: $marker"
    echo "$DELETE_OUTPUT"
    exit 1
  fi
done

printf "%s\n" "Ontology deletion is blocked when it would leave a dangling semantic reference" \
  > /tmp/semantic-contract-crud-delete.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/delete-dangling-reference.txt" \
  /tmp/semantic-contract-crud-delete.out \
  "semantic CRUD delete blocker output mismatch"

assert_diff \
  /tmp/semantic-contract-crud-before-delete.md \
  "$TEST_DIR/specifications/SemanticContracts.md" \
  "failed semantic delete should not persist file changes"

OVERRIDE_TAX_CONTRACT='### Tax Ontology

Tax ontology terms without the VAT rate declaration.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Billing Ontology](#billing-ontology)

#### Ontology
```turtle
@prefix tax: <urn:reqvire:test:tax:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

tax:OtherRate a owl:DatatypeProperty .
```'

set +e
OVERRIDE_OUTPUT=$(cd "$TEST_DIR" && printf "%s\n" "$OVERRIDE_TAX_CONTRACT" | "$REQVIRE_BIN" add specifications/SemanticContracts.md --override 2>&1)
OVERRIDE_EXIT=$?
set -e

if [ $OVERRIDE_EXIT -eq 0 ]; then
  echo "FAILED: overriding Tax Contract should fail because it drops a referenced declaration"
  echo "$OVERRIDE_OUTPUT"
  exit 1
fi

for marker in \
  "Semantic reference not found" \
  "specifications/SemanticContracts.md#billing-shape-contract" \
  "sh:path" \
  "urn:reqvire:test:tax:VatRate" \
  "Removed declaration source: specifications/SemanticContracts.md#tax-ontology" \
  "Update or remove the SHACL reference before deleting or editing the declaring ontology"
do
  if ! echo "$OVERRIDE_OUTPUT" | grep -Fq "$marker"; then
    echo "FAILED: override error missing marker: $marker"
    echo "$OVERRIDE_OUTPUT"
    exit 1
  fi
done

assert_diff \
  /tmp/semantic-contract-crud-before-delete.md \
  "$TEST_DIR/specifications/SemanticContracts.md" \
  "failed semantic override should not persist file changes"

exit 0
