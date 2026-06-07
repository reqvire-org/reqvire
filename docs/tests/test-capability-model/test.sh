#!/bin/bash

set -uo pipefail

mkdir -p "${TEST_DIR}/specifications"

cat > "${TEST_DIR}/specifications/ValidCapabilityModel.md" << 'EOF'
# Elements

### Billing Capability

Billing product capability.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Corrective Invoice Requirement](#corrective-invoice-requirement)
---

### Billing Child Capability

Derived billing capability.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Billing Capability](#billing-capability)
---

### Corrective Invoice Requirement

The system shall support corrective invoices.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Capability](#billing-capability)
---

### Corrective Invoice Payload Requirement

The system shall produce corrective invoice payloads.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Corrective Invoice Requirement](#corrective-invoice-requirement)
---
EOF

if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/capability-valid.out 2>&1); then
  echo "FAILED: valid capability model should validate"
  cat /tmp/capability-valid.out
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

### Billing Capability

Capability.

#### Metadata
  * type: capability

#### Relations
  * derive: [Billing Requirement](#billing-requirement)
---

### Billing Requirement

The system shall bill customers.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Billing Capability](#billing-capability)
---
EOF

cat > "${TEST_DIR}/invalid-capability-satisfaction.fixture" << 'EOF'
# Elements

### Billing Capability

Capability.

#### Metadata
  * type: capability

#### Relations
  * satisfiedBy: [Implementation](src/billing.rs)
---
EOF

cat > "${TEST_DIR}/valid-capability-verification.fixture" << 'EOF'
# Elements

### Billing Capability

Capability.

#### Metadata
  * type: capability

#### Relations
  * verifiedBy: [Billing Verification](#billing-verification)
---

### Billing Verification

Verification.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Billing Capability](#billing-capability)
---
EOF

assert_invalid "invalid-legacy-type.fixture" "Invalid element type"
assert_invalid "invalid-derive-bridge.fixture" "incompatible"
assert_invalid "invalid-capability-satisfaction.fixture" "incompatible"

rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"
cp "${TEST_DIR}/valid-capability-verification.fixture" "${TEST_DIR}/specifications/Requirements.md"
if ! (cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/capability-verification-valid.out 2>&1); then
  echo "FAILED: valid capability verification should validate"
  cat /tmp/capability-verification-valid.out
  exit 1
fi

exit 0
