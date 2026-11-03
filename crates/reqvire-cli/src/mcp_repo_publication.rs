//! Coordinated publication reuses the existing bounded GitHub operations.
use super::*;

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
        reqvire_workspace::publication::changed(&self.metadata, base)
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
                if reqvire_workspace::paths::canonicalize_existing(&root)? != root {
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
        let owners = std::iter::once((
            PathBuf::new(),
            self.github.repository_key(),
            status["branch"].as_str(),
        ))
        .chain(dependencies.iter().map(|d| {
            (
                d.path.clone(),
                d.access.repository_key(),
                d.metadata["branch"].as_str(),
            )
        }))
        .filter_map(|(path, repository, branch)| Some((path, repository?, branch?.to_owned())));
        reqvire_workspace::publication::distinct_refs(owners)?;
        if name == "reqvire.git.push" {
            let base = context
                .change_base
                .read()
                .expect("composition base poisoned")
                .clone()
                .map(|reference| self.publication_base(context, worker, &reference))
                .transpose()?
                .unwrap_or(Value::Null);
            let paths = dependencies
                .iter()
                .map(|d| d.path.clone())
                .collect::<Vec<_>>();
            return Ok(reqvire_workspace::publication::publish(
                &paths,
                reqvire_workspace::publication::Kind::Push,
                |path| -> Result<Value, ReqvireError> {
                    let dependency = dependencies
                        .iter()
                        .find(|d| d.path == path)
                        .expect("declared publication owner");
                    let root = context.root.join(path);
                    if dependency.changed(&base) && dependency.metadata["branch"].is_string() {
                        dependency
                            .access
                            .execute(&root, name, &json!({}), &dependency.status()?)
                    } else {
                        dependency
                            .access
                            .push_dependency(&root, text(&dependency.metadata, "commit")?)
                    }
                },
                |_| self.github.execute(&context.root, name, args, status),
            ));
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
        let paths = changed.iter().map(|d| d.path.clone()).collect::<Vec<_>>();
        let mut result = reqvire_workspace::publication::publish(
            &paths,
            reqvire_workspace::publication::Kind::PullRequests,
            |path| -> Result<Value, ReqvireError> {
                let dependency = changed
                    .iter()
                    .find(|d| d.path == path)
                    .expect("changed publication owner");
                let mut selected_args = args.clone();
                selected_args["base"] = targets.expect("validated dependency targets")
                    [path.to_string_lossy().as_ref()]
                .clone();
                dependency.access.execute(
                    &context.root.join(path),
                    name,
                    &selected_args,
                    &dependency.status()?,
                )
            },
            |prs| {
                let mut root_args = args.clone();
                root_args["body"] = json!(reqvire_workspace::publication::linked_body(
                    args["body"].as_str().unwrap_or(""),
                    prs
                ));
                self.github.execute(&context.root, name, &root_args, status)
            },
        );
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
