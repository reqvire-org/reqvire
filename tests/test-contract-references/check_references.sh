#!/usr/bin/env bash
# Contract-reference acceptance cases, each in its own disposable Git worktree.
set -Eeuo pipefail
suite=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
output=$TEST_DIR/output
binary=$REQVIRE_BIN
model=specifications/Model.md
target=$model#error-response-specification
reference=$'#### Contract References\n  * [Error Response Specification](#error-response-specification)\n\n'
mkdir -p "$output"
: > "$output/checks.txt"
: > "$output/failures.txt"
run() {
    local expected=0 executable=$binary rc=0
    if [[ "$1" == --reject ]]; then expected=1; shift; fi
    case "$1" in add|rm|mv|rename|merge|mv-file|mv-folder|link|unlink|relink|mv-asset|rm-asset|format) executable=$MCP_OPERATION_BIN ;; esac
    (cd "$workspace"; timeout 40 "$executable" "$@") > "$case_output/last.stdout" 2> "$case_output/last.stderr" || rc=$?
    if ((rc==124 || rc==137 || (expected==0 && rc!=0) || (expected==1 && rc==0))); then
        printf 'Unexpected exit %s: %s\n' "$rc" "$*" >&2
        cat "$case_output/last.stdout" "$case_output/last.stderr" >&2
        return 1
    fi
    cat "$case_output/last.stdout"
    if ((expected)); then cat "$case_output/last.stderr"; fi
}
data() {
    local command=$1
    case "$command" in model|containment) run "$@" ;; *) run "$@" --json ;; esac
}
row() { jq --arg name "$1" 'first(..|objects|select(.name==$name)) // error("Missing element: "+$name)'; }
assert_refs() {
    local name=$1 expected=$2
    data model | row "$name" | jq -e --argjson expected "$expected" '(.contract_references//[])==$expected' >/dev/null
}
coverage() {
    data coverage | jq --arg name "${1:-Contract Owner}" 'first((.covered_requirements.files[][],.uncovered_requirements.files[][])|select(.name==$name)) // error("Missing requirement")'
}
replace() {
    jq -Rs -j --arg old "$1" --arg new "$2" '
      index($old) as $at | if $at==null then error("Missing fixture text: "+$old)
      else .[:$at]+$new+.[($at+($old|length)):] end
    ' "$workspace/$model" > "$case_output/replaced.txt"
    cp "$case_output/replaced.txt" "$workspace/$model"
}
block() {
    jq -Rs -j --arg name "$1" '
      index("### "+$name+"\n") as $start |
      if $start==null then error("Missing fixture element") else
        .[$start:]|index("\n---") as $end | .[:$end] end
    ' "$workspace/$model"
}
dependency() {
    local source=$1 contract=$2 section=${3:-Contract References} old new slug
    old=$(block "$source")
    slug=${contract,,}; slug=${slug// /-}
    new=${old/'#### Relations'/$'#### '"$section"$'\n  * ['"$contract"'](#'"$slug"$')\n\n#### Relations'}
    replace "$old" "$new"
}
input_data() { jq -j --arg key "$1" '.[$key]' "$suite/reference-cases.json" > "$case_output/input.txt"; }
snapshot() {
    (cd "$workspace"; find . -type d -name .git -prune -o -type f -name '*.md' -print0 | LC_ALL=C sort -z | xargs -0 -r sha256sum)
}
before() { snapshot > "$case_output/before.txt"; }
unchanged() { snapshot > "$case_output/after.txt"; diff -u "$case_output/before.txt" "$case_output/after.txt"; }
commit() {
    git -C "$workspace" add .
    git -C "$workspace" -c commit.gpgSign=false commit -qm Fixture
}
parsed() {
    run validate >/dev/null
    assert_refs Documentation '["specifications/Model.md#error-response-specification"]'
    data model | row Documentation | jq -e '.contract_bindings==[]' >/dev/null
}
target_type() { replace '  * type: specification' "  * type: $1"; parsed; }
invalid_target() {
    replace "$reference" "${reference/'#error-response-specification'/$1}"
    run --reject validate | jq -e -Rs 'ascii_downcase|contains("reference")' >/dev/null
}
invalid_source() {
    replace $'### Documentation\n\nThe system SHALL document error responses.\n\n#### Metadata\n  * type: requirement' $'### Documentation\n\nThe system SHALL document error responses.\n\n#### Metadata\n  * type: capability'
    run --reject validate | jq -e -Rs 'ascii_downcase|contains("contract_reference") or contains("contract references")' >/dev/null
}
invalid_reference() {
    case "$1" in
        unowned) replace $'  * definedBy: [Error Response Specification](#error-response-specification)\n' '' ;;
        duplicate) replace "$reference" "${reference%$'\n\n'}"$'\n  * [Duplicate](#error-response-specification)\n\n' ;;
        binding) replace "$reference" "$reference${reference/'Contract References'/'Contract Bindings'}" ;;
        mixed) replace "$reference" "$reference"$'#### Contract Bindings\n  * [Other Specification](#other-specification)\n\n' ;;
    esac
    run --reject validate >/dev/null
}
coverage_isolation() {
    parsed
    coverage | jq -e '
      .coverage_source=="uncovered" and .is_terminal==true and .evidence==[] and .contributing_requirements==[] and
      .blocking_requirements==["specifications/Model.md#contract-owner"] and
      .aggregate_terminal_requirements==1 and .aggregate_covered_terminal_requirements==0' >/dev/null
    replace $'The system SHALL implement endpoint errors.\n' $'The system SHALL implement endpoint errors.\n\n#### Contract Bindings\n  * [Error Response Specification](#error-response-specification)\n'
    coverage | jq -e '.coverage_source=="contract_consumer_rollup" and .contributing_requirements==["specifications/Model.md#endpoint-implementation"]' >/dev/null
}
adapter_obligation() {
    input_data adapter_output_obligation
    cat "$case_output/input.txt" >> "$workspace/$model"
    run validate >/dev/null
    coverage | jq -e '.coverage_source=="uncovered" and .is_terminal==true and .evidence==[] and .contributing_requirements==[]' >/dev/null
    coverage 'Trace Adapter' | jq -e '.coverage_source=="requirement_rollup" and .contributing_requirements==["specifications/Model.md#adapter-output"]' >/dev/null
    coverage 'Other Contract Owner' | jq -e '.coverage_source=="contract_consumer_rollup" and .contributing_requirements==["specifications/Model.md#adapter-output"]' >/dev/null
}
collection() {
    data collect Documentation | row 'Error Response Specification' | jq -e '.source_type=="contract_reference_element" and .content=="Error responses contain a code and message."' >/dev/null
}
projections() {
    for command in search containment; do data "$command" | row Documentation | jq -e '.contract_references|length>0' >/dev/null; done
    run export --output site >/dev/null
    jq -Rs 'sub("^window\\.reqvireProjectStore\\s*=\\s*";"")|sub(";\\s*$";"")|fromjson' "$workspace/site/assets/project-store.js" > "$case_output/store.json"
    jq -e --arg target "$target" '
      [.contract_references[]|[.source_id,.target]]==[["specifications/Model.md#documentation",$target]] and
      any(.knowledge_graph.edges[];.kind=="contract_references" and .source=="specifications/Model.md#documentation" and .target==$target) and
      .contract_bindings==[]' "$case_output/store.json" >/dev/null
}
rdf() {
    run semantic export --layer model | jq -e -Rs 'contains("referencesContract") and contains("contractReferencedBy") and contains("contractReferencesTargetIdentifier") and (contains("bindsContract")|not)' >/dev/null
}
impact() {
    replace 'Error responses contain a code and message.' 'Error responses contain a code, message, and details.'
    data change-impact | jq -e 'tostring|contains("contract_references") and contains("Documentation Test") and contains("evidence/documentation.txt")' >/dev/null
}
reverse_impact() {
    parsed
    replace 'The system SHALL document error responses.' 'The system SHALL document all error responses.'
    data change-impact | jq -e 'all(..|objects;.name!="Contract Owner" and .name!="Error Response Specification")' >/dev/null
}
same_hierarchy() { replace '  * specify: [Contract Consumer](#contract-consumer)' '  * derivedFrom: [Contract Owner](#contract-owner)'; parsed; }
link_unlink() {
    replace "$reference" ''
    before
    run link Documentation referenceContract 'Error Response Specification' --dry-run | jq -e -Rs 'contains("Contract References")' >/dev/null
    unchanged
    run link Documentation referenceContract 'Error Response Specification' >/dev/null
    assert_refs Documentation '["specifications/Model.md#error-response-specification"]'
    diff -u "$suite/expected/linked.md" "$workspace/$model"
    run unlink Documentation 'Error Response Specification' >/dev/null
    assert_refs Documentation '[]'
    diff -u "$suite/expected/unlinked.md" "$workspace/$model"
}
invalid_mutation() {
    before
    run --reject link Documentation referenceContract 'Contract Owner' >/dev/null
    unchanged
    run link 'Endpoint Implementation' referenceContract 'Other Specification' >/dev/null
    assert_refs 'Endpoint Implementation' '["specifications/Model.md#other-specification"]'
}
mixed_mutations() {
    before
    run --reject link Documentation bindContract "$model#other-specification" | jq -e -Rs 'contains("Contract References") and contains("Contract Bindings")' >/dev/null
    unchanged
    run link 'Endpoint Implementation' bindContract "$model#other-specification" >/dev/null
    before
    run --reject link 'Endpoint Implementation' referenceContract 'Error Response Specification' | jq -e -Rs 'contains("Contract References") and contains("Contract Bindings")' >/dev/null
    unchanged
}
rename_move() {
    run rename 'Error Response Specification' 'Error Schema' >/dev/null
    assert_refs Documentation '["specifications/Model.md#error-schema"]'
    run mv 'Error Schema' specifications/Contracts.md >/dev/null
    assert_refs Documentation '["specifications/Contracts.md#error-schema"]'
    run mv Documentation docs/Requirements.md >/dev/null
    assert_refs Documentation '["specifications/Contracts.md#error-schema"]'
    run mv-file specifications/Contracts.md specifications/Shared.md >/dev/null
    assert_refs Documentation '["specifications/Shared.md#error-schema"]'
    run mv-folder specifications model >/dev/null
    assert_refs Documentation '["model/Shared.md#error-schema"]'
    run validate >/dev/null
    mkdir -p "$workspace/nested"
    for name in Model.md nested/Model.md; do cp "$suite/fixtures/same-basename/$name" "$workspace/$name"; done
    run validate >/dev/null
    before
    run rename 'Collision Specification' 'Renamed Collision Specification' --dry-run --json | jq -e 'tostring|contains("../Model.md#renamed-collision-specification")' >/dev/null
    unchanged
    run rename 'Collision Specification' 'Renamed Collision Specification' >/dev/null
    for name in Model.md nested/Model.md; do diff -u "$suite/expected/same-basename/$name" "$workspace/$name"; done
    assert_refs 'Collision Reference Consumer' '["Model.md#renamed-collision-specification"]'
    run validate >/dev/null
}
remove_target() { parsed; run rm 'Error Response Specification' >/dev/null; assert_refs Documentation '[]'; run validate >/dev/null; }
format_references() {
    replace "$reference" "${reference/'[Error Response Specification](#error-response-specification)'/'[Wrong label](./Model.md#error-response-specification)'}"
    run format --fix >/dev/null
    assert_refs Documentation '["specifications/Model.md#error-response-specification"]'
    jq -e -Rs --arg reference "$reference" 'contains($reference)' "$workspace/$model" >/dev/null
    before; run format --fix >/dev/null; unchanged
}
single_contract() {
    jq -Rs -j '
      index("### Error Response Specification\n") as $start | index("### Other Contract Owner\n") as $end |
      .[:$start]+.[$end:] | split("[Error Response Specification](#error-response-specification)") |
      join("[Error Response Specification](Contract.md#error-response-specification)")' "$suite/$model" > "$workspace/$model"
    input_data single_contract
    cp "$case_output/input.txt" "$workspace/specifications/Contract.md"
    run validate >/dev/null
    assert_refs Documentation '["specifications/Contract.md#error-response-specification"]'
}
override_merge() {
    input_data override_and_merge
    run add "$model" < "$case_output/input.txt" >/dev/null
    assert_refs 'Additional Documentation' '["specifications/Model.md#other-specification"]'
    before
    sed '/^#### Relations$/i\#### Contract Bindings\n  * [Error Response Specification](#error-response-specification)\n' "$case_output/input.txt" > "$case_output/bad-input.txt"
    run --reject add "$model" --override < "$case_output/bad-input.txt" >/dev/null
    unchanged
    run merge Documentation 'Additional Documentation' >/dev/null
    assert_refs Documentation '["specifications/Model.md#error-response-specification","specifications/Model.md#other-specification"]'
    run link 'Endpoint Implementation' bindContract "$model#other-specification" >/dev/null
    before
    run --reject merge Documentation 'Endpoint Implementation' >/dev/null
    unchanged
}
pure_relocation() {
    parsed
    run mv 'Error Response Specification' specifications/Shared.md >/dev/null
    assert_refs Documentation '["specifications/Shared.md#error-response-specification"]'
    data change-impact | jq -e 'all(.changed|..|objects;.name!="Documentation")' >/dev/null
}
changed_references() {
    run link Documentation referenceContract 'Other Specification' >/dev/null
    data change-impact | jq '.changed' | row Documentation | jq -e '.changed_contract_references==["specifications/Model.md#other-specification"]' >/dev/null
    commit
    run unlink Documentation 'Other Specification' >/dev/null
    data change-impact | jq '.changed' | row Documentation | jq -e '.changed_contract_references==["specifications/Model.md#other-specification"]' >/dev/null
}
collect_once() {
    replace '  * specify: [Contract Consumer](#contract-consumer)' '  * derivedFrom: [Contract Owner](#contract-owner)'
    data collect Documentation | jq -e --arg target "$target" '
      [.items[]|select(.identifier==$target)] as $found | ($found|length)==1 and $found[0].file_path=="specifications/Model.md" and .metadata.total_items==(.items|length)' >/dev/null
}
descendant_impact() {
    input_data descendant_impact
    run add "$model" < "$case_output/input.txt" >/dev/null
    commit
    replace 'Error responses contain a code and message.' 'Error responses contain a code, message, and details.'
    data change-impact | jq -e 'tostring|contains("Detailed Documentation")' >/dev/null
}
valid_override() { input_data valid_override; run add "$model" --override < "$case_output/input.txt" >/dev/null; assert_refs Documentation '["specifications/Model.md#other-specification"]'; }
relation_conflict() {
    replace '  * satisfiedBy: [documentation.txt]' $'  * definedBy: [Error Response Specification](#error-response-specification)\n  * satisfiedBy: [documentation.txt]'
    run --reject validate | jq -e -Rs 'contains("Contract References") and contains("Relations")' >/dev/null
}
circular_error() {
    jq -e -Rs '(ascii_downcase|contains("circular")) and contains("Contract References") and contains("#") and (ascii_downcase|contains("contract"))' >/dev/null
}
rejected_cycle() {
    run --reject validate > "$case_output/cycle-first.txt"
    circular_error < "$case_output/cycle-first.txt"
    run --reject validate > "$case_output/cycle-second.txt"
    diff -u "$case_output/cycle-first.txt" "$case_output/cycle-second.txt"
    run --reject coverage --json | circular_error
    run --reject change-impact --json | circular_error
}
own_documentation_contract() {
    replace '  * satisfiedBy: [documentation.txt]' $'  * definedBy: [Documentation Specification](#documentation-specification)\n  * satisfiedBy: [documentation.txt]'
    input_data own_documentation_contract
    cat "$case_output/input.txt" >> "$workspace/$model"
}
cycle() {
    case "$1" in
        reciprocal)
            dependency 'Contract Owner' 'Other Specification'
            dependency 'Other Contract Owner' 'Error Response Specification' ;;
        self) dependency 'Contract Owner' 'Error Response Specification' ;;
        long)
            own_documentation_contract
            dependency 'Contract Owner' 'Other Specification'
            dependency 'Other Contract Owner' 'Documentation Specification' ;;
        mixed)
            dependency 'Contract Owner' 'Other Specification'
            dependency 'Other Contract Owner' 'Error Response Specification' 'Contract Bindings' ;;
        hierarchy)
            dependency 'Contract Owner' 'Other Specification'
            old=$(block 'Other Contract Owner')
            replace "$old" "${old/'specify: [Contract Provider](#contract-provider)'/'derivedFrom: [Contract Owner](#contract-owner)'}" ;;
    esac
    rejected_cycle
}
dependent_body() {
    local body=$1 section=$2 contract=$3 slug
    slug=${contract,,}; slug=${slug// /-}
    printf '%s' "${body/'#### Relations'/$'#### '"$section"$'\n  * ['"$contract"'](#'"$slug"$')\n\n#### Relations'}"
}
atomic_cycle() {
    local command=$1
    dependency 'Contract Owner' 'Other Specification'
    run validate >/dev/null
    before
    case "$command" in
        link) run --reject link 'Other Contract Owner' referenceContract 'Error Response Specification' > "$case_output/cycle.txt" ;;
        dry-run) run --reject link 'Other Contract Owner' referenceContract 'Error Response Specification' --dry-run > "$case_output/cycle.txt" ;;
        binding-link) run --reject link 'Other Contract Owner' bindContract "$target" > "$case_output/cycle.txt" ;;
        override)
            dependent_body "$(block 'Other Contract Owner')" 'Contract References' 'Error Response Specification' > "$case_output/input.txt"
            run --reject add "$model" --override < "$case_output/input.txt" > "$case_output/cycle.txt" ;;
        hierarchy-edit)
            block 'Other Contract Owner' | sed 's/specify: \[Contract Provider\](#contract-provider)/derivedFrom: [Contract Owner](#contract-owner)/' > "$case_output/input.txt"
            run --reject add "$model" --override < "$case_output/input.txt" > "$case_output/cycle.txt" ;;
        merge) run --reject merge 'Contract Owner' 'Other Contract Owner' > "$case_output/cycle.txt" ;;
    esac
    circular_error < "$case_output/cycle.txt"
    unchanged
    run validate >/dev/null
}
relink_cycle() {
    own_documentation_contract
    dependency 'Other Contract Owner' 'Documentation Specification'
    run validate >/dev/null
    before
    run --reject relink Documentation referenceContract 'Error Response Specification' 'Other Specification' | circular_error
    unchanged
    run validate >/dev/null
}
create_cycle() {
    input_data create_cycle
    body=$(cat "$case_output/input.txt")
    before
    dependent_body "$body" 'Contract References' 'Error Response Specification' > "$case_output/bad-input.txt"
    run --reject add "$model" < "$case_output/bad-input.txt" | circular_error
    unchanged
    run add "$model" < "$case_output/input.txt" >/dev/null
    run validate >/dev/null
}
relink_reference() {
    before
    run relink Documentation referenceContract 'Error Response Specification' 'Other Specification' --dry-run >/dev/null
    unchanged
    run relink Documentation referenceContract 'Error Response Specification' 'Other Specification' >/dev/null
    assert_refs Documentation '["specifications/Model.md#other-specification"]'
    before
    run --reject relink Documentation referenceContract 'Other Specification' Documentation >/dev/null
    unchanged
}
invalid_mutations() {
    before
    run --reject link 'Contract Provider' referenceContract 'Error Response Specification' >/dev/null
    unchanged
    for mode in source target owner; do
        case "$mode" in
            source) block Documentation | sed 's/type: requirement/type: capability/' > "$case_output/input.txt" ;;
            target) block 'Error Response Specification' | sed 's/type: specification/type: capability/' > "$case_output/input.txt" ;;
            owner) block 'Contract Owner' | sed '/^  \* definedBy: \[Error Response Specification\](#error-response-specification)$/d' > "$case_output/input.txt" ;;
        esac
        run --reject add "$model" --override < "$case_output/input.txt" >/dev/null
        unchanged
    done
    run validate >/dev/null
}
override_contract() {
    block 'Error Response Specification' | sed 's/code and message\./code, message, and details./' > "$case_output/input.txt"
    before
    run add "$model" --override --dry-run < "$case_output/input.txt" >/dev/null
    unchanged
    run add "$model" --override < "$case_output/input.txt" >/dev/null
    assert_refs Documentation '["specifications/Model.md#error-response-specification"]'
    run validate >/dev/null
    data collect Documentation | row 'Error Response Specification' | jq -e '.content|contains("details")' >/dev/null
}
search_names() { data search "$@" | jq '[.files[].elements[].name]|sort'; }
reference_search() {
    search_names --has-contract-references | jq -e '.==["Documentation"]' >/dev/null
    search_names --filter-contract-references '*#error-response-*' | jq -e '.==["Documentation"]' >/dev/null
    search_names --filter-contract-references "$target" --short | jq -e '.==["Documentation"]' >/dev/null
    run search --has-contract-references --short | jq -e -Rs 'contains("Documentation") and (contains("Endpoint Implementation")|not)' >/dev/null
}
search_composition() { search_names --has-contract-references --have-relations satisfiedBy --filter-name '^Documentation$' | jq -e '.==["Documentation"]' >/dev/null; }
search_empty() {
    search_names --filter-contract-references '*#absent' | jq -e '.==[]' >/dev/null
    search_names --has-contract-references --filter-name '^Contract Owner$' | jq -e '.==[]' >/dev/null
}
separate_filters() {
    run link 'Endpoint Implementation' bindContract "$target" >/dev/null
    search_names --has-contract-references | jq -e '.==["Documentation"]' >/dev/null
    search_names --has-contract-bindings | jq -e '.==["Endpoint Implementation"]' >/dev/null
    search_names --has-contract-references --has-contract-bindings | jq -e '.==[]' >/dev/null
}
invalid_glob() { run --reject search --filter-contract-references '[' | jq -e -Rs 'ascii_downcase|contains("glob")' >/dev/null; }
invalid_relation_filters() {
    for flag in --have-relations --not-have-relations; do
        for keyword in referenceContract bindContract; do
            run --reject search "$flag" "$keyword" | jq -e -Rs 'contains("Invalid relation type") and contains("Valid types")' >/dev/null
        done
    done
}
expected_files() {
    local expected_root=$suite/expected/$1
    (cd "$expected_root"; find . -type f -name '*.md' -print | LC_ALL=C sort) > "$case_output/expected-files.txt"
    [[ -s "$case_output/expected-files.txt" ]]
    (cd "$workspace"; find . -type d -name .git -prune -o -type f -name '*.md' -print | LC_ALL=C sort) > "$case_output/actual-files.txt"
    diff -u "$case_output/expected-files.txt" "$case_output/actual-files.txt"
    while IFS= read -r name; do diff -u "$expected_root/$name" "$workspace/$name"; done < "$case_output/expected-files.txt"
}
owner_type_override() {
    local extra=()
    [[ "$1" != dry-run ]] || extra=(--dry-run)
    run validate >/dev/null
    before
    block 'Contract Owner' | sed 's/type: requirement/type: capability/;s/specify: \[Contract Provider\]/derivedFrom: [Contract Provider]/' > "$case_output/input.txt"
    run --reject add "$model" --override "${extra[@]}" < "$case_output/input.txt" | jq -e -Rs 'contains("Contract Owner") and contains("Error Response Specification")' >/dev/null
    unchanged
    run validate >/dev/null
}
cross_file_override() {
    local mode=$1 extra=()
    [[ "$mode" != dry-run ]] || extra=(--dry-run)
    run validate >/dev/null
    before
    block 'Error Response Specification' > "$case_output/input.txt"
    run add specifications/Other.md --override --json "${extra[@]}" < "$case_output/input.txt" > "$case_output/result.json"
    if [[ "$mode" == dry-run ]]; then
        unchanged
        run validate >/dev/null
        jq -e --arg model "$model" '
          [.diffs[]|select(.file_path==$model)|.lines[]|select(.color=="green")|.content|ltrimstr("+")|gsub("^\\s+|\\s+$";"")] as $added |
          all(["* [Error Response Specification](Other.md#error-response-specification)","* definedBy: [Error Response Specification](Other.md#error-response-specification)"][];
            . as $line|$added|index($line)!=null)' "$case_output/result.json" >/dev/null
    else
        assert_refs Documentation '["specifications/Other.md#error-response-specification"]'
        run validate >/dev/null
        expected_files cross-file-override
    fi
}
same_owner_merge() {
    local mode=$1 extra=()
    [[ "$mode" != dry-run ]] || extra=(--dry-run)
    replace $'  * definedBy: [Other Specification](#other-specification)\n' ''
    replace $'  * definedBy: [Error Response Specification](#error-response-specification)\n' $'  * definedBy: [Error Response Specification](#error-response-specification)\n  * definedBy: [Other Specification](#other-specification)\n'
    replace "$reference" "${reference%$'\n\n'}"$'\n  * [Other Specification](#other-specification)\n\n'
    run validate >/dev/null
    before
    run merge 'Error Response Specification' 'Other Specification' --json "${extra[@]}" > "$case_output/result.json"
    if [[ "$mode" == dry-run ]]; then
        unchanged
        run validate >/dev/null
        jq -e 'any(.diffs[].lines[];.color=="red" and (.content|ltrimstr("-")|gsub("^\\s+|\\s+$";""))=="### Other Specification")' "$case_output/result.json" >/dev/null
    else
        assert_refs Documentation '["specifications/Model.md#error-response-specification"]'
        run validate >/dev/null
        expected_files merged-contracts
    fi
}
normalized_reference() { replace "$reference" "${reference/'#error-response-specification'/$1}"; parsed; }
duplicate_normalized() {
    replace "$reference" "${reference%$'\n\n'}"$'\n  * [Same contract](#Error-Response-Specification)\n\n'
    before
    run --reject validate | jq -e -Rs 'ascii_downcase|contains("duplicate") and contains("reference")' >/dev/null
    unchanged
}
normalized_command() {
    local href=$1
    replace "$reference" ''
    run validate >/dev/null
    before
    run link Documentation referenceContract "$href" --dry-run >/dev/null
    unchanged
    run link Documentation referenceContract "$href" >/dev/null
    parsed
    diff -u "$suite/expected/linked.md" "$workspace/$model"
}
convergent_impact() {
    replace '  * specify: [Contract Consumer](#contract-consumer)' '  * derivedFrom: [Contract Owner](#contract-owner)'
    run validate >/dev/null
    commit
    replace 'Error responses contain a code and message.' 'Error responses contain a code, message, and details.'
    data change-impact | jq -e '
      [.changed[].name]==["Error Response Specification"] and
      [.invalidated_verifications[].target_text]==["Documentation Test"] and
      (.changed[0].change_impact_tree as $tree |
        any($tree|..|objects;(.identifier//""|endswith("/evidence/documentation.txt"))) and
        ([$tree|..|objects|select(has("contract_references"))|.contract_references] as $edges |
          [$edges[].name]==["Documentation"] and
          ($edges[0].identifier|endswith("/specifications/Model.md#documentation"))))' >/dev/null
}
failures=0
check() {
    local label=$1 status=0
    shift
    case_output=$output/cases/$label
    workspace=$case_output/repo
    mkdir -p "$workspace"
    (set -e
        trap 'printf "Assertion failed at line %s\n" "$LINENO" >&2' ERR
        cp -a "$suite/specifications" "$suite/evidence" "$workspace/"
        printf 'site/\n' > "$workspace/.reqvireignore"
        git -C "$workspace" init -q
        git -C "$workspace" config user.name 'Contract reference tests'
        git -C "$workspace" config user.email tests@example.com
        git -C "$workspace" remote add origin https://example.com/model.git
        commit
        "$@"
    ) > "$case_output/check.log" 2>&1 &
    local pid=$!
    wait "$pid" || status=$?
    if ((status)); then
        failures=$((failures+1))
        printf 'FAIL %s\n' "$label" | tee -a "$output/checks.txt"
        { printf '%s:\n' "$label"; cat "$case_output/check.log"; } | tee -a "$output/failures.txt"
    else printf 'PASS %s\n' "$label" >> "$output/checks.txt"; fi
}
check parse-reference parsed
for kind in source constraint behavior specification state input-output; do check "target-type-$kind" target_type "$kind"; done
check reject-target-missing invalid_target '#missing'
check reject-target-file invalid_target ../evidence/endpoint.txt
check reject-target-url invalid_target https://example.com/spec#schema
check reject-target-requirement invalid_target '#contract-owner'
check reject-target-capability invalid_target '#contract-provider'
check reject-target-verification invalid_target '#documentation-test'
check reject-source invalid_source
check reject-unowned invalid_reference unowned
check reject-duplicate invalid_reference duplicate
check reject-binding-conflict invalid_reference binding
check reject-mixed-distinct-targets invalid_reference mixed
check coverage-isolation coverage_isolation
check coverage-adapter-reference-output-binding adapter_obligation
check collect-reference collection
check public-projections projections
check semantic-projection rdf
check contract-impact impact
check impact-direction reverse_impact
check same-hierarchy same_hierarchy
check link-unlink-dry-run link_unlink
check atomic-invalid-link invalid_mutation
check atomic-mixed-sections mixed_mutations
check rename-move-preservation rename_move
check remove-target remove_target
check format-reference format_references
check single-element-contract single_contract
check create-override-merge override_merge
check reference-relocation pure_relocation
check reference-membership-impact changed_references
check collection-deduplication collect_once
check reference-descendant-impact descendant_impact
check valid-reference-override valid_override
check reject-relation-reference-conflict relation_conflict
check reject-reference-cycle cycle reciprocal
check reject-reference-self-cycle cycle self
check reject-long-reference-cycle cycle long
check reject-reference-binding-cycle cycle mixed
check reject-reference-hierarchy-cycle cycle hierarchy
for command in link dry-run binding-link override hierarchy-edit merge; do check "atomic-reference-cycle-$command" atomic_cycle "$command"; done
check atomic-reference-cycle-relink relink_cycle
check atomic-reference-cycle-create create_cycle
check reference-relink-success-dry-run-invalid relink_reference
check atomic-invalid-source-target-ownership-edits invalid_mutations
check override-referenced-contract-retains-consumers override_contract
check reference-search-presence-target reference_search
check reference-search-composition search_composition
check reference-search-empty search_empty
check reference-search-binding-separation separate_filters
check reference-search-invalid-glob invalid_glob
check reject-dependency-keyword-relation-filter invalid_relation_filters
for mode in apply dry-run; do check "reject-reference-owner-type-$mode" owner_type_override "$mode"; done
for mode in apply dry-run; do check "cross-file-reference-override-$mode" cross_file_override "$mode"; done
for mode in apply dry-run; do check "merge-referenced-contracts-$mode" same_owner_merge "$mode"; done
check normalize-authored-reference-fragment normalized_reference '#Error-Response-Specification'
check normalize-authored-reference-file-qualified normalized_reference 'Model.md#Error-Response-Specification'
check reject-normalized-reference-duplicate duplicate_normalized
check normalize-reference-command-fragment normalized_command '#Error-Response-Specification'
check normalize-reference-command-file-qualified normalized_command 'Model.md#Error-Response-Specification'
check convergent-impact-preserves-reference-edge convergent_impact
if ((failures)); then printf '%s failed\n' "$failures"; exit 1; fi
