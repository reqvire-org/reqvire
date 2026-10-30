#!/usr/bin/env bash
# Hold concurrent extractors at a barrier and inject one partial extraction.
set -euo pipefail
touch "$BARRIER/$$"
if [[ -n "${BARRIER_COUNT:-}" ]]; then
    deadline=$((SECONDS+10))
    while (($(find "$BARRIER" -mindepth 1 -maxdepth 1 -printf x | wc -c)<BARRIER_COUNT)); do
        if ((SECONDS>deadline)); then exit 88; fi
        sleep 0.01
    done
fi
fail=${FAIL_EXTRACT:-0}
if [[ -n "${FAIL_ONE:-}" ]] && (set -o noclobber; : > "$BARRIER/failed") 2>/dev/null; then fail=1; fi
rc=0
"$REQVIRE_TEST_REAL_TAR" "$@" || rc=$?
if [[ "$fail" == 1 ]]; then
    args=("$@")
    destination=''
    for ((i=0;i<${#args[@]}-1;i++)); do
        if [[ "${args[$i]}" == -C ]]; then destination=${args[$((i+1))]}; break; fi
    done
    [[ -n "$destination" ]]
    printf 'partial executable' > "$destination/reqvire-$REQVIRE_TEST_TAR_TARGET"
    sleep 0.05
    exit 17
fi
exit "$rc"
