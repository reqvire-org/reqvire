#!/usr/bin/env bash
# Black-box checks against independently authored requirement assessments.
set -euo pipefail
binary=$1 workspace=$2 expected_path=$3
suite=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$workspace"
mkdir -p output
expected=$expected_path/requirements.json
run() { timeout 30 "$binary" "$@"; }
equal_json() {
    if jq -e --slurpfile wanted "$1" '.==$wanted[0]' "$2" >/dev/null; then return; fi
    jq -S . "$1" > output/equal-expected.json
    jq -S . "$2" > output/equal-actual.json
    diff -u output/equal-expected.json output/equal-actual.json
}
assessments() {
    jq -e --slurpfile wanted "$2" '
      [.covered_requirements.files[][]|{record:.,covered:true}] +
      [.uncovered_requirements.files[][]|{record:.,covered:false}] |
      (group_by(.record.name)|all(.[];length==1)) and
      all(.[]; ($wanted[0][.record.name].coverage_source!="uncovered")==.covered)
    ' "$1" >/dev/null
    jq '[.covered_requirements.files[][],.uncovered_requirements.files[][]]|map({key:.name,value:.})|from_entries' "$1" > output/assessments.json
    equal_json "$2" output/assessments.json
}
summary() {
    jq -n --slurpfile wanted "$2" '
      ($wanted[0]|[.[]]) as $records |
      ($records|map(select(.is_terminal))) as $terminal |
      ($records|map(select(.coverage_source!="uncovered"))) as $covered |
      ($terminal|map(select(.coverage_source!="uncovered"))) as $implemented |
      {total_requirements_in_scope:($records|length),covered_requirements:($covered|length),
       uncovered_requirements:(($records|length)-($covered|length)),
       total_terminal_requirements:($terminal|length),covered_terminal_requirements:($implemented|length),
       uncovered_terminal_requirements:(($terminal|length)-($implemented|length)),
       implementation_coverage_percentage:(if ($terminal|length)==0 then 0 else
         ((($implemented|length)*10000/($terminal|length))|round)/100 end),
       coverage_sources:(["direct_satisfied","requirement_rollup","contract_consumer_rollup","combined_rollup"] |
         map(. as $source|{key:$source,value:([$covered[]|select(.coverage_source==$source)]|length)})|from_entries)}
    ' > output/summary-expected.json
    jq --slurpfile expected output/summary-expected.json '.summary|with_entries(select(.key as $key|$expected[0]|has($key)))' "$1" > output/summary-actual.json
    equal_json output/summary-expected.json output/summary-actual.json
}
wanted_scope() {
    jq --arg name "$1" '.[$name]' "$expected_path/scopes.json" > output/scope-ids.json
    jq --slurpfile ids output/scope-ids.json 'with_entries(select(.value.identifier as $id|$ids[0]|index($id)!=null))' "$expected" > output/scope-wanted.json
}
scoped() {
    local name=$1
    run coverage --from "$name" --json > output/scoped.json
    wanted_scope "$name"
    jq '.scope.requirement_ids' output/scoped.json > output/scoped-ids.json
    equal_json output/scope-ids.json output/scoped-ids.json
    assessments output/scoped.json output/scope-wanted.json
    summary output/scoped.json output/scope-wanted.json
    jq -e --arg name "$name" --slurpfile wanted output/scope-wanted.json '
      .summary as $s | any(.capability_coverage.capabilities[];
        .name==$name and .aggregate_terminal_requirements==$s.total_terminal_requirements and
        .aggregate_covered_terminal_requirements==$s.covered_terminal_requirements and
        .implementation_coverage_percentage==$s.implementation_coverage_percentage and
        .implementation_covered==(($wanted[0]|length>0) and all($wanted[0][];.coverage_source!="uncovered")))
    ' output/scoped.json >/dev/null
}
verification_leaves() {
    jq -e '[.verified_leaf_requirements.files[][].name]|sort==["Complete Owner","Unused Owner"]' output/whole.json >/dev/null
}
text_evidence() {
    run coverage > output/whole.txt
    jq -e -Rs '
      contains("Terminal Requirements") and contains("Covered Terminal Requirements") and
      (split("## Uncovered Requirements")[1]|split("## Capability Coverage")[0]|
        contains("evidence/parent.txt") and contains("Blocking requirements") and contains("Contributing requirements"))
    ' output/whole.txt >/dev/null
}
cp specifications/Model.md output/original-model.txt
order_independence() {
    trap 'cp output/original-model.txt specifications/Model.md' EXIT
    jq -Rs -j -f "$suite/../reorder_model.jq" output/original-model.txt > specifications/Model.md
    run coverage --json > output/reordered.json
    assessments output/reordered.json "$expected"
}
mutation() {
    trap 'cp output/original-model.txt specifications/Model.md' EXIT
    jq -Rs -j '
      split("### Complete Consumer B\n") |
      if length!=2 then error("Consumer B missing") else
        .[0]+"### Complete Consumer B\n"+(.[1]|sub("  \\* satisfiedBy: \\[b\\]\\(../evidence/b.txt\\)\n";"")) end
    ' output/original-model.txt > specifications/Model.md
    run coverage --from 'Delegated Service' --json > output/mutation.json
    jq -e '
      .uncovered_requirements.files[][] as $owner |
      $owner.name=="Complete Owner" and $owner.evidence==["evidence/a.txt"] and
      $owner.blocking_requirements==["specifications/Model.md#complete-consumer","specifications/Model.md#complete-consumer-b"] and
      $owner.aggregate_terminal_requirements==2 and $owner.aggregate_covered_terminal_requirements==1 and
      .summary.total_terminal_requirements==0
    ' output/mutation.json >/dev/null
}
exported_store() {
    run export --output output/site > output/export.stdout
    jq -Rs 'sub("^window\\.reqvireProjectStore\\s*=\\s*";"")|sub(";\\s*$";"")|fromjson|.coverage' output/site/assets/project-store.js > output/store-coverage.json
    assessments output/store-coverage.json "$expected"
    summary output/store-coverage.json "$expected"
    while IFS= read -r name; do
        wanted_scope "$name"
        jq --arg name "$name" '.scope_index["specifications/Model.md#"+($name|ascii_downcase|gsub(" ";"-"))]' output/store-coverage.json > output/store-scoped.json
        jq '.scope.requirement_ids' output/store-scoped.json > output/store-ids.json
        equal_json output/scope-ids.json output/store-ids.json
        summary output/store-scoped.json output/scope-wanted.json
    done < <(jq -r 'keys[]' "$expected_path/scopes.json")
}
failures=()
check() {
    local label=$1 status=0
    shift
    (set -e; "$@") > output/rollup-check.log 2>&1 &
    local pid=$!
    wait "$pid" || status=$?
    if ((status)); then failures+=("$label"); printf 'FAIL %s: ' "$label"; cat output/rollup-check.log
    else printf 'PASS %s\n' "$label"; fi
}
run coverage --json > output/whole.json
check 'whole model assessments' assessments output/whole.json "$expected"
check 'terminal implementation percentage' summary output/whole.json "$expected"
check 'verification leaves differ from implementation terminals' verification_leaves
while IFS= read -r name; do check "scope $name" scoped "$name"; done < <(jq -r 'keys_unsorted[]' "$expected_path/scopes.json")
check 'text retains partial evidence and blockers' text_evidence
check 'declaration and relation order independence' order_independence
check 'external evidence removal invalidates recursive owner' mutation
check 'Explorer export reuses shared assessments and scope summaries' exported_store
if ((${#failures[@]})); then printf '%s checks failed\n' "${#failures[@]}"; exit 1; fi
