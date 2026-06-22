//! Embedded Reqvire runtime ontology artifact.
//!
//! The authored source of truth remains the Reqvire model under
//! `system-model/Ontologies`. This module embeds the generated Turtle artifact
//! that runtime/bootstrap code can consume without reparsing the system model.

pub const REQVIRE_TTL: &str = include_str!("runtime_ontology/reqvire.ttl");
