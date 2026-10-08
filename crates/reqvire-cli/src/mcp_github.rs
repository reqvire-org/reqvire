//! Optional typed same-repository publication. No generic gh or shell proxy.
use crate::mcp_process::{git, run, Output};
use reqvire::error::ReqvireError;
use serde_json::{json, Value};
use std::{path::Path, time::Duration};
const CHECK_TIMEOUT: Duration = Duration::from_secs(10);
const OP_TIMEOUT: Duration = Duration::from_secs(45);
fn error(message: impl Into<String>) -> ReqvireError {
    ReqvireError::ProcessError(message.into())
}
#[derive(Clone, Debug, PartialEq)]
struct Repository {
    host: String,
    name: String,
}
impl Repository {
    fn parse(url: &str) -> Result<Self, ReqvireError> {
        let (host, path) = if let Some(rest) = url.strip_prefix("https://") {
            let (host, path) = rest
                .split_once('/')
                .ok_or_else(|| error("Remote requires host/owner/repository"))?;
            if host.contains('@') {
                return Err(error("Credential-bearing remote URLs are not supported"));
            }
            (host, path)
        } else if let Some(rest) = url.strip_prefix("ssh://git@") {
            rest.split_once('/')
                .ok_or_else(|| error("Invalid SSH repository URL"))?
        } else if let Some(rest) = url.strip_prefix("git@") {
            rest.split_once(':')
                .ok_or_else(|| error("Invalid SSH repository URL"))?
        } else {
            return Err(error(
                "GitHub publication requires an HTTPS or git@ SSH remote",
            ));
        };
        let path = path
            .trim_end_matches('/')
            .strip_suffix(".git")
            .unwrap_or_else(|| path.trim_end_matches('/'));
        let parts: Vec<_> = path.split('/').collect();
        if host.is_empty()
            || !host
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b".-".contains(&c))
            || parts.len() != 2
            || parts.iter().any(|s| {
                s.is_empty()
                    || *s == "."
                    || *s == ".."
                    || !s
                        .bytes()
                        .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
            })
        {
            return Err(error(
                "Remote must identify one GitHub host/owner/repository",
            ));
        }
        Ok(Self {
            host: host.to_ascii_lowercase(),
            name: path.to_ascii_lowercase(),
        })
    }
    fn argument(&self) -> String {
        format!("{}/{}", self.host, self.name)
    }
}
#[derive(Clone)]
pub struct GithubAccess {
    pub requested: bool,
    pub remote: String,
    repository: Option<Repository>,
    fetch_url: String,
    push_url: String,
    reason: Option<String>,
    permission: Option<String>,
    gh_executable: String,
    check_timeout: Duration,
    operation_timeout: Duration,
}
impl GithubAccess {
    pub fn check(root: &Path, requested: bool, remote: &str) -> Self {
        let mut access = Self {
            requested,
            remote: remote.into(),
            repository: None,
            fetch_url: String::new(),
            push_url: String::new(),
            reason: None,
            permission: None,
            gh_executable: "gh".into(),
            check_timeout: CHECK_TIMEOUT,
            operation_timeout: OP_TIMEOUT,
        };
        if !requested {
            return access;
        }
        if let Err(failure) = access.check_inner(root) {
            access.repository = None;
            access.reason = Some(failure.to_string());
        }
        access
    }
    fn remote_urls(root: &Path, remote: &str) -> Result<(String, String), ReqvireError> {
        if remote.is_empty() || remote.starts_with('-') || remote.contains(['\n', '\r']) {
            return Err(error("Invalid configured remote name"));
        }
        let fetch = git(root, &["remote", "get-url", "--all", remote])?;
        let push = git(root, &["remote", "get-url", "--push", "--all", remote])?;
        if fetch.lines().count() != 1 || push.lines().count() != 1 {
            return Err(error(
                "Publication requires exactly one fetch and push destination",
            ));
        }
        Ok((fetch, push))
    }
    fn check_inner(&mut self, root: &Path) -> Result<(), ReqvireError> {
        let (fetch, push) = Self::remote_urls(root, &self.remote)?;
        let repo = Repository::parse(&fetch)?;
        if Repository::parse(&push)? != repo {
            return Err(error(
                "Fetch and push destinations must be the same repository; forks are unsupported",
            ));
        }
        let version = checked(
            run(
                root,
                &self.gh_executable,
                &["--version"],
                None,
                self.check_timeout,
            )?,
            "gh installation check",
        )?;
        if !version.trim_start().starts_with("gh version ") {
            return Err(error("Invalid gh version response"));
        }
        checked(
            run(
                root,
                &self.gh_executable,
                &["auth", "status", "--active", "--hostname", &repo.host],
                None,
                self.check_timeout,
            )?,
            "Active gh authentication check",
        )?;
        let data = checked(
            run(
                root,
                &self.gh_executable,
                &[
                    "repo",
                    "view",
                    &repo.argument(),
                    "--json",
                    "nameWithOwner,viewerPermission",
                ],
                None,
                self.check_timeout,
            )?,
            "Repository access check",
        )?;
        let data: Value =
            serde_json::from_str(&data).map_err(|_| error("Invalid repository lookup result"))?;
        if data["nameWithOwner"]
            .as_str()
            .map(str::to_ascii_lowercase)
            .as_deref()
            != Some(repo.name.as_str())
        {
            return Err(error(
                "Repository lookup did not confirm the configured repository",
            ));
        }
        self.permission = data["viewerPermission"].as_str().map(str::to_owned);
        self.repository = Some(repo);
        self.fetch_url = fetch;
        self.push_url = push;
        Ok(())
    }
    pub const fn available(&self) -> bool {
        self.repository.is_some()
    }
    pub fn status(&self) -> Value {
        json!({"requested":self.requested,"available":self.available(),"remote":self.remote,
        "repository":self.repository.as_ref().map(Repository::argument),"reason":self.reason,"viewer_permission":self.permission})
    }
    fn verify(&self, root: &Path) -> Result<&Repository, ReqvireError> {
        let repo=self.repository.as_ref().ok_or_else(||error("GitHub publication is unavailable for this session; inspect status and restart after repair"))?;
        let (fetch, push) = Self::remote_urls(root, &self.remote)?;
        if fetch != self.fetch_url || push != self.push_url {
            return Err(error(
                "Configured repository destination changed; restart before publication",
            ));
        }
        Ok(repo)
    }
    fn gh_json(&self, root: &Path, args: &[&str]) -> Result<Value, ReqvireError> {
        let value = checked(
            run(
                root,
                &self.gh_executable,
                args,
                None,
                self.operation_timeout,
            )?,
            "GitHub query",
        )?;
        serde_json::from_str(&value).map_err(|_| error("GitHub returned malformed structured data"))
    }
    fn remote_head(&self, root: &Path, branch: &str) -> Result<Option<String>, ReqvireError> {
        let reference = format!("refs/heads/{branch}");
        let result = git(
            root,
            &["ls-remote", "--heads", "--", &self.push_url, &reference],
        )?;
        Ok(result.lines().find_map(|line| {
            line.split_once('\t')
                .filter(|(_, r)| *r == reference)
                .map(|(sha, _)| sha.to_string())
        }))
    }
    pub fn execute(
        &self,
        root: &Path,
        tool: &str,
        args: &Value,
        status: &Value,
    ) -> Result<Value, ReqvireError> {
        let repo = self.verify(root)?;
        let repository = repo.argument();
        let branch = status["branch"]
            .as_str()
            .ok_or_else(|| error("Missing owned branch"))?;
        let head = status["head"]
            .as_str()
            .ok_or_else(|| error("Missing accepted HEAD"))?;
        let mut result = match tool {
            "reqvire.git.push" => {
                if args
                    .get("remote")
                    .and_then(Value::as_str)
                    .is_some_and(|r| r != self.remote)
                {
                    return Err(error("Push remote must match the startup-pinned remote"));
                }
                let previous = self.remote_head(root, branch)?;
                if previous.as_deref() == Some(head) {
                    self.push_result(root, branch, head, "no_op")
                } else {
                    let refspec = format!("{head}:refs/heads/{branch}");
                    let output = run(
                        root,
                        "git",
                        &[
                            "-c",
                            "push.followTags=false",
                            "-c",
                            "remote.pushDefault=",
                            "push",
                            "--porcelain",
                            "--no-follow-tags",
                            "--",
                            &self.push_url,
                            &refspec,
                        ],
                        None,
                        self.operation_timeout,
                    );
                    let confirmed = self.remote_head(root, branch);
                    if matches!(&confirmed,Ok(Some(current)) if current==head) {
                        self.push_result(root, branch, head, "completed")
                    } else {
                        match output {
                            Ok(out) if !out.success && is_definite_rejection(&out) => {
                                return Err(error(publication_failure(&out)))
                            }
                            _ => {
                                json!({"outcome":"unknown","commit":head,"remote_branch":branch,"error":"Push outcome could not be confirmed; inspect remote state before retrying"})
                            }
                        }
                    }
                }
            }
            "reqvire.github.pr.create" => {
                let base = required(args, "base")?;
                let title = required(args, "title")?;
                let body = args["body"]
                    .as_str()
                    .ok_or_else(|| error("body must be a string"))?;
                git(root, &["check-ref-format", &format!("refs/heads/{base}")])?;
                if base == branch {
                    return Err(error("PR base must differ from the owned branch"));
                }
                if self.remote_head(root, branch)?.as_deref() != Some(head) {
                    return Err(error(
                        "Push the accepted HEAD before creating a pull request",
                    ));
                }
                if self.remote_head(root, base)?.is_none() {
                    return Err(error("PR base does not exist in the pinned repository"));
                }
                let existing = self.open_prs(root, &repository, branch)?;
                if let Some(pr) = matching_pr(&existing, base)? {
                    pr_result(pr, "no_op", head)
                } else {
                    // Fixed endpoint, not a caller-supplied API proxy. Encode branch names as path data.
                    let encoded_base = percent_encoding::utf8_percent_encode(
                        base,
                        percent_encoding::NON_ALPHANUMERIC,
                    );
                    let encoded_head = percent_encoding::utf8_percent_encode(
                        branch,
                        percent_encoding::NON_ALPHANUMERIC,
                    );
                    let endpoint = format!(
                        "repos/{}/compare/{encoded_base}...{encoded_head}",
                        repo.name
                    );
                    let comparison =
                        self.gh_json(root, &["api", "--hostname", &repo.host, &endpoint])?;
                    if comparison["ahead_by"].as_u64().unwrap_or(0) == 0 {
                        return Err(error("No proposed commits ahead of the selected PR base"));
                    }
                    let mut command = vec![
                        "pr",
                        "create",
                        "--repo",
                        &repository,
                        "--head",
                        branch,
                        "--base",
                        base,
                        "--title",
                        title,
                        "--body-file",
                        "-",
                    ];
                    if args["draft"].as_bool().unwrap_or(false) {
                        command.push("--draft");
                    }
                    let output = run(
                        root,
                        &self.gh_executable,
                        &command,
                        Some(body.as_bytes()),
                        self.operation_timeout,
                    );
                    match self
                        .open_prs(root, &repository, branch)
                        .and_then(|items| Ok(matching_pr(&items, base)?.cloned()))
                    {
                        Ok(Some(pr)) => pr_result(&pr, "completed", head),
                        _ => match output {
                            Ok(out) if !out.success && is_definite_rejection(&out) => {
                                return Err(error(publication_failure(&out)))
                            }
                            _ => {
                                json!({"outcome":"unknown","head":branch,"base":base,"commit":head,"error":"Pull request creation could not be confirmed; inspect open PRs before retrying"})
                            }
                        },
                    }
                }
            }
            "reqvire.github.pr.comment" => {
                let number = args["pr_number"]
                    .as_u64()
                    .filter(|n| *n > 0)
                    .ok_or_else(|| error("pr_number must be a positive integer"))?
                    .to_string();
                let body = required(args, "body")?;
                let pr = self.gh_json(
                    root,
                    &[
                        "pr",
                        "view",
                        &number,
                        "--repo",
                        &repository,
                        "--json",
                        "number,url",
                    ],
                )?;
                if pr["number"].as_u64().map(|n| n.to_string()).as_deref() != Some(number.as_str())
                    || !valid_pr_url(repo, &pr["url"], pr["number"].as_u64().unwrap_or(0))
                {
                    return Err(error("Pull request lookup did not confirm the target"));
                }
                let output = run(
                    root,
                    &self.gh_executable,
                    &[
                        "pr",
                        "comment",
                        &number,
                        "--repo",
                        &repository,
                        "--body-file",
                        "-",
                    ],
                    Some(body.as_bytes()),
                    self.operation_timeout,
                );
                match output {
                    Ok(out) if out.success => {
                        let url = out.stdout.trim();
                        let prefix = format!(
                            "https://{}/{}/pull/{}#issuecomment-",
                            repo.host, repo.name, number
                        );
                        url
                            .strip_prefix(&prefix)
                            .and_then(|id| id.parse::<u64>().ok()).map_or_else(|| json!({"outcome":"unknown","pr_number":pr["number"],"pr_url":pr["url"],"error":"Comment may have been posted; confirm the target PR before retrying"}), |id| json!({"outcome":"completed","pr_number":pr["number"],"pr_url":pr["url"],"comment_id":id,"comment_url":url}))
                    }
                    Ok(out) if is_definite_rejection(&out) => {
                        return Err(error(publication_failure(&out)))
                    }
                    _ => {
                        json!({"outcome":"unknown","pr_number":pr["number"],"pr_url":pr["url"],"error":"Comment outcome is unknown; inspect the target PR before retrying"})
                    }
                }
            }
            _ => return Err(error("Unknown publication operation")),
        };
        result["repository"] = json!(repository);
        result["remote"] = json!(self.remote);
        result["branch"] = json!(branch);
        Ok(result)
    }
    fn open_prs(&self, root: &Path, repo: &str, branch: &str) -> Result<Vec<Value>, ReqvireError> {
        let value = self.gh_json(
            root,
            &[
                "pr",
                "list",
                "--repo",
                repo,
                "--head",
                branch,
                "--state",
                "open",
                "--limit",
                "100",
                "--json",
                "number,url,baseRefName,headRefName,isDraft,isCrossRepository",
            ],
        )?;
        let items = value
            .as_array()
            .ok_or_else(|| error("Invalid PR list response"))?;
        let pinned = self
            .repository
            .as_ref()
            .ok_or_else(|| error("Missing pinned repository"))?;
        for pr in items {
            let number = pr["number"]
                .as_u64()
                .filter(|n| *n > 0)
                .ok_or_else(|| error("Invalid PR number in lookup response"))?;
            if !valid_pr_url(pinned, &pr["url"], number)
                || !pr["baseRefName"].is_string()
                || !pr["headRefName"].is_string()
                || !pr["isDraft"].is_boolean()
                || !pr["isCrossRepository"].is_boolean()
            {
                return Err(error("Invalid PR lookup response"));
            }
        }
        if items.len() >= 100 {
            return Err(error(
                "PR lookup is truncated; resolve open PR ambiguity before publication",
            ));
        }
        Ok(items
            .iter()
            .filter(|pr| pr["isCrossRepository"] == false && pr["headRefName"] == branch)
            .cloned()
            .collect())
    }
    fn push_result(&self, root: &Path, branch: &str, head: &str, outcome: &str) -> Value {
        let mut result =
            json!({"outcome":outcome,"commit":head,"remote_branch":branch,"remote_head":head});
        if self.set_upstream(root, branch).is_err() {
            result["warning"] = json!(
                "Remote publication confirmed, but upstream tracking could not be configured"
            );
        }
        result
    }
    fn set_upstream(&self, root: &Path, branch: &str) -> Result<(), ReqvireError> {
        git(
            root,
            &["config", &format!("branch.{branch}.remote"), &self.remote],
        )?;
        git(
            root,
            &[
                "config",
                &format!("branch.{branch}.merge"),
                &format!("refs/heads/{branch}"),
            ],
        )?;
        Ok(())
    }
}
fn required<'a>(args: &'a Value, key: &str) -> Result<&'a str, ReqvireError> {
    args[key]
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| error(format!("{key} must not be empty")))
}
fn matching_pr<'a>(items: &'a [Value], base: &str) -> Result<Option<&'a Value>, ReqvireError> {
    if items.len() > 1 || items.first().is_some_and(|pr| pr["baseRefName"] != base) {
        return Err(error(format!(
            "Conflicting open PRs for this head: {}",
            items
                .iter()
                .map(|pr| format!(
                    "#{} {} (base {})",
                    pr["number"],
                    pr["url"].as_str().unwrap_or(""),
                    pr["baseRefName"].as_str().unwrap_or("")
                ))
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }
    if let Some(pr) = items.first() {
        if !pr["number"].is_u64() || !pr["url"].is_string() {
            return Err(error("Incomplete pull request response"));
        }
    }
    Ok(items.first())
}
fn pr_result(pr: &Value, outcome: &str, head: &str) -> Value {
    json!({"outcome":outcome,"pr_number":pr["number"],"pr_url":pr["url"],"head":pr["headRefName"],"base":pr["baseRefName"],"draft":pr["isDraft"],"commit":head})
}
fn checked(output: Output, operation: &str) -> Result<String, ReqvireError> {
    if output.success {
        Ok(output.stdout)
    } else {
        Err(error(format!(
            "{operation} failed; check local authentication and repository permissions"
        )))
    }
}
fn is_definite_rejection(out: &Output) -> bool {
    let s = format!("{} {}", out.stderr, out.stdout).to_ascii_lowercase();
    [
        "403",
        "401",
        "404",
        "permission denied",
        "not authorized",
        "authentication failed",
        "[rejected]",
        "non-fast-forward",
        "fetch first",
        "pre-receive hook declined",
        "gh006",
        "gh013",
    ]
    .iter()
    .any(|n| s.contains(n))
}
fn publication_failure(out: &Output) -> &'static str {
    let s = format!("{} {}", out.stderr, out.stdout).to_ascii_lowercase();
    if s.contains("non-fast-forward") || s.contains("fetch first") {
        "Remote history diverged; automatic force push or merge is not permitted"
    } else if is_definite_rejection(out) {
        "Publication rejected by authentication or repository permissions"
    } else {
        "Publication failed; inspect repository and branch rules before retrying"
    }
}

fn valid_pr_url(repo: &Repository, url: &Value, number: u64) -> bool {
    url.as_str().is_some_and(|url| {
        url.to_ascii_lowercase() == format!("https://{}/{}/pull/{number}", repo.host, repo.name)
    })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};
    struct Fixture {
        _temp: tempfile::TempDir,
        root: PathBuf,
        state: PathBuf,
        gh: PathBuf,
        bare: PathBuf,
    }
    impl Fixture {
        fn new() -> Self {
            let temp = tempfile::tempdir().expect("validated worktree context invariant");
            let root = temp.path().join("repo");
            fs::create_dir(&root).expect("validated worktree context invariant");
            git(&root, &["init", "-qb", "main"]).expect("validated worktree context invariant");
            git(&root, &["config", "user.name", "Publication Test"])
                .expect("validated worktree context invariant");
            git(&root, &["config", "user.email", "test@example.invalid"])
                .expect("validated worktree context invariant");
            fs::write(root.join("model.txt"), "baseline")
                .expect("validated worktree context invariant");
            git(&root, &["add", "."]).expect("validated worktree context invariant");
            git(&root, &["commit", "-qm", "baseline"])
                .expect("validated worktree context invariant");
            let bare = temp.path().join("remote.git");
            git(
                &root,
                &[
                    "init",
                    "--bare",
                    bare.to_str().expect("validated worktree context invariant"),
                ],
            )
            .expect("validated worktree context invariant");
            git(
                &root,
                &[
                    "remote",
                    "add",
                    "origin",
                    "https://github.com/test/project.git",
                ],
            )
            .expect("validated worktree context invariant");
            let state = temp.path().join("state");
            fs::create_dir(&state).expect("validated worktree context invariant");
            fs::write(state.join("scenario.json"), "{}")
                .expect("validated worktree context invariant");
            let gh = temp.path().join("gh");
            let script = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/test-mcp-publication/fixtures/gh-double.py");
            fs::write(&gh,format!("#!/usr/bin/python3\nimport os, runpy\nos.environ['REQVIRE_GH_TEST_STATE'] = {}\nrunpy.run_path({}, run_name='__main__')\n",json!(state.to_str().expect("validated worktree context invariant")),json!(script.to_str().expect("validated worktree context invariant")))).expect("validated worktree context invariant");
            fs::set_permissions(&gh, fs::Permissions::from_mode(0o755))
                .expect("validated worktree context invariant");
            Self {
                _temp: temp,
                root,
                state,
                gh,
                bare,
            }
        }
        fn scenario(&self, scenario: Value) {
            fs::write(self.state.join("scenario.json"), scenario.to_string())
                .expect("validated worktree context invariant");
        }
        fn access(&self) -> GithubAccess {
            self.access_with_check_timeout(CHECK_TIMEOUT)
        }
        fn access_with_check_timeout(&self, timeout: Duration) -> GithubAccess {
            let mut access = GithubAccess::check(&self.root, false, "origin");
            access.requested = true;
            access.gh_executable = self
                .gh
                .to_str()
                .expect("validated worktree context invariant")
                .into();
            access.check_timeout = timeout;
            if let Err(failure) = access.check_inner(&self.root) {
                access.repository = None;
                access.reason = Some(failure.to_string());
            }
            access
        }
        fn local_transport(&self) -> GithubAccess {
            let mut access = self.access();
            assert!(access.available(), "{}", access.status());
            // Transport tests use a local bare repo after startup identity has been tested independently.
            git(
                &self.root,
                &[
                    "remote",
                    "set-url",
                    "origin",
                    self.bare
                        .to_str()
                        .expect("validated worktree context invariant"),
                ],
            )
            .expect("validated worktree context invariant");
            access.fetch_url = self
                .bare
                .to_str()
                .expect("validated worktree context invariant")
                .into();
            access.push_url = access.fetch_url.clone();
            access
        }
        fn status(&self, branch: &str) -> Value {
            json!({"branch":branch,"head":git(&self.root,&["rev-parse","HEAD"]).expect("validated worktree context invariant")})
        }
        fn calls(&self) -> Vec<Value> {
            fs::read_to_string(self.state.join("calls.jsonl"))
                .unwrap_or_default()
                .lines()
                .map(|line| {
                    serde_json::from_str(line).expect("validated worktree context invariant")
                })
                .collect()
        }
    }
    #[test]
    fn startup_accepts_a_slow_executable_within_the_production_deadline() {
        let fixture = Fixture::new();
        let script = fs::read_to_string(&fixture.gh)
            .expect("read test fixture")
            .replace(
                "import os, runpy\n",
                "import os, runpy, sys, time\nif sys.argv[1:] == ['--version']: time.sleep(0.4)\n",
            );
        fs::write(&fixture.gh, script).expect("write test fixture");
        let access = fixture.access();
        assert!(access.available(), "{}", access.status());
        assert_eq!(access.check_timeout, CHECK_TIMEOUT);
        assert_eq!(access.operation_timeout, OP_TIMEOUT);
    }

    #[test]
    fn stalled_authentication_times_out_after_reaching_the_intended_check() {
        let fixture = Fixture::new();
        fixture.scenario(json!({"timeout":"auth status"}));
        let access = fixture.access_with_check_timeout(Duration::from_secs(2));
        assert!(!access.available());
        assert!(access
            .reason
            .as_deref()
            .expect("test fixture operation should succeed")
            .contains("timed out"));
        let calls = fixture.calls();
        assert!(
            calls
                .iter()
                .any(|call| { call["args"][0] == "auth" && call["args"][1] == "status" }),
            "must reach stalled authentication: {calls:?}"
        );
        assert!(calls.iter().all(|call| call["args"][0] != "repo"));
    }

    #[test]
    fn availability_is_bounded_sanitized_and_checks_active_auth_without_publication() {
        let fixture = Fixture::new();
        for scenario in [
            json!({"fail":"auth status"}),
            json!({"fail":"repo view"}),
            json!({"malformed":"repo view"}),
            json!({"malformed":"--version"}),
            json!({"repository":"other/repo"}),
        ] {
            fixture.scenario(scenario);
            let access = fixture.access();
            assert!(!access.available());
            assert!(!access.status().to_string().contains("secret-token"));
        }
        fixture.scenario(json!({"permission":"READ"}));
        assert!(fixture.access().available());
        let mut missing = fixture.access();
        missing.gh_executable = fixture.state.join("missing").to_string_lossy().into_owned();
        assert!(missing.check_inner(&fixture.root).is_err());
        assert!(fixture
            .calls()
            .iter()
            .all(|call| !matches!(call["args"][0].as_str(), Some("pr" | "api"))));
        git(
            &fixture.root,
            &[
                "remote",
                "set-url",
                "--add",
                "--push",
                "origin",
                "https://github.com/other/fork.git",
            ],
        )
        .expect("validated worktree context invariant");
        assert!(!fixture.access().available());
    }
    #[test]
    fn exact_push_noop_divergence_and_changed_remote_are_safe() {
        let fixture = Fixture::new();
        let access = fixture.local_transport();
        let initial = fixture.status("main");
        let pushed = access
            .execute(&fixture.root, "reqvire.git.push", &json!({}), &initial)
            .expect("validated worktree context invariant");
        assert_eq!(pushed["outcome"], "completed");
        assert_eq!(
            git(&fixture.root, &["config", "branch.main.remote"])
                .expect("validated worktree context invariant"),
            "origin"
        );
        assert_eq!(
            access
                .execute(&fixture.root, "reqvire.git.push", &json!({}), &initial)
                .expect("validated worktree context invariant")["outcome"],
            "no_op"
        );
        fs::write(fixture.root.join("model.txt"), "new tip")
            .expect("validated worktree context invariant");
        git(&fixture.root, &["commit", "-am", "second"])
            .expect("validated worktree context invariant");
        let second = fixture.status("main");
        access
            .execute(&fixture.root, "reqvire.git.push", &json!({}), &second)
            .expect("validated worktree context invariant");
        assert!(access
            .execute(&fixture.root, "reqvire.git.push", &json!({}), &initial)
            .is_err());
        assert_eq!(
            git(&fixture.root, &["rev-parse", "HEAD"])
                .expect("validated worktree context invariant"),
            second["head"]
        );
        assert!(access
            .execute(
                &fixture.root,
                "reqvire.git.push",
                &json!({"remote":"elsewhere"}),
                &second
            )
            .is_err());
        git(
            &fixture.root,
            &[
                "remote",
                "set-url",
                "origin",
                "https://github.com/other/project",
            ],
        )
        .expect("validated worktree context invariant");
        assert!(access
            .execute(&fixture.root, "reqvire.git.push", &json!({}), &second)
            .is_err());
    }
    #[test]
    fn pull_requests_preserve_arguments_reconcile_lost_response_and_detect_conflicts() {
        let fixture = Fixture::new();
        let access = fixture.local_transport();
        access
            .execute(
                &fixture.root,
                "reqvire.git.push",
                &json!({}),
                &fixture.status("main"),
            )
            .expect("validated worktree context invariant");
        git(&fixture.root, &["checkout", "-qb", "feature"])
            .expect("validated worktree context invariant");
        fs::write(fixture.root.join("model.txt"), "feature")
            .expect("validated worktree context invariant");
        git(&fixture.root, &["commit", "-am", "feature"])
            .expect("validated worktree context invariant");
        let status = fixture.status("feature");
        let args = json!({"base":"main","title":"--title ' $() `literal`","body":"Line 1\n\n--body $() `literal`\n","draft":true});
        assert!(access
            .execute(&fixture.root, "reqvire.github.pr.create", &args, &status)
            .is_err()); // unpushed
        access
            .execute(&fixture.root, "reqvire.git.push", &json!({}), &status)
            .expect("validated worktree context invariant");
        fixture.scenario(json!({"ahead_by":0}));
        assert!(access
            .execute(&fixture.root, "reqvire.github.pr.create", &args, &status)
            .is_err());
        fixture.scenario(json!({"lost_create":true}));
        let created = access
            .execute(&fixture.root, "reqvire.github.pr.create", &args, &status)
            .expect("validated worktree context invariant");
        assert_eq!(created["outcome"], "completed");
        assert_eq!(created["pr_number"], 7);
        assert_eq!(created["draft"], true);
        assert_eq!(
            access
                .execute(&fixture.root, "reqvire.github.pr.create", &args, &status)
                .expect("validated worktree context invariant")["outcome"],
            "no_op"
        );
        let calls = fixture.calls();
        let creates: Vec<_> = calls
            .iter()
            .filter(|c| c["args"][0] == "pr" && c["args"][1] == "create")
            .collect();
        assert_eq!(creates.len(), 1);
        assert_eq!(creates[0]["body"], args["body"]);
        assert!(creates[0]["args"]
            .as_array()
            .expect("validated worktree context invariant")
            .contains(&args["title"]));
        let mut prs: Value = serde_json::from_str(
            &fs::read_to_string(fixture.state.join("prs.json"))
                .expect("validated worktree context invariant"),
        )
        .expect("validated worktree context invariant");
        prs[0]["baseRefName"] = json!("different-base");
        fs::write(fixture.state.join("prs.json"), prs.to_string())
            .expect("validated worktree context invariant");
        let error = access
            .execute(&fixture.root, "reqvire.github.pr.create", &args, &status)
            .expect_err("test fixture operation should fail");
        assert!(error.to_string().contains("#7"));
        // Stacked PRs use an explicit existing feature branch as their base.
        fs::remove_file(fixture.state.join("prs.json"))
            .expect("validated worktree context invariant");
        fixture.scenario(json!({}));
        git(&fixture.root, &["checkout", "-qb", "stacked"])
            .expect("validated worktree context invariant");
        git(
            &fixture.root,
            &["commit", "--allow-empty", "-qm", "stacked change"],
        )
        .expect("validated worktree context invariant");
        let stacked = fixture.status("stacked");
        access
            .execute(&fixture.root, "reqvire.git.push", &json!({}), &stacked)
            .expect("validated worktree context invariant");
        for base in ["missing-branch", "stacked"] {
            assert!(access
                .execute(
                    &fixture.root,
                    "reqvire.github.pr.create",
                    &json!({"base":base,"title":"Stack","body":""}),
                    &stacked
                )
                .is_err());
        }
        let result = access
            .execute(
                &fixture.root,
                "reqvire.github.pr.create",
                &json!({"base":"feature","title":"Stack","body":""}),
                &stacked,
            )
            .expect("validated worktree context invariant");
        assert_eq!(result["outcome"], "completed");
        let prs: Value = serde_json::from_str(
            &fs::read_to_string(fixture.state.join("prs.json"))
                .expect("validated worktree context invariant"),
        )
        .expect("validated worktree context invariant");
        assert_eq!(prs[0]["baseRefName"], "feature");
        assert_eq!(prs[0]["isDraft"], false);
    }
    #[test]
    fn comments_report_ambiguous_outcomes_without_reposting_and_keep_multiline_text() {
        let fixture = Fixture::new();
        let access = fixture.access();
        let status = fixture.status("main");
        let args = json!({"pr_number":22,"body":"--flag\n\n$HOME `data` $(data)\n"});
        let result = access
            .execute(&fixture.root, "reqvire.github.pr.comment", &args, &status)
            .expect("validated worktree context invariant");
        assert_eq!(result["comment_id"], 123);
        assert_eq!(result["pr_number"], 22);
        fixture.scenario(json!({"lost_comment":true}));
        assert_eq!(
            access
                .execute(&fixture.root, "reqvire.github.pr.comment", &args, &status)
                .expect("validated worktree context invariant")["outcome"],
            "unknown"
        );
        let records = fs::read_to_string(fixture.state.join("comments.jsonl"))
            .expect("validated worktree context invariant");
        assert_eq!(records.lines().count(), 2);
        for record in records.lines() {
            assert_eq!(
                serde_json::from_str::<Value>(record)
                    .expect("validated worktree context invariant")["body"],
                args["body"]
            );
        }
        fixture.scenario(json!({"fail":"pr comment"}));
        let failure = access
            .execute(&fixture.root, "reqvire.github.pr.comment", &args, &status)
            .expect_err("test fixture operation should fail")
            .to_string();
        assert!(!failure.contains("secret-token"));
        assert!(failure.contains("permission"));
        assert_eq!(fixture.status("main"), status);
    }
    #[test]
    fn repository_identity_rejects_credentials_forks_and_malformed_urls() {
        assert_eq!(
            Repository::parse("https://github.com/Owner/Repo.git")
                .expect("validated worktree context invariant"),
            Repository::parse("git@github.com:owner/repo.git")
                .expect("validated worktree context invariant")
        );
        for url in [
            "https://token@github.com/a/b",
            "https://github.com/a/b/extra",
            "file:///tmp/repo",
            "git@evil/a/b",
            "https://github.com/../b",
        ] {
            assert!(Repository::parse(url).is_err(), "{url}");
        }
    }
}
