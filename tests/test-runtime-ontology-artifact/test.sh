#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$TEST_SCRIPT_DIR/../.." && pwd)"

normalize_turtle_bnodes() {
  local input="$1"
  local output="$2"
  python3 - "$input" "$output" <<'PY'
import re
import sys

source, target = sys.argv[1], sys.argv[2]
mapping = {}
counter = 0
pattern = re.compile(r'_:([A-Za-z][A-Za-z0-9_-]*)')

def replace(match):
    global counter
    label = match.group(1)
    if label not in mapping:
        counter += 1
        mapping[label] = f"_:b{counter}"
    return mapping[label]

with open(source, "r", encoding="utf-8") as handle:
    content = handle.read()

with open(target, "w", encoding="utf-8") as handle:
    handle.write(pattern.sub(replace, content).rstrip() + "\n")
PY
}

GENERATED_RUNTIME_DIR="$(mktemp -d -t reqvire-runtime-artifacts-XXXXXX)"
NORMALIZED_EXPECTED="$(mktemp -t reqvire-runtime-ontology-expected-XXXXXX.ttl)"
NORMALIZED_GENERATED="$(mktemp -t reqvire-runtime-ontology-generated-XXXXXX.ttl)"
NORMALIZED_SHACL_EXPECTED="$(mktemp -t reqvire-runtime-shacl-expected-XXXXXX.ttl)"
NORMALIZED_SHACL_GENERATED="$(mktemp -t reqvire-runtime-shacl-generated-XXXXXX.ttl)"

cleanup_runtime_ontology_test() {
  rm -rf "$GENERATED_RUNTIME_DIR"
  rm -f "$NORMALIZED_EXPECTED" "$NORMALIZED_GENERATED" "$NORMALIZED_SHACL_EXPECTED" "$NORMALIZED_SHACL_GENERATED"
}
trap cleanup_runtime_ontology_test EXIT

set +e
(
  cd "$REPO_ROOT" && \
  "$REQVIRE_BIN" semantic export \
    --layer ontologies \
    --namespace-base https://www.reqvire.org/ontology# \
    --output "$GENERATED_RUNTIME_DIR/reqvire.ttl" >/dev/null && \
  "$REQVIRE_BIN" semantic export \
    --layer shapes \
    --namespace-base https://www.reqvire.org/ontology# \
    --output "$GENERATED_RUNTIME_DIR/reqvire-shacl.ttl" >/dev/null
)
GENERATE_EXIT=$?
set -e

if [ $GENERATE_EXIT -ne 0 ]; then
  echo "FAILED: could not regenerate runtime Reqvire ontology artifacts."
  exit 1
fi

FORBIDDEN_EXTRA_NAMESPACE="https://www.reqvire.org/ontology""-extra"
if grep -q "$FORBIDDEN_EXTRA_NAMESPACE" "$GENERATED_RUNTIME_DIR/reqvire.ttl"; then
  echo "FAILED: runtime Reqvire ontology artifact must not include non-canonical extra ontology vocabulary."
  exit 1
fi

if grep -q "# Kind: shapes" "$GENERATED_RUNTIME_DIR/reqvire.ttl"; then
  echo "FAILED: runtime Reqvire ontology artifact must not include semantic-contract SHACL shape blocks."
  exit 1
fi

if grep -q "# Kind: ontology" "$GENERATED_RUNTIME_DIR/reqvire-shacl.ttl"; then
  echo "FAILED: runtime Reqvire SHACL artifact must not include ontology vocabulary blocks."
  exit 1
fi

normalize_turtle_bnodes "$REPO_ROOT/crates/reqvire-core/src/runtime_ontology/reqvire.ttl" "$NORMALIZED_EXPECTED"
normalize_turtle_bnodes "$GENERATED_RUNTIME_DIR/reqvire.ttl" "$NORMALIZED_GENERATED"

if ! diff -u "$NORMALIZED_EXPECTED" "$NORMALIZED_GENERATED"; then
  echo "FAILED: crates/reqvire-core/src/runtime_ontology/reqvire.ttl is stale."
  echo "Regenerate it with: scripts/update-runtime-ontology-artifacts.sh"
  exit 1
fi

normalize_turtle_bnodes "$REPO_ROOT/crates/reqvire-core/src/runtime_ontology/reqvire-shacl.ttl" "$NORMALIZED_SHACL_EXPECTED"
normalize_turtle_bnodes "$GENERATED_RUNTIME_DIR/reqvire-shacl.ttl" "$NORMALIZED_SHACL_GENERATED"

if ! diff -u "$NORMALIZED_SHACL_EXPECTED" "$NORMALIZED_SHACL_GENERATED"; then
  echo "FAILED: crates/reqvire-core/src/runtime_ontology/reqvire-shacl.ttl is stale."
  echo "Regenerate it with: scripts/update-runtime-ontology-artifacts.sh"
  exit 1
fi

exit 0
