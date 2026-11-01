//! Coordinated publication reuses the existing bounded GitHub operations.
use super::*;
use reqvire::repo_workspace::normalize_url;

struct Dependency {
    path: PathBuf,
    metadata: Value,
    access: GithubAccess,
}

impl Dependency {
    fn status(&self) -> Result<Value, ReqvireError> {
        Ok(json!({"branch":text(&self.metadata,"branch")?,"head":text(&self.metadata,"commit")?}))
    }
    fn changed(&self, base: &Value) -> bool {
        if base.is_null() {
            return true;
        }
        let identity = normalize_url(
            self.metadata["repository"]
                .as_str()
                .or_else(|| self.metadata["fetch_url"].as_str())
                .unwrap_or(""),
        );
        let entries = base["projects"].as_array();
        let matched = entries.into_iter().flatten().find(|entry| {
            normalize_url(entry["repository"].as_str().unwrap_or("")) == identity
                && entry["path"] == self.metadata["path"]
        });
        matched.is_none_or(|entry| entry["commit"] != self.metadata["commit"])
    }
}

fn annotated(mut result: Value, dependency: &Dependency) -> Value {
    result["path"] = json!(dependency.path);
    if result.get("pr_number").is_some() {
        result["number"] = result["pr_number"].clone();
        result["url"] = result["pr_url"].clone();
    }
    result
}

impl Worktrees {
    fn dependencies(
        &self,
        context: &Context,
        status: &Value,
    ) -> Result<Vec<Dependency>, ReqvireError> {
        let mut cache = context
            .github_dependencies
            .lock()
            .expect("dependency publication cache poisoned");
        let metadata: Vec<_> = status["projects"]
            .as_array()
            .into_iter()
            .flatten()
            .cloned()
            .collect();
        if cache.is_none() {
            let mut accesses = BTreeMap::new();
            for item in &metadata {
                let path = PathBuf::from(text(item, "path")?);
                if path.as_os_str().is_empty()
                    || path
                        .components()
                        .any(|part| !matches!(part, std::path::Component::Normal(_)))
                {
                    return Err(error("Invalid declared publication owner path"));
                }
                let root = context.root.join(&path);
                if root.canonicalize()? != root {
                    return Err(error("Publication owner changed its checkout boundary"));
                }
                let access = GithubAccess::check(&root, true, text(item, "remote")?);
                let expected = item["repository"]
                    .as_str()
                    .or_else(|| item["fetch_url"].as_str())
                    .unwrap_or("");
                if !access.available() || !access.matches_identity(expected) {
                    return Err(error(format!(
                        "Declared repository {} publication is unavailable or its identity changed",
                        path.display()
                    )));
                }
                accesses.insert(path, access);
            }
            *cache = Some(accesses);
        }
        let accesses = cache.as_ref().expect("captured publication owners");
        let result = metadata
            .into_iter()
            .map(|metadata| {
                let path = PathBuf::from(text(&metadata, "path")?);
                let access = accesses
                    .get(&path)
                    .ok_or_else(|| {
                        error("Composition changed without explicit validated admission")
                    })?
                    .clone();
                Ok(Dependency {
                    path,
                    metadata,
                    access,
                })
            })
            .collect();
        drop(cache);
        result
    }

    fn publication_base(
        &self,
        context: &Context,
        worker: &mut Option<Arc<Worker>>,
        reference: &str,
    ) -> Result<Value, ReqvireError> {
        Ok(context.request_locked(
            worker,
            &json!({"operation":"composition_base","reference":reference}),
        )?["result"]
            .clone())
    }

    pub(super) fn publish_composition(
        &self,
        context: &Context,
        worker: &mut Option<Arc<Worker>>,
        name: &str,
        args: &Value,
        status: &Value,
    ) -> Result<Value, ReqvireError> {
        if status["workspace_mode"] != "repo" {
            if args["repository_bases"]
                .as_object()
                .is_some_and(|map| !map.is_empty())
                || args["repository_path"]
                    .as_str()
                    .is_some_and(|path| !path.is_empty())
            {
                return Err(error(
                    "No non-root repository owners are declared in this context",
                ));
            }
            let mut result = self.github.execute(&context.root, name, args, status)?;
            if name == "reqvire.github.pr.create"
                && matches!(result["outcome"].as_str(), Some("completed" | "no_op"))
            {
                if let Err(failure) = self.remember_review_target(context, args) {
                    result["warning"]=json!(format!("PR confirmed, but its local review baseline could not be recorded: {failure}"));
                }
            }
            return Ok(result);
        }
        let dependencies = self.dependencies(context, status)?;
        if name == "reqvire.github.pr.comment" {
            if let Some(path) = args["repository_path"]
                .as_str()
                .filter(|path| !path.is_empty())
            {
                let dependency = dependencies
                    .iter()
                    .find(|dependency| dependency.path == Path::new(path))
                    .ok_or_else(|| error("Comment target is not a declared repository owner"))?;
                let mut selected = dependency.metadata.clone();
                selected["head"] = selected["commit"].clone();
                if !selected["branch"].is_string() {
                    selected["branch"] = status["branch"].clone();
                }
                return dependency
                    .access
                    .execute(&context.root.join(&dependency.path), name, args, &selected)
                    .map(|result| annotated(result, dependency));
            }
            return self.github.execute(&context.root, name, args, status);
        }
        // Distinct checkouts must never publish competing values to one ref.
        let mut destinations = BTreeMap::new();
        if let Some(repository) = self.github.repository_key() {
            destinations.insert(
                (repository, text(status, "branch")?.to_owned()),
                PathBuf::new(),
            );
        }
        for dependency in &dependencies {
            if let (Some(repository), Some(branch)) = (
                dependency.access.repository_key(),
                dependency.metadata["branch"].as_str(),
            ) {
                if let Some(previous) =
                    destinations.insert((repository, branch.to_owned()), dependency.path.clone())
                {
                    return Err(error(format!(
                        "Repository owners {} and {} target the same published branch {branch}; prepare distinct authoring branches before publication",
                        if previous.as_os_str().is_empty() { "workspace root".into() } else { previous.display().to_string() },
                        dependency.path.display()
                    )));
                }
            }
        }
        if name == "reqvire.git.push" {
            let base = context
                .change_base
                .read()
                .expect("composition base poisoned")
                .clone()
                .map(|reference| self.publication_base(context, worker, &reference))
                .transpose()?
                .unwrap_or(Value::Null);
            let mut outcomes = Vec::new();
            for dependency in &dependencies {
                let root = context.root.join(&dependency.path);
                let operation =
                    if dependency.changed(&base) && dependency.metadata["branch"].is_string() {
                        dependency
                            .access
                            .execute(&root, name, &json!({}), &dependency.status()?)
                    } else {
                        dependency
                            .access
                            .push_dependency(&root, text(&dependency.metadata, "commit")?)
                    };
                match operation {
                    Ok(result) => {
                        let unknown = result["outcome"] == "unknown";
                        outcomes.push(annotated(result, dependency));
                        if unknown {
                            return Ok(
                                json!({"outcome":"unknown","projects":outcomes,"error":"Dependency push could not be confirmed; root was not published"}),
                            );
                        }
                    }
                    Err(failure) => {
                        outcomes.push(json!({"path":dependency.path,"commit":dependency.metadata["commit"],"outcome":"failed"}));
                        return Ok(
                            json!({"outcome":"failed","projects":outcomes,"error":failure.to_string()}),
                        );
                    }
                }
            }
            let mut result = match self.github.execute(&context.root, name, args, status) {
                Ok(value) => value,
                Err(failure) => json!({"outcome":"failed","error":failure.to_string()}),
            };
            result["projects"] = json!(outcomes);
            return Ok(result);
        }
        let base_name = text(args, "base")?;
        let (target, _) = self.github.preflight_pr(
            &context.root,
            base_name,
            text(status, "branch")?,
            text(status, "head")?,
        )?;
        let reference = git(
            &context.root,
            &["merge-base", text(status, "head")?, &target],
        )?;
        let base = self.publication_base(context, worker, &reference)?;
        let targets = args["repository_bases"].as_object();
        for path in targets.into_iter().flat_map(|map| map.keys()) {
            if !dependencies
                .iter()
                .any(|dependency| dependency.path == Path::new(path))
            {
                return Err(error(format!(
                    "PR target {path} is not a declared non-root repository owner"
                )));
            }
        }
        let changed: Vec<_> = dependencies
            .iter()
            .filter(|dependency| dependency.changed(&base))
            .collect();
        // Validate every dependency target before creating the first PR.
        for dependency in &changed {
            let path = dependency.path.to_string_lossy();
            let target=targets.and_then(|map|map.get(path.as_ref())).and_then(Value::as_str)
                .ok_or_else(||error(format!("Explicit PR base is required for changed repository {path} in repository_bases")))?;
            let selected = dependency.status()?;
            dependency.access.preflight_pr(
                &context.root.join(&dependency.path),
                target,
                text(&selected, "branch")?,
                text(&selected, "head")?,
            )?;
        }
        let mut prs = Vec::new();
        for dependency in changed {
            let mut selected_args = args.clone();
            selected_args["base"] = targets.expect("validated dependency targets")
                [dependency.path.to_string_lossy().as_ref()]
            .clone();
            match dependency.access.execute(
                &context.root.join(&dependency.path),
                name,
                &selected_args,
                &dependency.status()?,
            ) {
                Ok(result) => {
                    let unknown = result["outcome"] == "unknown";
                    prs.push(annotated(result, dependency));
                    if unknown {
                        return Ok(
                            json!({"outcome":"unknown","pull_requests":prs,"error":"Dependency PR creation could not be confirmed; root PR was not created"}),
                        );
                    }
                }
                Err(failure) => {
                    return Ok(
                        json!({"outcome":"failed","pull_requests":prs,"error":failure.to_string()}),
                    )
                }
            }
        }
        let mut root_args = args.clone();
        if !prs.is_empty() {
            let mut body = text(args, "body").unwrap_or("").to_owned();
            body.push_str("\n\n## Coordinated pull requests\n\n");
            for pr in &prs {
                body.push_str(&format!(
                    "- [{}]({}) — commit `{}`\n",
                    text(pr, "path")?,
                    text(pr, "pr_url")?,
                    text(pr, "commit")?
                ));
            }
            root_args["body"] = json!(body);
        }
        let mut result = match self.github.execute(&context.root, name, &root_args, status) {
            Ok(result) => result,
            Err(failure) => {
                return Ok(
                    json!({"outcome":"failed","pull_requests":prs,"error":failure.to_string()}),
                )
            }
        };
        let mut root = result.clone();
        root["path"] = json!("");
        root["number"] = root["pr_number"].clone();
        root["url"] = root["pr_url"].clone();
        prs.push(root);
        result["pull_requests"] = json!(prs);
        if matches!(result["outcome"].as_str(), Some("completed" | "no_op")) {
            if let Err(failure) = self.remember_review_target(context, args) {
                result["warning"]=json!(format!("PR set confirmed, but its local review baseline could not be recorded: {failure}"));
            }
            if result["outcome"] == "no_op" {
                let mut warning = result["warning"].as_str().unwrap_or("").to_owned();
                if !warning.is_empty() {
                    warning.push_str("; ");
                }
                warning.push_str("Existing root PR reused without editing its body; explicitly update links if the coordinated PR set changed");
                result["warning"] = json!(warning);
            }
        }
        Ok(result)
    }

    fn remember_review_target(&self, context: &Context, args: &Value) -> Result<(), ReqvireError> {
        let target = text(args, "base")?;
        let commit = self
            .github
            .remote_head(&context.root, target)?
            .ok_or_else(|| error("Confirmed PR target became unavailable"))?;
        git(
            &context.root,
            &[
                "config",
                "--local",
                &format!("branch.{}.reqvireReviewBase", context.branch),
                &commit,
            ],
        )?;
        git(
            &context.root,
            &[
                "config",
                "--local",
                &format!("branch.{}.reqvireReviewTarget", context.branch),
                target,
            ],
        )?;
        Ok(())
    }
}
