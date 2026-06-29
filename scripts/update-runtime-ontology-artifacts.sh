#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OUTPUT_DIR="$REPO_ROOT/crates/reqvire-core/src/runtime_ontology"

if [[ $# -ne 0 ]]; then
  echo "usage: $0" >&2
  exit 2
fi

mkdir -p "$OUTPUT_DIR"

if [[ -n "${REQVIRE_BIN:-}" ]]; then
  REQVIRE_CMD=("$REQVIRE_BIN")
elif [[ -x "$REPO_ROOT/target/debug/reqvire" ]]; then
  REQVIRE_CMD=("$REPO_ROOT/target/debug/reqvire")
else
  REQVIRE_CMD=(cargo run -q -p reqvire-cli --)
fi

(
  cd "$REPO_ROOT"
  "${REQVIRE_CMD[@]}" semantic export \
    --layer ontologies \
    --namespace-base https://www.reqvire.org/ontology# \
    --output "$OUTPUT_DIR/reqvire.ttl" >/dev/null

  "${REQVIRE_CMD[@]}" semantic export \
    --layer shapes \
    --namespace-base https://www.reqvire.org/ontology# \
    --output "$OUTPUT_DIR/reqvire-shacl.ttl" >/dev/null
)

echo "Updated runtime ontology artifacts in $OUTPUT_DIR"
