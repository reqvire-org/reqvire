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

curate_runtime_ontology() {
  local input="$1"
  local output="$2"
  python3 - "$input" "$output" <<'PY'
import sys

source, target = sys.argv[1], sys.argv[2]
maps_to_concept = "https://www.reqvire.org/ontology#mapsToConcept"

with open(source, "r", encoding="utf-8") as handle:
    lines = handle.readlines()

blocks = []
current = []
for line in lines:
    current.append(line)
    if line.strip().endswith("."):
        blocks.append(current)
        current = []
if current:
    blocks.append(current)

def curate_block(block):
    if maps_to_concept in block[0]:
        return []

    concept_import = "<http://www.w3.org/2002/07/owl#imports> <https://www.reqvire.org/concepts>"
    curated = []
    pending_subject = None

    for line in block:
        if maps_to_concept in line:
            continue
        if concept_import in line:
            prefix = line.split(concept_import, 1)[0]
            if prefix.strip():
                pending_subject = prefix
            continue
        if pending_subject is not None and line.startswith("\t"):
            curated.append(pending_subject + line.lstrip())
            pending_subject = None
            continue
        if pending_subject is not None:
            curated.append(pending_subject.rstrip() + " .\n")
            pending_subject = None
        curated.append(line)

    if pending_subject is not None:
        curated.append(pending_subject.rstrip() + " .\n")

    content_indexes = [
        index
        for index, line in enumerate(curated)
        if line.strip() and not line.startswith("#")
    ]
    if content_indexes:
        last_index = content_indexes[-1]
        stripped = curated[last_index].rstrip()
        if stripped.endswith(";"):
            curated[last_index] = stripped[:-1].rstrip() + " .\n"

    return curated

with open(target, "w", encoding="utf-8") as handle:
    for block in blocks:
        handle.writelines(curate_block(block))
PY
}

GENERATED_RUNTIME_ONTOLOGY="$(mktemp -t reqvire-runtime-ontology-XXXXXX.ttl)"
CURATED_RUNTIME_ONTOLOGY="$(mktemp -t reqvire-runtime-ontology-curated-XXXXXX.ttl)"
NORMALIZED_EXPECTED="$(mktemp -t reqvire-runtime-ontology-expected-XXXXXX.ttl)"
NORMALIZED_GENERATED="$(mktemp -t reqvire-runtime-ontology-generated-XXXXXX.ttl)"

cleanup_runtime_ontology_test() {
  rm -f "$GENERATED_RUNTIME_ONTOLOGY" "$CURATED_RUNTIME_ONTOLOGY" "$NORMALIZED_EXPECTED" "$NORMALIZED_GENERATED"
}
trap cleanup_runtime_ontology_test EXIT

set +e
(cd "$REPO_ROOT" && "$REQVIRE_BIN" semantic graph --namespace-base https://www.reqvire.org/ontology# --output "$GENERATED_RUNTIME_ONTOLOGY" >/dev/null)
GENERATE_EXIT=$?
set -e

if [ $GENERATE_EXIT -ne 0 ]; then
  echo "FAILED: could not regenerate runtime Reqvire ontology artifact."
  exit 1
fi

curate_runtime_ontology "$GENERATED_RUNTIME_ONTOLOGY" "$CURATED_RUNTIME_ONTOLOGY"

FORBIDDEN_EXTRA_NAMESPACE="https://www.reqvire.org/ontology""-extra"
if grep -q "$FORBIDDEN_EXTRA_NAMESPACE" "$CURATED_RUNTIME_ONTOLOGY"; then
  echo "FAILED: runtime Reqvire ontology artifact must not include non-canonical extra ontology vocabulary."
  exit 1
fi

if grep -q "https://www.reqvire.org/ontology#mapsToConcept" "$CURATED_RUNTIME_ONTOLOGY"; then
  echo "FAILED: runtime Reqvire ontology artifact must not include reqvire:mapsToConcept vocabulary or bridge facts."
  exit 1
fi

normalize_turtle_bnodes "$REPO_ROOT/crates/reqvire-core/src/runtime_ontology/reqvire.ttl" "$NORMALIZED_EXPECTED"
normalize_turtle_bnodes "$CURATED_RUNTIME_ONTOLOGY" "$NORMALIZED_GENERATED"

if ! diff -u "$NORMALIZED_EXPECTED" "$NORMALIZED_GENERATED"; then
  echo "FAILED: crates/reqvire-core/src/runtime_ontology/reqvire.ttl is stale."
  echo "Regenerate it by exporting the Reqvire namespace, then curating concept-layer imports and reqvire:mapsToConcept facts out of the runtime artifact."
  exit 1
fi

exit 0
