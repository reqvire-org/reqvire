#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$TEST_SCRIPT_DIR/../.." && pwd)"

set +e
DEPRECATED_OUTPUT=$(rg -n "owl:deprecated true" "$REPO_ROOT/system-model/Ontologies" 2>&1)
DEPRECATED_EXIT=$?
set -e

if [ $DEPRECATED_EXIT -eq 0 ]; then
  echo "FAILED: Reqvire authored ontologies must not leave deprecated vocabulary declarations behind."
  echo "Use rdfs:label/rdfs:comment for presentation metadata, or remove the stale property entirely."
  echo "$DEPRECATED_OUTPUT"
  exit 1
fi

if [ $DEPRECATED_EXIT -ne 1 ]; then
  echo "FAILED: authored ontology hygiene scan failed."
  echo "$DEPRECATED_OUTPUT"
  exit 1
fi

exit 0
