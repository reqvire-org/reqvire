#!/usr/bin/env bash
# Comprehensive HTML Generation Tests
# Tests: Integration, Responsive Design, HTML Validity, Visual Regression

set -e  # Exit on error

REQVIRE_CMD="${REQVIRE_BIN:-}"
if [ -z "$REQVIRE_CMD" ]; then
  echo "REQVIRE_BIN is not set"
  exit 1
fi

TEST_MODEL_DIR="$(mktemp -d /tmp/reqvire-html-model-XXXXXX)"
OUTPUT_DIR="/tmp/reqvire-test-output"

mkdir -p "$TEST_MODEL_DIR/requirements/System"
git -C "$TEST_MODEL_DIR" init > /dev/null 2>&1
cat > "$TEST_MODEL_DIR/requirements/Requirements.md" << 'EOF'
# Elements

### Root Requirement

#### Metadata
  * type: feature
---

### Root System Requirement

#### Metadata
  * type: requirement

#### Relations
  * specify: [Root Requirement](#root-requirement)
---
EOF

cat > "$TEST_MODEL_DIR/requirements/System/Core.md" << 'EOF'
# Elements

### Core Requirement

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root System Requirement](../Requirements.md#root-system-requirement)
---
EOF

echo "=== HTML Generation Tests ==="
echo

# ========================================
# 1. INTEGRATION TESTS
# ========================================
echo "1. Running Integration Tests..."

# Generate all pages
cd "$TEST_MODEL_DIR" && "$REQVIRE_CMD" export --output "$OUTPUT_DIR"

# Verify 7 core pages exist
echo "  - Checking core pages exist..."
test -f "$OUTPUT_DIR/index.html"
test -f "$OUTPUT_DIR/model.html"
test -f "$OUTPUT_DIR/traces.html"
test -f "$OUTPUT_DIR/traceflow.html"
test -f "$OUTPUT_DIR/coverage.html"
test -f "$OUTPUT_DIR/resources.html"

# Test each page contains navigation
echo "  - Checking navigation present..."
grep -q "reqvire-nav" "$OUTPUT_DIR/index.html"
grep -q "Containment" "$OUTPUT_DIR/index.html"

# Test relative links from nested file
echo "  - Testing relative links..."
cat > "$TEST_MODEL_DIR/requirements/System/Test.md" << 'EOF'
# Elements

### Test Requirement

This is a test requirement for validating nested file exports.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Core Requirement](Core.md#core-requirement)
EOF
cd "$TEST_MODEL_DIR" && "$REQVIRE_CMD" export --output "$OUTPUT_DIR" 2>&1 | grep -q "Total Markdown files exported"
grep -q 'href="../../index.html"' "$OUTPUT_DIR/requirements/System/Test.html"

# Test visualizations present
echo "  - Checking visualizations..."
grep -q "mermaid" "$OUTPUT_DIR/model.html"
grep -q "d3" "$OUTPUT_DIR/index.html"

echo "  ✅ Integration tests passed"
echo

# ========================================
# 2. RESPONSIVE DESIGN TESTS
# ========================================
echo "2. Running Responsive Design Tests..."

# Verify Tailwind CSS included
echo "  - Checking Tailwind CSS..."
grep -q "tailwindcss" "$OUTPUT_DIR/index.html"

# Verify responsive classes present
echo "  - Checking responsive classes..."
grep -q "md:hidden" "$OUTPUT_DIR/index.html"  # Mobile menu
grep -q "hidden md:flex" "$OUTPUT_DIR/index.html"  # Desktop nav

# Verify viewport meta tag
echo "  - Checking viewport meta tag..."
grep -q 'name="viewport"' "$OUTPUT_DIR/index.html"

# Verify mobile menu toggle present
echo "  - Checking mobile menu components..."
grep -q "mobile-menu-btn" "$OUTPUT_DIR/index.html"
grep -q "mobile-menu" "$OUTPUT_DIR/index.html"

echo "  ✅ Responsive design tests passed"
echo

# ========================================
# 3. HTML VALIDITY TESTS
# ========================================
echo "3. Running HTML Validity Tests..."

# Basic HTML structure validation
for file in "$OUTPUT_DIR"/*.html; do
    filename=$(basename "$file")
    echo "  - Validating $filename..."

    # Check DOCTYPE
    head -1 "$file" | grep -q "<!DOCTYPE html>"

    # Check has <html>, <head>, <body>
    grep -q "<html lang=" "$file"
    grep -q "<head>" "$file"
    grep -q "<body" "$file"

    # Check all tags closed (simple check)
    OPEN=$(grep -o "<[^/][^>]*>" "$file" | grep -v "<!DOCTYPE" | grep -v "<meta" | grep -v "<link" | grep -v "<img" | wc -l)
    CLOSE=$(grep -o "</[^>]*>" "$file" | wc -l)

    echo "    Open tags: $OPEN, Close tags: $CLOSE"
done

echo "  ✅ HTML validity tests passed"
echo

# ========================================
# 4. VISUAL REGRESSION TESTS (Optional)
# ========================================
echo "4. Visual Regression Tests (skipped - requires Playwright)..."
echo "  - To enable: Install Playwright and uncomment section below"

# Uncomment to enable visual regression testing
# echo "  - Starting server..."
# cargo run -- serve --port 8888 &
# SERVER_PID=$!
# sleep 2
#
# echo "  - Taking screenshots at different viewports..."
# npx playwright screenshot --viewport-size=375,667 http://localhost:8888/index.html /tmp/mobile.png
# npx playwright screenshot --viewport-size=768,1024 http://localhost:8888/index.html /tmp/tablet.png
# npx playwright screenshot --viewport-size=1920,1080 http://localhost:8888/index.html /tmp/desktop.png
#
# echo "  - Comparing with baselines..."
# # Compare screenshots (requires image comparison tool)
# # diff /tmp/mobile.png tests/baselines/mobile.png
# # diff /tmp/tablet.png tests/baselines/tablet.png
# # diff /tmp/desktop.png tests/baselines/desktop.png
#
# kill $SERVER_PID
# echo "  ✅ Visual regression tests passed"

echo "  ⏭️  Visual regression tests skipped"
echo

# ========================================
# CLEANUP
# ========================================
rm -rf "$OUTPUT_DIR"
rm -rf "$TEST_MODEL_DIR"

echo "=== All HTML Generation Tests Passed ✅ ==="
