#!/usr/bin/env bash
# Count actual CLI/worker builds before an intentionally invalid listener.
set -Eeuo pipefail
binary=$(realpath "$1")
output=$(realpath -m "$2")
mkdir -p "$output"
# Aggregate and empty workspaces must be outside the runner's enclosing Git repo.
fixtures=$(mktemp -d /tmp/reqvire-serve-startup-XXXXXX)
failed=0
: > "$output/checks.txt"
model() {
    printf '# Elements\n\n### %s\n\nStartup fixture.\n\n#### Metadata\n  * type: capability\n---\n' "$1"
}
git() { timeout 10 git -C "$1" "${@:2}"; }
repo() {
    local directory=$1 name=${2:-Startup Root}
    mkdir -p "$directory"
    git "$directory" init -qb main
    git "$directory" config user.name 'Startup Test'
    git "$directory" config user.email startup@example.invalid
    model "$name" > "$directory/Model.md"
    git "$directory" add .
    git "$directory" -c commit.gpgSign=false commit -qm fixture
}
check() {
    local name=$1 directory=$2 builds=$3 runtimes=$4 error=$5 rc=0 actual_builds actual_runtimes
    shift 5
    RUST_LOG=error,reqvire::model=debug,reqvire::explorer_runtime=debug \
      timeout --kill-after=5s 30s "$binary" --workspace "$directory" serve --host ::invalid "$@" \
      > "$output/$name.stdout" 2> "$output/$name.stderr" || rc=$?
    cat "$output/$name.stdout" "$output/$name.stderr" > "$output/$name.log"
    actual_builds=$(jq -Rs '[scan("Starting two-pass validation architecture")]|length' "$output/$name.log")
    actual_runtimes=$(jq -Rs '[scan("Building Explorer runtime assets from validated model")]|length' "$output/$name.log")
    if ((rc!=0 && rc!=124 && rc!=137 && actual_builds==builds && actual_runtimes==runtimes)) &&
       jq -e -Rs --arg error "$error" 'contains($error) and ($error=="Failed to start server" or (contains("Failed to start server")|not))' "$output/$name.log" >/dev/null; then
        printf 'PASS %s\n' "$name" >> "$output/checks.txt"
    else
        printf 'FAIL %s\n' "$name" >> "$output/checks.txt"
        printf '%s: expected builds/runtime %s/%s and %s; got %s/%s, exit %s\n' "$name" "$builds" "$runtimes" "$error" "$actual_builds" "$actual_runtimes" "$rc" >&2
        cat "$output/$name.log" >&2
        failed=1
    fi
}
for name in plain read-only-mcp writable auto-commits; do
    flags=()
    case "$name" in
        read-only-mcp) flags=(--enable-mcp) ;;
        writable) flags=(--enable-mcp --enable-mutations) ;;
        auto-commits) flags=(--enable-mcp --enable-mutations --enable-commits) ;;
    esac
    directory=$fixtures/$name
    repo "$directory"
    check "$name" "$directory" 1 1 'Failed to start server' "${flags[@]}"
    model Broken | sed 's/capability/invalid-type/' > "$directory/Model.md"
    git "$directory" add .
    git "$directory" -c commit.gpgSign=false commit -qm 'invalid fixture'
    check "$name-invalid" "$directory" 1 0 invalid-type "${flags[@]}"
    case "$name" in writable|auto-commits)
        model Dirty > "$directory/Model.md"
        check "$name-dirty" "$directory" 0 0 clean "${flags[@]}" ;;
    esac
done
aggregate=$fixtures/aggregate
repo "$aggregate/first" 'First Root'
repo "$aggregate/second" 'Second Root'
check aggregate "$aggregate" 1 1 'Failed to start server'
check aggregate-read-only-mcp "$aggregate" 1 1 'Failed to start server' --enable-mcp
check aggregate-writable-rejected "$aggregate" 0 0 'Git rev-parse failed' --enable-mcp --enable-mutations
printf '# Elements\n\n### Broken\n\n#### Metadata\n  * type: invalid-type\n' > "$aggregate/second/Model.md"
check aggregate-invalid "$aggregate" 1 0 invalid-type
check aggregate-read-only-mcp-invalid "$aggregate" 1 0 invalid-type --enable-mcp
mkdir "$fixtures/empty"
check no-eligible-worktree "$fixtures/empty" 1 0 Git
if ((failed)); then printf 'Retained startup fixtures: %s\n' "$fixtures" >&2; exit 1; fi
rm -rf "$fixtures"
