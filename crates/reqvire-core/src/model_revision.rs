//! Canonical parsed-element projection, version 2.
//!
//! This format is independent of Rust's `Hash` and the runtime Element layout.
//! Changing its fields, framing, or ordering requires a new encoding version.
//! Explorer uses the same SHA-256 primitive with its own exact wire bytes.

use crate::element::{ContractBindingTarget, Element};
use crate::error::ReqvireError;
use crate::hashing::sha256_hex;
use crate::relation::LinkType;
use std::collections::BTreeSet;
use std::path::{Component, Path};

#[cfg(test)]
mod tests;

pub fn fingerprint(elements: &[&Element]) -> Result<String, ReqvireError> {
    Ok(sha256_hex(&canonical_model_bytes(elements)?))
}

fn count(bytes: &mut Vec<u8>, value: usize) {
    bytes.extend_from_slice(&(value as u64).to_be_bytes());
}

fn frame(bytes: &mut Vec<u8>, value: &[u8]) {
    count(bytes, value.len());
    bytes.extend_from_slice(value);
}

fn string(bytes: &mut Vec<u8>, value: &str) {
    frame(bytes, value.as_bytes());
}

fn canonical_model_bytes(elements: &[&Element]) -> Result<Vec<u8>, ReqvireError> {
    let mut ordered = elements
        .iter()
        .map(|element| Ok((identifier(&element.identifier)?, *element)))
        .collect::<Result<Vec<_>, ReqvireError>>()?;
    ordered.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut bytes = Vec::new();
    string(&mut bytes, "reqvire.model-revision.v2");
    count(&mut bytes, ordered.len());
    for (_, element) in ordered {
        frame(&mut bytes, &canonical_element_bytes(element)?);
    }
    Ok(bytes)
}

/// Complete versioned record, reusable by future element-level consumers.
/// Inputs come from the resolved graph, including generated inverse relations.
pub fn canonical_element_bytes(element: &Element) -> Result<Vec<u8>, ReqvireError> {
    let mut bytes = Vec::new();
    string(&mut bytes, "reqvire.element.v2");
    string(&mut bytes, &identifier(&element.identifier)?);
    string(&mut bytes, &element.name);
    string(&mut bytes, element.element_type.as_str());
    string(&mut bytes, &element.content);
    string(&mut bytes, &relative_path(Path::new(&element.file_path))?);

    let mut metadata: Vec<_> = element
        .metadata
        .iter()
        .filter(|(key, _)| key.as_str() != "_single_element_format")
        .collect();
    metadata.sort_unstable_by(|a, b| a.0.cmp(b.0));
    count(&mut bytes, metadata.len());
    for (key, value) in metadata {
        string(&mut bytes, key);
        string(&mut bytes, value);
    }

    // Do not use Relation's Eq/Ord or LinkType::as_str: they discard target
    // kinds, and the latter silently replaces non-UTF-8 paths with empty text.
    let relations = element
        .relations
        .iter()
        .map(|relation| {
            let (kind, target) = match &relation.target.link {
                LinkType::Identifier(id) => ("identifier", identifier(id)?),
                LinkType::InternalPath(path) => ("internal_path", relative_path(path)?),
                LinkType::ExternalUrl(url) => ("external_url", url.clone()),
            };
            Ok((relation.relation_type.name, kind, target))
        })
        .collect::<Result<BTreeSet<_>, ReqvireError>>()?;
    count(&mut bytes, relations.len());
    for (relation_type, kind, target) in relations {
        string(&mut bytes, relation_type);
        string(&mut bytes, kind);
        string(&mut bytes, &target);
    }

    let bindings = element
        .contract_bindings
        .iter()
        .map(|binding| {
            Ok(match &binding.target {
                ContractBindingTarget::ElementIdentifier(id) => ("identifier", identifier(id)?),
                ContractBindingTarget::FilePath(path) => ("internal_path", relative_path(path)?),
            })
        })
        .collect::<Result<BTreeSet<_>, ReqvireError>>()?;
    count(&mut bytes, bindings.len());
    for (kind, target) in bindings {
        string(&mut bytes, kind);
        string(&mut bytes, &target);
    }
    let references = element
        .contract_references
        .iter()
        .map(|reference| match &reference.target {
            ContractBindingTarget::ElementIdentifier(id) => identifier(id),
            ContractBindingTarget::FilePath(_) => Err(ReqvireError::InvalidContractReference(
                "Contract References require element identifiers".into(),
            )),
        })
        .collect::<Result<BTreeSet<_>, ReqvireError>>()?;
    count(&mut bytes, references.len());
    for target in references {
        string(&mut bytes, "identifier");
        string(&mut bytes, &target);
    }
    Ok(bytes)
}

fn identifier(value: &str) -> Result<String, ReqvireError> {
    match value.split_once('#') {
        Some((path, fragment)) => Ok(format!("{}#{fragment}", relative_path(Path::new(path))?)),
        None => relative_path(Path::new(value)),
    }
}

fn relative_path(path: &Path) -> Result<String, ReqvireError> {
    let mut parts = Vec::new();
    for part in path.components() {
        match part {
            Component::Normal(value) => parts.push(value.to_str().ok_or_else(|| {
                ReqvireError::PathError(format!("Model revision path is not UTF-8: {path:?}"))
            })?),
            Component::CurDir => {}
            _ => {
                return Err(ReqvireError::PathError(format!(
                    "Model revision requires resolved workspace-relative paths: {path:?}"
                )))
            }
        }
    }
    Ok(parts.join("/"))
}
