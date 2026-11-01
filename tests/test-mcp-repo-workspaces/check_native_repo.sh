#!/bin/bash
# Native Repo selection uses real local Git remotes and actual Repo metadata.
# The root Git history owns the source manifest; Repo administration is derived.
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

native_fixture() {
    local root_model=${2:-model} layout=${3:-flat} source_name=default.xml
    [[ "$layout" != nested ]] || source_name=manifests/default.xml
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
    git -C "$native_case/remotes/component" branch next "$omega"
    git -C "$native_case/remotes/component" update-ref refs/heads/main "$alpha"
    central="$native_case/central"
    git -C "$central" init -qb main
    if [[ "$root_model" == model ]]; then
        cp "$TEST_SCRIPT_DIR/fixtures/central.md.txt" "$central/Model.md"
    fi
    printf '.repo/\nrepositories/\n' > "$central/.gitignore"
    mkdir -p "$(dirname "$central/$source_name")"
    cat > "$central/$source_name" <<EOF
<manifest>
  <remote name="local" fetch="file://$native_case/remotes/"/>
  <project name="component" path="repositories/component" remote="local" revision="$alpha"/>
</manifest>
EOF
    git -C "$central" add .
    git -C "$central" -c commit.gpgSign=false commit -qm Central
    git clone -q --bare "$central" "$native_case/remotes/central"
    git -C "$central" remote add origin "file://$native_case/remotes/central"
    (
        cd "$central"
        repo init -q -u "file://$native_case/remotes/central" -b main -m "$source_name" \
            --repo-url "file://$native_source" \
            --repo-rev "$(git -C "$native_source" rev-parse HEAD)" \
            --no-repo-verify --no-clone-bundle
        repo sync -q --no-clone-bundle --no-tags
    )
    source_manifest="$central/$source_name"
    native="$central/.repo/manifests/$source_name"
}
