#!/bin/bash
# Native Repo selection uses real local Git remotes and actual Repo metadata.
# Keep the root manifest absent, so filename substitution cannot pass these cases.
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

native_fixture() {
    native_case="$TEST_DIR/native-$1"
    mkdir -p "$native_case/remotes" "$native_case/component" "$native_case/central"
    export GIT_ALLOW_PROTOCOL=file GIT_TERMINAL_PROMPT=0
    export GIT_AUTHOR_NAME='Repo E2E' GIT_AUTHOR_EMAIL='repo-e2e@example.invalid'
    export GIT_COMMITTER_NAME="$GIT_AUTHOR_NAME" GIT_COMMITTER_EMAIL="$GIT_AUTHOR_EMAIL"
    native_source="$TEST_DIR/repo-tool"
    git -C "$native_case/component" init -qb main
    cp "$TEST_SCRIPT_DIR/fixtures/component.md.txt" "$native_case/component/Model.md"
    git -C "$native_case/component" add .
    git -C "$native_case/component" -c commit.gpgSign=false commit -qm alpha
    alpha=$(git -C "$native_case/component" rev-parse HEAD)
    sed -i 's/behavior is alpha/behavior is omega/' "$native_case/component/Model.md"
    git -C "$native_case/component" -c commit.gpgSign=false commit -qam omega
    omega=$(git -C "$native_case/component" rev-parse HEAD)
    git clone -q --bare "$native_case/component" "$native_case/remotes/component"
    central="$native_case/central"
    git -C "$central" init -qb main
    cp "$TEST_SCRIPT_DIR/fixtures/central.md.txt" "$central/Model.md"
    printf '.repo/\nrepositories/\n' > "$central/.gitignore"
    cat > "$central/default.xml" <<EOF
<manifest>
  <remote name="local" fetch="file://$native_case/remotes/"/>
  <project name="component" path="repositories/component" remote="local" revision="$alpha"/>
</manifest>
EOF
    git -C "$central" add .
    git -C "$central" -c commit.gpgSign=false commit -qm Central
    git clone -q --bare "$central" "$native_case/remotes/central"
    git -C "$central" remote add origin "$native_case/remotes/central"
    (
        cd "$central"
        repo init -q -u "file://$native_case/remotes/central" -b main \
            --repo-url "file://$native_source" \
            --repo-rev "$(git -C "$native_source" rev-parse HEAD)" \
            --no-repo-verify --no-clone-bundle
        repo sync -q --no-clone-bundle --no-tags
    )
    git -C "$central" rm -q default.xml
    git -C "$central" -c commit.gpgSign=false commit -qm 'Use native Repo manifest only'
    test ! -e "$central/default.xml"
    native="$central/.repo/manifests/default.xml"
}

native_cli_names() {
    (cd "$central" && "$REQVIRE_BIN" "$@" search --json) > "$native_case/search.json"
    jq '[.files[].elements[].name] | sort' "$native_case/search.json" > "$native_case/names.json"
    diff -u "$TEST_SCRIPT_DIR/expected/native-names.json" "$native_case/names.json"
}

native_server_names() {
    local mode=$1 callback=${2:-} port server_pid ready=false
    # Python is used only to allocate the server's listening port.
    port=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1",0)); print(s.getsockname()[1])')
    local options=()
    if [[ "$mode" == serve ]]; then options+=(--enable-mcp); fi
    (
        cd "$central"
        exec "$REQVIRE_BIN" "$mode" --host 127.0.0.1 --port "$port" \
            --enable-mutations "${options[@]}"
    ) > "$native_case/$mode-server.log" 2>&1 &
    server_pid=$!
    trap "kill $server_pid 2>/dev/null || true; wait $server_pid 2>/dev/null || true" EXIT
    for ((attempt=0; attempt<200; attempt++)); do
        if curl -fsS --max-time 1 -D "$native_case/$mode-headers" \
            -H 'Content-Type: application/json' -H 'Accept: application/json, text/event-stream' \
            -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"native-repo-e2e","version":"1"}}}' \
            "http://127.0.0.1:$port/mcp" > "$native_case/$mode-initialize.json" 2>/dev/null; then
            ready=true
            break
        fi
        kill -0 "$server_pid" 2>/dev/null || { cat "$native_case/$mode-server.log"; return 1; }
        sleep 0.1
    done
    [[ "$ready" == true ]] || { cat "$native_case/$mode-server.log"; return 1; }
    local session
    session=$(awk 'tolower($1)=="mcp-session-id:" {gsub("\r", "", $2); print $2}' "$native_case/$mode-headers")
    curl -fsS --max-time 15 \
        -H 'Content-Type: application/json' -H 'Accept: application/json, text/event-stream' \
        -H "Mcp-Session-Id: $session" \
        -d '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"reqvire.search","arguments":{"branch":"main"}}}' \
        "http://127.0.0.1:$port/mcp" > "$native_case/$mode-search.json"
    jq -e '.result.isError != true' "$native_case/$mode-search.json" >/dev/null
    jq '[.result.structuredContent.files[].elements[].name] | sort' \
        "$native_case/$mode-search.json" > "$native_case/$mode-names.json"
    diff -u "$TEST_SCRIPT_DIR/expected/native-names.json" "$native_case/$mode-names.json"
    if [[ -n "$callback" ]]; then "$callback"; fi
    kill "$server_pid"
    wait "$server_pid" 2>/dev/null || true
    trap - EXIT
}

native_rpc() {
    curl -fsS --max-time 30 \
        -H 'Content-Type: application/json' -H 'Accept: application/json, text/event-stream' \
        -H "Mcp-Session-Id: $session" --data-binary "@$1" \
        "http://127.0.0.1:$port/mcp" > "$2"
    jq -e '.result.isError != true and .error == null' "$2" >/dev/null || { cat "$2"; return 1; }
}

native_pin_mutation() {
    jq -n --rawfile content "$TEST_SCRIPT_DIR/fixtures/addition.md.txt" \
        '{jsonrpc:"2.0",id:3,method:"tools/call",params:{name:"reqvire.add_element",arguments:{branch:"main",file:"repositories/component/Added.md",content:$content}}}' \
        > "$native_case/add.json"
    native_rpc "$native_case/add.json" "$native_case/add-result.json"
    diff -u "$TEST_SCRIPT_DIR/expected/added.md.txt" "$central/repositories/component/Added.md"
    printf '%s\n' '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"reqvire.git.commit","arguments":{"branch":"main","message":"Save native Repo component addition"}}}' \
        > "$native_case/commit.json"
    native_rpc "$native_case/commit.json" "$native_case/commit-result.json"
}

native_repo_pin_checkpoint() {
    cp "$central/.repo/manifest.xml" "$native_case/expected-wrapper.xml"
    cp "$native" "$native_case/expected-includes.xml"
    central_head=$(git -C "$central" rev-parse HEAD)
    native_pin_mutation
    saved=$(git -C "$central/repositories/component" rev-parse HEAD)
    jq -e --arg saved "$saved" '.result.structuredContent.context.projects[] | select(.name == "component") | .revision == $saved' \
        "$native_case/commit-result.json" >/dev/null
    test "$saved" != "$alpha"
    test "$(git -C "$central/repositories/component" rev-parse HEAD^)" = "$alpha"
    test "$(git -C "$central" rev-parse HEAD)" = "$central_head"
    diff -u "$native_case/expected-wrapper.xml" "$central/.repo/manifest.xml"
    diff -u "$native_case/expected-includes.xml" "$native"
    grep -q "revision=\"$saved\"" "$central/.repo/manifests/parts/projects.xml"
    grep -q "revision=\"$saved\"" "$central/.repo/local_manifests/pins.xml"
    (cd "$central" && repo manifest -o "$native_case/saved-repo-manifest.xml")
    grep -q "revision=\"$saved\"" "$native_case/saved-repo-manifest.xml"
    (cd "$central" && "$REQVIRE_BIN" search --json) > "$native_case/saved-search.json"
    jq '[.files[].elements[].name] | sort' "$native_case/saved-search.json" > "$native_case/saved-names.json"
    jq '. + ["Added Capability"] | sort' "$TEST_SCRIPT_DIR/expected/native-names.json" > "$native_case/expected-saved-names.json"
    diff -u "$native_case/expected-saved-names.json" "$native_case/saved-names.json"
    test -z "$(git -C "$central" ls-files .repo default.xml)"
    test -z "$(git -C "$central" status --porcelain)"
}

native_worktree_mutation() {
    jq -n '{jsonrpc:"2.0",id:3,method:"tools/call",params:{name:"reqvire.branch.create",arguments:{branch:"native-topic",from_branch:"main",base_ref:"main"}}}' > "$native_case/create.json"
    native_rpc "$native_case/create.json" "$native_case/create-result.json"
    local child saved
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/create-result.json")
    test -f "$child/.repo/manifests/default.xml"
    test ! -e "$child/default.xml"
    jq -n --rawfile content "$TEST_SCRIPT_DIR/fixtures/addition.md.txt" \
        '{jsonrpc:"2.0",id:4,method:"tools/call",params:{name:"reqvire.add_element",arguments:{branch:"native-topic",file:"repositories/component/Added.md",content:$content}}}' > "$native_case/add.json"
    native_rpc "$native_case/add.json" "$native_case/add-result.json"
    jq -n '{jsonrpc:"2.0",id:5,method:"tools/call",params:{name:"reqvire.git.commit",arguments:{branch:"native-topic",message:"Save native branch component"}}}' > "$native_case/commit.json"
    native_rpc "$native_case/commit.json" "$native_case/commit-result.json"
    saved=$(git -C "$child/repositories/component" rev-parse HEAD)
    test "$saved" != "$alpha"
    jq -n '{jsonrpc:"2.0",id:6,method:"tools/call",params:{name:"reqvire.branch.close",arguments:{branch:"native-topic"}}}' > "$native_case/remove.json"
    native_rpc "$native_case/remove.json" "$native_case/remove-result.json"
    jq -n '{jsonrpc:"2.0",id:7,method:"tools/call",params:{name:"reqvire.branch.open",arguments:{branch:"native-topic"}}}' > "$native_case/open.json"
    native_rpc "$native_case/open.json" "$native_case/open-result.json"
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/open-result.json")
    test "$(git -C "$child/repositories/component" rev-parse HEAD)" = "$saved"
    grep -q "revision=\"$saved\"" "$child/.repo/manifests/default.xml"
    diff -u "$TEST_SCRIPT_DIR/expected/added.md.txt" "$child/repositories/component/Added.md"
    test "$(git -C "$central/repositories/component" rev-parse HEAD)" = "$alpha"
    test ! -e "$central/repositories/component/Added.md"
    test ! -e "$central/default.xml"
}

native_pass() {
    printf 'PASS %s\n' "$1" >> "$TEST_DIR/output/native-checks.txt"
}

active_repo_session() {
    # native_server_names has checked the initial snapshot on this session.
    native_pass cli/active-repo-manifest-selection
    native_repo_pin_checkpoint
    native_pass mcp/native-repo-pin-checkpoint
}

standalone_repo_session() {
    native_pass cli/standalone-repo-manifest-selection
    native_worktree_mutation
    native_pass mcp/native-repo-worktree
}

active_repo_manifest() {
    native_fixture active
    mkdir -p "$central/.repo/manifests/parts" "$central/.repo/local_manifests"
    sed "s/$alpha/$omega/" "$native" > "$central/.repo/manifests/parts/projects.xml"
    printf '<manifest><include name="parts/projects.xml"/></manifest>\n' > "$native"
    printf '<manifest><extend-project name="component" revision="%s"/></manifest>\n' \
        "$alpha" > "$central/.repo/local_manifests/pins.xml"
    (cd "$central" && repo manifest -o "$native_case/repo-manifest.xml")
    # The CLI can succeed only if it reads the include and local pin overlay:
    # the included manifest selects omega while the checkout is still alpha.
    cp "$native" "$native_case/expected-manifest.xml"
    git -C "$central/repositories/component" ls-files --stage > "$native_case/expected-index"
    native_cli_names
    diff -u "$native_case/expected-manifest.xml" "$native"
    git -C "$central/repositories/component" ls-files --stage > "$native_case/actual-index"
    diff -u "$native_case/expected-index" "$native_case/actual-index"
    test "$(git -C "$central/repositories/component" rev-parse HEAD)" = "$alpha"
    # Cover the entry symlink emitted by older Repo releases.
    mv "$central/.repo/manifest.xml" "$native_case/generated-wrapper.xml"
    ln -s manifests/default.xml "$central/.repo/manifest.xml"
    native_cli_names
    native_server_names serve
    # The CLI and embedded server cover the legacy symlink. Use the generated
    # entry for the shared MCP loading/pin-save sequence below.
    mv "$native_case/generated-wrapper.xml" "$central/.repo/manifest.xml"
    # An override changes the file location, including within Repo's own Git
    # manifest checkout; it must not require a tracked Central copy.
    sed "s/$omega/$alpha/" "$central/.repo/manifests/parts/projects.xml" \
        > "$central/.repo/manifests/alternate.xml"
    git -C "$central/.repo/manifests" add alternate.xml
    git -C "$central/.repo/manifests" -c commit.gpgSign=false commit -qm 'Alternate manifest location'
    manifest_head=$(git -C "$central/.repo/manifests" rev-parse HEAD)
    printf '<manifest>' > "$native"
    native_cli_names --manifest .repo/manifests/alternate.xml
    test "$(git -C "$central/.repo/manifests" rev-parse HEAD)" = "$manifest_head"
    cp "$native_case/expected-manifest.xml" "$native"
    git -C "$central/repositories/component" checkout -q --detach "$omega"
    if (cd "$central" && "$REQVIRE_BIN" validate) > "$native_case/mismatch.log" 2>&1; then
        echo 'Mismatching checkout was accepted' >&2
        return 1
    fi
    grep -Eq '\.repo/.*repositories/component|repositories/component.*\.repo/' "$native_case/mismatch.log"
    test ! -e "$central/default.xml"
    test -z "$(git -C "$central" status --porcelain)"
    git -C "$central/repositories/component" checkout -q --detach "$alpha"
    native_server_names mcp active_repo_session
}

standalone_repo_manifest() {
    native_fixture standalone
    cp "$native" "$native_case/download.xml"
    (cd "$central" && repo init -q -u "file://$native_case/download.xml" \
        --standalone-manifest --no-clone-bundle)
    # Move the original download away: Repo's cached manifest must be sufficient.
    mv "$native_case/download.xml" "$native_case/expected-manifest.xml"
    (cd "$central" && repo manifest -o "$native_case/repo-manifest.xml")
    native_cli_names
    native_server_names serve
    diff -u "$native_case/expected-manifest.xml" "$native"
    test ! -e "$central/default.xml"
    test -z "$(git -C "$central" status --porcelain)"
    native_server_names mcp standalone_repo_session
}

run_native_repo_cases() {
    local name result
    : > "$TEST_DIR/output/native-checks.txt"
    for name in active standalone; do
        result=0
        bash "$TEST_SCRIPT_DIR/check_native_repo.sh" "$name" > "$TEST_DIR/output/native-$name.log" 2>&1 || result=$?
        if [[ "$result" != 0 ]]; then
            cat "$TEST_DIR/output/native-$name.log" >&2
            status=1
        fi
    done
    # Keep independent outcomes in the golden order even though related checks
    # now share one fixture and MCP session. An unrun dependent check must fail.
    for name in cli/active-repo-manifest-selection cli/standalone-repo-manifest-selection \
        mcp/native-repo-pin-checkpoint mcp/native-repo-worktree; do
        if grep -Fxq "PASS $name" "$TEST_DIR/output/native-checks.txt"; then
            printf 'PASS %s\n' "$name" >> "$TEST_DIR/output/checks.txt"
        else
            printf 'FAIL %s\n' "$name" >> "$TEST_DIR/output/checks.txt"
            status=1
        fi
    done
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    set -euo pipefail
    "${1}_repo_manifest"
fi
