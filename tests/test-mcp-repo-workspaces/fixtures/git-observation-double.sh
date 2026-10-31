#!/usr/bin/env bash
# Lose the observation after a real owned ref update, without repeating it.
set -euo pipefail
if [[ "$PWD" == "$REQVIRE_REF_TEST_OWNER" ]]; then
    if [[ "$1" == update-ref && "${3:-}" == 'reqvire: '* ]]; then
        "$REQVIRE_REF_TEST_GIT" "$@"
        printf '%s\n' "$*" >> "$REQVIRE_REF_TEST_ATTEMPTS"
        touch "$REQVIRE_REF_TEST_LOST"
        exit 0
    fi
    if [[ -f "$REQVIRE_REF_TEST_LOST" && "$1" == rev-parse && "${2:-}" == HEAD ]]; then
        printf 'Injected loss of post-publication HEAD observation\n' >&2
        exit 1
    fi
fi
exec "$REQVIRE_REF_TEST_GIT" "$@"
