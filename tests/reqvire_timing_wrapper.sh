#!/bin/bash
set +e

if [[ -z "${REAL_REQVIRE_BIN:-}" ]]; then
    echo "REAL_REQVIRE_BIN is not set" >&2
    exit 127
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

started_ns="$(now_ns)"
"$REAL_REQVIRE_BIN" "$@"
status=$?
elapsed_ms="$((( $(now_ns) - started_ns ) / 1000000))"

if [[ -n "${REQVIRE_BENCHMARK_INVOCATIONS:-}" ]]; then
    command_text="$(printf "%q " "$@")"
    command_text="${command_text% }"
    command_text="${command_text//$'\t'/ }"
    command_text="${command_text//$'\n'/ }"
    printf "%s\t%s\t%s\t%s\n" \
        "$elapsed_ms" \
        "${REQVIRE_BENCHMARK_TEST:-unknown}" \
        "$status" \
        "$command_text" >> "$REQVIRE_BENCHMARK_INVOCATIONS"
fi

exit "$status"
