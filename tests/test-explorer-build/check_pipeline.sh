#!/bin/bash
# Exercise real Bun/Make orchestration with recording fixtures for leaf tools.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$SCRIPT_DIR/../.." && pwd)"
source "$SCRIPT_DIR/../workflow_steps.sh"
mkdir -p "$1"
output="$(cd "$1" && pwd)"
bun=$(command -v bun)
built='["lint:ds","assets:ds","lint:style","typecheck","unit","icons","app"]'
fail() { printf '%s\n' "$*" >&2; exit 1; }

fixture() {
    repo="$output/$1"
    app="$repo/explorer"
    trace="$repo/trace.jsonl"
    mkdir -p "$app/scripts"
    # Keep production compositions; replace only costly leaf tools.
    jq 'del(.dependencies,.devDependencies) |
      .scripts["lint:ds"]="bun stage.cjs lint:ds" |
      .scripts["lint:style"]="bun stage.cjs lint:style" |
      .scripts["assets:ds"]="bun stage.cjs assets:ds" |
      .scripts["generate:icons"]="bun stage.cjs icons" |
      .scripts["build:mcp-previews"]="bun stage.cjs previews"
    ' "$root/explorer/package.json" > "$app/package.json"
    printf '{}' > "$app/bun.lock"
    cp "$root/Makefile" "$repo/Makefile"
    cp "$root/explorer/scripts/run-node.mjs" "$app/scripts/run-node.mjs"
    cp "$SCRIPT_DIR/fixtures/pipeline-stage.cjs" "$app/stage.cjs"
    export TRACE="$trace" BUN_INSTALL_CACHE_DIR="$repo/bun-cache"
    (cd "$app" && "$bun" stage.cjs ci)
    rm "$trace"
}

invoke() {
    local folder=$1
    shift
    local result=0
    (cd "$folder" && "$@") > "$repo/last-command.log" 2>&1 || result=$?
    if [[ "$result" != 0 ]]; then cat "$repo/last-command.log" >&2; fi
    return "$result"
}

calls_equal() {
    [[ -e "$trace" ]] || : > "$trace"
    jq -sS . "$trace" > "$repo/actual-stages.json"
    printf '%s\n' "$1" | jq -S . > "$repo/expected-stages.json"
    diff -u "$repo/expected-stages.json" "$repo/actual-stages.json"
}

composition() {
    local command=$1 expected=$2
    fixture "${command//:/-}"
    invoke "$app" "$bun" run "$command"
    calls_equal "$expected"
    local stage path
    for stage in app kit; do
        if [[ "$stage" == app ]]; then path="$app/dist/built"; else path="$app/design-system/dist-kit/built"; fi
        if jq -e --arg stage "$stage" 'index($stage) != null' <<< "$expected" > /dev/null; then
            [[ -e "$path" ]] || fail "$stage output missing"
        else
            [[ ! -e "$path" ]] || fail "unexpected $stage output"
        fi
    done
}

failures() {
    local expected index stage count
    expected=$(jq -c '. + ["kit"]' <<< "$built")
    for ((index=0; index<8; index++)); do
        stage=$(jq -r --argjson index "$index" '.[$index]' <<< "$expected")
        fixture "failure-$index-${stage//:/-}"
        count=$((index + 1))
        if FAIL_AT="$count" invoke "$app" "$bun" run build:all; then fail "accepted $stage failure"; fi
        calls_equal "$(jq -c --argjson count "$count" '.[:$count]' <<< "$expected")"
    done
}

make_run() {
    local failed=${1:-false} default=${2:-false} args=(make explorer)
    [[ "$default" != true ]] || args=(make)
    rm -f "$trace"
    if [[ "$failed" == true ]]; then
        FAIL_STAGE=ci invoke "$repo" "${args[@]}"
    else
        FAIL_STAGE='' invoke "$repo" "${args[@]}"
    fi
}

make_dependencies() {
    fixture make
    mkdir "$repo/commands"
    cat > "$repo/commands/bun" <<'BUN'
#!/bin/sh
if [ "$1" = install ]; then exec "$REAL_BUN" stage.cjs ci; fi
exec "$REAL_BUN" "$@"
BUN
    chmod +x "$repo/commands/bun"
    export REAL_BUN="$bun" PATH="$repo/commands:$PATH"
    local installed filename
    installed=$(jq -c '["ci"] + .' <<< "$built")
    make_run false true; calls_equal "$installed"
    make_run; calls_equal "$built"
    for filename in package.json bun.lock; do
        printf '\n' >> "$app/$filename"
        make_run; calls_equal "$installed"
        make_run
        jq -s -e 'index("ci") == null' "$trace" > /dev/null
    done
    printf '\n' >> "$app/bun.lock"
    if make_run true; then fail 'failed install was accepted'; fi
    calls_equal '["ci"]'
    make_run; calls_equal "$installed"
    rm -rf "$app/node_modules"
    make_run; calls_equal "$installed"
}

expand() {
    local rest=$1 part last name hook command
    while :; do
        part=${rest%%&&*}; last=false
        if [[ "$rest" == *'&&'* ]]; then rest=${rest#*&&}; else last=true; fi
        part="${part#"${part%%[![:space:]]*}"}"
        part="${part%"${part##*[![:space:]]}"}"
        if [[ "$part" == 'bun run '* ]]; then
            read -r _ _ name _ <<< "$part"
            hook=$(jq -r --arg name "pre$name" '.scripts[$name] // empty' "$root/explorer/package.json")
            [[ -z "$hook" ]] || expand "$hook"
            case "$name" in
                lint:ds|lint:style|assets:ds|typecheck|generate:icons|build:ds-bundle|test:ads|test:behavior|build:mcp-previews)
                    printf '%s\n' "$name" ;;
                *) command=$(jq -er --arg name "$name" '.scripts[$name]' "$root/explorer/package.json"); expand "$command" ;;
            esac
        else
            case "$part" in
                'bun node_modules/typescript/bin/tsc --noEmit') printf 'typecheck\n' ;;
                'bun scripts/run-node.mjs --max-old-space-size=1200 node_modules/vitest/vitest.mjs run') printf 'unit\n' ;;
                'bun scripts/run-node.mjs --max-old-space-size=1600 node_modules/vite/bin/vite.js build') printf 'app\n' ;;
            esac
        fi
        [[ "$last" != true ]] || break
    done
}

workflow_stages() {
    local filename job block command browser_line bundle_line cargo_line expected_kit
    while read -r filename job; do
        block="$output/$filename-$job.block"
        workflow_job "$root/.github/workflows/$filename" "$job" > "$block"
        if grep -Fq REQVIRE_BUILD_EXPLORER "$block"; then fail "$filename/$job rebuilds inside Cargo"; fi
        awk '/^    env:/{inside=1;next} inside && /^      /{print;next} inside{exit}' "$block" > "$block.env"
        if grep -Eq '\$\{\{[[:space:]]*runner\.' "$block.env"; then fail "$filename/$job uses runner context too early"; fi
        : > "$block.stages"
        while IFS= read -r command; do expand "$command" >> "$block.stages"; done \
            < <(sed -n 's/^        run: \(cd explorer && .*\)$/\1/p' "$block")
        jq -Rs 'split("\n") | map(select(length > 0))' "$block.stages" > "$block.json"
        jq -e '. as $stages |
          all(["lint:ds","lint:style","typecheck","unit","app"][];
            . as $stage | [$stages[] | select(. == $stage)] | length == 1) and
          (index("app") > ([index("lint:ds"),index("lint:style"),index("typecheck"),index("unit")] | max)) and
          (all(.[]; . != "test:ads" and . != "test:behavior" and . != "build:mcp-previews"))
        ' "$block.json" > /dev/null
        expected_kit=0; [[ "$filename" != pr.yml ]] || expected_kit=1
        jq -e --argjson expected "$expected_kit" '[.[] | select(. == "build:ds-bundle")] | length == $expected' "$block.json" > /dev/null
        bundle_line=$(awk 'index($0,"run: cd explorer && bun run build"){print NR;exit}' "$block")
        browser_line=$(awk '/^        run: cd explorer && .*playwright install.*chromium/{print NR;exit}' "$block")
        if [[ "$filename" == pr.yml ]]; then
            [[ -n "$browser_line" && -n "$bundle_line" && "$browser_line" -lt "$bundle_line" ]] || fail "$filename/$job Chromium setup is missing or late"
        else
            [[ -z "$browser_line" ]] || fail "$filename/$job unexpectedly provisions Chromium"
        fi
        cargo_line=$(awk 'index($0,"run: cargo build"){print NR;exit}' "$block")
        if [[ -n "$cargo_line" ]]; then
            [[ -n "$bundle_line" && "$bundle_line" -lt "$cargo_line" ]] || fail "$filename/$job builds Cargo before the bundle"
        fi
    done <<'JOBS'
pr.yml test
release.yml build
release.yml release
pages.yml deploy
JOBS
}

if [[ "${2:-}" == --case ]]; then shift 2; "$@"; exit; fi
: > "$output/checks.txt"
status=0
record() {
    local name=$1
    shift
    if bash "$SCRIPT_DIR/check_pipeline.sh" "$output" --case "$@" > "$output/$name.log" 2>&1; then
        printf 'PASS %s\n' "$name" >> "$output/checks.txt"
    else
        printf 'FAIL %s\n' "$name" >> "$output/checks.txt"
        cat "$output/$name.log" >&2; status=1
    fi
}
record pipeline-build composition build "$built"
record pipeline-check composition check '["lint:ds","lint:style","typecheck"]'
record pipeline-build:app composition build:app '["icons","app"]'
record pipeline-build:ds-bundle composition build:ds-bundle '["kit"]'
record pipeline-build:all composition build:all "$(jq -c '. + ["kit"]' <<< "$built")"
record pipeline-test:ads composition test:ads '["icons","previews","ads"]'
record pipeline-stops-on-failure failures
record make-dependency-reuse-and-retry make_dependencies
record ci-builds-and-checks-once workflow_stages
exit "$status"
