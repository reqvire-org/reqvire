#!/bin/bash
# Check taxonomy validation and mutation rejection against actual fixture files.
set -euo pipefail
shopt -s nullglob
binary=$1
root=$2
path="$root/specifications/Projection.md"
output="$root/output/taxonomy"
mkdir -p "$output"
cp "$path" "$output/original.md.txt"
trap 'cp "$output/original.md.txt" "$path"' EXIT
status=0

run() {
    local label=$1 command=$2
    shift 2
    local executable=$binary
    case "$command" in add|rm|mv|rename|merge|mv-file|mv-folder|link|unlink|relink|mv-asset|rm-asset|format) executable=$MCP_OPERATION_BIN ;; esac
    (cd "$root" && timeout 15 "$executable" "$command" "$@") > "$output/$label.stdout" 2> "$output/$label.stderr"
}

reject() {
    local label=$1
    shift
    local before="$output/$label-before" after="$output/$label-after" result=0 problem=false
    mkdir -p "$before" "$after"
    cp -a "$root/specifications/"*.md "$before/"
    run "$label" "$@" || result=$?
    cat "$output/$label.stdout" "$output/$label.stderr" > "$output/$label.diagnostic"
    if [[ "$result" == 0 ]]; then printf '%s: accepted invalid taxonomy\n' "$label" >&2; problem=true; fi
    local token
    for token in 'Concept taxonomy cycle' projection-child projection-parent; do
        if ! grep -Fq "$token" "$output/$label.diagnostic"; then
            printf '%s: missing taxonomy path diagnostic: %s\n' "$label" "$token" >&2; problem=true
        fi
    done
    local files=("$root/specifications/"*.md)
    [[ ${#files[@]} == 0 ]] || cp -a "${files[@]}" "$after/"
    if ! diff -ru "$before" "$after" > "$output/$label.diff"; then
        cat "$output/$label.diff" >&2; problem=true
    fi
    cp -a "$before/"*.md "$root/specifications/"
    if [[ "$problem" == true ]]; then
        printf 'FAIL %s\n' "$label"; cat "$output/$label.diagnostic" >&2; status=1
    else printf 'PASS %s\n' "$label"; fi
}

for suffix in broader mixed; do
    if [[ "$suffix" == broader ]]; then
        sed 's@narrower: \[Projection Child\](#projection-child)@broader: [Projection Child](#projection-child)@g' "$output/original.md.txt" > "$path"
    else
        sed -e 's@narrower: \[Projection Child\](#projection-child)@broader: [Projection Peer](#projection-peer)@g' \
            -e 's@related: \[Projection Peer\](#projection-peer)@narrower: [Projection Peer](#projection-peer)@g' "$output/original.md.txt" > "$path"
    fi
    reject "validate-$suffix" validate
    reject "export-$suffix" semantic export --layer concepts
done
cp "$output/original.md.txt" "$path"
run repaired-taxonomy validate
printf 'PASS repaired-taxonomy\n'
replacement=$(cat <<'MODEL'
### Projection Parent
Parent with an invalid closing edge.
#### Metadata
  * type: concept
#### Relations
  * derivedFrom: [Projection Alpha Scheme](#projection-alpha-scheme)
  * broader: [Projection Child](#projection-child)
MODEL
)
for mode in persist dry-run; do
    flags=()
    [[ "$mode" != dry-run ]] || flags+=(--dry-run)
    reject "link-$mode" link 'Projection Parent' broader 'Projection Child' "${flags[@]}"
    reject "replace-$mode" add specifications/Projection.md --override --content "$replacement" "${flags[@]}"
done
exit "$status"
