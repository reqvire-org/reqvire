#!/bin/bash

set -uo pipefail

mkdir -p "${TEST_DIR}/specifications"

cat > "${TEST_DIR}/specifications/ValidFeatureModel.md" << 'EOF'
# Elements

### Billing Feature

Billing product capability.

#### Metadata
  * type: feature

#### Relations
  * specifiedBy: [Corrective Invoice Requirement](#corrective-invoice-requirement)
---

### Billing Child Feature

Derived billing feature.

#### Metadata
  * type: feature

#### Relations
  * derivedFrom: [Billing Feature](#billing-feature)
---

### Corrective Invoice Requirement

The system shall support corrective invoices.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Feature](#billing-feature)
---

### Corrective Invoice Payload Requirement

The system shall produce corrective invoice payloads.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Corrective Invoice Requirement](#corrective-invoice-requirement)
---
EOF

if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/feature-valid.out 2>&1); then
  echo "FAILED: valid feature model should validate"
  cat /tmp/feature-valid.out
  exit 1
fi

assert_invalid() {
  local fixture="$1"
  local expected="$2"
  rm -rf "${TEST_DIR}/specifications"
  mkdir -p "${TEST_DIR}/specifications/src"
  touch "${TEST_DIR}/specifications/src/billing.rs"
  cp "${TEST_DIR}/${fixture}" "${TEST_DIR}/specifications/Requirements.md"
  local output
  output=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  local status=$?
  if [ $status -eq 0 ]; then
    echo "FAILED: ${fixture} should fail validation"
    exit 1
  fi
  if ! echo "$output" | grep -qi "$expected"; then
    echo "FAILED: ${fixture} error should mention '${expected}'"
    echo "$output"
    exit 1
  fi
}

cat > "${TEST_DIR}/invalid-legacy-type.fixture" << 'EOF'
# Elements

### Old Stakeholder Need

Old type.

#### Metadata
  * type: stakeholder-requirement
---
EOF

cat > "${TEST_DIR}/invalid-derive-bridge.fixture" << 'EOF'
# Elements

### Billing Feature

Feature.

#### Metadata
  * type: feature

#### Relations
  * derive: [Billing Requirement](#billing-requirement)
---

### Billing Requirement

The system shall bill customers.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Billing Feature](#billing-feature)
---
EOF

cat > "${TEST_DIR}/invalid-feature-satisfaction.fixture" << 'EOF'
# Elements

### Billing Feature

Feature.

#### Metadata
  * type: feature

#### Relations
  * satisfiedBy: [Implementation](src/billing.rs)
---
EOF

cat > "${TEST_DIR}/invalid-feature-verification.fixture" << 'EOF'
# Elements

### Billing Feature

Feature.

#### Metadata
  * type: feature

#### Relations
  * verifiedBy: [Billing Verification](#billing-verification)
---

### Billing Verification

Verification.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Billing Feature](#billing-feature)
---
EOF

assert_invalid "invalid-legacy-type.fixture" "Invalid element type"
assert_invalid "invalid-derive-bridge.fixture" "incompatible"
assert_invalid "invalid-feature-satisfaction.fixture" "incompatible"
assert_invalid "invalid-feature-verification.fixture" "incompatible"

exit 0
