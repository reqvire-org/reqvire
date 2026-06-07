#!/bin/bash
# Test: Traces SPA Projection Export
# Verifies that reqvire export seeds trace data into the single SPA export.

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

echo "Test 1: Single SPA export generation"
cd "$TEMP_DIR"
"$REQVIRE_BIN" export --output "$OUTPUT_DIR" > /dev/null 2>&1

if [ ! -f "$OUTPUT_DIR/index.html" ]; then
    echo "FAIL: index.html was not generated"
    exit 1
fi
echo "PASS: index.html was generated"

echo "Test 2: Removed TraceFlow standalone page is absent"
TRACEFLOW_ENTRY="$OUTPUT_DIR/traceflow"'.html'
if [ -f "$TRACEFLOW_ENTRY" ]; then
    echo "FAIL: standalone TraceFlow page must not be generated"
    exit 1
fi
echo "PASS: standalone TraceFlow page is absent"

echo "Test 3: SPA bundle is valid"
if ! grep -qi "<!doctype html>" "$OUTPUT_DIR/index.html" || ! grep -q "assets/explorer.js" "$OUTPUT_DIR/index.html"; then
    echo "FAIL: index.html is not the SPA bundle"
    exit 1
fi
echo "PASS: index.html is the SPA bundle"

echo "Test 4: Project Store contains Traces route and trace projection"
INDEX_FILE="$OUTPUT_DIR/index.html" node - <<'NODE'
const fs = require('fs');
const html = fs.readFileSync(process.env.INDEX_FILE, 'utf8');
const match = html.match(/(?:const|let|var)\s+reqvireProjectStore\s*=\s*(\{[\s\S]*?\});\s*<\/script>/);
if (!match) {
  console.error('FAIL: index.html missing Project Store seed');
  process.exit(1);
}
const store = JSON.parse(match[1]);
const route = (store.routes?.canonical || []).find((candidate) => candidate.id === 'traces');
if (!route || route.pattern !== '#/traces') {
  console.error('FAIL: Project Store missing canonical #/traces route');
  process.exit(1);
}
if (!store.traces || typeof store.traces !== "object") {
  console.error('FAIL: Project Store missing trace projection data');
  process.exit(1);
}
if ("legacy" in (store.routes || {})) {
  console.error('FAIL: Project Store must not advertise page compatibility routes');
  process.exit(1);
}
NODE
echo "PASS: Project Store contains Traces SPA data"

echo ""
echo "All Traces SPA projection tests passed!"
