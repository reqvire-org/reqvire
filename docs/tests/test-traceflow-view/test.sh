#!/bin/bash
# Test: TraceFlow View Generation
# Verifies that reqvire export generates traceflow.html with Sankey diagram

set -e

# Get the directory where the test script is located
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REQVIRE_BIN="${TEST_DIR}/../../target/debug/reqvire"

# Create temporary directory for test
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Initialize git repo (required for reqvire)
cd "$TEMP_DIR"
git init -q
git config user.email "test@test.com"
git config user.name "Test"

# Create test specifications
mkdir -p "$TEMP_DIR/specifications"

cat > "$TEMP_DIR/specifications/Capabilities.md" << 'EOF'
# Capabilities

### System Access

System access is the capability for authenticated use of protected system capabilities.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [User Login](#user-login)
---

### User Login

The system shall allow users to log into protected system capabilities.

#### Metadata
  * type: requirement

#### Relations
  * specify: [System Access](#system-access)
---
EOF

cat > "$TEMP_DIR/specifications/SystemRequirements.md" << 'EOF'
# System Requirements

### Authentication Module

The system shall implement authentication.

#### Relations
  * derivedFrom: [User Login](Capabilities.md#user-login)
---

### Session Management

The system shall manage user sessions.

#### Relations
  * derivedFrom: [User Login](Capabilities.md#user-login)
---
EOF

cat > "$TEMP_DIR/specifications/Verifications.md" << 'EOF'
# Verifications

### Auth Test

Test for authentication module.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Authentication Module](SystemRequirements.md#authentication-module)
---

### Session Test

Test for session management.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Session Management](SystemRequirements.md#session-management)
---
EOF

# Create output directory
OUTPUT_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR $OUTPUT_DIR" EXIT

echo "Test 1: TraceFlow file generation"
cd "$TEMP_DIR"
"$REQVIRE_BIN" export --output "$OUTPUT_DIR" > /dev/null 2>&1

if [ ! -f "$OUTPUT_DIR/traceflow.html" ]; then
    echo "FAIL: traceflow.html was not generated"
    exit 1
fi
echo "PASS: traceflow.html was generated"

echo "Test 2: TraceFlow HTML is valid"
if ! grep -q "<!DOCTYPE html>" "$OUTPUT_DIR/traceflow.html"; then
    echo "FAIL: traceflow.html is not valid HTML"
    exit 1
fi
echo "PASS: traceflow.html is valid HTML"

echo "Test 3: TraceFlow contains Sankey diagram"
if ! grep -q "d3-sankey" "$OUTPUT_DIR/traceflow.html"; then
    echo "FAIL: traceflow.html does not contain Sankey diagram"
    exit 1
fi
echo "PASS: traceflow.html contains Sankey diagram"

echo "Test 4: TraceFlow page has title"
if ! grep -q "TraceFlow" "$OUTPUT_DIR/traceflow.html"; then
    echo "FAIL: traceflow.html does not have TraceFlow title"
    exit 1
fi
echo "PASS: traceflow.html has TraceFlow title"

echo "Test 5: Navigation omits TraceFlow link from primary Explorer header"
if grep -q 'traceflow.html.*TraceFlow' "$OUTPUT_DIR/index.html"; then
    echo "FAIL: Navigation should not contain TraceFlow link"
    exit 1
fi
echo "PASS: Navigation omits TraceFlow link"

echo "Test 6: Primary Explorer navigation still contains Traces before Ontologies"
TRACES_POS=$(grep -b -o 'traces.html.*Traces<' "$OUTPUT_DIR/index.html" | head -1 | cut -d: -f1)
ONTOLOGIES_POS=$(grep -b -o 'ontologies.html.*Ontologies<' "$OUTPUT_DIR/index.html" | head -1 | cut -d: -f1)
if [ -z "$TRACES_POS" ]; then
    echo "FAIL: Traces link not found"
    exit 1
fi
if [ -z "$ONTOLOGIES_POS" ]; then
    echo "FAIL: Ontologies link not found"
    exit 1
fi
if [ "$ONTOLOGIES_POS" -le "$TRACES_POS" ]; then
    echo "FAIL: Ontologies link is not positioned after Traces (traces pos: $TRACES_POS, ontologies pos: $ONTOLOGIES_POS)"
    exit 1
fi
echo "PASS: Primary Explorer navigation order is valid"

echo "Test 7: All HTML files omit TraceFlow from primary Explorer navigation"
for html_file in "$OUTPUT_DIR"/*.html; do
    if grep -q 'traceflow.html.*TraceFlow' "$html_file"; then
        echo "FAIL: $html_file exposes TraceFlow in navigation"
        exit 1
    fi
done
echo "PASS: All HTML files omit TraceFlow navigation"

echo ""
echo "All TraceFlow view tests passed!"
