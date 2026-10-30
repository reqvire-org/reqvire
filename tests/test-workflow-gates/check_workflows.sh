#!/bin/bash
# Execute the repository's literal workflow shell steps in isolated fixtures.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$SCRIPT_DIR/../.." && pwd)"
source "$SCRIPT_DIR/../workflow_steps.sh"
binary=${REAL_REQVIRE_BIN:-$1}
output=$2
mkdir -p "$(dirname "$output")"
work="$(dirname "$output")/workflow-cases"
mkdir -p "$work"
pr="$root/.github/workflows/pr.yml"
release="$root/.github/workflows/release.yml"
fail() { printf '%s\n' "$*" >&2; exit 1; }

execute_step() {
    local command=$1 folder=$2
    shift 2
    (cd "$folder" && env "$@" timeout 60 bash --noprofile --norc -e -o pipefail "$command") \
        > "$folder/step.stdout" 2> "$folder/step.stderr"
}

workers() {
    local workflow=$1 label=$2 stale=${3:-false} failed=${4:-none}
    local folder="$work/$label-$stale-$failed" command block worker value result=0
    mkdir -p "$folder/commands" "$folder/target/release"
    if [[ "$stale" == true ]]; then
        printf '#!/bin/sh\necho stale-worker\n' > "$folder/target/release/reqvire"
        chmod +x "$folder/target/release/reqvire"
    fi
    cat > "$folder/commands/cargo" <<'CARGO'
#!/bin/bash
set -eu
echo "$1" >> "$TRACE"
case "$1" in
  build)
    [[ "${FAIL_STAGE:-}" != build ]] || exit 7
    [[ "$*" == 'build --release --locked --bin reqvire' ]]
    printf '#!/bin/sh\necho current-worker\n' > target/release/reqvire
    chmod +x target/release/reqvire
    ;;
  test)
    [[ "$REQVIRE_TEST_BIN" == "$PWD/target/release/reqvire" ]]
    [[ "$("$REQVIRE_TEST_BIN")" == current-worker ]]
    [[ "${FAIL_STAGE:-}" != test ]] || exit 9
    ;;
esac
CARGO
    chmod +x "$folder/commands/cargo"
    workflow_steps "$workflow" "$folder/steps"
    for command in "$folder"/steps/*.run; do
        if ! grep -Fq 'cargo test ' "$command" &&
            ! { grep -Fq 'cargo build ' "$command" && grep -Fq -- '--bin reqvire' "$command"; }; then continue; fi
        block="${command%.run}.block"
        local environment=("PATH=$folder/commands:$PATH" "TRACE=$folder/commands.txt" "FAIL_STAGE=$failed" RUNNER_OS=Linux)
        worker=$(sed -n 's/^          REQVIRE_TEST_BIN: //p' "$block")
        if [[ -n "$worker" ]]; then
            worker=${worker#\"}; worker=${worker%\"}
            value='${{ github.workspace }}'
            worker=${worker//"$value"/"$folder"}
            environment+=("REQVIRE_TEST_BIN=$worker")
        fi
        if execute_step "$command" "$folder" "${environment[@]}"; then result=0; else result=$?; break; fi
    done
    printf 'build\n' > "$folder/expected-commands.txt"
    [[ "$failed" == build ]] || printf 'test\n' >> "$folder/expected-commands.txt"
    diff -u "$folder/expected-commands.txt" "$folder/commands.txt"
    local wanted=0
    case "$failed" in build) wanted=7 ;; test) wanted=9 ;; esac
    [[ "$result" == "$wanted" ]] || { cat "$folder/step.stderr" >&2; fail "unexpected workflow status: $result, expected $wanted"; }
}

policy() {
    local folder="$work/policy" upload shell command
    mkdir -p "$folder"
    awk '/^jobs:/{exit} {print}' "$pr" > "$folder/trigger"
    grep -Fq pull_request: "$folder/trigger"
    if grep -Fq branches: "$folder/trigger"; then fail 'stacked PR targets are excluded'; fi
    grep -Fq 'name: Run Rust Tests' "$pr"
    workflow_job "$pr" validate > "$folder/validate"
    if grep -Eq 'continue-on-error:|scripts/install.sh' "$folder/validate"; then fail 'validation uses a released validator or tolerates errors'; fi
    grep -Fq 'needs: test' "$folder/validate"
    grep -Fq 'uses: actions/download-artifact@' "$folder/validate"
    [[ $(grep -Fc 'name: reqvire-pr-cli' "$pr") == 2 ]] || fail 'built/downloaded CLI artifact names differ'
    awk '/^      - name: Share current CLI with model validation$/{inside=1;next} inside && /^      - name:/{exit} inside{print}' "$pr" > "$folder/upload"
    grep -Fq 'id: share_cli' "$folder/upload"
    grep -Fq 'cli_artifact_id: ${{ steps.share_cli.outputs.artifact-id }}' "$pr"
    grep -Fq "if: \${{ !cancelled() && needs.test.outputs.cli_artifact_id != '' }}" "$folder/validate"
    grep -Fq 'path: target/release/reqvire' "$folder/upload"
    grep -Fq 'path: target/release' "$folder/validate"
    workflow_steps "$pr" "$folder/steps"
    shell=''
    for command in "$folder"/steps/*.run; do
        if [[ $(cat "${command%.run}.name") == 'Run shell end-to-end tests' ]]; then shell="${command%.run}.block"; break; fi
    done
    [[ -n "$shell" ]] || fail 'shell end-to-end step is missing'
    grep -Fq 'REQVIRE_BIN: ${{ github.workspace }}/target/release/reqvire' "$shell"
}

model_gate() {
    local invalid=$1 folder="$work/model-$1" command selected=''
    mkdir -p "$folder/target/release"
    workflow_steps "$pr" "$folder/steps"
    for command in "$folder"/steps/*.run; do
        if [[ $(cat "${command%.run}.name") == 'Validate Requirements via validate command' ]]; then selected=$command; break; fi
    done
    [[ -n "$selected" ]] || fail 'model validation step is missing'
    grep -Fq './target/release/reqvire validate' "$selected"
    ln -s "$binary" "$folder/target/release/reqvire"
    if [[ "$invalid" == true ]]; then
        sed 's/type: capability/type: invalid-type/g' "$root/tests/test-cache-integration/fixtures/model.md.txt" > "$folder/Model.md"
    else
        cp "$root/tests/test-cache-integration/fixtures/model.md.txt" "$folder/Model.md"
    fi
    printf 'reports/**\ntarget/**\n' > "$folder/.reqvireignore"
    git -C "$folder" init -q
    git -C "$folder" config user.name 'Workflow Test'
    git -C "$folder" config user.email test@example.invalid
    git -C "$folder" add .
    git -C "$folder" -c commit.gpgSign=false commit -qm fixture
    local result=0
    execute_step "$selected" "$folder" || result=$?
    if { [[ "$invalid" == true && "$result" == 0 ]]; } ||
       { [[ "$invalid" != true && "$result" != 0 ]]; }; then
        cat "$folder/step.stdout" "$folder/step.stderr" >&2
        fail 'unexpected model validation outcome'
    fi
    [[ -s "$folder/reports/validation_report.txt" ]] || fail 'validation diagnostics not retained'
}

if [[ "${3:-}" == --case ]]; then shift 3; "$@"; exit; fi
: > "$output"
status=0
record() {
    local name=$1
    shift
    if bash "$SCRIPT_DIR/check_workflows.sh" "$binary" "$output" --case "$@" > "$work/$name.log" 2>&1; then
        printf 'PASS %s\n' "$name" >> "$output"
    else
        printf 'FAIL %s\n' "$name" >> "$output"
        cat "$work/$name.log" >&2; status=1
    fi
}
for label in pr release; do
    workflow="$root/.github/workflows/$label.yml"
    record "$label-clean-worker" workers "$workflow" "$label" false
    record "$label-stale-worker" workers "$workflow" "$label" true
    record "$label-build-failure" workers "$workflow" "$label" false build
    record "$label-test-failure" workers "$workflow" "$label" false test
done
record pr-current-revision-policy policy
record valid-model-gate model_gate false
record invalid-model-fails-gate model_gate true
exit "$status"
