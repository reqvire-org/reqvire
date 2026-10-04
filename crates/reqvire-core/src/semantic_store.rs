use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::rdf_store::{load_default_graph, load_named_graph};
use crate::semantic_contract::SemanticIndex;
use oxigraph::store::Store;
use std::fmt;
use std::sync::{Arc, OnceLock};

/// Stable named graph IRIs used inside Oxigraph semantic stores.
///
/// Public query stores mirror visible role graphs into the default graph for
/// backwards-compatible SPARQL. The raw external source graph is loaded only in
/// the derivation store so public outputs can expose the future used subset
/// without allowing raw dependency dumps.
pub const GRAPH_AUTHORED_ONTOLOGY: &str = "urn:reqvire:semantic-graph:authored-ontology";
pub const GRAPH_AUTHORED_MODEL: &str = "urn:reqvire:semantic-graph:authored-model";
pub const GRAPH_GENERATED: &str = "urn:reqvire:semantic-graph:generated";
const GRAPH_RAW_EXTERNAL_SOURCE: &str = "urn:reqvire:semantic-graph:raw-external-source";
pub const GRAPH_EXTERNAL_USED_SUBSET: &str = "urn:reqvire:semantic-graph:external-used-subset";

#[derive(Clone)]
pub struct SemanticModelStore {
    pub index: Arc<SemanticIndex>,
    captured: Arc<CapturedQueryStores>,
}

/// Strings and initialization cells have one owner per completed snapshot.
/// Deferred work never receives a mutable registry or a filesystem path.
struct CapturedQueryStores {
    parts: SemanticStoreTurtleParts,
    authored: OnceLock<Result<Store, ReqvireError>>,
    authored_external: OnceLock<Result<Store, ReqvireError>>,
    full: OnceLock<Result<Store, ReqvireError>>,
    full_external: OnceLock<Result<Store, ReqvireError>>,
    derivation: OnceLock<Result<Store, ReqvireError>>,
    #[cfg(test)]
    before_init: std::sync::Mutex<Option<Arc<dyn Fn() + Send + Sync>>>,
}

impl CapturedQueryStores {
    fn new(parts: SemanticStoreTurtleParts) -> Self {
        Self {
            parts,
            authored: OnceLock::new(),
            authored_external: OnceLock::new(),
            full: OnceLock::new(),
            full_external: OnceLock::new(),
            derivation: OnceLock::new(),
            #[cfg(test)]
            before_init: std::sync::Mutex::new(None),
        }
    }

    fn initialize<'a>(
        &'a self,
        cell: &'a OnceLock<Result<Store, ReqvireError>>,
        build: impl FnOnce(&SemanticStoreTurtleParts) -> Result<Store, ReqvireError>,
    ) -> Result<&'a Store, ReqvireError> {
        cell.get_or_init(|| {
            #[cfg(test)]
            {
                let hook = self
                    .before_init
                    .lock()
                    .expect("store test hook lock")
                    .clone();
                if let Some(hook) = hook {
                    hook();
                }
            }
            // The builder owns a private store until every graph has loaded.
            build(&self.parts)
        })
        .as_ref()
        .map_err(Clone::clone)
    }
}

impl fmt::Debug for SemanticModelStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SemanticModelStore")
            .field("summary", &self.index.summary)
            .finish_non_exhaustive()
    }
}

impl SemanticModelStore {
    #[cfg(test)]
    pub(crate) fn initialized_store_count(&self) -> usize {
        [
            &self.captured.authored,
            &self.captured.authored_external,
            &self.captured.full,
            &self.captured.full_external,
            &self.captured.derivation,
        ]
        .iter()
        .filter(|cell| cell.get().is_some())
        .count()
    }

    #[cfg(test)]
    pub(crate) fn set_store_init_hook(&self, hook: impl Fn() + Send + Sync + 'static) {
        *self
            .captured
            .before_init
            .lock()
            .expect("store test hook lock") = Some(Arc::new(hook));
    }

    /// Capture RDF from the same index used to validate this resolved graph.
    pub fn from_index(
        registry: &GraphRegistry,
        index: Arc<SemanticIndex>,
    ) -> Result<Self, ReqvireError> {
        let turtle_parts = SemanticStoreTurtleParts::from_index(registry, &index)?;

        Ok(Self {
            captured: Arc::new(CapturedQueryStores::new(turtle_parts)),
            index,
        })
    }

    /// Prepare one read-only query variant from this snapshot's captured RDF.
    /// Clones share the result, including a failed initialization.
    pub fn store(&self, full: bool, include_external: bool) -> Result<&Store, ReqvireError> {
        let cell = match (full, include_external) {
            (false, false) => &self.captured.authored,
            (false, true) => &self.captured.authored_external,
            (true, false) => &self.captured.full,
            (true, true) => &self.captured.full_external,
        };
        self.captured.initialize(cell, |parts| {
            build_public_store(parts, full, include_external)
        })
    }

    #[allow(dead_code)]
    pub(crate) fn derivation_store(&self) -> Result<&Store, ReqvireError> {
        self.captured
            .initialize(&self.captured.derivation, build_derivation_store)
    }
}

struct SemanticStoreTurtleParts {
    authored_ontology: String,
    authored_model: String,
    generated: String,
    raw_external_source: String,
    external_used_subset: String,
}

impl SemanticStoreTurtleParts {
    fn from_index(registry: &GraphRegistry, index: &SemanticIndex) -> Result<Self, ReqvireError> {
        Ok(Self {
            authored_ontology: index.to_authored_ontology_turtle_string()?,
            authored_model: index.to_authored_model_layer_turtle_string(registry)?,
            generated: index.to_generated_layer_turtle_string(registry)?,
            raw_external_source: index.to_raw_external_turtle_string()?,
            external_used_subset: index.to_used_external_subset_turtle_string()?,
        })
    }
}

fn build_public_store(
    parts: &SemanticStoreTurtleParts,
    full: bool,
    include_external_subset: bool,
) -> Result<Store, ReqvireError> {
    let store = new_store()?;
    load_public_role(
        &store,
        &parts.authored_ontology,
        GRAPH_AUTHORED_ONTOLOGY,
        "authored ontology graph",
    )?;

    if full {
        load_public_role(
            &store,
            &parts.authored_model,
            GRAPH_AUTHORED_MODEL,
            "authored model graph",
        )?;
        load_public_role(&store, &parts.generated, GRAPH_GENERATED, "generated graph")?;
    }

    if include_external_subset {
        load_public_role(
            &store,
            &parts.external_used_subset,
            GRAPH_EXTERNAL_USED_SUBSET,
            "derived external used-subset graph",
        )?;
    }

    Ok(store)
}

fn build_derivation_store(parts: &SemanticStoreTurtleParts) -> Result<Store, ReqvireError> {
    let store = build_public_store(parts, true, true)?;
    load_named_graph(
        &store,
        &parts.raw_external_source,
        GRAPH_RAW_EXTERNAL_SOURCE,
        "raw external source graph",
    )?;
    Ok(store)
}

fn load_public_role(
    store: &Store,
    turtle: &str,
    graph_iri: &str,
    label: &str,
) -> Result<(), ReqvireError> {
    load_named_graph(store, turtle, graph_iri, label)?;
    load_default_graph(store, turtle, label)
}

fn new_store() -> Result<Store, ReqvireError> {
    Store::new().map_err(|error| {
        ReqvireError::ProcessError(format!("Failed to create RDF store: {}", error))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::model::{GraphNameRef, NamedNodeRef, QuadRef};

    fn empty_snapshot() -> Result<SemanticModelStore, ReqvireError> {
        let registry = GraphRegistry::new();
        let index = crate::semantic_contract::build_semantic_index(&registry);
        SemanticModelStore::from_index(&registry, Arc::new(index))
    }

    #[test]
    fn lazy_variants_are_shared_by_snapshot_clones() -> Result<(), ReqvireError> {
        let snapshot = empty_snapshot()?;
        let clone = snapshot.clone();
        assert_eq!(snapshot.initialized_store_count(), 0);
        let first = clone.store(false, false)?;
        assert_eq!(snapshot.initialized_store_count(), 1);
        assert!(std::ptr::eq(first, snapshot.store(false, false)?));
        for (full, external) in [(false, true), (true, false), (true, true)] {
            snapshot.store(full, external)?;
        }
        assert_eq!(clone.initialized_store_count(), 4);
        snapshot.derivation_store()?;
        assert_eq!(clone.initialized_store_count(), 5);
        Ok(())
    }

    #[test]
    fn concurrent_first_use_shares_one_complete_store_or_failure() -> Result<(), ReqvireError> {
        use std::sync::atomic::{AtomicUsize, Ordering};
        for fail in [false, true] {
            let mut snapshot = empty_snapshot()?;
            let mut parts = parts_with_raw_external();
            if fail {
                // The authored graph loads first; the malformed model graph
                // must not leave a partially populated public store behind.
                parts.authored_model = "invalid turtle".into();
            }
            snapshot.captured = Arc::new(CapturedQueryStores::new(parts));
            let starts = Arc::new(AtomicUsize::new(0));
            let observed = Arc::clone(&starts);
            snapshot.set_store_init_hook(move || {
                observed.fetch_add(1, Ordering::SeqCst);
            });
            let barrier = Arc::new(std::sync::Barrier::new(4));
            let workers: Vec<_> = (0..4)
                .map(|_| {
                    let cloned = snapshot.clone();
                    let barrier = Arc::clone(&barrier);
                    std::thread::spawn(move || {
                        barrier.wait();
                        cloned
                            .store(true, false)
                            .map(|store| store as *const Store as usize)
                            .map_err(|error| error.to_string())
                    })
                })
                .collect();
            let results: Vec<_> = workers
                .into_iter()
                .map(|w| w.join().expect("query worker"))
                .collect();
            assert_eq!(starts.load(Ordering::SeqCst), 1);
            assert_eq!(snapshot.initialized_store_count(), 1);
            assert!(results.iter().all(|result| result == &results[0]));
            assert_eq!(results[0].is_err(), fail);
            if fail {
                assert!(results[0]
                    .as_ref()
                    .err()
                    .expect("load error")
                    .contains("authored model graph"));
                assert!(snapshot.store(true, false).is_err());
                assert_eq!(starts.load(Ordering::SeqCst), 1);
                // A different variant and a new snapshot have independent state.
                assert!(snapshot.store(false, false).is_ok());
                let repaired = empty_snapshot()?;
                assert!(repaired.store(true, false).is_ok());
            }
        }
        Ok(())
    }

    #[test]
    fn lazy_variants_preserve_default_and_named_graph_visibility(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut snapshot = empty_snapshot()?;
        let mut parts = parts_with_raw_external();
        parts.external_used_subset =
            "<https://example.test/used> <https://example.test/p> <https://example.test/o> ."
                .into();
        snapshot.captured = Arc::new(CapturedQueryStores::new(parts));
        for (full, external) in [(false, false), (false, true), (true, false), (true, true)] {
            let store = snapshot.store(full, external)?;
            let facts = [
                ("authored", GRAPH_AUTHORED_ONTOLOGY, true),
                ("context", GRAPH_AUTHORED_MODEL, full),
                ("projection", GRAPH_GENERATED, full),
                ("used", GRAPH_EXTERNAL_USED_SUBSET, external),
                ("raw", GRAPH_RAW_EXTERNAL_SOURCE, false),
            ];
            for (subject, graph, present) in facts {
                let iri = format!("https://example.test/{subject}");
                for graph in [GraphNameRef::DefaultGraph, NamedNodeRef::new(graph)?.into()] {
                    assert_eq!(
                        store.contains(QuadRef::new(
                            NamedNodeRef::new(&iri)?,
                            NamedNodeRef::new("https://example.test/p")?,
                            NamedNodeRef::new("https://example.test/o")?,
                            graph,
                        ))?,
                        present,
                        "{subject}: full={full}, external={external}"
                    );
                }
            }
            assert_eq!(
                store.len()?,
                2 + if full { 4 } else { 0 } + if external { 2 } else { 0 }
            );
        }
        assert_eq!(snapshot.initialized_store_count(), 4);
        let derivation = snapshot.derivation_store()?;
        assert_eq!(derivation.len()?, 9);
        assert!(!derivation.contains(QuadRef::new(
            NamedNodeRef::new("https://example.test/raw")?,
            NamedNodeRef::new("https://example.test/p")?,
            NamedNodeRef::new("https://example.test/o")?,
            GraphNameRef::DefaultGraph,
        ))?);
        Ok(())
    }

    fn parts_with_raw_external() -> SemanticStoreTurtleParts {
        SemanticStoreTurtleParts {
            authored_ontology: "<https://example.test/authored> <https://example.test/p> <https://example.test/o> .".to_string(),
            authored_model: "<https://example.test/context> <https://example.test/p> <https://example.test/o> .".to_string(),
            generated: "<https://example.test/projection> <https://example.test/p> <https://example.test/o> .".to_string(),
            raw_external_source: "<https://example.test/raw> <https://example.test/p> <https://example.test/o> .".to_string(),
            external_used_subset: String::new(),
        }
    }

    #[test]
    fn public_store_does_not_load_raw_external_graph() -> Result<(), Box<dyn std::error::Error>> {
        let parts = parts_with_raw_external();
        let store = build_public_store(&parts, true, true)?;
        let raw = NamedNodeRef::new("https://example.test/raw")?;
        let predicate = NamedNodeRef::new("https://example.test/p")?;
        let object = NamedNodeRef::new("https://example.test/o")?;

        assert!(!store.contains(QuadRef::new(
            raw,
            predicate,
            object,
            GraphNameRef::DefaultGraph
        ))?);
        assert!(!store.contains(QuadRef::new(
            raw,
            predicate,
            object,
            NamedNodeRef::new(GRAPH_RAW_EXTERNAL_SOURCE)?
        ))?);
        Ok(())
    }

    #[test]
    fn derivation_store_loads_raw_external_only_as_named_graph(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let parts = parts_with_raw_external();
        let store = build_derivation_store(&parts)?;
        let raw = NamedNodeRef::new("https://example.test/raw")?;
        let predicate = NamedNodeRef::new("https://example.test/p")?;
        let object = NamedNodeRef::new("https://example.test/o")?;

        assert!(!store.contains(QuadRef::new(
            raw,
            predicate,
            object,
            GraphNameRef::DefaultGraph
        ))?);
        assert!(store.contains(QuadRef::new(
            raw,
            predicate,
            object,
            NamedNodeRef::new(GRAPH_RAW_EXTERNAL_SOURCE)?
        ))?);
        Ok(())
    }
}
