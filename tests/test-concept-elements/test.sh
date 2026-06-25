#!/usr/bin/env bash
set -uo pipefail

set +e
VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATE_EXIT=$?
CONCEPTS_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" semantic export --layer concepts 2>&1)
CONCEPTS_EXIT=$?
MODEL_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --filter-type concept-scheme --json 2>&1)
MODEL_EXIT=$?
SCHEME_SEARCH_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type concept-scheme --json 2>&1)
SCHEME_SEARCH_EXIT=$?
CONCEPT_SEARCH_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type concept --json 2>&1)
CONCEPT_SEARCH_EXIT=$?
LINT_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --json 2>&1)
LINT_EXIT=$?
FORMAT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" format 2>&1)
FORMAT_EXIT=$?
set -e

if [ $VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: native concept fixture should validate"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

if [ $CONCEPTS_EXIT -ne 0 ]; then
  echo "FAILED: semantic export --layer concepts command failed"
  echo "$CONCEPTS_OUTPUT"
  exit 1
fi

for token in \
  "concept:SparseConceptScheme" \
  "skos:ConceptScheme" \
  "concept:SparseConcept" \
  "concept:DetailedConcept" \
  "skos:Concept" \
  "skos:inScheme" \
  "skos:altLabel" \
  "skos:scopeNote" \
  "skos:example" \
  "skos:broader" \
  "skos:narrower" \
  "skos:related" \
  "skos:exactMatch" \
  "skos:closeMatch" \
  '"Sparse Concept"'; do
  if ! grep -qF "$token" <<< "$CONCEPTS_OUTPUT"; then
    echo "FAILED: semantic export --layer concepts output missing token: $token"
    echo "$CONCEPTS_OUTPUT"
    exit 1
  fi
done

if [ $MODEL_EXIT -ne 0 ]; then
  echo "FAILED: model --filter-type concept-scheme command failed"
  echo "$MODEL_JSON"
  exit 1
fi

if [ $SCHEME_SEARCH_EXIT -ne 0 ]; then
  echo "FAILED: search --filter-type concept-scheme command failed"
  echo "$SCHEME_SEARCH_JSON"
  exit 1
fi

if [ $CONCEPT_SEARCH_EXIT -ne 0 ]; then
  echo "FAILED: search --filter-type concept command failed"
  echo "$CONCEPT_SEARCH_JSON"
  exit 1
fi

if ! jq -e '
  any(.elements[];
    .name == "Sparse Concept Scheme"
    and .element_type == "concept-scheme"
    and any(.relations[]; .relation_type == "derive" and .element.name == "Sparse Concept"))
' >/dev/null 2>&1 <<< "$MODEL_JSON"; then
  echo "FAILED: model output missing concept scheme root and child concept"
  echo "$MODEL_JSON"
  exit 1
fi

if ! jq -e '
  any(.files[].elements[];
    .name == "Sparse Concept Scheme"
    and .concept_scheme.iri == "https://example.test/native-concepts#SparseConceptScheme"
    and .concept_scheme.namespace_base == "https://example.test/native-concepts"
    and .concept_scheme.namespace_prefix == "concept"
    and .concept_scheme.source_element_identifier == "specifications/Concepts.md#sparse-concept-scheme")
' >/dev/null 2>&1 <<< "$SCHEME_SEARCH_JSON"; then
  echo "FAILED: search JSON missing concept-scheme namespace/source payload fields"
  echo "$SCHEME_SEARCH_JSON"
  exit 1
fi

if ! jq -e '
  any(.files[].elements[];
    .name == "Sparse Concept"
    and .concept.iri == "https://example.test/native-concepts#SparseConcept"
    and .concept.scheme_iri == "https://example.test/native-concepts#SparseConceptScheme"
    and .concept.namespace_base == "https://example.test/native-concepts"
    and .concept.namespace_prefix == "concept"
    and .concept.source_element_identifier == "specifications/Concepts.md#sparse-concept")
' >/dev/null 2>&1 <<< "$CONCEPT_SEARCH_JSON"; then
  echo "FAILED: search JSON missing concept namespace/source payload fields"
  echo "$CONCEPT_SEARCH_JSON"
  exit 1
fi

if ! jq -e '
  any(.files[].elements[];
    .name == "Detailed Concept"
    and .concept.iri == "https://example.test/native-concepts#DetailedConcept"
    and .concept.definition == "Detailed concept definition generated from the main element body."
    and any(.concept.labels[]; .kind == "altLabel" and .value == "Detailed vocabulary term")
    and .concept.scope_note == "Use this concept to verify Markdown-native SKOS concept payload extraction."
    and any(.concept.examples[]; .value == "A concept author writes labels, examples, mappings, and taxonomy in Markdown.")
    and any(.concept.broader[]; .target == "specifications/Concepts.md#concept-taxonomy-parent")
    and any(.concept.related[]; .target == "specifications/Concepts.md#concept-taxonomy-peer")
    and any(.concept.exact_match[]; .target == "https://external.example/concepts/DetailedConcept")
    and any(.concept.close_match[]; .target == "https://external.example/concepts/Detail"))
' >/dev/null 2>&1 <<< "$CONCEPT_SEARCH_JSON"; then
  echo "FAILED: search JSON missing detailed concept payload fields"
  echo "$CONCEPT_SEARCH_JSON"
  exit 1
fi

if [ $LINT_EXIT -ne 0 ]; then
  echo "FAILED: lint --json command failed"
  echo "$LINT_JSON"
  exit 1
fi

if [ $FORMAT_EXIT -ne 0 ]; then
  echo "FAILED: format command failed"
  echo "$FORMAT_OUTPUT"
  exit 1
fi

if ! grep -q "No formatting changes needed" <<< "$FORMAT_OUTPUT"; then
  echo "FAILED: native concept fixture should already be formatter-stable"
  echo "$FORMAT_OUTPUT"
  exit 1
fi

for warning in \
  missing-definition \
  isolated-concept; do
  if ! jq -e --arg warning "$warning" '
    any(.needs_manual_review[]; .type == "concept_authoring_warning" and .warning == $warning)
  ' >/dev/null 2>&1 <<< "$LINT_JSON"; then
    echo "FAILED: lint JSON missing native concept warning: $warning"
    echo "$LINT_JSON"
    exit 1
  fi
done

if ! jq -e '
  any(.needs_manual_review[]; .type == "concept_authoring_warning" and .element.name == "Sparse Concept Scheme")
  and any(.needs_manual_review[]; .type == "concept_authoring_warning" and .element.name == "Sparse Concept")
' >/dev/null 2>&1 <<< "$LINT_JSON"; then
  echo "FAILED: lint JSON missing expected concept warning elements"
  echo "$LINT_JSON"
  exit 1
fi

INVALID_CONCEPT_FILE="$TEST_DIR/specifications/InvalidConceptSections.md"
FORMAT_ORDER_FILE="$TEST_DIR/specifications/FormatConceptOrder.md"
FIXTURE_DIR="$TEST_DIR/fixtures"
cleanup_invalid_concept_file() {
  rm -f "$INVALID_CONCEPT_FILE"
  rm -f "$FORMAT_ORDER_FILE"
}
trap cleanup_invalid_concept_file EXIT

cp "$FIXTURE_DIR/FormatConceptOrder.md.fixture" "$FORMAT_ORDER_FILE"

set +e
FORMAT_ORDER_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" format --json 2>&1)
FORMAT_ORDER_EXIT=$?
set -e

if [ $FORMAT_ORDER_EXIT -ne 0 ]; then
  echo "FAILED: format command failed for concept relation ordering fixture"
  echo "$FORMAT_ORDER_OUTPUT"
  exit 1
fi

if ! jq -e '
  [.diffs[]
    | select(.file_path == "specifications/FormatConceptOrder.md")
    | .lines[]
    | select(.color == "green")
    | .content] as $added
  | ($added | index("+     * broader: [Format Parent Concept](#format-parent-concept)")) as $broader
  | ($added | index("+     * narrower: [Format Child Concept](#format-child-concept)")) as $narrower
  | ($added | index("+     * related: [Format Peer Concept](#format-peer-concept)")) as $related
  | $broader != null
    and $narrower != null
    and $related != null
    and $broader < $narrower
    and $narrower < $related
' >/dev/null 2>&1 <<< "$FORMAT_ORDER_OUTPUT"; then
  echo "FAILED: format output should canonicalize concept relation order"
  echo "$FORMAT_ORDER_OUTPUT"
  exit 1
fi

cp "$FIXTURE_DIR/InvalidConceptSections.md.fixture" "$INVALID_CONCEPT_FILE"

set +e
INVALID_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
INVALID_VALIDATE_EXIT=$?
set -e

if [ $INVALID_VALIDATE_EXIT -eq 0 ]; then
  echo "FAILED: forbidden native concept sections should fail validation"
  echo "$INVALID_VALIDATE_OUTPUT"
  exit 1
fi

for forbidden_section in "#### Top Concepts" "#### Definition" "#### Details"; do
  if ! grep -q "must not contain a $forbidden_section section" <<< "$INVALID_VALIDATE_OUTPUT"; then
    echo "FAILED: validation output missing forbidden concept section diagnostic for $forbidden_section"
    echo "$INVALID_VALIDATE_OUTPUT"
    exit 1
  fi
done

for forbidden_metadata in "concept_id" "language" "concept_kind" "pref_label"; do
  if ! grep -q "must not declare concept-specific metadata key '$forbidden_metadata'" <<< "$INVALID_VALIDATE_OUTPUT"; then
    echo "FAILED: validation output missing forbidden concept metadata diagnostic for $forbidden_metadata"
    echo "$INVALID_VALIDATE_OUTPUT"
    exit 1
  fi
done

for required_diagnostic in \
  "Concept scheme 'Missing Concept Base Scheme' must define non-empty concept_base metadata." \
  "Concept scheme 'Missing Concept Prefix Scheme' must define non-empty concept_prefix metadata." \
  "Duplicate concept_prefix 'duplicateconcept'" \
  "Duplicate concept namespace <https://example.test/duplicate-namespace#>" \
  "Concept 'Orphan Concept' must derive from a concept-scheme or another concept with scheme context." \
  "Concept 'Concept Under Ontology' must derive from a concept-scheme or another concept with scheme context." \
  "Concept taxonomy relation crosses concept schemes: concept 'Invalid Cross Scheme Broader Concept' uses broader to concept 'Cross Scheme Parent Concept'" \
  "Concept taxonomy relation crosses concept schemes: concept 'Invalid Cross Scheme Narrower Concept' uses narrower to concept 'Cross Scheme Parent Concept'" \
  "Concept schemes are standalone roots and own concept_base/concept_prefix directly."; do
  if ! grep -q "$required_diagnostic" <<< "$INVALID_VALIDATE_OUTPUT"; then
    echo "FAILED: validation output missing concept standalone diagnostic: $required_diagnostic"
    echo "$INVALID_VALIDATE_OUTPUT"
    exit 1
  fi
done

exit 0
