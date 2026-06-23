//! Embedded Reqvire runtime semantic artifacts.
//!
//! The authored source of truth remains the Reqvire model under
//! `system-model/Ontologies`. This module embeds generated Turtle artifacts
//! that runtime/bootstrap code can consume without reparsing the system model.

pub const REQVIRE_ONTOLOGY_TTL: &str = include_str!("runtime_ontology/reqvire.ttl");
pub const REQVIRE_SHACL_TTL: &str = include_str!("runtime_ontology/reqvire-shacl.ttl");

pub const REQVIRE_TTL: &str = REQVIRE_ONTOLOGY_TTL;
