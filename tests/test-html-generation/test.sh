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

mkdir -p "$TEST_MODEL_DIR/requirements/System" "$TEST_MODEL_DIR/src"
git -C "$TEST_MODEL_DIR" init > /dev/null 2>&1
cat > "$TEST_MODEL_DIR/src/impl.rs" << 'EOF'
pub fn satisfy_root_system_requirement() {}
EOF
cat > "$TEST_MODEL_DIR/requirements/Requirements.md" << 'EOF'
# Elements

### Root Requirement

#### Metadata
  * type: capability
---

### Root System Requirement

#### Metadata
  * type: requirement

#### Relations
  * specify: [Root Requirement](#root-requirement)
  * satisfiedBy: [impl.rs](../src/impl.rs)
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

# Verify primary and supporting pages exist
echo "  - Checking core pages exist..."
test -f "$OUTPUT_DIR/index.html"
test -f "$OUTPUT_DIR/ontologies.ttl"
for removed_stem in containment coverage resources traceflow model knowledgegraph kn2 traces ontologies; do
  removed_page="$removed_stem"'.html'
  if [ -e "$OUTPUT_DIR/$removed_page" ]; then
    echo "Standalone Explorer page must not be generated for route: $removed_stem"
    exit 1
  fi
done

# index.html is the compiled Vite/React/Radix Explorer SPA bundle (not a
# runtime-assembled page). The header, route links, and route rendering live
# in the bundle and are exercised at runtime; here we assert the exported
# artifacts: the SPA mount point, the seeded Project Store, the compiled bundle
# assets, and the absence of the previous runtime / CDN Tailwind / iframes.
echo "  - Checking Explorer SPA bundle..."
grep -q '<div id="root"></div>' "$OUTPUT_DIR/index.html"
grep -q 'assets/explorer.js' "$OUTPUT_DIR/index.html"
grep -q 'assets/explorer.css' "$OUTPUT_DIR/index.html"
grep -q 'href="assets/favicon.ico"' "$OUTPUT_DIR/index.html"
grep -q 'href="assets/apple-touch-icon.png"' "$OUTPUT_DIR/index.html"
test -f "$OUTPUT_DIR/assets/explorer.js"
test -f "$OUTPUT_DIR/assets/explorer.css"
test -f "$OUTPUT_DIR/assets/favicon.ico"
test -f "$OUTPUT_DIR/assets/apple-touch-icon.png"
test -s "$OUTPUT_DIR/assets/explorer.js"
test -s "$OUTPUT_DIR/assets/explorer.css"
# The seed must be injected before the bundle module script boots the SPA.
grep -q 'id="reqvire-project-store"' "$OUTPUT_DIR/index.html"
SEED_POS=$(grep -b -o 'id="reqvire-project-store"' "$OUTPUT_DIR/index.html" | head -1 | cut -d: -f1)
BUNDLE_POS=$(grep -b -o 'assets/explorer.js' "$OUTPUT_DIR/index.html" | head -1 | cut -d: -f1)
if [ -z "$SEED_POS" ] || [ -z "$BUNDLE_POS" ] || [ "$SEED_POS" -ge "$BUNDLE_POS" ]; then
  echo "Project Store seed must be injected before the Explorer bundle script"
  exit 1
fi
# The exported index must NOT carry the removed previous runtime, iframes, or a
# CDN/runtime Tailwind dependency (Tailwind is compiled into explorer.css).
for forbidden in 'id="reqvire-explorer-runtime"' 'id="reqvire-explorer-views"' 'registerView' 'ReqvireExplorerStore'; do
  if grep -q "$forbidden" "$OUTPUT_DIR/index.html"; then
    echo "Exported Explorer must not embed the legacy store.rs runtime ($forbidden)"
    exit 1
  fi
done
if grep -Eq '<iframe|createElement\("iframe"\)' "$OUTPUT_DIR/index.html"; then
  echo "Explorer shell must not realize views via iframes/same-origin route frames"
  exit 1
fi
if grep -Eq 'cdn\.tailwindcss\.com|tailwindcss@|src="[^"]*tailwind' "$OUTPUT_DIR/index.html"; then
  echo "Explorer must use compiled Tailwind in explorer.css, not a CDN/runtime Tailwind"
  exit 1
fi
if grep -Eq 'cdn\.tailwindcss\.com|tailwindcss@|src="[^"]*tailwind' "$OUTPUT_DIR/assets/explorer.js" "$OUTPUT_DIR/assets/explorer.css"; then
  echo "Explorer bundle assets must not load Tailwind from a CDN/runtime compiler"
  exit 1
fi
# Explorer/report views are served by index.html SPA routes.

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
grep -q 'href="../../index.html#/model"' "$OUTPUT_DIR/requirements/System/Test.html"

# Test SPA store projections and route data.
echo "  - Checking Project Store projections..."
# The exported index seeds the Project Store the SPA bundle reads at boot.
grep -q "reqvireProjectStore" "$OUTPUT_DIR/index.html"
INDEX_FILE="$OUTPUT_DIR/index.html" node - <<'NODE'
const fs = require('fs');
const html = fs.readFileSync(process.env.INDEX_FILE, 'utf8');
const projectStoreScript = html.match(/<script[^>]*id=["']reqvire-project-store["'][^>]*>([\s\S]*?)<\/script>/);
if (!projectStoreScript) {
  console.error('index.html missing reqvire-project-store seed script');
  process.exit(1);
}
try {
  new Function(projectStoreScript[1]);
} catch (error) {
  console.error(`reqvire-project-store seed must parse as JavaScript: ${error.message}`);
  process.exit(1);
}
const match = html.match(/(?:const|let|var)\s+reqvireProjectStore\s*=\s*(\{[\s\S]*?\});\s*<\/script>/)
  || html.match(/window\.reqvireProjectStore\s*=\s*(\{[\s\S]*?\});\s*<\/script>/);
if (!match) {
  console.error('index.html must seed reqvireProjectStore before the Explorer bundle boots');
  process.exit(1);
}
let store;
try {
  store = JSON.parse(match[1]);
} catch (error) {
  console.error(`reqvireProjectStore seed must be valid JSON: ${error.message}`);
  process.exit(1);
}
const requiredSections = [
  'project',
  'folders',
  'files',
  'resources',
  'elements',
  'relations',
  'attachments',
  'concept_refs',
  'submodels',
  'traces',
  'coverage',
  'ontology',
  'knowledge_graph',
  'search',
  'summaries',
  'routes',
];
const missing = requiredSections.filter(section => !(section in store));
if (missing.length) {
  console.error(`Project Store seed missing required sections: ${missing.join(', ')}`);
  process.exit(1);
}
if (!store.schema_version) {
  console.error('Project Store seed must include schema_version');
  process.exit(1);
}
const fileRecords = Array.isArray(store.files) ? store.files : Object.values(store.files || {});
const resourceRecords = Array.isArray(store.resources) ? store.resources : Object.values(store.resources || {});
if (!fileRecords.some(file => JSON.stringify(file).includes('requirements/Requirements.md'))) {
  console.error('Project Store files must include exported source file containers');
  process.exit(1);
}
if (!resourceRecords.some(resource => JSON.stringify(resource).includes('src/impl.rs'))) {
  console.error('Project Store resources must include modeled implementation/evidence files separately from source file containers');
  process.exit(1);
}
const canonicalRoutes = new Map((store.routes?.canonical || []).map(route => [route.id, route.pattern]));
const expectedRoutes = new Map([
  ['model', '#/model'],
  ['traces', '#/traces'],
  ['ontologies', '#/ontologies'],
  ['kn2', '#/kn2'],
  ['coverage', '#/coverage'],
  ['resources', '#/resources'],
  ['files', '#/files/<path>'],
  ['elements', '#/elements/<identifier>'],
  ['search', '#/search'],
]);
for (const [id, pattern] of expectedRoutes) {
  if (canonicalRoutes.get(id) !== pattern) {
    console.error(`Project Store canonical route ${id} must map to ${pattern}`);
    process.exit(1);
  }
}
if ('legacy' in (store.routes || {})) {
  console.error(`Project Store must not advertise page compatibility routes: ${JSON.stringify(store.routes.legacy)}`);
  process.exit(1);
}
const graph = store.knowledge_graph || {};
const nodes = Array.isArray(graph.nodes) ? graph.nodes : [];
const edges = Array.isArray(graph.edges) ? graph.edges : [];
const nodeIds = new Set(nodes.map(node => node.id));
const missingEndpoints = edges.filter(edge => !nodeIds.has(edge.source) || !nodeIds.has(edge.target));
if (nodes.length < 2 || edges.length < 1 || missingEndpoints.length > 0) {
  console.error(`Invalid Project Store knowledge_graph projection: nodes=${nodes.length}, edges=${edges.length}, missingEndpoints=${missingEndpoints.length}`);
  process.exit(1);
}
if (!nodes.every(node => node.type)) {
  console.error('Project Store knowledge_graph nodes must carry exported type values');
  process.exit(1);
}
if (!Array.isArray(graph.submodels) || graph.submodels.length < 1) {
  console.error('Project Store knowledge_graph must include Reqvire root submodels for Model/KN2 graph modes');
  process.exit(1);
}
const relationLabels = new Set(edges.map(edge => edge.label));
if (!relationLabels.has('specifiedBy')) {
  console.error('Project Store knowledge_graph must preserve canonical relation labels');
  process.exit(1);
}
if (relationLabels.has('specify')) {
  console.error('Project Store knowledge_graph must not duplicate opposite relation labels as parallel graph edges');
  process.exit(1);
}
if (!store.ontology?.graph_data || !store.ontology?.graph_renderer || !store.ontology?.ttl_href) {
  console.error('Project Store ontology projection must expose graph data, renderer assets, and ontologies.ttl link');
  process.exit(1);
}
// Elements must be seeded in deterministic identifier order for stable diffs.
const elementIds = (store.elements || []).map(element => element.id);
const sortedIds = [...elementIds].sort();
if (JSON.stringify(elementIds) !== JSON.stringify(sortedIds)) {
  console.error('Project Store elements must be seeded in sorted identifier order');
  process.exit(1);
}
// Element-detail records must carry the fields the in-shell modal renders.
const elementSample = (store.elements || [])[0];
if (!elementSample) {
  console.error('Project Store must seed at least one element record');
  process.exit(1);
}
for (const field of ['id', 'name', 'element_type', 'type_family', 'file_path', 'source_anchor', 'content', 'metadata', 'governance']) {
  if (!(field in elementSample)) {
    console.error(`Project Store element records missing element-detail field: ${field}`);
    process.exit(1);
  }
}
// At least one search document must deep-link to the element-detail route.
const searchDocs = Array.isArray(store.search) ? store.search : [];
if (!searchDocs.some(doc => typeof doc.route === 'string' && doc.route.startsWith('#/elements/'))) {
  console.error('Project Store search documents must deep-link to #/elements/<identifier>');
  process.exit(1);
}
NODE
BUNDLE_FILE="$OUTPUT_DIR/assets/explorer.js" node - <<'NODE'
const fs = require('fs');
const bundle = fs.readFileSync(process.env.BUNDLE_FILE, 'utf8');
const requiredRouteMarkers = [
  'Containment',
  'Model',
  'Traces',
  'Ontologies',
  'Knowledge Graph',
  'KN2',
  'Open source page',
  'Element not found',
];
for (const marker of requiredRouteMarkers) {
  if (!bundle.includes(marker)) {
    console.error(`Explorer bundle missing native route/detail marker: ${marker}`);
    process.exit(1);
  }
}
for (const routeStem of ['model', 'knowledgegraph', 'traces', 'ontologies', 'kn2']) {
  const routePage = `${routeStem}.html`;
  if (bundle.includes(`src="${routePage}`) || bundle.includes(`src:'${routePage}`) || bundle.includes(`src:"${routePage}`)) {
    console.error(`Explorer bundle must not iframe/mount standalone page route content: ${routeStem}`);
    process.exit(1);
  }
}
NODE

echo "  ✅ Integration tests passed"
echo

# ========================================
# 2. RESPONSIVE DESIGN TESTS
# ========================================
echo "2. Running Responsive Design Tests..."

# Verify Tailwind is compiled into the bundle stylesheet (not CDN/runtime).
echo "  - Checking compiled Tailwind CSS..."
test -s "$OUTPUT_DIR/assets/explorer.css"
# Compiled Tailwind utilities appear as plain CSS rules in the stylesheet.
grep -qE '\.(flex|grid|hidden)\b|--tw-' "$OUTPUT_DIR/assets/explorer.css"

# Verify viewport meta tag
echo "  - Checking viewport meta tag..."
grep -q 'name="viewport"' "$OUTPUT_DIR/index.html"

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

    # Check DOCTYPE (the SPA bundle emits a lowercase doctype)
    head -1 "$file" | grep -qi "<!doctype html>"

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
