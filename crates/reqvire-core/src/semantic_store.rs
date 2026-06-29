use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::rdf_store::{load_default_graph, load_named_graph};
use crate::semantic_contract::{self, SemanticIndex};
use oxigraph::store::Store;
use std::fmt;

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
    pub index: SemanticIndex,
    authored_store: Store,
    authored_external_store: Store,
    full_store: Store,
    full_external_store: Store,
    #[allow(dead_code)]
    derivation_store: Store,
}

impl fmt::Debug for SemanticModelStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SemanticModelStore")
            .field("summary", &self.index.summary)
            .finish_non_exhaustive()
    }
}

impl SemanticModelStore {
    pub fn build(registry: &GraphRegistry) -> Result<Self, ReqvireError> {
        let index = semantic_contract::build_semantic_index(registry);
        let turtle_parts = SemanticStoreTurtleParts::from_index(registry, &index)?;

        Ok(Self {
            authored_store: build_public_store(&turtle_parts, false, false)?,
            authored_external_store: build_public_store(&turtle_parts, false, true)?,
            full_store: build_public_store(&turtle_parts, true, false)?,
            full_external_store: build_public_store(&turtle_parts, true, true)?,
            derivation_store: build_derivation_store(&turtle_parts)?,
            index,
        })
    }

    pub fn store(&self, full: bool, include_external: bool) -> &Store {
        match (full, include_external) {
            (false, false) => &self.authored_store,
            (false, true) => &self.authored_external_store,
            (true, false) => &self.full_store,
            (true, true) => &self.full_external_store,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn derivation_store(&self) -> &Store {
        &self.derivation_store
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
