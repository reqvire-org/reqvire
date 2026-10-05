use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::rdf_store::{load_default_graph, load_named_graph};
use crate::semantic_contract::{
    SemanticBlock, SemanticExportFormat, SemanticExportLayer, SemanticIndex,
};
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
    index: Arc<SemanticIndex>,
    captured: Arc<CapturedQueryStores>,
}

/// Captured RDF and initialization cells have one owner per completed snapshot.
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
    /// Read the semantic index accepted with this snapshot.
    pub fn index(&self) -> &Arc<SemanticIndex> {
        &self.index
    }

    /// Export semantic layers from this completed snapshot.
    pub fn serialize_export_layers(
        &self,
        format: SemanticExportFormat,
        layers: &[SemanticExportLayer],
        namespace_base: Option<&str>,
    ) -> Result<String, ReqvireError> {
        self.index
            .serialize_export_layers_with_subset(format, layers, namespace_base, |filtered| {
                self.subset_for_namespace(filtered, namespace_base)
            })
    }

    /// Export the ontology command's clean or full projection from this snapshot.
    pub fn serialize_with_options_and_filter(
        &self,
        format: SemanticExportFormat,
        full: bool,
        include_external: bool,
        namespace_base: Option<&str>,
    ) -> Result<String, ReqvireError> {
        self.index.serialize_with_options_and_filter_with_subset(
            format,
            full,
            include_external,
            namespace_base,
            |filtered| self.subset_for_namespace(filtered, namespace_base),
        )
    }

    fn subset_for_namespace(
        &self,
        filtered: &SemanticIndex,
        namespace_base: Option<&str>,
    ) -> Result<Option<SemanticBlock>, ReqvireError> {
        if namespace_base.is_some_and(|value| !value.trim().is_empty()) {
            filtered.used_external_subset_block()
        } else {
            Ok(self.used_external_subset().cloned())
        }
    }

    /// Project external visibility for this completed snapshot.
    pub fn index_with_external_visibility(
        &self,
        include_external: bool,
    ) -> Result<SemanticIndex, ReqvireError> {
        let mut index = self.index.as_ref().clone();
        index.apply_external_visibility_with_subset(
            include_external,
            self.used_external_subset().cloned(),
        );
        Ok(index)
    }

    pub(crate) fn used_external_subset(&self) -> Option<&SemanticBlock> {
        self.captured.parts.external_used_subset.as_ref()
    }

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
    external_used_subset: Option<SemanticBlock>,
}

impl SemanticStoreTurtleParts {
    fn from_index(registry: &GraphRegistry, index: &SemanticIndex) -> Result<Self, ReqvireError> {
        Ok(Self {
            authored_ontology: index.to_authored_ontology_turtle_string()?,
            authored_model: index.to_authored_model_layer_turtle_string(registry)?,
            generated: index.to_generated_layer_turtle_string(registry)?,
            raw_external_source: index.to_raw_external_turtle_string()?,
            external_used_subset: index.used_external_subset_block()?,
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
            parts
                .external_used_subset
                .as_ref()
                .map_or("", |block| block.content.as_str()),
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
    use crate::semantic_contract::{
        build_semantic_index, ExternalOntologySource, SemanticBlock, SemanticBlockKind,
        SUBSET_DERIVATIONS,
    };
    use crate::test_support::parse_test_quads;
    use oxigraph::model::{GraphNameRef, NamedNodeRef, QuadRef};

    fn empty_snapshot() -> Result<SemanticModelStore, ReqvireError> {
        let registry = GraphRegistry::new();
        let index = crate::semantic_contract::build_semantic_index(&registry);
        SemanticModelStore::from_index(&registry, Arc::new(index))
    }

    fn external_fixture() -> (GraphRegistry, SemanticIndex) {
        let mut registry = GraphRegistry::new();
        for prefix in ["alpha", "beta"] {
            let content = format!("### {prefix}\n#### Metadata\n  * type: ontology\n  * ontology_base: https://example.test/{prefix}\n  * ontology_prefix: {prefix}\n#### Ontology\n```turtle\n@prefix {prefix}: <https://example.test/{prefix}#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n<https://example.test/{prefix}> a owl:Ontology .\n{prefix}:Local a owl:Class ; rdfs:subClassOf <https://example.test/external#{prefix}> .\n```\n");
            let element = crate::parser::parse_single_element(&content, "model.md").unwrap();
            registry.register_element(element, "model.md").unwrap();
        }
        let mut index = build_semantic_index(&registry);
        let content = "@prefix ext: <https://example.test/external#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\next:alpha a owl:Class ; rdfs:label \"Alpha external\" ; rdfs:subClassOf ext:Support .\next:beta a owl:Class ; rdfs:label \"Beta external\" .\next:Support a owl:Class ; rdfs:label \"Support\" .\next:Unused a owl:Class ; rdfs:label \"Unused\" .\n";
        index.external_blocks = vec![SemanticBlock {
            kind: SemanticBlockKind::ExternalOntology,
            source: "references/external.ttl".into(),
            source_name: "External vocabulary".into(),
            file_path: "references/external.ttl".into(),
            line_number: 1,
            language: "turtle".into(),
            external_materialization: None,
            content: content.into(),
            quads: parse_test_quads(content),
        }];
        index.external_sources = vec![ExternalOntologySource {
            owner_identifier: "model.md#alpha".into(),
            owner_name: "alpha".into(),
            prefix: "ext".into(),
            namespace: "https://example.test/external#".into(),
            resource: Some("https://example.test/external".into()),
            source: "references/external.ttl".into(),
            format: "turtle".into(),
            line_number: 1,
            builtin: false,
        }];
        // Isolate authored-graph seeds to exercise different namespace subsets.
        index.model_context_turtle.clear();
        index.ontology_projection.constructs.clear();
        index.ontology_projection.symbols.clear();
        (registry, index)
    }

    fn subset_export(snapshot: &SemanticModelStore, namespace: Option<&str>) -> String {
        snapshot
            .serialize_export_layers(
                SemanticExportFormat::Turtle,
                &[SemanticExportLayer::ExternalUsed],
                namespace,
            )
            .unwrap()
    }

    fn triple_set(turtle: &str) -> std::collections::BTreeSet<String> {
        parse_test_quads(turtle)
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    #[test]
    fn external_subset_is_shared_by_capture_exports_visibility_and_explorer() {
        let (registry, index) = external_fixture();
        let expected = index.with_external_visibility(true).unwrap();
        let expected_export = index
            .serialize_export_layers(
                SemanticExportFormat::Turtle,
                &[SemanticExportLayer::ExternalUsed],
                None,
            )
            .unwrap();
        let before = SUBSET_DERIVATIONS.get();
        let snapshot = SemanticModelStore::from_index(&registry, Arc::new(index)).unwrap();
        assert_eq!(SUBSET_DERIVATIONS.get() - before, 1);
        let mut model = crate::model::ModelManager::new();
        model.graph_registry = registry;
        model.semantic_store = Some(snapshot.clone());
        for _ in 0..2 {
            assert_eq!(
                triple_set(&subset_export(&snapshot, None)),
                triple_set(&expected_export)
            );
            let visible = snapshot.index_with_external_visibility(true).unwrap();
            assert_eq!(
                serde_json::to_value(&visible).unwrap(),
                serde_json::to_value(&expected).unwrap()
            );
            for full in [false, true] {
                snapshot.store(full, true).unwrap();
            }
            let runtime = crate::explorer_runtime::build_runtime_assets(&model).unwrap();
            assert!(runtime.project_store_json.contains("Alpha external"));
            assert!(!runtime.project_store_json.contains("Unused\""));
        }
        assert_eq!(
            SUBSET_DERIVATIONS.get() - before,
            1,
            "all consumers must reuse the captured subset"
        );
    }

    #[test]
    fn external_subset_filtered_exports_have_independent_inputs() {
        let (registry, index) = external_fixture();
        let snapshot = SemanticModelStore::from_index(&registry, Arc::new(index)).unwrap();
        let whole = subset_export(&snapshot, None);
        for namespace in ["alpha", "beta", "alpha"] {
            let before = SUBSET_DERIVATIONS.get();
            let output = subset_export(
                &snapshot,
                Some(&format!("https://example.test/{namespace}")),
            );
            assert_eq!(
                SUBSET_DERIVATIONS.get() - before,
                1,
                "one derivation per filtered export"
            );
            assert!(output.contains(if namespace == "alpha" {
                "Alpha external"
            } else {
                "Beta external"
            }));
            assert!(!output.contains(if namespace == "alpha" {
                "Beta external"
            } else {
                "Alpha external"
            }));
            assert_eq!(
                triple_set(&subset_export(&snapshot, None)),
                triple_set(&whole)
            );
        }
    }

    #[test]
    fn external_subset_ontology_exports_reuse_capture_in_both_formats_and_projections() {
        let (registry, index) = external_fixture();
        let formats = [SemanticExportFormat::Turtle, SemanticExportFormat::JsonLd];
        let expected: Vec<_> = formats
            .into_iter()
            .flat_map(|format| {
                [false, true].map(|full| {
                    (
                        format,
                        full,
                        index
                            .serialize_with_options_and_filter(format, full, true, None)
                            .unwrap(),
                    )
                })
            })
            .collect();
        let before = SUBSET_DERIVATIONS.get();
        let snapshot = SemanticModelStore::from_index(&registry, Arc::new(index)).unwrap();
        for _ in 0..2 {
            for (format, full, content) in &expected {
                assert_eq!(
                    &snapshot
                        .serialize_with_options_and_filter(*format, *full, true, None)
                        .unwrap(),
                    content
                );
            }
        }
        assert_eq!(
            SUBSET_DERIVATIONS.get() - before,
            1,
            "ontology exports must reuse capture"
        );
        for namespace in ["alpha", "beta"] {
            let before = SUBSET_DERIVATIONS.get();
            let content = snapshot
                .serialize_with_options_and_filter(
                    SemanticExportFormat::Turtle,
                    false,
                    true,
                    Some(&format!("https://example.test/{namespace}")),
                )
                .unwrap();
            assert_eq!(SUBSET_DERIVATIONS.get() - before, 1);
            assert!(content.contains(if namespace == "alpha" {
                "Alpha external"
            } else {
                "Beta external"
            }));
            assert!(!content.contains(if namespace == "alpha" {
                "Beta external"
            } else {
                "Alpha external"
            }));
        }
    }

    #[test]
    fn external_subset_concurrent_readers_do_not_rederive() {
        let (registry, index) = external_fixture();
        let snapshot = SemanticModelStore::from_index(&registry, Arc::new(index)).unwrap();
        let expected = triple_set(&subset_export(&snapshot, None));
        std::thread::scope(|scope| {
            let threads: Vec<_> = (0..4)
                .map(|_| {
                    let snapshot = snapshot.clone();
                    scope.spawn(move || {
                        let before = SUBSET_DERIVATIONS.get();
                        let output = subset_export(&snapshot, None);
                        snapshot.index_with_external_visibility(true).unwrap();
                        snapshot.store(true, true).unwrap();
                        (triple_set(&output), SUBSET_DERIVATIONS.get() - before)
                    })
                })
                .collect();
            for thread in threads {
                let (output, derivations) = thread.join().unwrap();
                assert_eq!(output, expected);
                assert_eq!(derivations, 0, "snapshot clone must share derived subset");
            }
        });
    }

    #[test]
    fn external_subset_failed_and_edited_candidates_preserve_accepted_snapshot() {
        let (registry, index) = external_fixture();
        let snapshot = SemanticModelStore::from_index(&registry, Arc::new(index)).unwrap();
        let accepted = subset_export(&snapshot, None);
        let mut invalid = snapshot.index().as_ref().clone();
        invalid.model_context_turtle = "invalid turtle".into();
        let invalid = Arc::new(invalid);
        let first = SemanticModelStore::from_index(&registry, invalid.clone())
            .unwrap_err()
            .to_string();
        let second = SemanticModelStore::from_index(&registry, invalid)
            .unwrap_err()
            .to_string();
        assert_eq!(first, second);
        assert!(first.contains("model"), "{first}");
        let mut edited = snapshot.index().as_ref().clone();
        let block = &mut edited.external_blocks[0];
        block.content = block.content.replace("Alpha external", "Edited external");
        block.quads = parse_test_quads(&block.content);
        let before = SUBSET_DERIVATIONS.get();
        let candidate = SemanticModelStore::from_index(&registry, Arc::new(edited)).unwrap();
        assert_eq!(SUBSET_DERIVATIONS.get() - before, 1);
        assert!(subset_export(&candidate, None).contains("Edited external"));
        assert_eq!(
            triple_set(&subset_export(&snapshot, None)),
            triple_set(&accepted)
        );
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
        let content = "<https://example.test/used> <https://example.test/p> <https://example.test/o> .";
        parts.external_used_subset = Some(SemanticBlock {
            kind: crate::semantic_contract::SemanticBlockKind::ExternalOntology,
            source: "reqvire:external-used-subset".into(),
            source_name: "Used subset".into(),
            file_path: String::new(),
            line_number: 0,
            language: "turtle".into(),
            external_materialization: Some("used_subset".into()),
            content: content.into(),
            quads: crate::test_support::parse_test_quads(content),
        });
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
            external_used_subset: None,
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
