use crate::error::ReqvireError;
use oxigraph::io::{RdfFormat, RdfParser};
use oxigraph::model::NamedNode;
use oxigraph::store::Store;

pub(crate) fn load_default_graph(
    store: &Store,
    turtle: &str,
    label: &str,
) -> Result<(), ReqvireError> {
    if turtle.trim().is_empty() {
        return Ok(());
    }
    store
        .load_from_reader(
            RdfParser::from_format(RdfFormat::Turtle).without_named_graphs(),
            turtle.as_bytes(),
        )
        .map_err(|error| {
            ReqvireError::ProcessError(format!("Failed to load {} into Oxigraph: {}", label, error))
        })
}

pub(crate) fn load_named_graph(
    store: &Store,
    turtle: &str,
    graph_iri: &str,
    label: &str,
) -> Result<(), ReqvireError> {
    if turtle.trim().is_empty() {
        return Ok(());
    }
    let graph_name = NamedNode::new(graph_iri).map_err(|error| {
        ReqvireError::ProcessError(format!(
            "Invalid semantic named graph IRI '{}': {}",
            graph_iri, error
        ))
    })?;
    store
        .load_from_reader(
            RdfParser::from_format(RdfFormat::Turtle)
                .without_named_graphs()
                .with_default_graph(graph_name),
            turtle.as_bytes(),
        )
        .map_err(|error| {
            ReqvireError::ProcessError(format!("Failed to load {} into Oxigraph: {}", label, error))
        })
}
