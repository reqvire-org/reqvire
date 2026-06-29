#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RUNS="${REQVIRE_BENCHMARK_RUNS:-5}"
PROFILE_ARGS=(--release)
PROFILE_LABEL="release"
REQVIRE_BIN="$REPO_ROOT/target/release/reqvire"

usage() {
    cat >&2 <<'USAGE'
Usage: scripts/benchmark-cargo-run.sh [--runs N] [--debug] [--release] [--] [reqvire-args...]

Builds Reqvire with Cargo, then benchmarks the compiled binary from the
repository root.
If no Reqvire command is provided, defaults to `validate`.

Examples:
  scripts/benchmark-cargo-run.sh
  scripts/benchmark-cargo-run.sh --runs 10 validate
  scripts/benchmark-cargo-run.sh --runs 3 -- semantic export --layer model
  scripts/benchmark-cargo-run.sh --debug -- model --json
USAGE
}

REQVIRE_ARGS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        -n|--runs)
            [[ $# -ge 2 ]] || { echo "Missing value for $1" >&2; usage; exit 2; }
            RUNS="$2"
            shift 2
            ;;
        --release)
            PROFILE_ARGS=(--release)
            PROFILE_LABEL="release"
            REQVIRE_BIN="$REPO_ROOT/target/release/reqvire"
            shift
            ;;
        --debug)
            PROFILE_ARGS=()
            PROFILE_LABEL="debug"
            REQVIRE_BIN="$REPO_ROOT/target/debug/reqvire"
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        --)
            shift
            REQVIRE_ARGS=("$@")
            break
            ;;
        *)
            REQVIRE_ARGS=("$@")
            break
            ;;
    esac
done

if [[ "${#REQVIRE_ARGS[@]}" -eq 0 ]]; then
    REQVIRE_ARGS=(validate)
fi

if ! [[ "$RUNS" =~ ^[1-9][0-9]*$ ]]; then
    usage
    exit 2
fi

now_ns() {
    local value
    value="$(date +%s%N 2>/dev/null)"
    if [[ "$value" == *N* ]]; then
        echo "$(($(date +%s) * 1000000000))"
    else
        echo "$value"
    fi
}

format_ms() {
    local ms="$1"
    printf "%d.%03ds" "$((ms / 1000))" "$((ms % 1000))"
}

cd "$REPO_ROOT"

output_file="$(mktemp -t reqvire-validate-benchmark-XXXXXX.out)"
cleanup() {
    rm -f "$output_file"
}
trap cleanup EXIT

echo "Benchmarking Reqvire over this repository"
echo "Workspace: $REPO_ROOT"
printf "Command: %q" "$REQVIRE_BIN"
printf " %q" "${REQVIRE_ARGS[@]}"
printf "\n"
echo "Profile: $PROFILE_LABEL"
echo "Runs: $RUNS"
echo ""
echo "Preparing cargo build outside measured runs..."
cargo build --quiet "${PROFILE_ARGS[@]}" --bin reqvire
echo ""

total_ms=0
passed=0
failed=0

for ((run = 1; run <= RUNS; run++)); do
    started_ns="$(now_ns)"
    if "$REQVIRE_BIN" "${REQVIRE_ARGS[@]}" >"$output_file" 2>&1; then
        status=0
        passed=$((passed + 1))
    else
        status=$?
        failed=$((failed + 1))
    fi
    elapsed_ms="$((( $(now_ns) - started_ns ) / 1000000))"
    total_ms=$((total_ms + elapsed_ms))

    if [[ "$status" -eq 0 ]]; then
        printf "Run %d: %s PASSED\n" "$run" "$(format_ms "$elapsed_ms")"
    else
        printf "Run %d: %s FAILED\n" "$run" "$(format_ms "$elapsed_ms")"
        sed 's/^/  /' "$output_file"
    fi
done

average_ms=$((total_ms / RUNS))

echo ""
echo "⏱ Benchmark summary"
printf "Total elapsed: %s\n" "$(format_ms "$total_ms")"
printf "Average elapsed: %s\n" "$(format_ms "$average_ms")"
printf "Runs: %d passed, %d failed, %d total\n" "$passed" "$failed" "$RUNS"

if [[ "$failed" -gt 0 ]]; then
    exit 1
fi
