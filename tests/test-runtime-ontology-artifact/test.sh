#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$TEST_SCRIPT_DIR/../.." && pwd)"

normalize_turtle_bnodes() {
  local input="$1"
  local output="$2"
  awk '
    {
      rest=$0; line=""
      while (match(rest, /_:[A-Za-z][A-Za-z0-9_-]*/)) {
        label=substr(rest,RSTART+2,RLENGTH-2)
        if (!(label in mapping)) mapping[label]="_:b" ++counter
        line=line substr(rest,1,RSTART-1) mapping[label]
        rest=substr(rest,RSTART+RLENGTH)
      }
      lines[NR]=line rest
      if (lines[NR] !~ /^[[:space:]]*$/) last=NR
    }
    END {
      if (!last) { print ""; exit }
      sub(/[[:space:]]+$/, "", lines[last])
      for (i=1;i<=last;i++) print lines[i]
    }
  ' "$input" > "$output"
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
