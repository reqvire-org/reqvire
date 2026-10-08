//! Context routing and Git worktree administration. Model state stays in fixed-root workers.
use crate::{
    live_store::LiveStore, mcp_github::GithubAccess, mcp_process::git, mcp_session::lock_file,
    mcp_worker::WorkerOptions,
};
use reqvire::{
    error::ReqvireError,
    explorer_runtime::{ExplorerRuntimeAssets, ExplorerRuntimeData},
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc, Arc, LazyLock, Mutex, RwLock,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn error(message: impl Into<String>) -> ReqvireError {
    ReqvireError::ProcessError(message.into())
}
fn text<'a>(value: &'a Value, field: &str) -> Result<&'a str, ReqvireError> {
    value[field]
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| error(format!("{field} must be a non-empty string")))
}
fn parse_json(bytes: &str) -> Result<Value, ReqvireError> {
    let mut deserializer = serde_json::Deserializer::from_str(bytes);
    deserializer.disable_recursion_limit();
    let value = Value::deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(value)
}

pub struct PublishedRuntime {
    pub assets: ExplorerRuntimeAssets,
    pub live: LiveStore,
}
impl PublishedRuntime {
    pub fn new(data: ExplorerRuntimeData) -> Result<Self, ReqvireError> {
        let live = LiveStore::new(&data.project_store, &data.ontologies_ttl)?;
        let assets = data.into_assets()?;
        Ok(Self { assets, live })
    }

    fn from_worker(mut value: Value) -> Result<Self, ReqvireError> {
        let fields = value
            .as_object_mut()
            .ok_or_else(|| error("Invalid worker runtime"))?;
        let project_store = fields
            .remove("project_store")
            .ok_or_else(|| error("Missing worker Project Store"))?;
        let Some(Value::String(ontologies_ttl)) = fields.remove("ontologies_ttl") else {
            return Err(error("Missing worker ontology data"));
        };
        Self::new(ExplorerRuntimeData {
            project_store,
            ontologies_ttl,
        })
    }
}
struct Published {
    status: Value,
    sequence: u64,
    runtime: Option<Arc<PublishedRuntime>>,
    runtime_error: Option<String>,
    failure: Option<String>,
    stopped: bool,
}
type WorkerReply = Result<Value, String>;
type PendingReplies = Arc<Mutex<BTreeMap<u64, mpsc::SyncSender<WorkerReply>>>>;
struct Worker {
    child: Mutex<Child>,
    input: Mutex<ChildStdin>,
    pending: PendingReplies,
    next_id: AtomicU64,
    alive: Arc<AtomicBool>,
}
fn fail_pending(pending: &PendingReplies, message: &str) {
    let replies = std::mem::take(&mut *pending.lock().expect("worker replies poisoned"));
    for (_, reply) in replies {
        let _ = reply.send(Err(message.into()));
    }
}
impl Worker {
    fn spawn(
        executable: &Path,
        root: &Path,
        options: &WorkerOptions,
    ) -> Result<(Self, Value), ReqvireError> {
        let mut child = Command::new(executable)
            .arg("__mcp-worktree-worker")
            .current_dir(root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let input = child.stdin.take().expect("piped stdin");
        let output = child.stdout.take().expect("piped stdout");
        let (ready_send, ready_receive) = mpsc::sync_channel(1);
        let alive = Arc::new(AtomicBool::new(true));
        let reader_alive = Arc::clone(&alive);
        let pending: PendingReplies = Arc::new(Mutex::new(BTreeMap::new()));
        let reader_pending = Arc::clone(&pending);
        std::thread::spawn(move || {
            let mut lines = BufReader::new(output).lines();
            let parse = |line: std::io::Result<String>| {
                line.map_err(|e| e.to_string())
                    .and_then(|line| parse_json(&line).map_err(|e| e.to_string()))
            };
            let first = lines
                .next()
                .ok_or_else(|| "Worker exited before initialization".to_owned())
                .and_then(parse);
            let _ = ready_send.send(first);
            let mut failure =
                "Worktree worker exited; inspect its files before reopening".to_owned();
            for line in lines {
                let value = match parse(line) {
                    Ok(value) => value,
                    Err(e) => {
                        failure = e;
                        break;
                    }
                };
                let reply = value["request_id"].as_u64().and_then(|id| {
                    reader_pending
                        .lock()
                        .expect("worker replies poisoned")
                        .remove(&id)
                });
                let Some(reply) = reply else {
                    failure = "Invalid or unmatched worker response ID".into();
                    break;
                };
                let _ = reply.send(Ok(value));
            }
            reader_alive.store(false, Ordering::Release);
            fail_pending(&reader_pending, &failure);
        });
        let worker = Self {
            child: Mutex::new(child),
            input: Mutex::new(input),
            pending,
            next_id: AtomicU64::new(1),
            alive,
        };
        worker.send(&json!(options))?;
        let ready = ready_receive
            .recv_timeout(Duration::from_secs(120))
            .map_err(|_| error("Worktree initialization stopped or timed out"))?
            .map_err(error)?;
        if let Some(message) = ready["error"].as_str() {
            return Err(error(message));
        }
        Ok((worker, ready))
    }
    fn send(&self, request: &Value) -> Result<(), ReqvireError> {
        let mut input = self.input.lock().expect("worker input poisoned");
        serde_json::to_writer(&mut *input, request)?;
        input.write_all(b"\n")?;
        input.flush()?;
        drop(input);
        Ok(())
    }
    fn request(&self, value: &Value) -> Result<Value, ReqvireError> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (send, receive) = mpsc::sync_channel(1);
        {
            let mut pending = self.pending.lock().expect("worker replies poisoned");
            if !self.alive.load(Ordering::Acquire) {
                return Err(error("Worktree worker is unavailable"));
            }
            pending.insert(id, send);
        }
        let mut request = value.clone();
        request["request_id"] = json!(id);
        let result = self.send(&request).and_then(|_| {
            receive.recv_timeout(Duration::from_secs(120)).map_err(|_| {
                error("Worktree request stopped or timed out; effects may be unknown; inspect files before reopening")
            })?.map_err(error)
        });
        if result.is_err() {
            self.stop();
        }
        result
    }
    fn stop(&self) {
        self.alive.store(false, Ordering::Release);
        let mut child = self.child.lock().expect("worker process poisoned");
        let _ = child.kill();
        let _ = child.wait();
        drop(child);
        fail_pending(
            &self.pending,
            "Worktree worker stopped; effects may be unknown; inspect files before reopening",
        );
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.stop();
    }
}
pub struct Context {
    pub id: String,
    pub root: PathBuf,
    pub branch: String,
    original: bool,
    managed: bool,
    read_only: bool,
    // This is the per-context gate, also held through commits, publication and removal.
    worker: Mutex<Option<Arc<Worker>>>,
    published: RwLock<Published>,
    alive: Arc<AtomicBool>,
}
impl Context {
    fn update(&self, response: &mut Value) -> Result<(), ReqvireError> {
        let runtime = response
            .get_mut("runtime")
            .map(Value::take)
            .map(PublishedRuntime::from_worker)
            .map(|runtime| runtime.map(Arc::new))
            .transpose();
        let mut published = self.published.write().expect("context registry poisoned");
        if let Some(status) = response.get("status") {
            let sequence = response["sequence"]
                .as_u64()
                .unwrap_or(published.sequence + 1);
            if sequence >= published.sequence {
                published.status = status.clone();
                published.sequence = sequence;
            }
        }
        match runtime {
            Ok(Some(runtime)) => {
                published.runtime = Some(runtime);
                published.runtime_error = None;
            }
            Err(failure) => published.runtime_error = Some(failure.to_string()),
            _ => {}
        }
        if let Some(message) = response["runtime_error"].as_str() {
            published.runtime_error = Some(message.into());
        } else if response["runtime_unchanged"] == true {
            published.runtime_error = None;
        }
        drop(published);
        Ok(())
    }
    fn request_locked(
        &self,
        worker: &mut Option<Arc<Worker>>,
        request: &Value,
    ) -> Result<Value, ReqvireError> {
        let result = worker
            .as_mut()
            .ok_or_else(|| error("Worktree context is unavailable"))?
            .request(request);
        let mut response = match result {
            Ok(response) => response,
            Err(failure) => {
                if let Some(worker) = worker.take() {
                    worker.stop();
                }
                self.published
                    .write()
                    .expect("context registry poisoned")
                    .failure = Some(failure.to_string());
                return Err(failure);
            }
        };
        self.update(&mut response)?;
        if let Some(message) = response["error"].as_str() {
            return Err(error(message));
        }
        Ok(response)
    }
    fn request(&self, request: &Value) -> Result<Value, ReqvireError> {
        self.request_locked(
            &mut self.worker.lock().expect("worktree lock poisoned"),
            request,
        )
    }
    #[cfg(test)]
    pub(super) fn publish_test_response(&self, mut response: Value) -> Result<(), ReqvireError> {
        self.update(&mut response)
    }
    #[cfg(test)]
    pub(super) fn test_control_gate(&self) -> impl Drop + '_ {
        self.worker.lock().expect("worktree lock poisoned")
    }
    #[cfg(test)]
    pub(super) fn test_refresh_runtime(&self) -> Result<Value, ReqvireError> {
        self.request(&json!({"operation":"runtime"}))
    }
    fn read_request(&self, request: &Value) -> Result<Value, ReqvireError> {
        let worker = self
            .worker
            .lock()
            .expect("worktree lock poisoned")
            .as_ref()
            .cloned()
            .ok_or_else(|| error("Worktree context is unavailable"))?;
        let mut response = match worker.request(request) {
            Ok(response) => response,
            Err(failure) => {
                self.published
                    .write()
                    .expect("context registry poisoned")
                    .failure = Some(failure.to_string());
                return Err(failure);
            }
        };
        self.update(&mut response)?;
        let published = self.published.read().expect("context registry poisoned");
        if !self.alive.load(Ordering::Acquire) || published.stopped || published.failure.is_some() {
            return Err(error("Worktree context became unavailable during read"));
        }
        drop(published);
        Ok(response)
    }
    pub fn metadata(&self) -> Value {
        let published = self.published.read().expect("context registry poisoned");
        self.decorate_metadata(published.status.clone(), &published)
    }
    fn read_metadata(&self, status: Value) -> Value {
        let published = self.published.read().expect("context registry poisoned");
        self.decorate_metadata(status, &published)
    }
    fn decorate_metadata(&self, mut value: Value, published: &Published) -> Value {
        value["worktree_id"] = json!(self.id);
        value["original"] = json!(self.original);
        value["managed"] = json!(self.managed);
        let alive = self.alive.load(Ordering::Acquire);
        let available = alive
            && published.failure.is_none()
            && !published.stopped
            && value["available"].as_bool() == Some(true);
        value["available"] = json!(available);
        value["writes_available"] =
            json!(available && !self.read_only && value["writes_available"] == true);
        value["owned"] =
            json!(!self.read_only && alive && !published.stopped && published.failure.is_none());
        value["state"] = json!(if published.stopped {
            "stopped"
        } else if available && value["recovery_required"] == true {
            "recovery_required"
        } else if available {
            "ready"
        } else {
            "unavailable"
        });
        value["diagnostic"] = json!(published
            .failure
            .as_deref()
            .or_else(|| (!alive && !published.stopped)
                .then_some("Worktree worker exited; reopen to recover"))
            .or_else(|| value["diagnostic"].as_str()));
        value["explorer_available"] =
            json!(available && published.runtime.is_some() && published.runtime_error.is_none());
        value["explorer_diagnostic"] = json!(published.runtime_error);
        value
    }
    pub fn runtime(&self) -> Result<Arc<PublishedRuntime>, ReqvireError> {
        let published = self.published.read().expect("context registry poisoned");
        if !self.alive.load(Ordering::Acquire)
            || published.stopped
            || published.failure.is_some()
            || published.status["available"].as_bool() != Some(true)
        {
            return Err(error(format!("Worktree {} is unavailable", self.id)));
        }
        if let Some(message) = &published.runtime_error {
            return Err(error(message));
        }
        published
            .runtime
            .clone()
            .ok_or_else(|| error("Explorer runtime is unavailable for this worktree"))
    }
    pub fn load_for_browser(&self) -> Result<(), ReqvireError> {
        if self.read_only {
            self.request(&json!({"operation":"load"}))?;
        }
        self.runtime().map(|_| ())
    }
}

struct SessionToolCatalog {
    definitions: Vec<Value>,
    // The boolean distinguishes core schemas from wrapper-local operations.
    by_name: BTreeMap<String, (usize, bool)>,
}

impl SessionToolCatalog {
    fn new(github_available: bool) -> Self {
        #[cfg(test)]
        tests::CATALOG_BUILDS.with(|count| count.set(count.get() + 1));
        let mut definitions = reqvire::tool_interface::tool_definitions(true);
        let core_count = definitions.len();
        for tool in &mut definitions {
            if tool["name"] != "reqvire.tool_contract" {
                tool["inputSchema"]["properties"]["worktree_id"] = selector();
            }
        }
        definitions.extend(local_definitions().iter().cloned());
        if github_available {
            definitions.extend(publication_definitions());
        }
        let by_name = definitions
            .iter()
            .enumerate()
            .map(|(index, tool)| {
                (
                    tool["name"]
                        .as_str()
                        .expect("tool definition name")
                        .to_owned(),
                    (index, index < core_count),
                )
            })
            .collect();
        Self {
            definitions,
            by_name,
        }
    }

    fn get(&self, name: &str) -> Option<(&Value, bool)> {
        self.by_name
            .get(name)
            .map(|(index, core)| (&self.definitions[*index], *core))
    }
}

pub struct Worktrees {
    root: PathBuf,
    common: PathBuf,
    storage: PathBuf,
    executable: PathBuf,
    enable_commits: bool,
    with_size_estimates: bool,
    explorer: bool,
    read_only: bool,
    pub original: String,
    contexts: RwLock<BTreeMap<String, Arc<Context>>>,
    browser_branches: Mutex<BTreeMap<String, String>>,
    admin: Mutex<()>,
    github: GithubAccess,
    tool_catalog: SessionToolCatalog,
}
struct WorktreePolicy {
    enable_commits: bool,
    with_size_estimates: bool,
    explorer: bool,
    read_only: bool,
}
impl Worktrees {
    pub fn start(
        root: &Path,
        executable: &Path,
        enable_commits: bool,
        with_size_estimates: bool,
        explorer: bool,
        enable_github: bool,
        remote: &str,
    ) -> Result<Arc<Self>, ReqvireError> {
        Self::start_mode(
            root,
            executable,
            WorktreePolicy {
                enable_commits,
                with_size_estimates,
                explorer,
                read_only: false,
            },
            enable_github,
            remote,
        )
    }
    pub fn start_read_only(root: &Path, executable: &Path) -> Result<Arc<Self>, ReqvireError> {
        Self::start_mode(
            root,
            executable,
            WorktreePolicy {
                enable_commits: false,
                with_size_estimates: false,
                explorer: true,
                read_only: true,
            },
            false,
            "origin",
        )
    }
    fn start_mode(
        root: &Path,
        executable: &Path,
        policy: WorktreePolicy,
        enable_github: bool,
        remote: &str,
    ) -> Result<Arc<Self>, ReqvireError> {
        let WorktreePolicy {
            enable_commits,
            with_size_estimates,
            explorer,
            read_only,
        } = policy;
        let root = root.canonicalize()?;
        let common = PathBuf::from(git(
            &root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?)
        .canonicalize()?;
        // Sibling storage cannot be scanned as part of the original or another context.
        let repository = PathBuf::from(git(&root, &["rev-parse", "--show-toplevel"])?);
        let parent = common
            .parent()
            .and_then(Path::parent)
            .or_else(|| repository.parent())
            .ok_or_else(|| error("Cannot allocate worktree storage beside repository"))?;
        let storage = parent.join(format!(
            ".reqvire-{}-worktrees",
            reqvire::utils::hash_content(&common.to_string_lossy())
        ));
        let github = GithubAccess::check(&root, enable_github, remote);
        if enable_github {
            eprintln!("MCP GitHub publication: {}", github.status());
        }
        let original = Self::new_id();
        let manager = Arc::new(Self {
            root: root.clone(),
            common,
            storage,
            executable: executable.into(),
            enable_commits,
            with_size_estimates,
            explorer,
            read_only,
            original: original.clone(),
            contexts: RwLock::new(BTreeMap::new()),
            browser_branches: Mutex::new(BTreeMap::new()),
            admin: Mutex::new(()),
            tool_catalog: SessionToolCatalog::new(github.available()),
            github,
        });
        manager.admit(root, original, true, false)?;
        Ok(manager)
    }
    fn new_id() -> String {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        format!(
            "wt-{:x}-{:x}-{:x}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        )
    }
    fn admit(
        &self,
        root: PathBuf,
        id: String,
        original: bool,
        managed: bool,
    ) -> Result<Arc<Context>, ReqvireError> {
        let root = root.canonicalize()?;
        if PathBuf::from(git(
            &root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?)
        .canonicalize()?
            != self.common
        {
            return Err(error("Worktree belongs to a different repository"));
        }
        for context in self
            .contexts
            .read()
            .expect("context registry poisoned")
            .values()
        {
            // Never allow a context to scan another context.
            if root.starts_with(&context.root) || context.root.starts_with(&root) {
                return Err(error("Overlapping worktree roots are not allowed"));
            }
        }
        let (worker, mut ready) = Worker::spawn(
            &self.executable,
            &root,
            &WorkerOptions {
                worktree_id: id.clone(),
                enable_commits: self.enable_commits,
                with_size_estimates: self.with_size_estimates,
                explorer: self.explorer,
                read_only: self.read_only,
                read_parallelism: crate::mcp_worker::available_parallelism(),
            },
        )?;
        let branch = text(&ready["status"], "branch")?.to_string();
        let context = Arc::new(Context {
            id: id.clone(),
            root,
            branch,
            original,
            managed,
            read_only: self.read_only,
            alive: Arc::clone(&worker.alive),
            worker: Mutex::new(Some(Arc::new(worker))),
            published: RwLock::new(Published {
                status: ready["status"].clone(),
                sequence: 0,
                runtime: None,
                runtime_error: None,
                failure: None,
                stopped: false,
            }),
        });
        context.update(&mut ready)?;
        self.contexts
            .write()
            .expect("validated worktree context invariant")
            .insert(id, Arc::clone(&context));
        Ok(context)
    }
    fn resolve(&self, id: Option<&str>) -> Result<Arc<Context>, ReqvireError> {
        let contexts = self.contexts.read().expect("context registry poisoned");
        let result = match id {
            Some(id) => contexts
                .get(id)
                .cloned()
                .ok_or_else(|| error(format!("Unknown or removed worktree_id: {id}"))),
            None if contexts.len() == 1 => Ok(Arc::clone(
                contexts
                    .values()
                    .next()
                    .expect("validated worktree context invariant"),
            )),
            None => Err(error(format!(
                "worktree_id is required with multiple contexts: {}",
                contexts.keys().cloned().collect::<Vec<_>>().join(", ")
            ))),
        };
        drop(contexts);
        result
    }
    fn resolve_browser(&self, id: Option<&str>) -> Result<Arc<Context>, ReqvireError> {
        let id = id.unwrap_or(&self.original);
        let context = match self.resolve(Some(id)) {
            Ok(context) => context,
            Err(failure) => {
                let branch = self
                    .browser_branches
                    .lock()
                    .expect("browser inventory poisoned")
                    .iter()
                    .find(|(_, known)| known.as_str() == id)
                    .map(|(branch, _)| branch.clone());
                let Some(branch) = branch else {
                    return Err(failure);
                };
                self.create_or_open(&json!({"branch":branch}), false)?;
                self.resolve(Some(id))?
            }
        };
        Ok(context)
    }
    pub fn browser_context(&self, id: Option<&str>) -> Result<Arc<Context>, ReqvireError> {
        let context = self.resolve_browser(id)?;
        context.runtime()?;
        Ok(context)
    }
    /// Manifest/chunk and asset reads cannot admit or initialize an unselected branch.
    pub fn published_browser_context(
        &self,
        id: Option<&str>,
    ) -> Result<Arc<Context>, ReqvireError> {
        let context = self.resolve(Some(id.unwrap_or(&self.original)))?;
        context.runtime()?;
        Ok(context)
    }
    pub fn load_browser_context(&self, id: Option<&str>) -> Result<Arc<Context>, ReqvireError> {
        let context = self.resolve_browser(id)?;
        context.load_for_browser()?;
        Ok(context)
    }
    pub fn browser_inventory(&self) -> Value {
        match self.branch_inventory() {
            Ok(rows) => json!({"original_worktree_id":self.original,"worktrees":rows}),
            Err(failure) => json!({"error":failure.to_string(),"worktrees":[]}),
        }
    }
    fn branch_inventory(&self) -> Result<Vec<Value>, ReqvireError> {
        let refs = git(
            &self.root,
            &[
                "for-each-ref",
                "--format=%(refname:short)%09%(objectname)",
                "refs/heads/",
            ],
        )?;
        let worktrees = repository_worktrees(&self.root)?;
        let contexts = self.contexts.read().expect("context registry poisoned");
        let mut choices = self
            .browser_branches
            .lock()
            .expect("browser inventory poisoned");
        let mut rows = Vec::new();
        let mut present = std::collections::BTreeSet::new();
        for line in refs.lines() {
            let Some((branch, head)) = line.split_once('\t') else {
                continue;
            };
            present.insert(branch.to_string());
            if let Some(context) = contexts.values().find(|c| c.branch == branch) {
                choices.insert(branch.into(), context.id.clone());
                rows.push(context.metadata());
            } else {
                let id = choices.entry(branch.into()).or_insert_with(Self::new_id);
                let root = worktrees
                    .iter()
                    .find(|r| r["branch"] == branch)
                    .and_then(|r| r["workspace_root"].as_str())
                    .unwrap_or("");
                rows.push(json!({"worktree_id":id,"branch":branch,"head":head,
                    "workspace_root":root,"state":"unloaded","owned":false,
                    "available":true,"explorer_available":true,"original":false}));
            }
        }
        choices.retain(|branch, _| present.contains(branch));
        // Keep an original detached/workspace context selectable as well.
        for context in contexts.values().filter(|c| !present.contains(&c.branch)) {
            rows.push(context.metadata());
        }
        drop(choices);
        drop(contexts);
        Ok(rows)
    }
    fn inventory(&self) -> Result<Value, ReqvireError> {
        let contexts = self.contexts.read().expect("context registry poisoned");
        let mut rows = repository_worktrees(&self.root)?;
        for row in &mut rows {
            if let (Some(path), Some(branch)) =
                (row["workspace_root"].as_str(), row["branch"].as_str())
            {
                row["managed"] = json!(self.is_managed(Path::new(path), branch));
            }
            if let Some(context) = contexts
                .values()
                .find(|c| row["workspace_root"].as_str() == c.root.to_str())
            {
                let metadata = context.metadata();
                row.as_object_mut()
                    .expect("validated worktree context invariant")
                    .extend(
                        metadata
                            .as_object()
                            .expect("validated worktree context invariant")
                            .clone(),
                    );
            }
        }
        Ok(
            json!({"worktrees":rows,"original_worktree_id":self.original,"github":self.github.status()}),
        )
    }
    fn managed_path(&self) -> Result<PathBuf, ReqvireError> {
        fs::create_dir_all(&self.storage)?;
        if fs::symlink_metadata(&self.storage)?
            .file_type()
            .is_symlink()
        {
            return Err(error("Managed storage must not be a symlink"));
        }
        Ok(self.storage.join(Self::new_id()))
    }
    fn provenance_path(&self, root: &Path) -> PathBuf {
        self.common.join(format!(
            "reqvire-managed-{}.json",
            reqvire::utils::hash_content(&root.to_string_lossy())
        ))
    }
    fn is_managed(&self, root: &Path, branch: &str) -> bool {
        if !root.starts_with(&self.storage) {
            return false;
        }
        fs::read(self.provenance_path(root))
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
            .is_some_and(|record| {
                record["workspace_root"] == json!(root) && record["branch"] == branch
            })
    }
    fn validate_branch(&self, branch: &str) -> Result<(), ReqvireError> {
        if branch.starts_with('-') || branch == "HEAD" {
            return Err(error("Invalid branch name"));
        }
        git(
            &self.root,
            &["check-ref-format", &format!("refs/heads/{branch}")],
        )?;
        Ok(())
    }
    fn create_or_open(&self, args: &Value, create: bool) -> Result<Value, ReqvireError> {
        let _gate = self.admin.lock().expect("worktree lock poisoned");
        let branch = text(args, "branch")?;
        self.validate_branch(branch)?;
        let existing = self
            .contexts
            .read()
            .expect("validated worktree context invariant")
            .values()
            .find(|c| c.branch == branch)
            .cloned();
        if let Some(context) = existing.as_ref() {
            if create {
                return Err(error("Branch already exists"));
            }
            // Idempotence does not import disk changes. A failed context must be explicitly readmitted.
            if context.request(&json!({"operation":"status"})).is_ok()
                && context.metadata()["available"] == true
            {
                if self.explorer && context.metadata()["explorer_available"] != true {
                    context.request(&json!({"operation":"runtime"}))?;
                }
                return Ok(context.metadata());
            }
        }
        // An already accepted context needs no Git administration. Acquire the
        // cross-process lock before changing registrations or preparing worktrees.
        let _repo_lock = lock_file(&self.common.join("reqvire-mcp-administration.lock"))?;
        if let Some(context) = existing {
            // Readers and browser handles can retain this context after it is
            // removed from the registry. Stop its process explicitly before
            // readmission so those handles cannot retain the old ownership.
            let mut worker = context.worker.lock().expect("worktree lock poisoned");
            if let Some(worker) = worker.take() {
                worker.stop();
            }
            context
                .published
                .write()
                .expect("context registry poisoned")
                .stopped = true;
            self.contexts
                .write()
                .expect("context registry poisoned")
                .remove(&context.id);
            self.browser_branches
                .lock()
                .expect("browser inventory poisoned")
                .remove(branch);
            drop(worker);
        }
        let source = if create {
            let base = text(args, "base_ref")?;
            let relative = base.starts_with("HEAD") || base.starts_with('@');
            if args.get("from_worktree_id").is_some()
                || relative
                || self
                    .contexts
                    .read()
                    .expect("context registry poisoned")
                    .len()
                    == 1
            {
                Some(self.resolve(args["from_worktree_id"].as_str())?)
            } else {
                self.contexts
                    .read()
                    .expect("validated worktree context invariant")
                    .values()
                    .find(|c| base == c.branch || base == format!("refs/heads/{}", c.branch))
                    .cloned()
            }
        } else {
            None
        };
        let mut source_gate = source
            .as_ref()
            .map(|s| s.worker.lock().expect("worktree lock poisoned"));
        if let (Some(source), Some(gate)) = (&source, source_gate.as_mut()) {
            source.request_locked(gate, &json!({"operation":"check_clean"}))?;
        }
        let resolution_root = source
            .as_ref()
            .map(|c| c.root.as_path())
            .unwrap_or(&self.root);
        let base = if create {
            text(args, "base_ref")?.to_string()
        } else {
            format!("refs/heads/{branch}")
        };
        let base_commit = git(
            resolution_root,
            &[
                "rev-parse",
                "--verify",
                "--end-of-options",
                &format!("{base}^{{commit}}"),
            ],
        )?;
        let existing_path = repository_worktrees(&self.root)?
            .into_iter()
            .find(|r| r["branch"].as_str() == Some(branch))
            .and_then(|r| r["workspace_root"].as_str().map(PathBuf::from));
        if create
            && (existing_path.is_some()
                || git(
                    &self.root,
                    &["show-ref", "--verify", &format!("refs/heads/{branch}")],
                )
                .is_ok())
        {
            return Err(error("Branch already exists"));
        }
        let created = existing_path.is_none();
        let root = match existing_path {
            Some(path) => path,
            None => self.managed_path()?,
        };
        if created {
            let path = root
                .to_str()
                .ok_or_else(|| error("Non-UTF8 worktree path"))?;
            let args = if create {
                vec!["worktree", "add", "-b", branch, "--", path, &base_commit]
            } else {
                vec!["worktree", "add", "--", path, branch]
            };
            if let Err(failure) = git(&self.root, &args) {
                return Err(self.creation_failure(failure, &root, branch, &base_commit, create));
            }
            if let Err(failure) = fs::write(
                self.provenance_path(&root),
                serde_json::to_vec(&json!({"workspace_root":root,"branch":branch}))?,
            ) {
                return Err(self.creation_failure(
                    format!("Cannot record managed worktree: {failure}"),
                    &root,
                    branch,
                    &base_commit,
                    create,
                ));
            }
        }
        let managed = self.is_managed(&root, branch);
        let id = self
            .browser_branches
            .lock()
            .expect("browser inventory poisoned")
            .entry(branch.into())
            .or_insert_with(Self::new_id)
            .clone();
        match self.admit(root.clone(), id, false, managed) {
            Ok(context) => {
                let mut value = context.metadata();
                value["base_commit"] = json!(base_commit);
                Ok(value)
            }
            Err(failure) => Err(if created {
                self.creation_failure(failure, &root, branch, &base_commit, create)
            } else {
                failure
            }),
        }
    }
    fn creation_failure(
        &self,
        failure: impl std::fmt::Display,
        root: &Path,
        branch: &str,
        base: &str,
        new_branch: bool,
    ) -> ReqvireError {
        let cleanup = match self.compensate_creation(root, branch, base, new_branch) {
            Ok(true) => return error(failure.to_string()),
            Ok(false) => String::new(),
            Err(reason) => format!("; cleanup failed: {reason}"),
        };
        error(format!(
            "{failure}{cleanup}; partial worktree/ref retained for recovery: {} ({branch})",
            root.display()
        ))
    }
    // Compensation is limited to a new, still-clean checkout at the exact expected branch and commit.
    fn compensate_creation(
        &self,
        root: &Path,
        branch: &str,
        base: &str,
        new_branch: bool,
    ) -> Result<bool, ReqvireError> {
        let _guard = lock_file(&self.common.join(format!(
            "reqvire-mcp-{}.lock",
            reqvire::utils::hash_content(&format!("refs/heads/{branch}"))
        )))?;
        if !repository_worktrees(&self.root).is_ok_and(|rows| {
            rows.iter().any(|row| {
                row["workspace_root"].as_str() == root.to_str() && row["branch"] == branch
            })
        }) || crate::mcp_session::require_clean(root).is_err()
            || git(root, &["rev-parse", "HEAD"]).ok().as_deref() != Some(base)
            || git(root, &["symbolic-ref", "--short", "HEAD"])
                .ok()
                .as_deref()
                != Some(branch)
        {
            return Ok(false);
        }
        if git(
            &self.root,
            &[
                "worktree",
                "remove",
                "--",
                root.to_str().expect("validated worktree context invariant"),
            ],
        )
        .is_err()
        {
            return Ok(false);
        }
        let _ = fs::remove_file(self.provenance_path(root));
        Ok(!new_branch
            || git(
                &self.root,
                &["update-ref", "-d", &format!("refs/heads/{branch}"), base],
            )
            .is_ok())
    }
    fn remove(&self, context: &Arc<Context>) -> Result<Value, ReqvireError> {
        let _gate = self.admin.lock().expect("worktree lock poisoned");
        let _repo_lock = lock_file(&self.common.join("reqvire-mcp-administration.lock"))?;
        if context.original || !context.managed {
            return Err(error(
                "Only server-created secondary worktrees may be removed",
            ));
        }
        let mut worker = context.worker.lock().expect("worktree lock poisoned");
        context.request_locked(&mut worker, &json!({"operation":"check_clean"}))?;
        let head = context.metadata()["head"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        // A Git worktree lock prevents cooperating Git removal/move while the child stops.
        git(
            &context.root,
            &[
                "worktree",
                "lock",
                "--reason",
                "Reqvire removal",
                context
                    .root
                    .to_str()
                    .expect("validated worktree context invariant"),
            ],
        )?;
        if let Some(worker) = worker.take() {
            worker.stop();
        }
        context
            .published
            .write()
            .expect("context registry poisoned")
            .stopped = true;
        drop(worker);
        let cleanup = (|| {
            let _branch_lock = lock_file(&self.common.join(format!(
                "reqvire-mcp-{}.lock",
                reqvire::utils::hash_content(&format!("refs/heads/{}", context.branch))
            )))?;
            let path = context.root.join(git(
                &context.root,
                &["rev-parse", "--git-path", "reqvire-mcp-worktree.lock"],
            )?);
            let _worktree_lock = lock_file(&path)?;
            crate::mcp_session::require_clean(&context.root)?;
            if git(&context.root, &["rev-parse", "HEAD"])? != head
                || git(&context.root, &["symbolic-ref", "--short", "HEAD"])? != context.branch
            {
                return Err(error("Branch or HEAD changed during removal"));
            }
            git(
                &self.root,
                &[
                    "worktree",
                    "unlock",
                    context
                        .root
                        .to_str()
                        .expect("validated worktree context invariant"),
                ],
            )?;
            git(
                &self.root,
                &[
                    "worktree",
                    "remove",
                    "--",
                    context
                        .root
                        .to_str()
                        .expect("validated worktree context invariant"),
                ],
            )?;
            Ok::<_, ReqvireError>(())
        })();
        if let Err(failure) = cleanup {
            let _ = git(
                &self.root,
                &[
                    "worktree",
                    "unlock",
                    context
                        .root
                        .to_str()
                        .expect("validated worktree context invariant"),
                ],
            );
            return Err(error(format!(
                "Worktree {} stopped; path retained at {}: {failure}",
                context.id,
                context.root.display()
            )));
        }
        self.contexts
            .write()
            .expect("context registry poisoned")
            .remove(&context.id);
        self.browser_branches
            .lock()
            .expect("browser inventory poisoned")
            .remove(&context.branch);
        let _ = fs::remove_file(self.provenance_path(&context.root));
        Ok(
            json!({"outcome":"completed","worktree_id":context.id,"branch":context.branch,"head":head,"removed":true}),
        )
    }
    pub fn handle(&self, method: &str, params: Value) -> Result<Value, rmcp::ErrorData> {
        let selected = if method == "resources/read" {
            params["uri"]
                .as_str()
                .and_then(|uri| resource_selector(uri).ok())
                .and_then(|(_, id)| self.resolve(id.as_deref()).ok())
        } else {
            self.resolve(params["arguments"]["worktree_id"].as_str())
                .ok()
        };
        if method == "tools/call" {
            let name = params["name"].as_str().unwrap_or("");
            let empty = json!({});
            let args = params.get("arguments").unwrap_or(&empty);
            let validation = (|| {
                let (definition, core_tool) = self
                    .tool_catalog
                    .get(name)
                    .ok_or_else(|| error(format!("Tool {name} is unavailable for this session")))?;
                validate_arguments(args, &definition["inputSchema"])?;
                if core_tool {
                    let mut core_args = args.clone();
                    if let Some(arguments) = core_args.as_object_mut() {
                        arguments.remove("worktree_id");
                    }
                    reqvire::tool_interface::validate_tool_arguments(name, &core_args, true)
                        .map_err(error)?;
                }
                Ok::<(), ReqvireError>(())
            })();
            if let Err(failure) = validation {
                return Err(rmcp::ErrorData::invalid_params(
                    "Invalid tool arguments or unavailable tool",
                    Some(
                        json!({"tool":name,"message":failure.to_string(),"context":selected.as_ref().map(|c|c.metadata())}),
                    ),
                ));
            }
        }
        if crate::mcp_worker::parallel_read(method, &params)
            && !(method == "resources/read" && params["uri"] == "reqvire://tools/contract")
        {
            return self.handle_read(method, params);
        }
        let result = self.handle_inner(method, &params);
        result.map_err(|e| {
            rmcp::ErrorData::invalid_params(
                e.to_string(),
                selected.map(|c| json!({"context":c.metadata()})),
            )
        })
    }
    fn handle_inner(&self, method: &str, params: &Value) -> Result<Value, ReqvireError> {
        match method {
            "tools/list" => return Ok(json!({"tools":self.definitions()})),
            "resources/list" => {
                let mut resources = Vec::new();
                for resource in reqvire::tool_interface::resource_definitions() {
                    if resource["uri"] == "reqvire://tools/contract" {
                        resources.push(resource);
                        continue;
                    }
                    for context in self
                        .contexts
                        .read()
                        .expect("context registry poisoned")
                        .values()
                    {
                        let mut resource = resource.clone();
                        resource["uri"] = json!(format!(
                            "{}?worktree_id={}",
                            resource["uri"]
                                .as_str()
                                .expect("validated worktree context invariant"),
                            context.id
                        ));
                        resource["name"] = json!(format!(
                            "{} ({})",
                            resource["name"]
                                .as_str()
                                .expect("validated worktree context invariant"),
                            context.branch
                        ));
                        resources.push(resource);
                    }
                }
                return Ok(json!({"resources":resources}));
            }
            "resources/templates/list" => {
                return Ok(
                    json!({"resourceTemplates":reqvire::tool_interface::resource_definitions().into_iter()
                .filter(|r|r["uri"]!="reqvire://tools/contract").map(|r|json!({"uriTemplate":format!("{}?worktree_id={{worktree_id}}",r["uri"].as_str().expect("validated worktree context invariant")),"name":r["name"],"mimeType":r["mimeType"]})).collect::<Vec<_>>() }),
                )
            }
            "prompts/list" => {
                let mut prompts = reqvire::mcp_prompts::prompt_definitions_json();
                // Core returns the definitions array, not a protocol envelope.
                for prompt in &mut prompts {
                    if !prompt["arguments"].is_array() {
                        prompt["arguments"] = json!([]);
                    }
                    prompt["arguments"].as_array_mut().expect("validated worktree context invariant").push(json!({"name":"worktree_id","description":"Selected worktree context; required when multiple contexts exist","required":false}));
                }
                return Ok(json!({"prompts":prompts}));
            }
            _ => {}
        }
        if method == "tools/call" {
            let name = text(params, "name")?.to_string();
            let args = params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));
            let result = self.call_tool(&name, args);
            return Ok(match result {
                Ok(value) => value,
                Err(failure) => crate::mcp::tool_error(&name, failure),
            });
        }
        if method == "resources/read" && params["uri"] == "reqvire://tools/contract" {
            return Ok(
                json!({"contents":[{"uri":"reqvire://tools/contract","mimeType":"application/json","text":self.contract().to_string()}]}),
            );
        }
        Err(error(format!("Unknown MCP method: {method}")))
    }
    fn handle_read(&self, method: &str, mut params: Value) -> Result<Value, rmcp::ErrorData> {
        if method == "tools/call" && params.get("arguments").is_none() {
            params["arguments"] = json!({});
        }
        let selected = (|| {
            let (id, original_uri) = if method == "resources/read" {
                let uri = text(&params, "uri")?.to_string();
                let (base, id) = resource_selector(&uri)?;
                params["uri"] = json!(base);
                (id, Some(uri))
            } else {
                let id = params["arguments"]
                    .get("worktree_id")
                    .map(|v| {
                        v.as_str()
                            .map(str::to_owned)
                            .ok_or_else(|| error("worktree_id must be a string"))
                    })
                    .transpose()?;
                if let Some(args) = params["arguments"].as_object_mut() {
                    args.remove("worktree_id");
                }
                (id, None)
            };
            Ok::<_, ReqvireError>((self.resolve(id.as_deref())?, original_uri))
        })();
        let (context, original_uri) = match selected {
            Ok(selected) => selected,
            // Schema validation already succeeded in handle(). Preserve the
            // same execution-error envelope as serialized tool dispatch.
            Err(failure) if method == "tools/call" => {
                return Ok(crate::mcp::tool_error(
                    params["name"].as_str().unwrap_or(method),
                    failure,
                ));
            }
            Err(failure) => {
                return Err(rmcp::ErrorData::invalid_params(failure.to_string(), None));
            }
        };
        let name = params["name"].as_str().unwrap_or(method);
        let reply =
            context.read_request(&json!({"operation":"rpc","method":method,"params":params}));
        let (mut result, metadata) = match reply {
            Ok(response) => {
                let metadata = response
                    .get("read_status")
                    .filter(|status| status.is_object())
                    .map(|status| context.read_metadata(status.clone()))
                    .unwrap_or_else(|| context.metadata());
                if let Some(failure) = response.get("rpc_error") {
                    let mut failure: rmcp::ErrorData = serde_json::from_value(failure.clone())
                        .unwrap_or_else(|_| {
                            rmcp::ErrorData::internal_error("Invalid worker error", None)
                        });
                    let data = failure.data.get_or_insert_with(|| json!({}));
                    data["context"] = metadata;
                    return Err(failure);
                }
                (response["result"].clone(), metadata)
            }
            Err(failure) if method == "tools/call" => {
                (crate::mcp::tool_error(name, failure), context.metadata())
            }
            Err(failure) => {
                return Err(rmcp::ErrorData::invalid_params(
                    failure.to_string(),
                    Some(json!({"context":context.metadata()})),
                ))
            }
        };
        result["_meta"] = json!({"reqvire/context":metadata});
        if method == "tools/call" {
            if name == "reqvire.workspace_status" {
                result["structuredContent"]["github"] = self.github.status();
            }
            result["structuredContent"]["context"] = metadata.clone();
            result["content"] = json!([{"type":"text","text":serde_json::to_string_pretty(&result["structuredContent"])
                .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?}]);
        }
        if let Some(uri) = original_uri {
            for content in result["contents"].as_array_mut().into_iter().flatten() {
                content["uri"] = json!(uri);
            }
        }
        if method == "prompts/get" {
            let messages = result["messages"].as_array_mut().ok_or_else(|| {
                rmcp::ErrorData::internal_error(
                    "Invalid prompt response",
                    Some(json!({"context":metadata})),
                )
            })?;
            messages.insert(0,json!({"role":"user","content":{"type":"text","text":format!("Use worktree_id={} for all workspace-bound tool calls in this workflow.",context.id)}}));
        }
        Ok(result)
    }
    fn call_tool(&self, name: &str, mut args: Value) -> Result<Value, ReqvireError> {
        // handle() has already validated the wrapper and core schemas before
        // dispatch. The fixed-root worker still validates its process boundary.
        match name {
            "reqvire.tool_contract" => return Ok(success(self.contract())),
            "reqvire.worktree.list" => return self.inventory().map(success),
            "reqvire.worktree.create" => return self.create_or_open(&args, true).map(success),
            "reqvire.worktree.open" => return self.create_or_open(&args, false).map(success),
            _ => {}
        }
        let context = self.resolve(args["worktree_id"].as_str())?;
        args.as_object_mut()
            .expect("validated worktree context invariant")
            .remove("worktree_id");
        // Capture response metadata while the same context gate still protects the result.
        let mut worker = if name == "reqvire.worktree.remove" {
            None
        } else {
            Some(context.worker.lock().expect("worktree lock poisoned"))
        };
        let outcome = (|| {
            if name == "reqvire.worktree.remove" {
                return self.remove(&context).map(success);
            }
            let mut result = if name == "reqvire.git.reconcile" {
                let response = context.request_locked(
                    worker.as_mut().expect("validated worktree context invariant"),
                    &json!({"operation":"reconcile","attempted_commit":args["attempted_commit"],"dry_run":args["dry_run"].as_bool().unwrap_or(true)}),
                )?;
                success(response["result"].clone())
            } else if name == "reqvire.git.commit" {
                let response = context.request_locked(
                    worker
                        .as_mut()
                        .expect("validated worktree context invariant"),
                    &json!({"operation":"commit","message":args["message"]}),
                )?;
                success(response["result"].clone())
            } else if matches!(
                name,
                "reqvire.git.push" | "reqvire.github.pr.create" | "reqvire.github.pr.comment"
            ) {
                context.request_locked(worker.as_mut().expect("validated worktree context invariant"),&json!({"operation":if name.ends_with(".comment"){"check_writable"}else{"check_clean"}}))?;
                let status = context.metadata();
                if status["writes_available"] != true {
                    return Err(error("Owned HEAD changed; restart before publication"));
                }
                if !name.ends_with(".comment")
                    && status["pending_changes"]
                        .as_array()
                        .is_some_and(|a| !a.is_empty())
                {
                    return Err(error("Commit pending accepted changes before publication"));
                }
                success(self.github.execute(&context.root, name, &args, &status)?)
            } else {
                let response=context.request_locked(worker.as_mut().expect("validated worktree context invariant"),&json!({"operation":"rpc","method":"tools/call","params":{"name":name,"arguments":args}}))?;
                if let Some(failure) = response.get("rpc_error") {
                    return Err(error(failure.to_string()));
                }
                response["result"].clone()
            };
            if name == "reqvire.workspace_status" {
                result["structuredContent"]["github"] = self.github.status();
            }
            Ok(result)
        })();
        let mut result = outcome.unwrap_or_else(|failure| crate::mcp::tool_error(name, failure));
        let metadata = context.metadata();
        drop(worker);
        if matches!(
            name,
            "reqvire.git.push" | "reqvire.github.pr.create" | "reqvire.github.pr.comment"
        ) {
            if result["structuredContent"]["outcome"] == "unknown" {
                result["isError"] = json!(true);
                let message = result["structuredContent"]["error"].take();
                result["structuredContent"]["error"] = json!({"code":"publication_unknown","message":message,"tool":name,"recoverability":"Inspect the remote target before retrying; no automatic retry occurred."});
            }
            if result["isError"] == true && result["structuredContent"]["outcome"].is_null() {
                result["structuredContent"]["outcome"] = json!("failed");
            }
            result["structuredContent"]["repository"] = self.github.status()["repository"].clone();
            result["structuredContent"]["remote"] = json!(self.github.remote);
            result["structuredContent"]["branch"] = metadata["branch"].clone();
            result["structuredContent"]["attempted_commit"] = metadata["head"].clone();
            if let Some(number) = args.get("pr_number") {
                result["structuredContent"]["pr_number"] = number.clone();
            }
        }
        result["structuredContent"]["context"] = metadata.clone();
        result["_meta"] = json!({"reqvire/context":metadata});
        result["content"] = json!([{"type":"text","text":serde_json::to_string_pretty(&result["structuredContent"])?}]);
        Ok(result)
    }
    fn contract(&self) -> Value {
        let mut contract = json!({"mcp_protocol_version":reqvire::tool_interface::MCP_PROTOCOL_VERSION,"tool_contract_version":reqvire::tool_interface::TOOL_CONTRACT_VERSION,"mutation_tools_enabled":true,"size_estimates_enabled":self.with_size_estimates});
        contract["tools"] = json!(self.definitions());
        contract["capabilities"]["worktree_contexts"] = json!(true);
        contract["capabilities"]["automatic_commits"] = json!(self.enable_commits);
        contract["github"] = self.github.status();
        contract
    }
    fn definitions(&self) -> &[Value] {
        &self.tool_catalog.definitions
    }
}
fn success(value: Value) -> Value {
    json!({"content":[{"type":"text","text":value.to_string()}],"structuredContent":value})
}
fn selector() -> Value {
    json!({"type":"string","minLength":1,"description":"Opaque worktree context ID; required when multiple contexts exist."})
}
fn definition(
    name: &str,
    description: &str,
    fields: Value,
    required: &[&str],
    read: bool,
    external: bool,
) -> Value {
    json!({"name":name,"description":description,"inputSchema":{"type":"object","properties":fields,"required":required,"additionalProperties":false},
        "outputSchema":{"type":"object","additionalProperties":true},"annotations":{"readOnlyHint":read,"destructiveHint":!read,"idempotentHint":read,"openWorldHint":external}})
}
pub fn local_definitions() -> &'static [Value] {
    static DEFINITIONS: LazyLock<Vec<Value>> = LazyLock::new(build_local_definitions);
    &DEFINITIONS
}

fn build_local_definitions() -> Vec<Value> {
    #[cfg(test)]
    tests::LOCAL_CATALOG_BUILDS.with(|count| count.set(count.get() + 1));
    vec![
        definition(
            "reqvire.worktree.list",
            "List worktrees and admitted contexts without changing ownership.",
            json!({}),
            &[],
            true,
            false,
        ),
        definition(
            "reqvire.worktree.create",
            "Create and admit a new branch from an explicit local commit/ref, without fetching.",
            json!({"branch":{"type":"string","minLength":1},"base_ref":{"type":"string","minLength":1},"from_worktree_id":selector()}),
            &["branch", "base_ref"],
            false,
            false,
        ),
        definition(
            "reqvire.worktree.open",
            "Admit a clean existing local branch in its own worktree.",
            json!({"branch":{"type":"string","minLength":1}}),
            &["branch"],
            false,
            false,
        ),
        definition(
            "reqvire.worktree.remove",
            "Remove only a clean server-created secondary worktree; preserve its branch.",
            json!({"worktree_id":selector()}),
            &["worktree_id"],
            false,
            false,
        ),
        definition(
            "reqvire.git.commit",
            "Commit only accepted pending changes, preserving unrelated staged and disk edits.",
            json!({"worktree_id":selector(),"message":{"type":"string","minLength":1}}),
            &["message"],
            false,
            false,
        ),
        definition(
            "reqvire.git.reconcile",
            "Verify a recorded unknown-outcome local commit; dry_run defaults to true. Apply completes its index and accepted state without another commit or ref update.",
            json!({"worktree_id":selector(),"attempted_commit":{"type":"string","minLength":1},"dry_run":{"type":"boolean","default":true}}),
            &["attempted_commit"],
            false,
            false,
        ),
    ]
}
fn publication_definitions() -> Vec<Value> {
    vec![
        definition(
            "reqvire.git.push",
            "Push the accepted committed HEAD to the startup-pinned same-repository branch.",
            json!({"worktree_id":selector(),"remote":{"type":"string","minLength":1}}),
            &[],
            false,
            true,
        ),
        definition(
            "reqvire.github.pr.create",
            "Create a same-repository PR from the already pushed owned branch to an explicit base.",
            json!({"worktree_id":selector(),"base":{"type":"string","minLength":1},"title":{"type":"string","minLength":1},"body":{"type":"string"},"draft":{"type":"boolean"}}),
            &["base", "title", "body"],
            false,
            true,
        ),
        definition(
            "reqvire.github.pr.comment",
            "Add a new comment to a PR in the pinned repository; no automatic retry.",
            json!({"worktree_id":selector(),"pr_number":{"type":"integer","minimum":1},"body":{"type":"string","minLength":1}}),
            &["pr_number", "body"],
            false,
            true,
        ),
    ]
}
fn validate_arguments(args: &Value, schema: &Value) -> Result<(), ReqvireError> {
    #[cfg(test)]
    tests::WRAPPER_VALIDATIONS.with(|count| count.set(count.get() + 1));
    let args = args
        .as_object()
        .ok_or_else(|| error("Arguments must be an object"))?;
    for required in schema["required"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        if !args.contains_key(required) {
            return Err(error(format!("Missing required argument: {required}")));
        }
    }
    for (key, value) in args {
        let field = &schema["properties"][key];
        if field.is_null() {
            return Err(error(format!("Unknown argument: {key}")));
        }
        let valid = match field["type"].as_str() {
            Some("string") => value.is_string(),
            Some("boolean") => value.is_boolean(),
            Some("integer") => value.is_i64() || value.is_u64(),
            Some("object") => value.is_object(),
            Some("array") => value.is_array(),
            _ => true,
        };
        if !valid {
            return Err(error(format!("Invalid argument type: {key}")));
        }
        if field["minLength"] == 1 && value.as_str().is_some_and(|s| s.trim().is_empty()) {
            return Err(error(format!("{key} must not be empty")));
        }
        if field["minimum"] == 1 && value.as_u64().unwrap_or(0) < 1 {
            return Err(error(format!("{key} must be positive")));
        }
    }
    Ok(())
}
fn resource_selector(uri: &str) -> Result<(String, Option<String>), ReqvireError> {
    let Some((base, query)) = uri.split_once('?') else {
        return Ok((uri.into(), None));
    };
    let id =
        parse_selector(Some(query))?.ok_or_else(|| error("Resource query requires worktree_id"))?;
    Ok((base.into(), Some(id)))
}
pub fn parse_selector(query: Option<&str>) -> Result<Option<String>, ReqvireError> {
    let mut id = None;
    for part in query.unwrap_or("").split('&').filter(|p| !p.is_empty()) {
        let (key, value) = part.split_once('=').unwrap_or((part, ""));
        if key != "worktree_id" {
            continue;
        }
        if id.is_some() {
            return Err(error("Duplicate worktree_id"));
        }
        let value = percent_encoding::percent_decode_str(value)
            .decode_utf8()
            .map_err(|_| error("Invalid worktree_id encoding"))?;
        if value.is_empty() {
            return Err(error("Empty worktree_id"));
        }
        id = Some(value.into_owned());
    }
    Ok(id)
}
pub fn repository_worktrees(root: &Path) -> Result<Vec<Value>, ReqvireError> {
    let data = git(root, &["worktree", "list", "--porcelain", "-z"])?;
    let mut rows = Vec::new();
    let mut row = json!({"managed":false,"original":false,"owned":null,"state":"unregistered"});
    for part in data.split('\0') {
        if part.is_empty() {
            if row.get("workspace_root").is_some() {
                rows.push(row);
                row = json!({"managed":false,"original":false,"owned":null,"state":"unregistered"});
            }
            continue;
        }
        if let Some(path) = part.strip_prefix("worktree ") {
            row["workspace_root"] = json!(path);
        } else if let Some(head) = part.strip_prefix("HEAD ") {
            row["head"] = json!(head);
        } else if let Some(branch) = part.strip_prefix("branch refs/heads/") {
            row["branch"] = json!(branch);
        } else if part == "detached" {
            row["detached"] = json!(true);
        }
    }
    if row.get("workspace_root").is_some() {
        rows.push(row);
    }
    Ok(rows)
}

#[cfg(test)]
pub fn local_catalog_build_count() -> usize {
    tests::LOCAL_CATALOG_BUILDS.with(std::cell::Cell::get)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn runtime_publication_consumes_payload_and_retains_last_valid_snapshot_on_failure() {
        let context = Context {
            id: "context-test".into(),
            root: PathBuf::from("/unused"),
            branch: "main".into(),
            original: true,
            managed: false,
            read_only: false,
            worker: Mutex::new(None),
            alive: Arc::new(AtomicBool::new(true)),
            published: RwLock::new(Published {
                status: json!({}),
                sequence: 0,
                runtime: None,
                runtime_error: None,
                failure: None,
                stopped: false,
            }),
        };
        let mut deep = json!("leaf");
        for _ in 0..140 {
            deep = json!({"child":deep});
        }
        let store = json!({
            "project":{"worktree_id":"context-test", "branch":"main"},
            "elements":[{"id":"Lärche 測定", "content":"</script>&\u{2028}\u{2029}", "number":-0.0}],
            "unknown":deep, "empty":[],
        });
        let response = json!({"runtime":{"project_store":store,"ontologies_ttl":"ontology"},
            "status":{"available":true},"result":{"preserve":"tool result"}});
        // This is the production pipe decoder, including its deep-store policy.
        let mut response = parse_json(&response.to_string()).unwrap();
        let ontology_address = response["runtime"]["ontologies_ttl"]
            .as_str()
            .unwrap()
            .as_ptr();
        context.update(&mut response).unwrap();
        assert!(
            response["runtime"].is_null(),
            "publication must consume the payload, not clone it"
        );
        assert_eq!(response["result"]["preserve"], "tool result");
        let accepted = Arc::clone(context.published.read().unwrap().runtime.as_ref().unwrap());
        assert_eq!(accepted.assets.ontologies_ttl.as_ptr(), ontology_address);
        assert_eq!(
            parse_json(&accepted.assets.project_store_json).unwrap(),
            store
        );
        let seed = accepted
            .assets
            .project_store_js
            .strip_prefix("window.reqvireProjectStore = ")
            .unwrap()
            .strip_suffix(";\n")
            .unwrap();
        assert_eq!(parse_json(seed).unwrap(), store);
        assert!(!seed.contains(['<', '>', '&', '\u{2028}', '\u{2029}']));
        let prior_revision = accepted.live.revision.clone();
        context
            .update(&mut json!({"sequence":42,"status":{"available":true,"model_revision":"new"}}))
            .unwrap();
        context
            .update(
                &mut json!({"sequence":41,"status":{"available":true,"model_revision":"old"},
            "read_status":{"model_revision":"old"}}),
            )
            .unwrap();
        assert_eq!(
            context.metadata()["model_revision"],
            "new",
            "late read rolled back status"
        );
        assert!(
            Arc::ptr_eq(&accepted, &context.runtime().unwrap()),
            "read replaced Explorer runtime"
        );
        for bad in [
            json!({"project_store":[],"ontologies_ttl":"other"}),
            json!({"project_store":store,"ontologies_ttl":null}),
        ] {
            let mut response = json!({"runtime":bad,"status":{"available":true}});
            context.update(&mut response).unwrap();
            let published = context.published.read().unwrap();
            assert!(published.runtime_error.is_some());
            assert!(Arc::ptr_eq(published.runtime.as_ref().unwrap(), &accepted));
            assert_eq!(accepted.live.revision, prior_revision);
            drop(published);
        }
        let mut response =
            json!({"runtime":{"project_store":{"elements":[]},"ontologies_ttl":"new"}});
        context.update(&mut response).unwrap();
        let published = context.published.read().unwrap();
        assert!(published.runtime_error.is_none());
        assert!(!Arc::ptr_eq(published.runtime.as_ref().unwrap(), &accepted));
        assert_eq!(accepted.live.revision, prior_revision);
        assert_ne!(
            published.runtime.as_ref().unwrap().live.revision,
            prior_revision
        );
        drop(published);
    }
    use std::cell::Cell;
    thread_local! {
        pub(super) static CATALOG_BUILDS: Cell<usize> = const { Cell::new(0) };
        pub(super) static LOCAL_CATALOG_BUILDS: Cell<usize> = const { Cell::new(0) };
        pub(super) static WRAPPER_VALIDATIONS: Cell<usize> = const { Cell::new(0) };
    }

    #[test]
    fn dispatch_reuses_session_catalog_and_validates_wrapper_once() {
        for commits in [false, true] {
            let temp = fixture();
            let manager = manager(temp.path(), commits);
            let expected = manager.handle("tools/list", json!({})).unwrap();
            let before = CATALOG_BUILDS.with(Cell::get);
            let validations = WRAPPER_VALIDATIONS.with(Cell::get);
            for _ in 0..3 {
                call(&manager, "reqvire.search", json!({"short":true}));
            }
            assert_eq!(
                CATALOG_BUILDS.with(Cell::get),
                before,
                "ordinary calls must reuse the session catalog"
            );
            assert_eq!(
                WRAPPER_VALIDATIONS.with(Cell::get),
                validations + 3,
                "validate once per wrapper call"
            );
            assert_eq!(manager.handle("tools/list", json!({})).unwrap(), expected);
        }
    }

    #[test]
    fn catalog_github_gating_keeps_shared_schemas_immutable() {
        let local = SessionToolCatalog::new(false);
        let github = SessionToolCatalog::new(true);
        for name in [
            "reqvire.git.push",
            "reqvire.github.pr.create",
            "reqvire.github.pr.comment",
        ] {
            assert!(local.get(name).is_none());
            let (definition, core) = github.get(name).unwrap();
            assert!(!core);
            assert_eq!(definition["annotations"]["openWorldHint"], true);
        }
        assert_eq!(
            &github.definitions[..local.definitions.len()],
            local.definitions
        );
        for enabled in [false, true] {
            for definition in reqvire::tool_interface::tool_definitions(enabled) {
                assert!(definition["inputSchema"]["properties"]
                    .get("worktree_id")
                    .is_none());
            }
        }
    }

    #[test]
    fn catalog_rejection_precedes_worker_dispatch() {
        let temp = fixture();
        let manager = manager(temp.path(), false);
        let context = manager.resolve(None).unwrap();
        let before = fs::read(temp.path().join("Model.md")).unwrap();
        let head = git(temp.path(), &["rev-parse", "HEAD"]).unwrap();
        // Holding the worker gate proves rejected inputs do not reach the worker.
        let gate = context.worker.lock().unwrap();
        let thread_manager = Arc::clone(&manager);
        let (done, completed) = mpsc::channel();
        let pending = std::thread::spawn(move || {
            for (name, args, diagnostic) in [
                (
                    "reqvire.not_a_tool",
                    json!({}),
                    "Tool reqvire.not_a_tool is unavailable for this session",
                ),
                (
                    "reqvire.search",
                    json!({"unknown":true}),
                    "Unknown argument: unknown",
                ),
                (
                    "reqvire.add_element",
                    json!({"file":"Model.md"}),
                    "Missing required argument: content",
                ),
                (
                    "reqvire.search",
                    json!({"short":"yes"}),
                    "Invalid argument type: short",
                ),
                (
                    "reqvire.collect",
                    json!({"element_name":"Root", "direction":"sideways"}),
                    "Argument 'direction' has unsupported value '\"sideways\"'",
                ),
                (
                    "reqvire.semantic.export",
                    json!({"layers":[42]}),
                    "Argument 'layers' must contain only strings",
                ),
                (
                    "reqvire.search",
                    json!({"worktree_id":" "}),
                    "worktree_id must not be empty",
                ),
                (
                    "reqvire.github.pr.comment",
                    json!({"pr_number":1,"body":"text"}),
                    "Tool reqvire.github.pr.comment is unavailable for this session",
                ),
            ] {
                let rejected = thread_manager
                    .handle("tools/call", json!({"name":name,"arguments":args}))
                    .unwrap_err();
                assert_eq!(rejected.code, rmcp::model::ErrorCode::INVALID_PARAMS);
                assert_eq!(
                    rejected.message,
                    "Invalid tool arguments or unavailable tool"
                );
                let data = rejected.data.unwrap();
                assert_eq!(data["tool"], name);
                assert!(
                    data["message"].as_str().unwrap().ends_with(diagnostic),
                    "{data}"
                );
            }
            done.send(()).unwrap();
        });
        let outcome = completed.recv_timeout(Duration::from_secs(3));
        drop(gate);
        outcome.expect("argument rejection must complete while the worker gate is held");
        pending.join().unwrap();
        // Bypass the wrapper to exercise the worker's independent schema gate.
        let mut worker = context.worker.lock().unwrap();
        let response = context.request_locked(&mut worker, &json!({
            "operation":"rpc", "method":"tools/call",
            "params":{"name":"reqvire.add_element", "arguments":{"file":"Model.md", "unexpected":true}}
        })).unwrap();
        drop(worker);
        assert_eq!(response["rpc_error"]["code"], -32602);
        assert_eq!(response["rpc_error"]["message"], "Invalid tool arguments");
        assert_eq!(fs::read(temp.path().join("Model.md")).unwrap(), before);
        assert_eq!(git(temp.path(), &["rev-parse", "HEAD"]).unwrap(), head);
    }

    const MODEL: &str = include_str!("../../../tests/test-cache-integration/fixtures/model.md.txt");
    const OTHER: &str = include_str!("../../../tests/test-cache-integration/fixtures/other.md.txt");
    struct TestRepo {
        _directory: tempfile::TempDir,
        root: PathBuf,
    }
    impl TestRepo {
        fn path(&self) -> &Path {
            &self.root
        }
    }
    fn fixture() -> TestRepo {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("repo");
        fs::create_dir(&path).unwrap();
        let root = path.as_path();
        git(root, &["init", "-qb", "main"]).unwrap();
        git(root, &["config", "user.name", "Worktree Test"]).unwrap();
        git(root, &["config", "user.email", "worktree@example.invalid"]).unwrap();
        fs::write(root.join("Model.md"), MODEL).unwrap();
        git(root, &["add", "."]).unwrap();
        git(root, &["commit", "-qm", "baseline"]).unwrap();
        TestRepo {
            _directory: temp,
            root: path,
        }
    }
    fn executable() -> PathBuf {
        std::env::var_os("REQVIRE_TEST_BIN")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(format!("reqvire{}", std::env::consts::EXE_SUFFIX))
            })
    }
    fn manager(root: &Path, commits: bool) -> Arc<Worktrees> {
        Worktrees::start(root, &executable(), commits, false, true, false, "origin").unwrap()
    }
    fn call(manager: &Worktrees, name: &str, args: Value) -> Value {
        let response = manager
            .handle("tools/call", json!({"name":name,"arguments":args}))
            .unwrap();
        assert_ne!(response["isError"], true, "{response}");
        response["structuredContent"].clone()
    }
    fn rejected(manager: &Worktrees, name: &str, args: Value) -> Value {
        let response = match manager.handle("tools/call", json!({"name":name,"arguments":args})) {
            Ok(response) => response,
            Err(error) => {
                assert_eq!(error.code, rmcp::model::ErrorCode::INVALID_PARAMS);
                json!({"isError":true,"structuredContent":{"error":error,"context":error.data.as_ref().and_then(|data|data.get("context"))}})
            }
        };
        assert_eq!(response["isError"], true, "{response}");
        response
    }
    #[cfg(unix)]
    fn reconcile_real_worker(commits: bool) {
        use std::os::unix::fs::PermissionsExt;
        let temp = fixture();
        let root = temp.path();
        let manager = manager(root, commits);
        let context = manager.resolve(Some(&manager.original)).unwrap();
        let original_head = context.metadata()["head"].clone();
        let content = OTHER.split_once("# Elements\n\n").unwrap().1;
        if !commits {
            call(
                &manager,
                "reqvire.add_element",
                json!({"file":"Model.md","content":content}),
            );
        }
        let accepted = context.metadata();
        let runtime = context.runtime().unwrap();
        let hook = root.join(".git/hooks/reference-transaction");
        fs::write(&hook,"#!/usr/bin/env python3\nimport sys,time\nfrom pathlib import Path\nif sys.argv[1]=='committed':\n    with Path('.git/ref-effects').open('a') as f: f.write('one\\n')\n    time.sleep(31)\n").unwrap();
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
        let start = std::time::Instant::now();
        let result = if commits {
            call(
                &manager,
                "reqvire.add_element",
                json!({"file":"Model.md","content":content}),
            )
        } else {
            call(
                &manager,
                "reqvire.git.commit",
                json!({"message":"timed out attempt"}),
            )
        };
        assert!(start.elapsed() >= Duration::from_secs(29));
        assert!(start.elapsed() < Duration::from_secs(45));
        assert_eq!(result["reconciled"], true, "{result}");
        let status = context.metadata();
        let commit = git(root, &["rev-parse", "HEAD"]).unwrap();
        assert_eq!(status["head"], commit);
        assert_eq!(status["recovery_attempt"], Value::Null);
        assert_eq!(status["recovery_required"], false);
        assert_eq!(status["writes_available"], true);
        assert_eq!(status["pending_changes"], json!([]));
        assert!(context
            .runtime()
            .unwrap()
            .assets
            .project_store_json
            .contains("Other Subject"));
        assert_eq!(Arc::ptr_eq(&runtime, &context.runtime().unwrap()), !commits);
        if !commits {
            assert_eq!(status["model_revision"], accepted["model_revision"]);
        }
        assert!(call(&manager, "reqvire.search", json!({}))
            .to_string()
            .contains("Other Subject"));
        assert_eq!(git(root, &["rev-parse", "HEAD^"]).unwrap(), original_head);
        assert_eq!(git(root, &["status", "--porcelain"]).unwrap(), "");
        assert_eq!(
            fs::read_to_string(root.join(".git/ref-effects")).unwrap(),
            "one\n"
        );
        assert!(!root.join(".git/index.lock").exists());
        rejected(
            &manager,
            "reqvire.git.reconcile",
            json!({"attempted_commit":commit,"dry_run":false}),
        );
        assert_eq!(git(root, &["rev-parse", "HEAD"]).unwrap(), commit);
        assert_eq!(
            call(
                &manager,
                "reqvire.git.commit",
                json!({"message":"already reconciled"})
            )["outcome"],
            "no_op"
        );
    }
    #[cfg(unix)]
    #[test]
    fn automatic_commit_reconciliation_through_real_worker() {
        reconcile_real_worker(true);
    }
    #[cfg(unix)]
    #[test]
    fn explicit_commit_reconciliation_through_real_worker() {
        reconcile_real_worker(false);
    }

    #[test]
    fn read_only_snapshot_worker_accepts_dirty_models_without_ownership() {
        let repo = fixture();
        let root = repo.path();
        let changed = MODEL.replace("Cache Subject", "Uncommitted Subject");
        fs::write(root.join("Model.md"), &changed).unwrap();
        git(root, &["config", "--unset", "user.name"]).unwrap();
        git(root, &["config", "--unset", "user.email"]).unwrap();
        let before = git(root, &["status", "--porcelain=v1"]).unwrap();
        let options: WorkerOptions = serde_json::from_value(json!({
            "worktree_id":"read-test", "enable_commits":false,
            "with_size_estimates":false, "explorer":true, "read_only":true
        }))
        .unwrap();
        let (_worker, ready) = Worker::spawn(&executable(), root, &options).unwrap();
        let store = &ready["runtime"]["project_store"];
        assert!(
            store.is_object(),
            "worker must transfer one structured store"
        );
        assert!(store.to_string().contains("Uncommitted Subject"));
        assert_eq!(store["project"]["worktree_id"], "read-test");
        assert_eq!(store["project"]["branch"], "main");
        assert!(ready["runtime"].get("project_store_json").is_none());
        assert_eq!(fs::read_to_string(root.join("Model.md")).unwrap(), changed);
        assert_eq!(git(root, &["status", "--porcelain=v1"]).unwrap(), before);
        assert_eq!(git(root, &["branch", "--show-current"]).unwrap(), "main");
        assert!(!fs::read_dir(root.join(".git")).unwrap().any(|entry| entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with("reqvire-mcp")));
    }

    #[test]
    fn healthy_context_reuse_does_not_require_repository_administration() {
        let repo = fixture();
        git(repo.path(), &["branch", "unopened"]).unwrap();
        let manager = manager(repo.path(), false);
        let original = manager.resolve(Some(&manager.original)).unwrap();
        let administration =
            lock_file(&manager.common.join("reqvire-mcp-administration.lock")).unwrap();
        let reused = call(&manager, "reqvire.worktree.open", json!({"branch":"main"}));
        assert_eq!(reused["worktree_id"], original.id);
        assert!(Arc::ptr_eq(
            &original,
            &manager.resolve(Some(&original.id)).unwrap()
        ));
        rejected(
            &manager,
            "reqvire.worktree.open",
            json!({"branch":"unopened"}),
        );
        assert_eq!(manager.contexts.read().unwrap().len(), 1);
        assert_eq!(repository_worktrees(repo.path()).unwrap().len(), 1);
        drop(administration);
        call(
            &manager,
            "reqvire.worktree.open",
            json!({"branch":"unopened"}),
        );
        assert_eq!(manager.contexts.read().unwrap().len(), 2);
    }

    #[test]
    fn lock_failures_identify_administration_path_without_changing_contexts() {
        let repo = fixture();
        git(repo.path(), &["branch", "unopened"]).unwrap();
        let manager = manager(repo.path(), false);
        let before = fs::read(repo.path().join("Model.md")).unwrap();
        let head = git(repo.path(), &["rev-parse", "HEAD"]).unwrap();
        let index = fs::read(repo.path().join(".git/index")).unwrap();
        let path = manager.common.join("reqvire-mcp-administration.lock");
        fs::create_dir(&path).unwrap();
        let rejected = rejected(
            &manager,
            "reqvire.worktree.open",
            json!({"branch":"unopened"}),
        );
        let failure = rejected.to_string();
        assert!(
            failure.contains(path.to_str().unwrap())
                && failure.contains("open")
                && failure.contains("os error"),
            "{failure}"
        );
        assert!(!failure.contains("already owned"), "{failure}");
        assert_eq!(manager.contexts.read().unwrap().len(), 1);
        assert_eq!(repository_worktrees(repo.path()).unwrap().len(), 1);
        assert_eq!(fs::read(repo.path().join("Model.md")).unwrap(), before);
        assert_eq!(fs::read(repo.path().join(".git/index")).unwrap(), index);
        assert_eq!(git(repo.path(), &["rev-parse", "HEAD"]).unwrap(), head);
        fs::remove_dir(path).unwrap();
        call(
            &manager,
            "reqvire.worktree.open",
            json!({"branch":"unopened"}),
        );
    }

    #[test]
    fn lock_failure_during_compensation_preserves_both_errors_and_residual_path() {
        let repo = fixture();
        let manager = manager(repo.path(), false);
        let path = manager.common.join(format!(
            "reqvire-mcp-{}.lock",
            reqvire::utils::hash_content("refs/heads/blocked")
        ));
        fs::create_dir(&path).unwrap();
        let failure = rejected(
            &manager,
            "reqvire.worktree.create",
            json!({"branch":"blocked", "base_ref":"main"}),
        )
        .to_string();
        assert!(
            failure.contains("cleanup failed")
                && failure.contains("retained for recovery")
                && failure.contains(path.to_str().unwrap()),
            "{failure}"
        );
        assert!(path.is_dir());
        assert_eq!(manager.contexts.read().unwrap().len(), 1);
        assert_eq!(
            git(repo.path(), &["branch", "--show-current"]).unwrap(),
            "main"
        );
    }

    #[test]
    fn browser_branches_load_on_demand_and_reject_dirty_admission() {
        let repo = fixture();
        let root = repo.path();
        git(root, &["branch", "unopened"]).unwrap();
        let dirty = root.parent().unwrap().join("dirty");
        git(
            root,
            &["worktree", "add", "-qb", "dirty", dirty.to_str().unwrap()],
        )
        .unwrap();
        let manager = manager(root, false);
        let inventory = manager.browser_inventory();
        let rows = inventory["worktrees"].as_array().unwrap();
        assert_eq!(
            rows.len(),
            3,
            "all branches must be listed without loading them"
        );
        assert_eq!(manager.contexts.read().unwrap().len(), 1);
        let id = |branch: &str| {
            rows.iter().find(|r| r["branch"] == branch).unwrap()["worktree_id"]
                .as_str()
                .unwrap()
        };
        assert!(manager
            .published_browser_context(Some(id("unopened")))
            .is_err());
        assert_eq!(
            manager.contexts.read().unwrap().len(),
            1,
            "transfer must not admit branches"
        );
        for state in ["unstaged", "staged", "untracked"] {
            // Invalid model content proves clean admission rejects before model validation.
            let path = dirty.join(if state == "untracked" {
                "Untracked.md"
            } else {
                "Model.md"
            });
            fs::write(
                &path,
                "# Elements\n\n### Invalid\n\n#### Metadata\n  * type: invalid-type\n",
            )
            .unwrap();
            if state == "staged" {
                git(&dirty, &["add", "."]).unwrap();
            }
            let before = git(&dirty, &["status", "--porcelain=v1"]).unwrap();
            let failure = manager
                .browser_context(Some(id("dirty")))
                .err()
                .expect("dirty target must fail");
            assert!(failure.to_string().contains("clean"), "{failure}");
            assert_eq!(git(&dirty, &["status", "--porcelain=v1"]).unwrap(), before);
            assert_eq!(manager.contexts.read().unwrap().len(), 1);
            git(&dirty, &["reset", "--hard", "HEAD"]).unwrap();
            if state == "untracked" {
                fs::remove_file(path).unwrap();
            }
        }
        let context = manager.browser_context(Some(id("dirty"))).unwrap();
        assert_eq!(context.id, id("dirty"));
        let result = call(
            &manager,
            "reqvire.add_element",
            json!({
                "worktree_id":context.id,"file":"Model.md", "content":
                "### Accepted Child\n\nAccepted change.\n\n#### Metadata\n  * type: requirement\n\n#### Relations\n  * specify: Cache Subject\n"
            }),
        );
        assert!(result.is_object());
        let runtime = context.runtime().unwrap();
        assert!(!git(&dirty, &["status", "--porcelain=v1"])
            .unwrap()
            .is_empty());
        fs::write(dirty.join("Model.md"), "external invalid change").unwrap();
        assert!(Arc::ptr_eq(
            &runtime,
            &manager
                .browser_context(Some(&context.id))
                .unwrap()
                .runtime()
                .unwrap()
        ));
        let mut pending = Vec::new();
        for _ in 0..3 {
            let manager = Arc::clone(&manager);
            let id = id("unopened").to_string();
            pending.push(std::thread::spawn(move || {
                manager.browser_context(Some(&id)).unwrap()
            }));
        }
        let contexts: Vec<_> = pending.into_iter().map(|t| t.join().unwrap()).collect();
        assert!(contexts.iter().all(|c| Arc::ptr_eq(c, &contexts[0])));
        assert_eq!(
            git(&contexts[0].root, &["branch", "--show-current"]).unwrap(),
            "unopened"
        );
        let path = contexts[0].root.clone();
        drop(contexts);
        drop(context);
        drop(manager);
        assert!(path.exists(), "managed worktrees persist on shutdown");
    }

    #[test]
    fn read_only_branch_loads_reuse_cache_refresh_inputs_and_recover() {
        let repo = fixture();
        let root = repo.path();
        let invalid = root.parent().unwrap().join("invalid");
        git(
            root,
            &[
                "worktree",
                "add",
                "-qb",
                "invalid",
                invalid.to_str().unwrap(),
            ],
        )
        .unwrap();
        fs::write(
            invalid.join("Model.md"),
            MODEL.replace("capability", "invalid-type"),
        )
        .unwrap();
        git(root, &["branch", "selected"]).unwrap();
        let manager = Worktrees::start_read_only(root, &executable()).unwrap();
        let inventory = manager.browser_inventory();
        let rows = inventory["worktrees"].as_array().unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(
            manager.contexts.read().unwrap().len(),
            1,
            "unselected worktrees must not start workers"
        );
        assert!(rows.iter().all(|r| r["owned"] == false));
        assert_eq!(
            rows.iter().find(|r| r["branch"] == "invalid").unwrap()["state"],
            "unloaded"
        );
        let id = rows.iter().find(|r| r["branch"] == "selected").unwrap()["worktree_id"]
            .as_str()
            .unwrap();
        // Discovery must not pin the branch's old commit.
        fs::write(
            root.join("Model.md"),
            MODEL.replace("Cache Subject", "Other Subject"),
        )
        .unwrap();
        git(root, &["add", "."]).unwrap();
        git(root, &["commit", "-qm", "new tip"]).unwrap();
        git(root, &["branch", "-f", "selected", "main"]).unwrap();
        let context = manager.load_browser_context(Some(id)).unwrap();
        assert_eq!(manager.contexts.read().unwrap().len(), 2);
        let initial = context.runtime().unwrap();
        assert!(initial.assets.project_store_json.contains("Other Subject"));
        context.load_for_browser().unwrap();
        assert!(
            Arc::ptr_eq(&initial, &context.runtime().unwrap()),
            "unchanged loads reuse runtime and core model"
        );
        let path = context.root.join("Model.md");
        let modified = fs::metadata(&path).unwrap().modified().unwrap();
        fs::write(&path, MODEL.replace("Cache Subject", "Fresh Subject")).unwrap();
        fs::File::open(&path)
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(modified))
            .unwrap();
        context.load_for_browser().unwrap();
        let changed = context.runtime().unwrap();
        assert!(!Arc::ptr_eq(&initial, &changed));
        assert!(changed.assets.project_store_json.contains("Fresh Subject"));
        // A new excluded source appears, then changes to exclusions make it visible.
        fs::write(context.root.join(".reqvireignore"), "Extra.md\n").unwrap();
        fs::write(
            context.root.join("Extra.md"),
            MODEL.replace("Cache Subject", "Hidden Subject"),
        )
        .unwrap();
        context.load_for_browser().unwrap();
        assert!(!context
            .runtime()
            .unwrap()
            .assets
            .project_store_json
            .contains("Hidden Subject"));
        fs::write(context.root.join(".reqvireignore"), "").unwrap();
        context.load_for_browser().unwrap();
        assert!(context
            .runtime()
            .unwrap()
            .assets
            .project_store_json
            .contains("Hidden Subject"));
        fs::write(&path, MODEL.replace("capability", "invalid-type")).unwrap();
        assert!(context.load_for_browser().is_err());
        assert!(
            context.runtime().is_err(),
            "invalid inputs cannot be reported as fresh success"
        );
        fs::write(&path, MODEL.replace("Cache Subject", "Fresh Subject")).unwrap();
        manager.load_browser_context(Some(id)).unwrap();
        assert!(context
            .runtime()
            .unwrap()
            .assets
            .project_store_json
            .contains("Fresh Subject"));
        let evidence = "### Delivered\n\nThe system SHALL expose evidence.\n\n#### Metadata\n  * type: requirement\n\n#### Relations\n  * specify: Fresh Subject\n  * satisfiedBy: evidence.txt\n";
        fs::write(
            &path,
            format!(
                "{}\n{evidence}",
                MODEL.replace("Cache Subject", "Fresh Subject")
            ),
        )
        .unwrap();
        fs::write(context.root.join("evidence.txt"), "Original evidence").unwrap();
        context.load_for_browser().unwrap();
        let evidence_runtime = context.runtime().unwrap();
        assert!(evidence_runtime
            .assets
            .project_store_json
            .contains("Original evidence"));
        fs::write(context.root.join("evidence.txt"), "Updated evidence").unwrap();
        context.load_for_browser().unwrap();
        let fresh_evidence = context.runtime().unwrap();
        assert!(!Arc::ptr_eq(&evidence_runtime, &fresh_evidence));
        assert!(fresh_evidence
            .assets
            .project_store_json
            .contains("Updated evidence"));
        context.load_for_browser().unwrap();
        assert!(Arc::ptr_eq(&fresh_evidence, &context.runtime().unwrap()));
        assert_eq!(manager.contexts.read().unwrap().len(), 2);
        assert!(!root.join(".git").read_dir().unwrap().any(|e| e
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with("reqvire-mcp-worktree")));
        let prepared = context.root.clone();
        drop(context);
        drop(manager);
        assert!(prepared.exists());
    }

    #[test]
    fn independent_workers_commit_only_accepted_changes_and_preserve_branches() {
        let temp = fixture();
        let manager = manager(temp.path(), false);
        let original = &manager.original;
        let initial = git(temp.path(), &["rev-parse", "HEAD"]).unwrap();
        let child = call(
            &manager,
            "reqvire.worktree.create",
            json!({"branch":"feature-a","base_ref":"main"}),
        );
        let id = child["worktree_id"].as_str().unwrap();
        let root = PathBuf::from(child["workspace_root"].as_str().unwrap());
        assert_eq!(child["base_commit"], initial);
        assert_ne!(id, original);
        assert_eq!(
            call(
                &manager,
                "reqvire.worktree.open",
                json!({"branch":"feature-a"})
            )["worktree_id"],
            id
        );
        rejected(&manager, "reqvire.search", json!({}));
        rejected(&manager, "reqvire.search", json!({"worktree_id":"missing"}));
        call(
            &manager,
            "reqvire.add_element",
            json!({"worktree_id":id,"file":"Model.md","content":OTHER.split_once("# Elements\n\n").unwrap().1}),
        );
        let a = call(&manager, "reqvire.search", json!({"worktree_id":original}));
        let b = call(&manager, "reqvire.search", json!({"worktree_id":id}));
        assert!(!a.to_string().contains("Other Subject"));
        assert!(b.to_string().contains("Other Subject"));
        assert_ne!(
            a["context"]["model_revision"],
            b["context"]["model_revision"]
        );
        assert!(!manager
            .browser_context(Some(original))
            .unwrap()
            .runtime()
            .unwrap()
            .assets
            .project_store_json
            .contains("Other Subject"));
        assert!(manager
            .browser_context(Some(id))
            .unwrap()
            .runtime()
            .unwrap()
            .assets
            .project_store_json
            .contains("Other Subject"));
        rejected(
            &manager,
            "reqvire.worktree.remove",
            json!({"worktree_id":id}),
        );
        fs::write(root.join("unrelated.txt"), "external staged work").unwrap();
        git(&root, &["add", "unrelated.txt"]).unwrap();
        // Accepted snapshot must survive an intervening disk edit during manual commit.
        fs::write(root.join("Model.md"), "external invalid model").unwrap();
        let commit = call(
            &manager,
            "reqvire.git.commit",
            json!({"worktree_id":id,"message":"Accepted\n\nExact multiline message."}),
        );
        assert_eq!(
            git(
                &root,
                &["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"]
            )
            .unwrap(),
            "Model.md"
        );
        assert_eq!(
            git(&root, &["diff", "--cached", "--name-only"]).unwrap(),
            "unrelated.txt"
        );
        assert_eq!(
            fs::read_to_string(root.join("Model.md")).unwrap(),
            "external invalid model"
        );
        assert_eq!(
            git(&root, &["show", "HEAD:Model.md"]).unwrap(),
            include_str!(
                "../../../tests/test-cache-integration/expected/runtime-after-write.md.txt"
            )
            .trim()
        );
        assert_eq!(git(temp.path(), &["rev-parse", "HEAD"]).unwrap(), initial);
        assert_eq!(
            call(
                &manager,
                "reqvire.git.commit",
                json!({"worktree_id":id,"message":"No changes"})
            )["outcome"],
            "no_op"
        );
        rejected(
            &manager,
            "reqvire.worktree.remove",
            json!({"worktree_id":original}),
        );
        git(&root, &["reset", "--hard", "HEAD"]).unwrap();
        let _ = fs::remove_file(root.join("unrelated.txt"));
        call(
            &manager,
            "reqvire.worktree.remove",
            json!({"worktree_id":id}),
        );
        assert!(!root.exists());
        assert_eq!(
            git(temp.path(), &["rev-parse", "feature-a"]).unwrap(),
            commit["commit"]
        );
        rejected(&manager, "reqvire.search", json!({"worktree_id":id}));
    }
    #[cfg(unix)]
    #[test]
    fn recovery_worker_retains_accepted_context() {
        use std::os::unix::fs::PermissionsExt;
        let temp = fixture();
        let manager = manager(temp.path(), true);
        let child = call(
            &manager,
            "reqvire.worktree.create",
            json!({"branch":"recovery","base_ref":"main"}),
        );
        let id = child["worktree_id"].as_str().unwrap();
        let root = PathBuf::from(child["workspace_root"].as_str().unwrap());
        let context = manager.resolve(Some(id)).unwrap();
        let accepted = context.runtime().unwrap();
        let revision = context.metadata()["model_revision"].clone();
        let head = context.metadata()["head"].clone();
        let hook = temp.path().join(".git/hooks/reference-transaction");
        fs::write(
            &hook,
            r#"#!/bin/sh
if [ "$1" = prepared ]; then rm -f Model.md; mkdir Model.md; exit 1; fi
"#,
        )
        .unwrap();
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
        let failed = rejected(
            &manager,
            "reqvire.add_element",
            json!({"worktree_id":id,"file":"Model.md","content":OTHER.split_once("# Elements\n\n").unwrap().1}),
        );
        fs::remove_file(hook).unwrap();
        assert!(failed.to_string().contains("recovery failed"), "{failed}");
        assert!(
            root.join("Model.md").is_dir(),
            "force an incomplete filesystem restoration"
        );
        let status = call(
            &manager,
            "reqvire.workspace_status",
            json!({"worktree_id":id}),
        );
        assert_eq!(status["context"]["state"], "recovery_required");
        assert_eq!(status["context"]["writes_available"], false);
        assert_eq!(status["context"]["model_revision"], revision);
        assert_eq!(status["context"]["head"], head);
        let read = call(&manager, "reqvire.search", json!({"worktree_id":id}));
        assert!(
            read.to_string().contains("Cache Subject")
                && !read.to_string().contains("Other Subject")
        );
        assert!(Arc::ptr_eq(&accepted, &context.runtime().unwrap()));
        let reopened = call(
            &manager,
            "reqvire.worktree.open",
            json!({"branch":"recovery"}),
        );
        assert_eq!(reopened["worktree_id"], id);
        assert_eq!(context.metadata()["recovery_required"], true);
        let resource = manager
            .handle(
                "resources/read",
                json!({"uri":format!("reqvire://workspace/status?worktree_id={id}")}),
            )
            .unwrap();
        assert_eq!(
            resource["_meta"]["reqvire/context"]["recovery_required"],
            true
        );
        rejected(
            &manager,
            "reqvire.git.commit",
            json!({"worktree_id":id,"message":"blocked"}),
        );
        rejected(
            &manager,
            "reqvire.worktree.remove",
            json!({"worktree_id":id}),
        );
        for name in [
            "reqvire.git.push",
            "reqvire.github.pr.create",
            "reqvire.github.pr.comment",
        ] {
            // Exercise the publication gate even when gh discovery hides these tools.
            let result = manager.call_tool(name, json!({"worktree_id":id})).unwrap();
            assert_eq!(result["isError"], true);
            assert!(result.to_string().contains("requires recovery"), "{result}");
        }
        call(
            &manager,
            "reqvire.add_element",
            json!({"worktree_id":manager.original,"file":"Model.md","content":OTHER.split_once("# Elements\n\n").unwrap().1}),
        );
        assert!(!call(&manager, "reqvire.search", json!({"worktree_id":id}))
            .to_string()
            .contains("Other Subject"));
        {
            let mut gate = context.worker.lock().unwrap();
            let worker = gate.as_mut().unwrap();
            worker.child.lock().unwrap().kill().unwrap();
            worker.child.lock().unwrap().wait().unwrap();
            drop(gate);
        }
        rejected(&manager, "reqvire.search", json!({"worktree_id":id}));
        assert_eq!(context.metadata()["available"], false);
        assert_eq!(context.metadata()["writes_available"], false);
        assert!(context.runtime().is_err());
        rejected(
            &manager,
            "reqvire.worktree.open",
            json!({"branch":"recovery"}),
        );
    }

    #[cfg(unix)]
    fn worker_permissions(commits: bool, track_file_mode: bool) {
        use std::os::unix::fs::PermissionsExt;
        let temp = fixture();
        let root = temp.path();
        let model_path = root.join("Model.md");
        let evidence_path = root.join("evidence.txt");
        fs::write(
            root.join("Evidence.md"),
            include_str!("../../../tests/test-cache-integration/fixtures/evidence.md.txt"),
        )
        .unwrap();
        fs::write(&evidence_path, "#!/bin/sh\nexit 0\n").unwrap();
        fs::set_permissions(&evidence_path, fs::Permissions::from_mode(0o750)).unwrap();
        git(root, &["config", "core.filemode", "true"]).unwrap();
        git(root, &["add", "."]).unwrap();
        git(root, &["commit", "-qm", "executable evidence"]).unwrap();
        let model_mode = if track_file_mode { 0o640 } else { 0o777 };
        if !track_file_mode {
            git(root, &["config", "core.filemode", "false"]).unwrap();
            fs::set_permissions(&evidence_path, fs::Permissions::from_mode(0o644)).unwrap();
        }
        fs::set_permissions(&model_path, fs::Permissions::from_mode(model_mode)).unwrap();
        let head = git(root, &["rev-parse", "HEAD"]).unwrap();
        let index = git(root, &["ls-files", "--stage"]).unwrap();
        let manager = manager(root, commits);
        call(
            &manager,
            "reqvire.add_element",
            json!({
                "file":"Model.md", "content":OTHER.split_once("# Elements\n\n").unwrap().1
            }),
        );
        assert_eq!(
            fs::metadata(&model_path).unwrap().permissions().mode() & 0o7777,
            model_mode
        );
        call(
            &manager,
            "reqvire.move_asset",
            json!({"old_path":"evidence.txt", "new_path":"moved.sh"}),
        );
        call(
            &manager,
            "reqvire.move_file",
            json!({"source_file":"Model.md", "target_file":"Renamed.md"}),
        );
        assert!(!model_path.exists());
        assert!(!evidence_path.exists());
        assert_eq!(
            fs::read_to_string(root.join("moved.sh")).unwrap(),
            "#!/bin/sh\nexit 0\n"
        );
        if track_file_mode {
            assert_ne!(
                fs::metadata(root.join("moved.sh"))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o111,
                0
            );
            assert_eq!(
                fs::metadata(root.join("Renamed.md"))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o111,
                0
            );
        }
        if !commits {
            assert_eq!(git(root, &["rev-parse", "HEAD"]).unwrap(), head);
            assert_eq!(git(root, &["ls-files", "--stage"]).unwrap(), index);
            call(
                &manager,
                "reqvire.git.commit",
                json!({"message":"Accepted permissions and moves"}),
            );
        }
        assert!(git(root, &["ls-tree", "HEAD", "--", "Renamed.md"])
            .unwrap()
            .starts_with("100644"));
        assert!(git(root, &["ls-tree", "HEAD", "--", "moved.sh"])
            .unwrap()
            .starts_with("100755"));
        assert_eq!(git(root, &["status", "--porcelain"]).unwrap(), "");
        let search = call(&manager, "reqvire.search", json!({}));
        assert!(
            search.to_string().contains("Renamed.md")
                && search.to_string().contains("Other Subject")
        );
        assert!(manager
            .browser_context(Some(&manager.original))
            .unwrap()
            .runtime()
            .unwrap()
            .assets
            .project_store_json
            .contains("moved.sh"));
    }

    #[cfg(unix)]
    #[test]
    fn worker_permissions_without_commits() {
        worker_permissions(false, true);
    }
    #[cfg(unix)]
    #[test]
    fn worker_permissions_with_commits() {
        worker_permissions(true, true);
    }
    #[cfg(unix)]
    #[test]
    fn worker_permissions_ignored_filemode_without_commits() {
        worker_permissions(false, false);
    }
    #[cfg(unix)]
    #[test]
    fn worker_permissions_ignored_filemode_with_commits() {
        worker_permissions(true, false);
    }

    #[test]
    fn resources_prompts_and_concurrent_model_reads_keep_context() {
        let temp = fixture();
        let manager = manager(temp.path(), true);
        let child = call(
            &manager,
            "reqvire.worktree.create",
            json!({"branch":"secondary","base_ref":"main"}),
        );
        let id = child["worktree_id"].as_str().unwrap();
        let contract = call(&manager, "reqvire.tool_contract", json!({}));
        assert_eq!(contract["capabilities"]["worktree_contexts"], true);
        assert!(contract["tools"]
            .as_array()
            .unwrap()
            .iter()
            .any(|tool| tool["name"] == "reqvire.worktree.create"));
        assert!(
            contract.get("context").is_none(),
            "server-scoped discovery acquired a context"
        );
        let resources = manager.handle("resources/list", json!({})).unwrap();
        assert!(resources.to_string().contains(&format!("worktree_id={id}")));
        assert!(manager
            .handle(
                "resources/read",
                json!({"uri":"reqvire://workspace/status"})
            )
            .is_err());
        let status = manager
            .handle(
                "resources/read",
                json!({"uri":format!("reqvire://workspace/status?worktree_id={id}")}),
            )
            .unwrap();
        assert_eq!(status["_meta"]["reqvire/context"]["worktree_id"], id);
        let prompts = manager.handle("prompts/list", json!({})).unwrap();
        assert!(prompts.to_string().contains("worktree_id"));
        let prompt = manager
            .handle(
                "prompts/get",
                json!({"name":"reqvire.workflow.verify_coverage","arguments":{"worktree_id":id}}),
            )
            .unwrap();
        assert!(prompt["messages"][0].to_string().contains(id));
        std::thread::scope(|scope| {
            let a=scope.spawn(||call(&manager,"reqvire.add_element",json!({"worktree_id":id,"file":"Model.md","content":OTHER.split_once("# Elements\n\n").unwrap().1})));
            let b = scope.spawn(|| {
                call(
                    &manager,
                    "reqvire.semantic.sparql",
                    json!({"worktree_id":manager.original,"query":"ASK { ?s ?p ?o }"}),
                )
            });
            assert!(a.join().unwrap().get("commit").is_some());
            assert!(b.join().unwrap().get("boolean").is_some());
        });
        assert_eq!(
            call(
                &manager,
                "reqvire.git.commit",
                json!({"worktree_id":id,"message":"Already committed"})
            )["outcome"],
            "no_op"
        );
        let definitions = manager.handle("tools/list", json!({})).unwrap();
        assert!(!definitions.to_string().contains("reqvire.git.push"));
        rejected(&manager, "reqvire.git.push", json!({"worktree_id":id}));
    }
    #[test]
    fn admission_recovery_dirty_sources_and_external_worktrees() {
        let temp = fixture();
        let manager = manager(temp.path(), false);
        rejected(
            &manager,
            "reqvire.worktree.create",
            json!({"branch":"main","base_ref":"main"}),
        );
        rejected(
            &manager,
            "reqvire.worktree.create",
            json!({"branch":"bad","base_ref":"missing"}),
        );
        fs::write(temp.path().join("untracked.txt"), "external").unwrap();
        rejected(
            &manager,
            "reqvire.worktree.create",
            json!({"branch":"dirty","base_ref":"main"}),
        );
        fs::remove_file(temp.path().join("untracked.txt")).unwrap();
        let external = tempfile::tempdir().unwrap();
        let path = external.path().join("external");
        git(
            temp.path(),
            &["worktree", "add", "-b", "external", path.to_str().unwrap()],
        )
        .unwrap();
        let opened = call(
            &manager,
            "reqvire.worktree.open",
            json!({"branch":"external"}),
        );
        let id = opened["worktree_id"].as_str().unwrap();
        assert_eq!(opened["managed"], false);
        rejected(
            &manager,
            "reqvire.worktree.remove",
            json!({"worktree_id":id}),
        );
        rejected(
            &manager,
            "reqvire.worktree.create",
            json!({"branch":"ambiguous","base_ref":"HEAD"}),
        );
        assert!(Worktrees::start(
            temp.path(),
            &executable(),
            false,
            false,
            false,
            false,
            "origin"
        )
        .is_err());
        // Ownership is released at stop; no worktree is deleted and IDs are session-local.
        let original = manager.original.clone();
        drop(manager);
        let restarted = Worktrees::start(
            temp.path(),
            &executable(),
            false,
            false,
            true,
            false,
            "origin",
        )
        .unwrap();
        assert_ne!(original, restarted.original);
        assert!(path.exists());
    }
    #[test]
    fn invalid_base_cleanup_worker_crash_and_context_gate_recovery() {
        let temp = fixture();
        // Build a committed invalid base without changing the valid startup branch.
        git(temp.path(), &["checkout", "-qb", "invalid-model"]).unwrap();
        fs::write(
            temp.path().join("Model.md"),
            "# Elements\n\n### Duplicate\n\n### Duplicate\n",
        )
        .unwrap();
        git(temp.path(), &["add", "."]).unwrap();
        git(temp.path(), &["commit", "-qm", "invalid fixture"]).unwrap();
        git(temp.path(), &["checkout", "-q", "main"]).unwrap();
        let manager = manager(temp.path(), false);
        rejected(
            &manager,
            "reqvire.worktree.create",
            json!({"branch":"rejected-model","base_ref":"invalid-model"}),
        );
        assert!(git(
            temp.path(),
            &["rev-parse", "--verify", "refs/heads/rejected-model"]
        )
        .is_err());
        assert_eq!(repository_worktrees(temp.path()).unwrap().len(), 1);
        let opened = call(
            &manager,
            "reqvire.worktree.create",
            json!({"branch":"recoverable","base_ref":"main"}),
        );
        let id = opened["worktree_id"].as_str().unwrap();
        let context = manager.resolve(Some(id)).unwrap();
        // Holding one context cannot block the original context's read or mutation.
        {
            let _gate = context.worker.lock().unwrap();
            call(
                &manager,
                "reqvire.add_element",
                json!({"worktree_id":manager.original,"file":"Model.md","content":OTHER.split_once("# Elements\n\n").unwrap().1}),
            );
            call(
                &manager,
                "reqvire.search",
                json!({"worktree_id":manager.original}),
            );
        }
        {
            let mut gate = context.worker.lock().unwrap();
            let worker = gate.as_mut().unwrap();
            worker.child.lock().unwrap().kill().unwrap();
            worker.child.lock().unwrap().wait().unwrap();
            drop(gate);
        }
        let failed = rejected(&manager, "reqvire.search", json!({"worktree_id":id}));
        assert_eq!(failed["structuredContent"]["context"]["worktree_id"], id);
        assert!(context.runtime().is_err());
        let resource_error = manager
            .handle(
                "resources/read",
                json!({"uri":format!("reqvire://workspace/status?worktree_id={id}")}),
            )
            .unwrap_err();
        assert_eq!(resource_error.data.unwrap()["context"]["worktree_id"], id);
        call(
            &manager,
            "reqvire.search",
            json!({"worktree_id":manager.original}),
        );
        let reopened = call(
            &manager,
            "reqvire.worktree.open",
            json!({"branch":"recoverable"}),
        );
        assert_ne!(reopened["worktree_id"], id);
        assert_eq!(reopened["managed"], true);
        call(
            &manager,
            "reqvire.worktree.remove",
            json!({"worktree_id":reopened["worktree_id"]}),
        );
        assert!(git(temp.path(), &["rev-parse", "--verify", "recoverable"]).is_ok());
        call(
            &manager,
            "reqvire.git.commit",
            json!({"worktree_id":manager.original,"message":"Keep accepted changes"}),
        );
        git(
            temp.path(),
            &["commit", "--allow-empty", "-qm", "external HEAD move"],
        )
        .unwrap();
        let rejected_commit = rejected(
            &manager,
            "reqvire.git.commit",
            json!({"worktree_id":manager.original,"message":"Must reject moved HEAD"}),
        );
        assert_eq!(
            rejected_commit["structuredContent"]["context"]["available"],
            false
        );
        for (method, params) in [
            (
                "resources/read",
                json!({"uri":format!("reqvire://workspace/status?worktree_id={}", manager.original)}),
            ),
            (
                "prompts/get",
                json!({"name":"reqvire.workflow.verify_coverage","arguments":{"worktree_id":manager.original}}),
            ),
        ] {
            let failure = manager.handle(method, params).unwrap_err();
            let metadata = &failure.data.as_ref().unwrap()["context"];
            assert_eq!(metadata["worktree_id"], manager.original);
            assert_eq!(metadata["available"], false);
            assert!(failure.message.contains("HEAD changed"), "{failure}");
        }
    }
    #[test]
    fn http_catalog_includes_context_administration_but_not_unavailable_publication() {
        let temp = fixture();
        let manager = manager(temp.path(), false);
        let definitions = manager.handle("tools/list", json!({})).unwrap();
        let names = definitions["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|tool| tool["name"].as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            include_str!("../../../tests/test-mcp-server/expected/http-mutation-tools.txt")
                .lines()
                .collect::<Vec<_>>()
        );
        let mut read = reqvire::tool_interface::tool_definitions(false);
        read.push(local_definitions()[0].clone());
        assert_eq!(
            read.iter()
                .map(|tool| tool["name"].as_str().unwrap())
                .collect::<Vec<_>>(),
            include_str!("../../../tests/test-mcp-server/expected/http-read-tools.txt")
                .lines()
                .collect::<Vec<_>>()
        );
        for name in [
            "reqvire.search",
            "reqvire.semantic.sparql",
            "reqvire.add_element",
        ] {
            let definition = definitions["tools"]
                .as_array()
                .unwrap()
                .iter()
                .find(|tool| tool["name"] == name)
                .unwrap();
            assert_eq!(
                definition["inputSchema"]["properties"]["worktree_id"]["type"],
                "string"
            );
        }
        for (name, args) in [
            (
                "reqvire.worktree.create",
                json!({"branch":"-bad","base_ref":"main"}),
            ),
            ("reqvire.git.commit", json!({"message":"  "})),
            (
                "reqvire.worktree.open",
                json!({"branch":"main","path":"/tmp/escape"}),
            ),
            ("reqvire.search", json!({"worktree_id":false})),
        ] {
            rejected(&manager, name, args);
        }
    }
}
