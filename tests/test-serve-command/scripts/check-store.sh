#!/bin/bash
# Check served/exported seed data without evaluating the JavaScript assignment.
set -euo pipefail

fail() { printf 'FAILED: %s\n' "$*" >&2; exit 1; }
[[ $# -gt 0 ]] || fail 'Expected a Project Store seed file'
seed=$1
shift
requirement_text='Test Requirement One'
evidence_text='serve command evidence'
trace_args=(--argjson expected_traces '[]')
compare_traces=false
while [[ $# -gt 0 ]]; do
    [[ $# -ge 2 ]] || fail "Missing value for $1"
    case "$1" in
        --requirement-text) requirement_text=$2 ;;
        --evidence-text) evidence_text=$2 ;;
        --traces-json) trace_args=(--slurpfile expected_traces "$2"); compare_traces=true ;;
        *) fail "Unknown option: $1" ;;
    esac
    shift 2
done

jq -e -Rs --arg requirement "$requirement_text" --arg evidence_text "$evidence_text" \
    --argjson compare_traces "$compare_traces" "${trace_args[@]}" '
  def require($condition; $message):
    if $condition then . else error("FAILED: " + $message) end;

  # The generated seed is one JSON assignment. Strip its known wrapper;
  # semicolons inside JSON strings remain intact.
  require(startswith("window.reqvireProjectStore = ");
    "Project Store seed assignment is missing") |
  ltrimstr("window.reqvireProjectStore = ") |
  require(test(";[[:space:]]*$"); "Project Store seed terminator is missing") |
  sub(";[[:space:]]*$"; "") | fromjson |
  (.files | map({key: .path, value: .}) | from_entries) as $files |
  require($files | has("specifications/Requirements.md");
    "Project Store is missing modeled source file records") |
  require($files["specifications/Requirements.md"].markdown_content | contains($requirement);
    "Modeled source content is missing") |
  require($files | has("scripts/evidence.sh") | not;
    "Resource-only evidence appeared as a modeled file") |
  require($files | has("notes/unrelated.md") | not;
    "Unrelated repository file appeared as a modeled file") |
  ([.resources[] | select(.id == "resource:scripts/evidence.sh")][0]) as $evidence |
  require($evidence != null; "Graph-referenced evidence resource is missing") |
  require($evidence.file_path == "scripts/evidence.sh"; "Evidence resource path is incorrect") |
  require($evidence.source_text | contains($evidence_text); "Evidence source preview is missing") |
  require(.schema_version == "2026-10-06.project-store.v5"; "Unexpected Project Store schema") |
  require(all(.relations[]; has("inverse_relation_type")); "Core inverse vocabulary is missing") |
  ([.relations[] | select(.resource_id == $evidence.id)][0]) as $relation |
  require($relation.canonical_relation_type == "satisfiedBy"; "Evidence relation is not canonical") |
  require($relation.inverse_relation_type == "satisfy"; "Evidence inverse vocabulary is missing") |

  reduce .traces.files[].verifications[] as $trace (.;
    ($trace.trace_graph.nodes | map({key: .id, value: .}) | from_entries) as $nodes |
    require(($nodes | length) == ($trace.trace_graph.nodes | length); "Repeated trace node") |
    require(all($nodes[]; has("children") | not); "Trace contains nested subtrees") |
    require(all($nodes[]; has("type") and has("is_directly_verified"));
      "Trace node metadata is missing") |
    require(all($trace.trace_graph.edges[];
      . as $edge | ($nodes | has($edge.source) and has($edge.target)));
      "Dangling trace edge") |
    require($trace.total_requirements_in_tree ==
      ([$nodes[] | select(.type == "requirement")] | length);
      "Trace requirement count is incorrect") |
    require(([$nodes[] | select(.is_directly_verified) | .id] | sort) ==
      $trace.directly_verified_requirements; "Direct verification identifiers differ")
  ) |
  require(($compare_traces | not) or
    (($expected_traces | length) == 1 and .traces == $expected_traces[0]);
    "Served/exported traces differ from CLI")
' "$seed" > /dev/null
