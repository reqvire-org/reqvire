#!/usr/bin/env bash
# CLI evidence for fenced Markdown, mutation parsing, and semantic source bytes.
set -euo pipefail
binary=$1
suite=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$2"
cases=$suite/fenced-cases.json
fixture=specifications/FencedCode.md
mkdir -p output
trap 'printf "FAIL fenced-code check at line %s\n" "$LINENO" >&2' ERR

run() {
    local executable=$binary
    case "$1" in add|format|link|rename|rm) executable=$MCP_OPERATION_BIN ;; esac
    if ! "$executable" "$@" > output/fenced-command.stdout 2> output/fenced-command.stderr; then
        printf 'FAIL command: %s\n' "$*" >&2
        cat output/fenced-command.stdout output/fenced-command.stderr >&2
        return 1
    fi
    cat output/fenced-command.stdout
}
document() {
    jq -jr --rawfile code output/fenced-code.txt '.document | split("__CODE__") | join($code)' "$cases" > "$fixture"
}
elements() {
    run search --json > output/fenced-search.json
    jq --arg path "${1:-$fixture}" '.files[$path].elements' output/fenced-search.json > output/fenced-elements.json
}
base_assertions() {
    jq -e '
      (map(.name)|sort) == ["Fence Example","Fence Following","Fence Parent"] and
      any(.[]; .name == "Fence Example" and .type == "capability") and
      any(.[]; .name == "Fence Following" and any(.relations[];
        .relation_type == "derivedFrom" and (.target.target|endswith("#fence-parent"))))
    ' output/fenced-elements.json >/dev/null
}

for mode in parse format; do
    for ((i=0; i<$(jq '.cases|length' "$cases"); i++)); do
        label=$(jq -r ".cases[$i].label" "$cases")
        jq -j ".cases[$i].code" "$cases" > output/fenced-code.txt
        document
        cp "$fixture" output/fenced-source-before.txt
        if [[ "$mode" == format ]]; then run format --fix >/dev/null; fi
        run validate >/dev/null
        elements
        base_assertions
        if [[ "$mode" == parse ]]; then
            jq -e --rawfile code output/fenced-code.txt '
              any(.[]; .name == "Fence Example" and
                (.content|contains($code|sub("^\\n+";"")|sub("\\n+$";""))) and
                all(.relations[]; (.target.target|endswith("#missing")|not)))
            ' output/fenced-elements.json >/dev/null
            diff -u output/fenced-source-before.txt "$fixture"
            printf 'PASS %s\n' "$label"
        else
            jq -jr --rawfile code output/fenced-code.txt '
              ($code|sub("\\n+$";"")) as $expected |
              (.[]|select(.name=="Fence Example").content) as $body |
              ($body|index($code|split("\n")[0])) as $start |
              if $start == null then error("formatted fence missing")
              else $body[$start:$start+($expected|length)] + "\n" end
            ' output/fenced-elements.json > output/fenced-format-after.txt
            jq -jr --rawfile code output/fenced-code.txt -n '$code|sub("\\n+$";"") + "\n"' > output/fenced-format-before.txt
            diff -u output/fenced-format-before.txt output/fenced-format-after.txt
            printf 'PASS format-%s\n' "$label"
        fi
    done
done

jq -j '.cases[0].code' "$cases" > output/fenced-code.txt
document
# The nested list fixture also exercises single-element add and override.
jq -j '.cases[6].code | split("\n")[1:] | join("\n") | sub("^    \\*";"  *")' "$cases" > output/fenced-added-code.txt
{
    printf '### Fence Added\n\nInitial example.\n\n'
    cat output/fenced-added-code.txt
    printf '\n#### Metadata\n  * type: capability\n\n#### Relations\n  * derivedFrom: [Fence Example](#fence-example)\n'
} > output/fenced-added.txt
for label in add override; do
    options=()
    if [[ "$label" == override ]]; then
        sed 's/Initial example\./Updated example./' output/fenced-added.txt > output/fenced-updated.txt
        cp output/fenced-updated.txt output/fenced-added.txt
        options+=(--override)
    fi
    run add "$fixture" "${options[@]}" < output/fenced-added.txt >/dev/null
    elements
    jq -e --rawfile code output/fenced-added-code.txt --arg label "$label" '
      length == 4 and any(.[]; .name == "Fence Added" and .type == "capability" and
        (.content|contains($code|sub("\\n+$";""))) and
        any(.relations[]; .relation_type=="derivedFrom" and (.target.target|endswith("#fence-example"))) and
        (if $label=="override" then (.content|contains("Updated example.")) and
          (.content|contains("Initial example.")|not) else true end))
    ' output/fenced-elements.json >/dev/null
    printf 'PASS %s\n' "$label"
done

document
jq -j '.cases[2].code | sub("     ```\n$";"      ```\n") | sub("  - ```markdown";"  * ```markdown")' "$cases" > output/fenced-semantic-code.txt
printf 'SELECT ("""first\n#### Metadata\n### Literal Query Heading\n---\nlast""" AS ?text) WHERE {}\n' > output/fenced-query.txt
{
    cat <<'EOF'
# Elements

### Fence Vocabulary

#### Metadata
  * type: ontology
  * ontology_base: https://example.org/fence
  * ontology_prefix: fence

EOF
    cat output/fenced-semantic-code.txt
    cat <<'EOF'

#### Ontology
```turtle
@prefix fence: <https://example.org/fence#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<https://example.org/fence> a owl:Ontology .
fence:Item a owl:Class .
```
---

### Fence Query

Query purpose.

EOF
    cat output/fenced-semantic-code.txt
    printf '\n#### Query\n```sparql\n'
    cat output/fenced-query.txt
    cat <<'EOF'
```

#### Metadata
  * type: semantic-query

#### Relations
  * use: [Fence Vocabulary](#fence-vocabulary)
---
EOF
} > specifications/FencedSemantic.md
run validate >/dev/null
elements specifications/FencedSemantic.md
jq -e '
  (map(.name)|sort)==["Fence Query","Fence Vocabulary"] and
  any(.[]; .name=="Fence Vocabulary" and .ontology.ontology.language=="turtle" and
    (.ontology.ontology.content|contains("fence:Item a owl:Class")))
' output/fenced-elements.json >/dev/null
run semantic query export --name 'Fence Query' --json > output/fenced-query.json
jq -e --rawfile text output/fenced-query.txt --argjson line "$(awk '$0=="```sparql" {print NR;exit}' specifications/FencedSemantic.md)" \
    '.content==$text and .line_number==$line' output/fenced-query.json >/dev/null
printf 'PASS semantic-subsections-and-source-location\n'

printf '<details>\n  * ```\n### Hidden HTML Heading\n</details>\n' > output/fenced-code.txt
document
run validate >/dev/null
elements
jq -e '(map(.name)|sort)==["Fence Example","Fence Following","Fence Parent"]' output/fenced-elements.json >/dev/null
printf 'PASS html-details\n'
