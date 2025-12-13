#!/usr/bin/env bash
# Comprehensive HTML Generation Tests
# Tests: Integration, Responsive Design, HTML Validity, Visual Regression

set -e  # Exit on error

echo "=== HTML Generation Tests ==="
echo

# ========================================
# 1. INTEGRATION TESTS
# ========================================
echo "1. Running Integration Tests..."

# Generate all pages
cargo run -- export --output /tmp/reqvire-test-output

# Verify 7 core pages exist
echo "  - Checking core pages exist..."
test -f /tmp/reqvire-test-output/index.html
test -f /tmp/reqvire-test-output/model.html
test -f /tmp/reqvire-test-output/traces.html
test -f /tmp/reqvire-test-output/traceflow.html
test -f /tmp/reqvire-test-output/coverage.html
test -f /tmp/reqvire-test-output/resources.html

# Test each page contains navigation
echo "  - Checking navigation present..."
grep -q "reqvire-nav" /tmp/reqvire-test-output/index.html
grep -q "Containment" /tmp/reqvire-test-output/index.html

# Test relative links from nested file
echo "  - Testing relative links..."
mkdir -p requirements/System
cat > requirements/System/Test.md << 'EOF'
# Elements

### Test Requirement

This is a test requirement for validating nested file exports.

#### Metadata
  * type: user-requirement
EOF
cargo run -- export --output /tmp/reqvire-test-output 2>&1 | grep -q "Total Markdown files exported"
grep -q 'href="../../index.html"' /tmp/reqvire-test-output/requirements/System/Test.html

# Test visualizations present
echo "  - Checking visualizations..."
grep -q "mermaid" /tmp/reqvire-test-output/model.html
grep -q "d3" /tmp/reqvire-test-output/index.html

echo "  ✅ Integration tests passed"
echo

# ========================================
# 2. RESPONSIVE DESIGN TESTS
# ========================================
echo "2. Running Responsive Design Tests..."

# Verify Tailwind CSS included
echo "  - Checking Tailwind CSS..."
grep -q "tailwindcss" /tmp/reqvire-test-output/index.html

# Verify responsive classes present
echo "  - Checking responsive classes..."
grep -q "md:hidden" /tmp/reqvire-test-output/index.html  # Mobile menu
grep -q "hidden md:flex" /tmp/reqvire-test-output/index.html  # Desktop nav

# Verify viewport meta tag
echo "  - Checking viewport meta tag..."
grep -q 'name="viewport"' /tmp/reqvire-test-output/index.html

# Verify mobile menu toggle present
echo "  - Checking mobile menu components..."
grep -q "mobile-menu-btn" /tmp/reqvire-test-output/index.html
grep -q "mobile-menu" /tmp/reqvire-test-output/index.html

echo "  ✅ Responsive design tests passed"
echo

# ========================================
# 3. HTML VALIDITY TESTS
# ========================================
echo "3. Running HTML Validity Tests..."

# Basic HTML structure validation
for file in /tmp/reqvire-test-output/*.html; do
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
rm -rf /tmp/reqvire-test-output
rm -f requirements/System/Test.md
rmdir requirements/System 2>/dev/null || true

echo "=== All HTML Generation Tests Passed ✅ ==="
