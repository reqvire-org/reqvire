#!/usr/bin/env bash
# CLI acceptance checks for native SPARQL artifacts and their model context.
set -euo pipefail
binary=$1
suite=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$2"
cases=$suite/query-cases.json
fixture=specifications/Queries.md
mkdir -p output
cp "$fixture" output/initial-query-model.txt
trap 'printf "FAIL semantic-query check at line %s\n" "$LINENO" >&2' ERR
run() {
    local expected=0 executable=$binary rc=0
    if [[ "$1" == --reject ]]; then expected=1; shift; fi
    case "$1" in add|rm|mv|rename|merge|mv-file|mv-folder|link|unlink|relink|mv-asset|rm-asset|format) executable=$MCP_OPERATION_BIN ;; esac
    "$executable" "$@" > output/command.stdout 2> output/command.stderr || rc=$?
    if { ((expected==0 && rc!=0)); } || { ((expected==1 && rc==0)); }; then
        printf 'Unexpected exit %s for %s\n' "$rc" "$*" >&2
        cat output/command.stdout output/command.stderr >&2
        return 1
    fi
    cat output/command.stdout
}
restore() { cp output/initial-query-model.txt "$fixture"; }
load() {
    jq -j "$1" "$cases" > output/case.txt
    data=$(cat output/case.txt; printf .)
    data=${data%.}
}
query() {
    printf '### Portable Query\n\nPortable query purpose.\n\n#### Query\n```sparql\n%s\n```\n%s\n#### Metadata\n  * type: semantic-query\n\n#### Relations\n' "$1" "${2:-}"
    if [[ "${3:-true}" == true ]]; then printf '  * use: [Item Ontology](#item-ontology)\n'; fi
}
append() { restore; printf '\n' >> "$fixture"; query "$@" >> "$fixture"; }
export_query() { run semantic query export --name "${1:-Portable Query}" --json; }
json_equal() { jq -S . "$1" > output/equal-left.json; jq -S . "$2" > output/equal-right.json; diff -u output/equal-left.json output/equal-right.json; }
hash() { sha256sum "$1" | cut -d ' ' -f 1; }

run semantic query list --json > output/records.json
jq -e 'length==1 and .[0].query_form=="SELECT" and .[0].iri=="urn:reqvire:semantic-query:item-lookup"' output/records.json >/dev/null
iri=$(jq -r '.[0].iri' output/records.json)
run semantic query list --namespace-base 'https://example.org/items#' --json > output/selected.json
json_equal output/records.json output/selected.json
run semantic query list --namespace-base 'https://unrelated.example' --json | jq -e '.==[]' >/dev/null
run collect 'Item Lookup' --json > output/collected.json
jq -e 'tostring|contains("Item Ontology") and contains("SELECT ?item")' output/collected.json >/dev/null
run collect 'Item Ontology' --direction DOWNSTREAM --json | jq -e 'tostring|contains("Item Lookup")' >/dev/null
jq -e --argjson line "$(awk '$0=="```sparql" {print NR;exit}' output/initial-query-model.txt)" '.[0].line_number==$line' output/records.json >/dev/null
printf 'PASS discovery-and-context-filter\n'

export_query 'Item Lookup' > output/artifact.json
run semantic query export --iri "$iri" > output/raw-query.txt
jq -e --rawfile raw output/raw-query.txt --arg hash "$(hash output/raw-query.txt)" '.content==$raw and .sha256==$hash' output/artifact.json >/dev/null
run semantic query export --name 'Item Lookup' --output output/query.sparql >/dev/null
diff -u output/raw-query.txt output/query.sparql
run semantic query check --name 'Item Lookup' --artifact output/query.sparql >/dev/null
printf 'stale\n' > output/query.sparql
cp output/query.sparql output/stale-before.txt
run --reject semantic query check --name 'Item Lookup' --artifact output/query.sparql --json > output/stale.json
jq -e --arg hash "$(hash output/stale-before.txt)" '.status=="stale" and .actual_sha256==$hash' output/stale.json >/dev/null
diff -u output/stale-before.txt output/query.sparql
run --reject semantic query check --name 'Item Lookup' --artifact output/missing --json | jq -e '.status=="missing" and .actual_sha256==null' >/dev/null
run --reject semantic query check --name 'Item Lookup' --artifact output --json > output/unreadable.txt
jq -e -Rs 'contains("\"status\": \"missing\"")|not' output/unreadable.txt >/dev/null
jq -e -Rs 'contains("\"status\": \"stale\"")|not' output/unreadable.txt >/dev/null
run --reject semantic query export --name 'Item Lookup' --output output >/dev/null
diff -u output/stale-before.txt output/query.sparql
run --reject semantic query export --name Missing >/dev/null
run --reject semantic query export --name 'Item Lookup' --iri "$iri" >/dev/null
run --reject semantic query export --name 'Item Lookup' --json --output output/query.sparql >/dev/null
run --reject semantic query export >/dev/null
diff -u output/stale-before.txt output/query.sparql
printf 'PASS artifacts-selection-and-drift\n'

for ((i=0;i<$(jq '.valid|length' "$cases");i++)); do
    form=$(jq -r ".valid[$i][0]" "$cases")
    load ".valid[$i][1]"; text=$data
    load ".valid[$i][2]"; produces=$data
    append $'PREFIX item: <https://example.org/items#>\n'"$text" "$produces"
    run validate >/dev/null
    export_query > output/portable.json
    jq -e --arg form "$form" '.query_form==$form' output/portable.json >/dev/null
    if [[ -n "$produces" ]]; then jq -e '.materializes_properties==["https://example.org/items#name"]' output/portable.json >/dev/null; fi
done
printf 'PASS four-forms-and-portable-runtime-features\n'
for ((i=0;i<$(jq '.invalid|length' "$cases");i++)); do
    load ".invalid[$i][0]"; text=$data
    load ".invalid[$i][1]"
    append $'PREFIX item: <https://example.org/items#>\n'"$text" "$data"
    run --reject validate >/dev/null
done
for text in 'ASK { ?s a item:Item }' 'ASK { <relative> ?p ?o }'; do append "$text"; run --reject validate >/dev/null; done
append 'ASK {}' '' false
run --reject validate >/dev/null
restore
printf 'PASS syntax-schema-role-and-produces-rejections\n'

cp "$fixture" output/expected-before.txt
query 'ASK { ?s a <https://example.org/items#Missing> }' > output/add.txt
run --reject add "$fixture" < output/add.txt >/dev/null
diff -u output/expected-before.txt "$fixture"
query 'ASK {}' > output/add.txt
run add "$fixture" < output/add.txt >/dev/null
cp "$fixture" output/expected-before.txt
query 'ASK { ?s a <https://example.org/items#Missing> }' > output/add.txt
run --reject add "$fixture" --override < output/add.txt >/dev/null
diff -u output/expected-before.txt "$fixture"
restore
printf 'PASS atomic-add-and-override\n'

load .document
append "$data"
jq -j .document "$cases" > output/document.txt
export_query > output/before-format.json
jq -j .content output/before-format.json > output/exact-before.txt
diff -u output/document.txt output/exact-before.txt
run format --fix >/dev/null
export_query > output/after-format.json
jq -j .content output/after-format.json > output/exact-after.txt
diff -u output/document.txt output/exact-after.txt
jq -e --slurpfile before output/before-format.json '.sha256==$before[0].sha256' output/after-format.json >/dev/null
sed 's/Portable query purpose\./Updated purpose./' "$fixture" > output/purpose.txt
cp output/purpose.txt "$fixture"
export_query | jq -e --slurpfile before output/before-format.json '.sha256==$before[0].sha256 and .purpose!=$before[0].purpose' >/dev/null
restore
printf 'PASS-exact-bytes-and-metadata-independent-hash\n'
run semantic export --layer queries --namespace-base 'https://example.org/items' | jq -e -Rs --arg iri "$iri" 'contains($iri) and contains("queryText")' >/dev/null
run semantic export --layer queries --namespace-base 'https://unrelated.example' | jq -e -Rs --arg iri "$iri" 'contains($iri)|not' >/dev/null
printf 'PASS-query-rdf-layer\n'
append 'ASK { ?s a <https://example.org/items#Missing> }'
run --reject semantic query validate --json | jq -e '.valid==false and (.queries|length)==2 and any(.queries[];.diagnostics|length>0) and (.model_errors|length>0)' >/dev/null
restore
printf 'PASS-candidate-validation-diagnostics\n'

load .construct
append "$data" $'\n#### Produces\n  * property: item:name\n'
jq -Rs -j '
  split("---")[0]|split("# Elements\n\n")[1]|
  split("https://example.org/items")|join("https://example.org/new-items")|
  split("ontology_prefix: item")|join("ontology_prefix: newitem")|
  split("@prefix item:")|join("@prefix newitem:")|
  split("item:Item")|join("newitem:Item")|split("item:name")|join("newitem:name")
' output/initial-query-model.txt > output/rebased-ontology.txt
run add "$fixture" --override < output/rebased-ontology.txt >/dev/null
export_query > output/rebased-query.json
jq -e '
  (.content|contains("PREFIX item: <https://example.org/new-items#>") and
   contains("# <https://example.org/items#name>") and
   contains("\"\"\"<https://example.org/items#name>\n#### Produces\n  * property: item:name\n\"\"\"")) and
  .materializes_properties==["https://example.org/new-items#name"]
' output/rebased-query.json >/dev/null
jq -e -Rs 'contains("* property: newitem:name")' "$fixture" >/dev/null
run validate >/dev/null
restore
printf 'PASS-ontology-rebase-query-and-produces\n'

load .child; child=$data
{ cat output/initial-query-model.txt; printf '%s' "$child"; query $'PREFIX item: <https://example.org/items#>\nASK { ?s a item:Child }'; } > "$fixture"
run --reject validate >/dev/null
{ cat output/initial-query-model.txt; printf '%s' "$child"; query $'PREFIX item: <https://example.org/items#>\nASK { ?s a item:Item, item:Child }' | sed 's/use: \[Item Ontology\](#item-ontology)/use: [Child Vocabulary](#child-vocabulary)/'; } > "$fixture"
run validate >/dev/null
restore
printf 'PASS-ontology-context-ancestor-boundary\n'

jq -j .external_ttl "$cases" > specifications/external.ttl
jq -Rs -j --slurpfile cases "$cases" 'sub("---";$cases[0].ext_section+"---")' output/initial-query-model.txt > output/with-external.txt
cp output/with-external.txt "$fixture"
run semantic query list --json | jq -e 'length==1' >/dev/null
{
    cat output/with-external.txt; printf '\n'
    query $'PREFIX ext: <https://imports.example/vocab#>\nASK { ?s a ext:Record ; ext:value ?v }'
    printf '\n---\n'
    query 'CONSTRUCT {} WHERE {}' $'\n#### Produces\n  * property: ext:output\n' | sed 's/### Portable Query/### External Output/'
} > "$fixture"
run validate >/dev/null
run semantic export --layer external-used | jq -e -Rs 'contains("Record") and contains("value") and contains("output") and (contains("Unused")|not)' >/dev/null
append $'PREFIX ext: <https://imports.example/vocab#>\nASK { ?s a ext:Record }'
run --reject validate >/dev/null
restore
rm specifications/external.ttl
printf 'PASS-imported-vocabulary-and-native-provenance\n'
jq -Rs -j 'split("  * use: [Item Ontology](#item-ontology)\n")|join("")|sub("---";"#### Relations\n  * usedBy: [Item Lookup](#item-lookup)\n---")' output/initial-query-model.txt > "$fixture"
run validate >/dev/null
run semantic query list --json | jq -e '.[0].namespaces==["https://example.org/items#"]' >/dev/null
restore
printf 'PASS-inverse-usedBy-context\n'
for ((i=0;i<$(jq '.runtime_queries|length' "$cases");i++)); do load ".runtime_queries[$i]"; append "$data"; run validate >/dev/null; done
restore
printf 'PASS-runtime-datasets-base-and-literal-headings\n'

query 'ASK {}' > output/good-query.txt
for mode in language empty missing duplicate protected purpose; do
    case "$mode" in
        language) sed 's/```sparql/```turtle/' output/good-query.txt > output/bad-query.txt ;;
        empty) query '' > output/bad-query.txt ;;
        missing) sed 's/#### Query/#### Other/' output/good-query.txt > output/bad-query.txt ;;
        duplicate) { cat output/good-query.txt; printf '\n#### Query\n```sparql\nASK {}\n```\n'; } > output/bad-query.txt ;;
        protected) sed '/^  \* type: semantic-query$/a\  * query_form: ASK' output/good-query.txt > output/bad-query.txt ;;
        purpose) { cat output/good-query.txt; printf '\n#### Purpose\nHidden purpose\n'; } > output/bad-query.txt ;;
    esac
    { cat output/initial-query-model.txt; printf '\n'; cat output/bad-query.txt; } > "$fixture"
    run --reject validate >/dev/null
    run --reject semantic query export --name 'Portable Query' --output output/forbidden.sparql >/dev/null
    [[ ! -e output/forbidden.sparql ]]
done
{ cat output/initial-query-model.txt; printf '\n### Unrelated Requirement\n\nThe system SHALL perform a task.\n\n#### Metadata\n  * type: requirement\n\n#### Produces\n  * property: item:name\n'; } > "$fixture"
run --reject validate >/dev/null
restore
printf 'PASS-query-grammar-and-export-failure-atomicity\n'

jq -Rs -j 'split("item:name a owl:DatatypeProperty ; rdfs:domain item:Item .")|join("item:name a owl:DatatypeProperty ; rdfs:domain item:Item .\nitem:annotation a owl:AnnotationProperty .\n<https://www.reqvire.org/ontology#RelationFamily> a owl:Class .\nitem:family a <https://www.reqvire.org/ontology#RelationFamily> .")' output/initial-query-model.txt > "$fixture"
{ printf '\n'; query 'CONSTRUCT {} WHERE {}' $'\n#### Produces\n  * property: item:name\n  * property: item:annotation\n  * property: rdfs:label\n  * family: item:family\n'; } >> "$fixture"
export_query > output/family-record.json
jq -e '(.materializes_properties|length)==3 and .materializes_families==["https://example.org/items#family"]' output/family-record.json >/dev/null
run export --output output/site >/dev/null
jq -Rs 'sub("^window\\.reqvireProjectStore\\s*=\\s*";"")|sub(";\\s*$";"")|fromjson|.ontology.graph_data' output/site/assets/project-store.js > output/graph.json
jq -e --slurpfile record output/family-record.json '
  . as $graph | [.nodes[]|select(.semantic_type=="semantic-query")] as $native |
  ($native|length)==2 and any($native[];
    . as $node | .label=="Portable Query" and .source_kind=="query" and .query.form=="CONSTRUCT" and
    .query.text==$record[0].content and .sources[0].link=="#/content/specifications/Queries.md#portable-query" and
    .query.ontologies[0].source=="specifications/Queries.md#item-ontology" and
    ([.query.produces_properties[].iri]|unique)==($record[0].materializes_properties|unique) and
    ([.query.produces_families[].iri]|unique)==($record[0].materializes_families|unique) and
    all(["https://example.org/items#name","https://example.org/items#annotation","https://example.org/items#family"][];
      . as $target | any($graph.nodes[];.id==$target) and
      any($graph.edges[];.source==$node.id and .target==$target and .label=="declares output")))
' output/graph.json >/dev/null
printf 'PASS-explorer-query-graph-projection\n'
sed '/^  \* property: item:annotation$/d' "$fixture" > output/without-annotation.txt
cp output/without-annotation.txt "$fixture"
export_query | jq -e --slurpfile before output/family-record.json '.sha256==$before[0].sha256' >/dev/null
restore
printf 'PASS-produces-property-kinds-and-families\n'

jq -j '.child|split("  * type: ontology")|join("  * type: ontology\n  * ontology_base: https://example.org/sub\n  * ontology_prefix: sub")|split("@prefix item: <https://example.org/items#> .")|join("@prefix sub: <https://example.org/sub#> .")|split("item:Child a owl:Class .")|join("<https://example.org/sub> a owl:Ontology ; owl:imports <https://example.org/items> .\nsub:Child a owl:Class .")' "$cases" > output/child-base.txt
{ cat output/initial-query-model.txt output/child-base.txt; query $'PREFIX item: <https://example.org/items#>\nPREFIX sub: <https://example.org/sub#>\nASK { ?s a item:Item, sub:Child }' | sed '/^  \* use: \[Item Ontology\](#item-ontology)$/a\  * use: [Child Vocabulary](#child-vocabulary)'; } > "$fixture"
run validate >/dev/null
for ns in https://example.org/items 'https://example.org/sub#'; do run semantic query list --name 'Portable Query' --namespace-base "$ns" --json | jq -e 'length==1' >/dev/null; done
restore
printf 'PASS-multiple-used-ontology-namespaces\n'
git add specifications/Queries.md
git -c user.name='Query Test' -c user.email=query@example.invalid -c commit.gpgSign=false commit --allow-empty -m 'Query baseline' > output/git-commit.txt
sed 's/item:Item a owl:Class \./item:Item a owl:Class ; rdfs:comment "Changed vocabulary documentation" ./' output/initial-query-model.txt > "$fixture"
run change-impact --json | jq -e 'any(.changed[];.name=="Item Ontology" and any(.change_impact_tree[];.usedBy.name=="Item Lookup"))' >/dev/null
sed 's/Lists items for downstream consumers\./Updated query purpose./' output/initial-query-model.txt > "$fixture"
run change-impact --json | jq -e 'any(.changed[];.name=="Item Lookup" and .change_impact_tree==[])' >/dev/null
restore
printf 'PASS-ontology-query-change-impact-direction\n'
export_query 'Item Lookup' > output/original-record.json
run mv 'Item Lookup' specifications/Moved.md >/dev/null
export_query 'Item Lookup' | jq -e --slurpfile before output/original-record.json '.iri==$before[0].iri and .sha256==$before[0].sha256 and .source_files==["specifications/Moved.md"]' >/dev/null
run rename 'Item Lookup' 'Renamed Lookup' >/dev/null
export_query 'Renamed Lookup' | jq -e --slurpfile before output/original-record.json '.iri=="urn:reqvire:semantic-query:renamed-lookup" and .sha256==$before[0].sha256' >/dev/null
run --reject semantic query export --iri "$iri" >/dev/null
cp "$fixture" output/expected-ontology.txt
run --reject rm 'Item Ontology' >/dev/null
diff -u output/expected-ontology.txt "$fixture"
printf 'PASS-move-rename-and-used-ontology-removal\n'
