#!/usr/bin/env bash
# Real Repo/Git acceptance with Bash, curl, jq and golden-file comparisons.
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$TEST_SCRIPT_DIR/check_change_impact.sh"

ws_manifest() {
    local revision=$1 fetch=${2:-file://$native_case/remotes/} name
    fixture_manifest > "$central/reqvire-workspace.toml"
    fixture_project component "$revision" component "$fetch" >> "$central/reqvire-workspace.toml"
    for name in secondary retained; do
        if [[ -f "$native_case/$name-revision" ]]; then
            fixture_project "$name" "$(cat "$native_case/$name-revision")" "$name" "$fetch" >> "$central/reqvire-workspace.toml"
        fi
    done
}

ws_fixture() {
    local name=$1 multiple=${2:-false} project source runtime
    native_case="$TEST_DIR/repo-cases/$name"
    central="$native_case/central"
    export GIT_ALLOW_PROTOCOL=file GIT_TERMINAL_PROMPT=0
    export GIT_AUTHOR_NAME='Repo Acceptance' GIT_AUTHOR_EMAIL='repo-test@example.invalid'
    export GIT_COMMITTER_NAME="$GIT_AUTHOR_NAME" GIT_COMMITTER_EMAIL="$GIT_AUTHOR_EMAIL"
    if [[ ! -f "$native_case/provenance.json" ]]; then
        mkdir -p "$native_case/remotes" "$native_case/component-seed" "$central"
        source=component.md.txt
        [[ "$multiple" != true ]] || source=commit-component.md.txt
        cp "$TEST_SCRIPT_DIR/fixtures/$source" "$native_case/component-seed/Model.md"
        git -C "$native_case/component-seed" init -qb main
        alpha=$(impact_commit "$native_case/component-seed" 'Component alpha')
        sed -i 's/behavior is alpha\./behavior is omega./' "$native_case/component-seed/Model.md"
        omega=$(impact_commit "$native_case/component-seed" 'Component omega')
        git clone -q --bare "$native_case/component-seed" "$native_case/remotes/component"
        if [[ "$multiple" == true ]]; then
            for project in secondary retained; do
                mkdir "$native_case/$project-seed"
                if [[ "$project" == secondary ]]; then
                    sed 's/Component/Secondary/g;s/component-/secondary-/g' "$TEST_SCRIPT_DIR/fixtures/$source" > "$native_case/$project-seed/Model.md"
                else cp "$TEST_SCRIPT_DIR/fixtures/retained.md.txt" "$native_case/$project-seed/Model.md"; fi
                git -C "$native_case/$project-seed" init -qb main
                impact_commit "$native_case/$project-seed" "Initial $project" > "$native_case/$project-revision"
                git clone -q --bare "$native_case/$project-seed" "$native_case/remotes/$project"
            done
        fi
        git -C "$central" init -qb main
        source=central.md.txt
        [[ "$multiple" != true ]] || source=commit-central.md.txt
        cp "$TEST_SCRIPT_DIR/fixtures/$source" "$central/Model.md"
        printf '/repositories\n/.repo\n' > "$central/.gitignore"
        printf '**/.repo/**\n' > "$central/.reqvireignore"
        ws_manifest "$alpha"
        initial_central=$(impact_commit "$central" 'Root alpha selection')
        git clone -q --bare "$central" "$native_case/remotes/central"
        git -C "$central" remote add origin "$native_case/remotes/central"
        ws_manifest "$omega"
        review=$(impact_commit "$central" 'Root omega selection')
        git -C "$central" branch review
        git -C "$central" reset -q --hard "$initial_central"
        (cd "$central" && "$REQVIRE_BIN" workspace sync) > "$native_case/workspace-setup.log" 2>&1
        for project in "$central"/repositories/*; do git -C "$project" checkout -q -B main; done
        jq -n --arg alpha "$alpha" --arg omega "$omega" --arg root "$initial_central" --arg review "$review" \
            '{alpha:$alpha,omega:$omega,root:$root,review:$review}' > "$native_case/provenance.json"
    fi
    alpha=$(jq -r .alpha "$native_case/provenance.json")
    omega=$(jq -r .omega "$native_case/provenance.json")
    initial_central=$(jq -r .root "$native_case/provenance.json")
    review=$(jq -r .review "$native_case/provenance.json")
}

ws_start() {
    local mode=${1:-mcp}
    if [[ -f "$native_case/session.json" ]]; then
        impact_pid=$(jq -r .pid "$native_case/session.json")
        impact_url=$(jq -r .url "$native_case/session.json")
        impact_session=$(jq -r .session "$native_case/session.json")
    else
        impact_start_server "$mode" central
        # The suite owns shared sessions, including when an individual case fails.
        trap - EXIT
        jq -cn --argjson pid "$impact_pid" --arg url "$impact_url" --arg session "$impact_session" \
            '{pid:$pid,url:$url,session:$session}' > "$native_case/session.json"
        jq -cn --arg fixture "$native_case" --arg mode "$mode" --argjson pid "$impact_pid" \
            '{fixture:$fixture,mode:$mode,pid:$pid}' >> "$TEST_DIR/output/repo-server-starts.jsonl"
    fi
}
ws_rpc() { impact_rpc "$1" "$2" "$3" > /dev/null; }
ws_cli() { (cd "$central" && "$REQVIRE_BIN" --manifest reqvire-workspace.toml "$@"); }
ws_names() { jq '[.result.structuredContent.files[].elements[].name] | sort' "$1"; }
ws_git_state() {
    local owner
    for owner in "$central" "$central"/repositories/*; do
        [[ -e "$owner/.git" ]] || continue
        printf 'OWNER %s\n' "$owner"
        git -C "$owner" rev-parse HEAD
        git -C "$owner" ls-files --stage
    done
}
ws_state() {
    ws_git_state
    local file
    for file in "$central/Model.md" "$central/reqvire-workspace.toml" "$central/reqvire-workspace.toml" "$central"/repositories/*/Model.md "$central"/repositories/*/Added.md; do
        if [[ -f "$file" ]]; then sha256sum "$file"; else printf 'ABSENT %s\n' "$file"; fi
    done
}
ws_retained() {
    git -C "$central/repositories/retained" rev-parse HEAD
    git -C "$central/repositories/retained" ls-files --stage
    git -C "$central/repositories/retained" for-each-ref --format='%(refname) %(objectname)'
    git -C "$central/repositories/retained" status --porcelain
    sha256sum "$central/repositories/retained/Model.md"
}
ws_add() {
    local args
    args=$(jq -n --rawfile content "$TEST_SCRIPT_DIR/fixtures/addition.md.txt" '{branch:"main",file:"repositories/component/Added.md",content:$content}')
    ws_rpc reqvire.add_element "$args" "$native_case/add.json"
}
ws_saved() {
    local owner line name revision
    : > "$native_case/expected-pins.txt"
    for owner in "$central"/repositories/*; do
        printf '%s %s\n' "${owner##*/}" "$(git -C "$owner" rev-parse HEAD)" >> "$native_case/expected-pins.txt"
        test -z "$(git -C "$owner" status --porcelain)"
    done
    git -C "$central" show HEAD:reqvire-workspace.toml > "$native_case/saved.toml"
    : > "$native_case/actual-pins.txt"
    awk '/^name = / {name=$3;gsub(/"/,"",name)} /^commit = / {commit=$3;gsub(/"/,"",commit);print name,commit}' \
        "$native_case/saved.toml" > "$native_case/actual-pins.txt"
    sort -o "$native_case/actual-pins.txt" "$native_case/actual-pins.txt"
    diff -u "$native_case/expected-pins.txt" "$native_case/actual-pins.txt"
    test -z "$(git -C "$central" ls-files repositories)"
    test -z "$(git -C "$central" status --porcelain)"
    ws_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status.json"
    jq -e --arg head "$(git -C "$central" rev-parse HEAD)" \
        '.result.structuredContent.context | .pending_changes == [] and .head == $head' "$native_case/status.json" > /dev/null
}

ws_snapshot() {
    ws_fixture "$1-snapshot"
    ws_start "$1"
    (cd "$central" && "$REQVIRE_BIN" search --json) > "$native_case/cli.json"
    jq '[.files[].elements[].name]|sort' "$native_case/cli.json" > "$native_case/cli-names.json"
    diff -u "$TEST_SCRIPT_DIR/expected/native-names.json" "$native_case/cli-names.json"
    ws_rpc reqvire.search '{"branch":"main"}' "$native_case/search.json"
    ws_names "$native_case/search.json" > "$native_case/mcp-names.json"
    diff -u "$native_case/cli-names.json" "$native_case/mcp-names.json"
    ws_rpc reqvire.read_element '{"branch":"main","name":"Component Specification"}' "$native_case/element.json"
    grep -Fq 'repositories/component/Model.md#component-specification' "$native_case/element.json"
    ws_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status.json"
    jq -e '.result.structuredContent.context.workspace_mode == "repo"' "$native_case/status.json" > /dev/null
}

ws_explicit() {
    ws_fixture explicit-manifest
    ws_cli search --json > "$native_case/initial.json"
    jq '[.files[].elements[].name]|sort' "$native_case/initial.json" > "$native_case/names.json"
    diff -u "$TEST_SCRIPT_DIR/expected/native-names.json" "$native_case/names.json"
    cp "$central/Model.md" "$native_case/original.md"
    mkdir "$central/evidence" "$native_case/resolver-bin"
    cp "$TEST_SCRIPT_DIR/fixtures/resolver-double.sh" "$native_case/resolver-bin/git"
    chmod +x "$native_case/resolver-bin/git"
    local count i first=0 current url
    for count in 1 64; do
        : > "$native_case/relations.txt"
        for ((i=0;i<count;i++)); do
            printf 'Evidence\n' > "$central/evidence/$i.txt"
            printf '  * satisfiedBy: [Evidence %s](evidence/%s.txt)\n' "$i" "$i" >> "$native_case/relations.txt"
        done
        awk -v file="$native_case/relations.txt" '{print} /^#### Relations$/ {while ((getline line < file)>0) print line; close(file)}' \
            "$native_case/original.md" > "$central/Model.md"
        : > "$native_case/resolver-launches.txt"
        REQVIRE_REPO_TEST_GIT=$(command -v git) REQVIRE_REPO_TEST_RESOLVER_LOG="$native_case/resolver-launches.txt" \
            PATH="$native_case/resolver-bin:$PATH" ws_cli validate > /dev/null
        current=$(wc -l < "$native_case/resolver-launches.txt")
        if [[ "$count" == 1 ]]; then first=$current; else test "$current" = "$first"; fi
        test "$current" -gt 0
    done
    cp "$native_case/original.md" "$central/Model.md"
    rm -r "$central/evidence"
    cp "$central/reqvire-workspace.toml" "$native_case/original.toml"
    ws_state > "$native_case/before.txt"
    ws_cli search --json > "$native_case/selected.json"
    ws_state > "$native_case/after.txt"
    diff -u "$native_case/before.txt" "$native_case/after.txt"
    git -C "$central/repositories/component" remote rename origin local
    for url in "$native_case/remotes/component" git@github.com:test/component.git; do
        sed "s|file://$native_case/remotes/component|$url|" "$native_case/original.toml" > "$central/reqvire-workspace.toml"
        git -C "$central/repositories/component" remote set-url local "$url"
        ws_cli search --json > "$native_case/alias.json"
        jq '[.files[].elements[].name]|sort' "$native_case/alias.json" > "$native_case/names.json"
        diff -u "$TEST_SCRIPT_DIR/expected/native-names.json" "$native_case/names.json"
    done
    if (cd "$central" && "$REQVIRE_BIN" --manifest missing.toml validate) > "$native_case/missing.log" 2>&1; then return 1; fi
    grep -Fq missing.toml "$native_case/missing.log"
    ! grep -Fq 'unexpected argument' "$native_case/missing.log"
}

ws_boundaries() {
    ws_fixture boundaries
    cp "$central/reqvire-workspace.toml" "$native_case/original.toml"
    ln -s "$native_case/component-seed" "$central/repositories/escape"
    local key replacement
    for key in duplicate escape commit missing symlink overlap foreign; do
        cp "$native_case/original.toml" "$central/reqvire-workspace.toml"
        case "$key" in
            duplicate|overlap)
                replacement=component
                [[ "$key" != overlap ]] || replacement+=/nested
                fixture_project another "$alpha" "$replacement" >> "$central/reqvire-workspace.toml" ;;
            escape|missing|symlink)
                replacement=../outside
                [[ "$key" != missing ]] || replacement=repositories/missing
                [[ "$key" != symlink ]] || replacement=repositories/escape
                sed -i "s@path = \"repositories/component\"@path = \"$replacement\"@" "$central/reqvire-workspace.toml" ;;
            commit) sed -i "s/$alpha/0000000000000000000000000000000000000000/" "$central/reqvire-workspace.toml" ;;
            foreign) git -C "$central/repositories/component" remote set-url origin "$native_case/foreign" ;;
        esac
        ws_state > "$native_case/before.txt"
        if ws_cli validate --json > "$native_case/$key.log" 2>&1; then echo "Invalid $key accepted" >&2; return 1; fi
        grep -Fq reqvire-workspace.toml "$native_case/$key.log"
        ws_state > "$native_case/after.txt"
        diff -u "$native_case/before.txt" "$native_case/after.txt"
    done
}

ws_branch() {
    ws_fixture explicit-commit true
    ws_start
    ws_state > "$native_case/before.txt"
    ws_rpc reqvire.branch.create '{"branch":"topic","from_branch":"main","base_ref":"review"}' "$native_case/create.json"
    local child
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/create.json")
    jq -e --arg review "$review" '.result.structuredContent.base_commit == $review' "$native_case/create.json" > /dev/null
    test "$(git -C "$child/repositories/component" rev-parse HEAD)" = "$omega"
    test "$(git -C "$child/repositories/component" symbolic-ref --short HEAD)" = topic
    test "$(git -C "$child/repositories/component" rev-parse --path-format=absolute --git-dir)" != "$(git -C "$central/repositories/component" rev-parse --path-format=absolute --git-dir)"
    ws_rpc reqvire.read_element '{"branch":"topic","name":"Component Specification"}' "$native_case/omega.json"
    grep -Fq omega "$native_case/omega.json"
    ws_rpc reqvire.read_element '{"branch":"main","name":"Component Specification"}' "$native_case/alpha.json"
    grep -Fq alpha "$native_case/alpha.json"
    ws_state > "$native_case/after.txt"
    diff -u "$native_case/before.txt" "$native_case/after.txt"
    ! git -C "$central/repositories/component" show-ref --verify --quiet refs/heads/topic
    ws_rpc reqvire.branch.close '{"branch":"topic"}' "$native_case/close.json"
    ws_rpc reqvire.branch.open '{"branch":"topic"}' "$native_case/open.json"
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/open.json")
    test "$(git -C "$child/repositories/component" rev-parse HEAD)" = "$omega"
}

ws_mutation() {
    ws_fixture mcp-snapshot
    ws_start
    ws_state > "$native_case/before.txt"
    ws_rpc reqvire.model_revision '{"branch":"main"}' "$native_case/revision-before.json"
    local args='{"branch":"main","element_name":"Component Specification","new_name":"Updated Component Specification"}'
    ws_rpc reqvire.rename_element "$(jq '.dry_run=true' <<< "$args")" "$native_case/preview.json"
    grep -Fq repositories/component/Model.md "$native_case/preview.json"
    if ws_rpc reqvire.rename_element "$(jq '.new_name="Central Capability"' <<< "$args")" "$native_case/rejected.json"; then return 1; fi
    ws_state > "$native_case/after.txt"
    diff -u "$native_case/before.txt" "$native_case/after.txt"
    ws_rpc reqvire.model_revision '{"branch":"main"}' "$native_case/revision-preview.json"
    jq -eS '.result.structuredContent.context.model_revision' "$native_case/revision-before.json" > "$native_case/revision-before.txt"
    jq -eS '.result.structuredContent.context.model_revision' "$native_case/revision-preview.json" > "$native_case/revision-preview.txt"
    diff -u "$native_case/revision-before.txt" "$native_case/revision-preview.txt"
    ws_git_state > "$native_case/git-before.txt"
    ws_rpc reqvire.rename_element "$args" "$native_case/rename.json"
    diff -u "$TEST_SCRIPT_DIR/expected/central-renamed.md.txt" "$central/Model.md"
    diff -u "$TEST_SCRIPT_DIR/expected/component-renamed.md.txt" "$central/repositories/component/Model.md"
    ws_git_state > "$native_case/git-after.txt"
    diff -u "$native_case/git-before.txt" "$native_case/git-after.txt"
    ws_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status.json"
    jq -e '.result.structuredContent.context.pending_changes | sort == ["Model.md","repositories/component/Model.md"]' "$native_case/status.json" > /dev/null
    jq -e --arg before "$(jq -er '.result.structuredContent.context.model_revision' "$native_case/revision-before.json")" \
        '.result.structuredContent.context.model_revision != $before' "$native_case/status.json" > /dev/null
}

ws_commits() {
    local automatic=$1 name project previous_root previous_component
    name=explicit-commit
    composition_server_options=()
    if [[ "$automatic" == true ]]; then name=automatic-commit; composition_server_options=(--enable-commits); fi
    ws_fixture "$name" true
    ws_start
    ws_retained > "$native_case/retained-before.txt"
    ws_git_state > "$native_case/git-before.txt"
    ws_rpc reqvire.rename_element '{"branch":"main","element_name":"Shared Specification","new_name":"Updated Shared Specification"}' "$native_case/rename.json"
    if [[ "$automatic" == false ]]; then
        ws_git_state > "$native_case/git-pending.txt"
        diff -u "$native_case/git-before.txt" "$native_case/git-pending.txt"
        ws_rpc reqvire.git.commit '{"branch":"main","message":"Save combined contract rename"}' "$native_case/commit.json"
    fi
    for project in component secondary; do
        diff -u "$TEST_SCRIPT_DIR/expected/commit-$project-renamed.md.txt" "$central/repositories/$project/Model.md"
        git -C "$central/repositories/$project" show HEAD:Model.md > "$native_case/$project-saved.md"
        diff -u "$TEST_SCRIPT_DIR/expected/commit-$project-renamed.md.txt" "$native_case/$project-saved.md"
        if [[ "$project" == component ]]; then name=$alpha; else name=$(cat "$native_case/secondary-revision"); fi
        test "$(git -C "$central/repositories/$project" rev-parse HEAD^)" = "$name"
    done
    diff -u "$TEST_SCRIPT_DIR/expected/commit-central-renamed.md.txt" "$central/Model.md"
    test "$(git -C "$central" rev-parse HEAD^)" = "$initial_central"
    ws_saved
    ws_retained > "$native_case/retained-after-rename.txt"
    diff -u "$native_case/retained-before.txt" "$native_case/retained-after-rename.txt"
    ws_cli search --json > "$native_case/saved-model.json"
    jq '[.files[].elements[].name]|sort' "$native_case/saved-model.json" > "$native_case/saved-names.json"
    diff -u "$TEST_SCRIPT_DIR/expected/commit-names.json" "$native_case/saved-names.json"
    previous_root=$(git -C "$central" rev-parse HEAD)
    previous_component=$(git -C "$central/repositories/component" rev-parse HEAD)
    git -C "$central/repositories/secondary" rev-parse HEAD > "$native_case/secondary-before.txt"
    ws_git_state > "$native_case/before-add.txt"
    ws_add
    if [[ "$automatic" == false ]]; then
        ws_git_state > "$native_case/pending-add.txt"
        diff -u "$native_case/before-add.txt" "$native_case/pending-add.txt"
        ws_rpc reqvire.git.commit '{"branch":"main","message":"Save component addition"}' "$native_case/commit.json"
    fi
    diff -u "$TEST_SCRIPT_DIR/expected/added.md.txt" "$central/repositories/component/Added.md"
    git -C "$central/repositories/component" show HEAD:Added.md > "$native_case/saved-addition.md"
    diff -u "$TEST_SCRIPT_DIR/expected/added.md.txt" "$native_case/saved-addition.md"
    test "$(git -C "$central" rev-parse HEAD^)" = "$previous_root"
    test "$(git -C "$central/repositories/component" rev-parse HEAD^)" = "$previous_component"
    git -C "$central/repositories/secondary" rev-parse HEAD > "$native_case/secondary-after.txt"
    diff -u "$native_case/secondary-before.txt" "$native_case/secondary-after.txt"
    ws_retained > "$native_case/retained-after.txt"
    diff -u "$native_case/retained-before.txt" "$native_case/retained-after.txt"
    ws_saved
    ws_rpc reqvire.search '{"branch":"main"}' "$native_case/added-model.json"
    jq -e 'any(.result.structuredContent.files[].elements[]; .name == "Added Capability")' "$native_case/added-model.json" > /dev/null
    ws_git_state > "$native_case/before-noop.txt"
    ws_rpc reqvire.git.commit '{"branch":"main","message":"No further changes"}' "$native_case/noop.json"
    ws_git_state > "$native_case/after-noop.txt"
    diff -u "$native_case/before-noop.txt" "$native_case/after-noop.txt"
}

ws_failed_commit() {
    ws_fixture checkpoint-failure
    ws_start
    ws_add
    printf '#!/bin/sh\nif [ "$1" = prepared ]; then exit 1; fi\n' > "$central/.git/hooks/reference-transaction"
    chmod +x "$central/.git/hooks/reference-transaction"
    if ws_rpc reqvire.git.commit '{"branch":"main","message":"Rejected root checkpoint"}' "$native_case/failure.json"; then return 1; fi
    test "$(git -C "$central" rev-parse HEAD)" = "$initial_central"
    test "$(git -C "$central/repositories/component" rev-parse HEAD)" != "$alpha"
    git -C "$central/repositories/component" show HEAD:Added.md > "$native_case/retained.md"
    diff -u "$TEST_SCRIPT_DIR/expected/added.md.txt" "$native_case/retained.md"
    ws_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status.json"
    jq -e '.result.structuredContent.context.pending_changes | length > 0' "$native_case/status.json" > /dev/null
    ws_rpc reqvire.search '{"branch":"main"}' "$native_case/accepted.json"
    jq -e 'any(.result.structuredContent.files[].elements[]; .name == "Added Capability")' "$native_case/accepted.json" > /dev/null
}

ws_push() {
    local denied=$1 initial_remote saved_root saved_component
    ws_fixture push
    if [[ ! -d "$native_case/bin" ]]; then
        mkdir "$native_case/bin" "$native_case/github-state"
        printf '{}\n' > "$native_case/github-state/scenario.json"
        cp "$TEST_SCRIPT_DIR/fixtures/git-double.sh" "$native_case/bin/git"
        cp "$TEST_SCRIPT_DIR/../test-mcp-publication/fixtures/gh-double.sh" "$native_case/bin/gh"
        chmod +x "$native_case/bin/git" "$native_case/bin/gh"
        git -C "$central" remote set-url origin https://github.com/test/project.git
        # Both checkouts represent this same source history under the fixture's
        # controlled web identity; Repo administration is not another owner.
        git -C "$central/repositories/component" remote set-url origin https://github.com/test/component
        ws_manifest "$alpha" https://github.com/test/
        impact_commit "$central" 'Register publication identities' > /dev/null
        git -C "$central/repositories/component" switch -qc dependency-review
        cp "$TEST_SCRIPT_DIR/expected/added.md.txt" "$central/repositories/component/Added.md"
        saved_component=$(impact_commit "$central/repositories/component" 'Saved component dependency')
        ws_manifest "$saved_component" https://github.com/test/
        impact_commit "$central" 'Saved root dependency selection' > /dev/null
    fi
    export REQVIRE_REPO_TEST_GIT_BINARY REQVIRE_REPO_TEST_REMOTES
    REQVIRE_REPO_TEST_GIT_BINARY=$(command -v git)
    REQVIRE_REPO_TEST_REMOTES=$(jq -cn --arg root "$native_case/remotes" \
        '{"https://github.com/test/project.git":($root+"/central"),"https://github.com/test/component":($root+"/component")}')
    export REQVIRE_GH_TEST_STATE="$native_case/github-state" REQVIRE_REPO_TEST_PUSHES="$native_case/pushes.jsonl"
    export REQVIRE_REPO_TEST_FAIL_COMPONENT="$native_case/deny-component"
    export PATH="$native_case/bin:$PATH"
    composition_server_options=(--enable-github)
    ws_start
    initial_remote=$(git -C "$native_case/remotes/central" rev-parse refs/heads/main)
    saved_root=$(git -C "$central" rev-parse HEAD)
    saved_component=$(git -C "$central/repositories/component" rev-parse HEAD)
    : > "$REQVIRE_REPO_TEST_PUSHES"
    if [[ "$denied" == true ]]; then
        touch "$REQVIRE_REPO_TEST_FAIL_COMPONENT"
        if ws_rpc reqvire.git.push '{"branch":"main"}' "$native_case/push-failed.json"; then return 1; fi
        test "$(git -C "$native_case/remotes/central" rev-parse refs/heads/main)" = "$initial_remote"
        jq -e -s 'all(.[]; all(.targets[]; contains("project.git") | not))' "$REQVIRE_REPO_TEST_PUSHES" > /dev/null
    else
        rm -f "$REQVIRE_REPO_TEST_FAIL_COMPONENT"
        ws_rpc reqvire.git.push '{"branch":"main"}' "$native_case/push.json"
        test "$(git -C "$native_case/remotes/central" rev-parse refs/heads/main)" = "$saved_root"
        test "$(git -C "$native_case/remotes/component" rev-parse refs/heads/dependency-review)" = "$saved_component"
    fi
    jq -e -s '.[0].targets[0] | contains("component")' "$REQVIRE_REPO_TEST_PUSHES" > /dev/null
    test "$(git -C "$central" rev-parse HEAD)" = "$saved_root"
    test "$(git -C "$central/repositories/component" rev-parse HEAD)" = "$saved_component"
}

ws_case() {
    local key=$1
    case "$key" in
        mcp|serve) ws_snapshot "$key" ;;
        explicit) ws_explicit ;;
        invalid)
            ws_fixture invalid
            printf 'version = [' > "$central/reqvire-workspace.toml"
            ws_state > "$native_case/before.txt"
            if (cd "$central" && "$REQVIRE_BIN" validate --json) > "$native_case/invalid.log" 2>&1; then return 1; fi
            grep -Fq reqvire-workspace.toml "$native_case/invalid.log"
            ws_state > "$native_case/after.txt"
            diff -u "$native_case/before.txt" "$native_case/after.txt" ;;
        boundaries) ws_boundaries ;;
        declared)
            ws_fixture declared
            mkdir "$central/repositories/undeclared"
            git -C "$central/repositories/undeclared" init -qb main
            printf '# Elements\n\n### Undeclared Capability\n\nOutside the selection.\n\n#### Metadata\n  * type: capability\n' > "$central/repositories/undeclared/Model.md"
            impact_commit "$central/repositories/undeclared" 'Undeclared source' > /dev/null
            (cd "$central" && "$REQVIRE_BIN" search --json) > "$native_case/search.json"
            jq '[.files[].elements[].name]|sort' "$native_case/search.json" > "$native_case/names.json"
            diff -u "$TEST_SCRIPT_DIR/expected/native-names.json" "$native_case/names.json" ;;
        branches) ws_branch ;;
        mutation) ws_mutation ;;
        commit|automatic) ws_commits "$([[ "$key" == automatic ]] && echo true || echo false)" ;;
        dirty)
            ws_fixture dirty
            printf '\nExternal edit\n' >> "$central/repositories/component/Model.md"
            ws_state > "$native_case/before.txt"
            if impact_start_server mcp central; then echo 'Dirty component admitted' >&2; return 1; fi
            jq -cn --arg fixture "$native_case" --argjson pid "$impact_pid" \
                '{fixture:$fixture,mode:"mcp",pid:$pid,admitted:false}' >> "$TEST_DIR/output/repo-server-starts.jsonl"
            grep -Fq repositories/component "$native_case/server.log"
            ws_state > "$native_case/after.txt"
            diff -u "$native_case/before.txt" "$native_case/after.txt" ;;
        failed_commit) ws_failed_commit ;;
        explorer)
            ws_fixture serve-snapshot
            ws_start serve
            curl -fsS "${impact_url%/mcp}/api/project-store?branch=main" > "$native_case/store.json"
            jq '[.store.elements[].name]|sort' "$native_case/store.json" > "$native_case/names.json"
            diff -u "$TEST_SCRIPT_DIR/expected/native-names.json" "$native_case/names.json"
            curl -fsS "${impact_url%/mcp}/api/project-store/manifest?branch=main" > "$native_case/manifest-before.json"
            ws_rpc reqvire.add_element "$(jq -n --rawfile content "$TEST_SCRIPT_DIR/fixtures/addition.md.txt" \
                '{branch:"main",file:"repositories/component/Added.md",content:$content,dry_run:true}')" "$native_case/preview.json"
            curl -fsS "${impact_url%/mcp}/api/project-store/manifest?branch=main" > "$native_case/manifest-preview.json"
            diff -u "$native_case/manifest-before.json" "$native_case/manifest-preview.json"
            ws_add
            curl -fsS "${impact_url%/mcp}/api/project-store?branch=main" > "$native_case/store-added.json"
            jq -e 'any(.store.elements[]; .name == "Added Capability")' "$native_case/store-added.json" > /dev/null
            curl -fsS "${impact_url%/mcp}/api/project-store/manifest?branch=main" > "$native_case/manifest-added.json"
            ! cmp -s "$native_case/manifest-before.json" "$native_case/manifest-added.json"
            ws_rpc reqvire.git.commit '{"branch":"main","message":"Save browser fixture"}' "$native_case/commit.json"
            ws_rpc reqvire.branch.create '{"branch":"topic","from_branch":"main","base_ref":"review"}' "$native_case/topic.json"
            bun "$TEST_SCRIPT_DIR/browser-repo.mjs" "${impact_url%/mcp}" "$native_case/browser-profile" > "$native_case/browser.log" 2>&1 ;;
        push_failed) ws_push true ;;
        push) ws_push false ;;
    esac
}

ws_cleanup() {
    local pid
    while IFS= read -r pid; do kill "$pid" 2>/dev/null || true; wait "$pid" 2>/dev/null || true; done \
        < <(jq -r .pid "$TEST_DIR/output/repo-server-starts.jsonl")
}
run_repo_workspace_cases() {
    local key label ws_parent=$BASHPID
    mkdir -p "$TEST_DIR/output"
    : > "$TEST_DIR/output/checks.txt"
    : > "$TEST_DIR/output/repo-server-starts.jsonl"
    trap '[[ "$BASHPID" != "$ws_parent" ]] || ws_cleanup' EXIT
    while read -r key label; do
        (set -Eeuo pipefail; trap 'printf "FAIL shell Repo case %s at line %s: %s\n" "$key" "$LINENO" "$BASH_COMMAND" >&2' ERR; ws_case "$key") \
            > "$TEST_DIR/output/repo-$key.log" 2>&1
        if [[ $? == 0 ]]; then printf 'PASS %s\n' "$label" >> "$TEST_DIR/output/checks.txt"
        else printf 'FAIL %s\n' "$label" >> "$TEST_DIR/output/checks.txt"; cat "$TEST_DIR/output/repo-$key.log" >&2; status=1; fi
    done <<'CASES'
mcp mcp/complete-declared-snapshot
serve serve/complete-declared-snapshot
explicit cli/explicit-manifest-selection
invalid cli/invalid-selected-manifest-rejected
boundaries cli/invalid-project-boundaries-rejected
declared cli/only-declared-project-sources
branches mcp/central-branch-component-revisions
mutation mcp/cross-project-preview-apply
commit mcp/explicit-component-first-commit
automatic mcp/automatic-component-first-commit
dirty mcp/dirty-component-admission
failed_commit mcp/failed-central-checkpoint-retains-component
explorer serve/combined-publication-and-branch-picker
push_failed mcp/failed-dependency-push-preserves-central
push mcp/dependency-first-push
CASES
    ws_cleanup
    trap - EXIT
    jq -s '{server_starts:length,servers:.}' "$TEST_DIR/output/repo-server-starts.jsonl" > "$TEST_DIR/output/repo-summary.json"
}
