#!/usr/bin/env bash
# Black-box fulfillment-cycle checks; compare every case with the authored oracle.
set -euo pipefail
binary=$1
root=$2
suite=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
fixtures=$root/cycle-fixtures
output=$root/output/fulfillment-cycles
mkdir -p "$output"
snapshot() {
    (cd "$1"; find . -type d -name .git -prune -o -type f -print0 |
        LC_ALL=C sort -z | xargs -0 -r sha256sum)
}
prepare() {
    workspace=$root/output/fulfillment-cycle-workspaces/$name
    mkdir -p "$workspace"
    cp "$fixtures/$fixture" "$workspace/Model.md"
    case "$transform" in
        reverse) jq -Rs -j -f "$suite/../reorder_model.jq" "$workspace/Model.md" > "$output/$name-model.txt" ;;
        binding|merge)
            jq -Rs -j '
              "#### Contract Bindings\n  * [Serve Contract](#serve-contract)\n\n" as $binding |
              if (split($binding)|length)!=2 then error("Expected exactly one reverse binding")
              else split($binding)|join("") end
            ' "$workspace/Model.md" > "$output/$name-model.txt"
            if [[ "$transform" == merge ]]; then sed '1{/^# Elements$/d;}' "$fixtures/merge-source.md" >> "$output/$name-model.txt"; fi ;;
        child)
            sed '/^  \* derive: \[Child Requirement\](#child-requirement)$/d; /  \* definedBy: \[Child Contract\](#child-contract)/i\  * specify: [Mixed Root](#mixed-root)' \
                "$workspace/Model.md" > "$output/$name-model.txt" ;;
        '') cp "$workspace/Model.md" "$output/$name-model.txt" ;;
        *) return 1 ;;
    esac
    cp "$output/$name-model.txt" "$workspace/Model.md"
    printf 'Implementation evidence for cycle fixtures.\n' > "$workspace/implementation.txt"
    git -C "$workspace" init -q
    snapshot "$workspace" > "$output/$name-before.txt"
}
run() {
    local label=$1 executable=$binary
    shift
    case "$1" in link|merge) executable=$MCP_OPERATION_BIN ;; esac
    rc=0
    (cd "$workspace"; timeout 30 "$executable" "$@") > "$output/$label.stdout" 2> "$output/$label.stderr" || rc=$?
    if ((rc==124 || rc==137)); then printf 'Timed out: %s\n' "$label" >&2; return 1; fi
}
unchanged() {
    snapshot "$workspace" > "$output/$name-after.txt"
    same=false
    if cmp -s "$output/$name-before.txt" "$output/$name-after.txt"; then same=true; fi
}
rejected() {
    prepare
    case "$1" in link|merge)
        run "$name-before" validate --json
        if ((rc!=0)); then cat "$output/$name-before.stdout" "$output/$name-before.stderr" >&2; return 1; fi ;;
    esac
    run "$name" "$@"
    json=false
    for arg in "$@"; do [[ "$arg" != --json ]] || json=true; done
    if [[ "$json" == true ]]; then
        jq '.errors // []' "$output/$name.stdout" > "$output/$name-errors.json"
    else
        jq -n --rawfile out "$output/$name.stdout" --rawfile err "$output/$name.stderr" '[$out+$err]' > "$output/$name-errors.json"
    fi
    unchanged
    jq --arg fixture "$fixture" --argjson rc "$rc" --argjson same "$same" \
       --argjson json "$json" --rawfile out "$output/$name.stdout" '
      map(select(contains("Circular dependency error:"))) as $errors |
      if all($errors[];
        ([scan("Model\\.md#[a-z0-9-]+")]) as $path |
        ($path|length)>=2 and $path[0]==$path[-1] and contains("-[contract consumer]->") and
        (if $fixture=="mixed.md" then contains("-[child requirement]->") else true end))
      then {rejected:($rc>0), cycle_reported:($errors|length>0),
        participants:([$errors[]|scan("Model\\.md#[a-z0-9-]+")]|unique),
        coverage_report_emitted:(if $json then ($out|fromjson|type=="object" and has("summary")) else false end),
        sources_unchanged:$same}
      else error("Expected a closed cycle with labeled contract/hierarchy edges") end
    ' "$output/$name-errors.json" > "$output/$name-result.json"
}
valid() {
    prepare
    run "$name-validate" validate --json
    local validation_rc=$rc
    run "$name-coverage" coverage --json
    unchanged
    jq -n --slurpfile validation "$output/$name-validate.stdout" --slurpfile coverage "$output/$name-coverage.stdout" \
      --argjson vrc "$validation_rc" --argjson crc "$rc" --argjson same "$same" '
      {validation_succeeded:($vrc==0), validation_errors:$validation[0].errors,
       coverage_succeeded:($crc==0),
       all_requirements_covered:($coverage[0].summary.uncovered_requirements==0 and ($coverage[0].summary.covered_requirements//0)>0),
       sources_unchanged:$same}
    ' > "$output/$name-result.json"
}
failures=()
check() {
    name=$1 fixture=$2 transform=$3
    shift 3
    # A background subshell retains errexit while the parent collects all failures.
    (set -e; "$@"; jq -S . "$output/$name-result.json" > "$output/$name-actual.json"
        jq -S --arg name "$name" '.[$name]' "$root/expected/fulfillment-cycles.json" > "$output/$name-expected.json"
        diff -u "$output/$name-expected.json" "$output/$name-actual.json") > "$output/$name-check.log" 2>&1 &
    local pid=$! status=0
    wait "$pid" || status=$?
    if ((status)); then failures+=("$name"); printf 'FAIL %s:\n' "$name"; cat "$output/$name-check.log"; fi
}
check same-root-text same-root.md '' rejected validate
check same-root-json same-root.md '' rejected validate --json
check same-root-coverage same-root.md '' rejected coverage --json
check three-roots-json three-roots.md '' rejected validate --json
check mixed-json mixed.md '' rejected validate --json
check mixed-coverage mixed.md '' rejected coverage --json
check same-root-reordered same-root.md reverse rejected validate --json
check repaired-binding same-root.md binding valid
check shared-dag shared-dag.md '' valid
for mode in apply dry-run; do
    suffix='' extra=()
    if [[ "$mode" == dry-run ]]; then suffix=-dry-run; extra=(--dry-run); fi
    check "binding-mutation$suffix" same-root.md binding rejected link 'Embedded Endpoint' bindContract 'Model.md#serve-contract' "${extra[@]}"
    check "child-mutation$suffix" mixed.md child rejected link 'Parent Requirement' derive 'Child Requirement' "${extra[@]}"
    check "merge-mutation$suffix" same-root.md merge rejected merge 'Embedded Endpoint' 'Merge Source' "${extra[@]}"
done
# Include completed cases even if a golden comparison failed.
results=()
while IFS= read -r file; do results+=("$file"); done < <(find "$output" -maxdepth 1 -name '*-result.json' | LC_ALL=C sort)
for file in "${results[@]}"; do
    key=${file##*/}; key=${key%-result.json}
    jq --arg key "$key" '{($key):.}' "$file"
done | jq -S -s add > "$output/actual.json"
jq -n --slurpfile expected "$root/expected/fulfillment-cycles.json" --args \
    '{checks:($expected[0]|length), passed:(($expected[0]|length)-($ARGS.positional|length)),failed:$ARGS.positional}' \
    -- "${failures[@]}" > "$output/summary.json"
if ((${#failures[@]})); then
    printf 'Fulfillment cycles: %s checks failed; artifacts: %s\n' "${#failures[@]}" "$output"
    exit 1
fi
