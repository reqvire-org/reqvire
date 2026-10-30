#!/usr/bin/env bash
# Observe Git calls and inject the fixture's before/after observation hook.
set -euo pipefail
jq -cn --args '$ARGS.positional' -- "$@" >> "$GIT_OBSERVATION_LOG"
config='{}'
if [[ -f "$GIT_OBSERVATION_HOOK" ]]; then config=$(cat "$GIT_OBSERVATION_HOOK"); fi
matched=false
if jq -e -n --argjson config "$config" --args \
    '($config|length)>0 and all(($config.match//[])[];. as $token|$ARGS.positional|index($token)!=null)' -- "$@" >/dev/null; then matched=true; fi
if [[ "$matched" == true ]] && jq -e .fail <<< "$config" >/dev/null; then exit 1; fi
hook() {
    local -a command=()
    mapfile -d '' -t command < <(jq -j '.run[] | ., "\u0000"' <<< "$config")
    rm "$GIT_OBSERVATION_HOOK"
    "$REAL_TEST_GIT" "${command[@]}" >/dev/null 2>&1
}
before=false
if jq -e .before <<< "$config" >/dev/null; then before=true; fi
if [[ "$matched" == true && "$before" == true ]]; then hook; fi
capture=$(mktemp -d "${GIT_OBSERVATION_LOG}.XXXXXX")
trap 'rm -rf "$capture"' EXIT
rc=0
"$REAL_TEST_GIT" "$@" > "$capture/stdout" 2> "$capture/stderr" || rc=$?
if [[ "$matched" == true && "$before" == false ]]; then hook; fi
cat "$capture/stdout"
cat "$capture/stderr" >&2
exit "$rc"
