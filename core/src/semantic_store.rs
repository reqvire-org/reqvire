use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::semantic_contract::{self, SemanticExportFormat, SemanticIndex};
use oxigraph::io::{RdfFormat, RdfParser};
use oxigraph::store::Store;
use std::fmt;

#[derive(Clone)]
pub struct SemanticModelStore {
    pub index: SemanticIndex,
    authored_store: Store,
    authored_external_store: Store,
    full_store: Store,
    full_external_store: Store,
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
        let authored_turtle = index.serialize(SemanticExportFormat::Turtle)?;
        let authored_external_turtle = index.to_turtle_string_with_external(true)?;
        let full_turtle = index.serialize_full(SemanticExportFormat::Turtle)?;
        let full_external_turtle = index.to_full_turtle_string_with_external(true)?;

        Ok(Self {
            index,
            authored_store: load_turtle_store(&authored_turtle, "authored semantic RDF")?,
            authored_external_store: load_turtle_store(
                &authored_external_turtle,
                "authored semantic RDF with external ontology sources",
            )?,
            full_store: load_turtle_store(&full_turtle, "full semantic RDF")?,
            full_external_store: load_turtle_store(
                &full_external_turtle,
                "full semantic RDF with external ontology sources",
            )?,
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
}

fn load_turtle_store(turtle: &str, label: &str) -> Result<Store, ReqvireError> {
    let store = Store::new().map_err(|error| {
        ReqvireError::ProcessError(format!("Failed to create RDF store: {}", error))
    })?;
    store
        .load_from_reader(RdfParser::from_format(RdfFormat::Turtle), turtle.as_bytes())
        .map_err(|error| {
            ReqvireError::ProcessError(format!("Failed to load {} into Oxigraph: {}", label, error))
        })?;
    Ok(store)
}
