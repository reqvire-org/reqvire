description = "Google Repo launcher and pinned source for offline workspace tests."
binaries = ["repo"]
requires = ["python3-3.12.3"]
source = "https://gerrit.googlesource.com/git-repo.git#46e9866ca0f161604bd45a0c3b7c7887c9415dcb"
env = { "REQVIRE_TEST_REPO_SOURCE": "${root}" }
version "2.70" {}

# Hermit's pinned Git source is detached. Expose its commit to Repo's local
# bootstrap fetch, which discovers heads/tags rather than a detached HEAD.
on "unpack" {
  run {
    cmd = "/usr/bin/git"
    args = ["update-ref", "refs/heads/reqvire-source", "HEAD"]
  }
}
