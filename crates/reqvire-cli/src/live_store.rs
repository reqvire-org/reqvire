//! Content-addressed Explorer snapshots. A manifest describes the complete
//! current store, so clients need neither earlier manifests nor mutation logs.
use axum::http::StatusCode;
use reqvire::error::ReqvireError;
use reqvire::hashing::sha256_hex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

pub const MAX_CHUNK_BATCH: usize = 512;

#[derive(Serialize)]
struct StoreManifest {
    protocol: &'static str,
    sections: BTreeMap<String, ManifestSection>,
    ontology_hash: String,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum ManifestSection {
    Value { hash: String },
    Array { hashes: Vec<String> },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkRequest {
    pub revision: String,
    pub hashes: Vec<String>,
}

pub struct LiveStore {
    pub revision: String,
    pub manifest_json: String,
    chunks: HashMap<String, String>,
}

fn content_hash(content: &str) -> String {
    sha256_hex(content.as_bytes())
}

fn valid_hash(hash: &str) -> bool {
    hash.len() == 64
        && hash
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

impl LiveStore {
    pub fn new(store: &Value, ontologies_ttl: &str) -> Result<Self, ReqvireError> {
        let object = store
            .as_object()
            .ok_or_else(|| ReqvireError::ProcessError("Explorer store must be an object".into()))?;
        let mut chunks = HashMap::new();
        let mut sections = BTreeMap::new();
        let mut add_chunk = |value: &Value| -> Result<String, ReqvireError> {
            let content = serde_json::to_string(value)?;
            let hash = content_hash(&content);
            chunks.entry(hash.clone()).or_insert(content);
            Ok(hash)
        };
        for (name, value) in object {
            let section = match value {
                Value::Array(items) => ManifestSection::Array {
                    hashes: items.iter().map(&mut add_chunk).collect::<Result<_, _>>()?,
                },
                value => ManifestSection::Value {
                    hash: add_chunk(value)?,
                },
            };
            sections.insert(name.clone(), section);
        }
        let manifest_json = serde_json::to_string(&StoreManifest {
            protocol: "reqvire-manifest.v1",
            sections,
            ontology_hash: content_hash(ontologies_ttl),
        })?;
        Ok(Self {
            revision: content_hash(&manifest_json),
            manifest_json,
            chunks,
        })
    }

    /// Reads only this immutable snapshot. The HTTP handler keeps an Arc to it
    /// throughout the response, even if an MCP mutation publishes a newer one.
    pub fn chunks_json(&self, request: &ChunkRequest) -> Result<String, StatusCode> {
        if !valid_hash(&request.revision)
            || request.hashes.len() > MAX_CHUNK_BATCH
            || request.hashes.iter().any(|hash| !valid_hash(hash))
        {
            return Err(StatusCode::BAD_REQUEST);
        }
        if request.revision != self.revision {
            return Err(StatusCode::CONFLICT);
        }
        let mut selected = BTreeMap::new();
        for hash in &request.hashes {
            let content = self.chunks.get(hash).ok_or(StatusCode::NOT_FOUND)?;
            selected.insert(hash, content);
        }
        serde_json::to_string(&serde_json::json!({
            "revision": self.revision,
            "chunks": selected,
        }))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn snapshot(store: Value, ontology: &str) -> LiveStore {
        LiveStore::new(&store, ontology).expect("build test live store")
    }

    #[test]
    fn uses_standard_sha256_wire_hashes() {
        assert_eq!(
            content_hash("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn canonical_store_preserves_wire_bytes_of_the_previous_json_input_path() {
        let store = json!({"numbers":[-0.0, 1.0, 1e-20, 1e20, 33.3, 66.7,
            9223372036854775807_i64], "text":"</script> 測定 &\u{2028}",
            "empty":[], "unknown":{"nested":[true,null,{"key":"value"}]}});
        let previous: Value = serde_json::from_str(&store.to_string()).unwrap();
        let direct = LiveStore::new(&store, "ontology").unwrap();
        let through_json = LiveStore::new(&previous, "ontology").unwrap();
        assert_eq!(direct.chunks, through_json.chunks);
        assert_eq!(direct.manifest_json, through_json.manifest_json);
        assert_eq!(direct.revision, through_json.revision);
    }

    #[test]
    fn accepts_deep_valid_generated_json_without_an_extra_depth_limit() {
        let mut nested = serde_json::json!("leaf");
        for _ in 0..140 {
            nested = serde_json::json!({"child": nested});
        }
        let store = serde_json::json!({"extra": nested});
        assert!(LiveStore::new(&store, "").is_ok());
    }

    #[test]
    fn complete_manifest_round_trips_unicode_numbers_order_and_empty_sections() {
        let store = json!({
            "elements": [{"id": "Lärche 測定", "number": -0.0}, {"id": "aster", "number": 1.0}],
            "empty": [], "project": {"name": "café"}, "extra": null,
        });
        let live = snapshot(store.clone(), "ontology");
        // Independent golden bytes pin the Explorer protocol across extraction
        // of SHA-256 into core. Model revision framing must never enter here.
        assert_eq!(
            live.manifest_json,
            include_str!("../tests/fixtures/live-store-manifest.json").trim_end()
        );
        assert_eq!(
            live.revision,
            "c93e908d4702c6c90e66f936b9dfbcd128e97ee5175554f1aae1c0782e519cf0"
        );
        assert_eq!(live.revision, content_hash(&live.manifest_json));
        let manifest: Value =
            serde_json::from_str(&live.manifest_json).expect("parse generated JSON");
        let mut recovered = serde_json::Map::new();
        for (name, section) in manifest["sections"]
            .as_object()
            .expect("expected a JSON object")
        {
            let value = if section["kind"] == "array" {
                Value::Array(
                    section["hashes"]
                        .as_array()
                        .expect("expected a JSON array")
                        .iter()
                        .map(|hash| {
                            let content =
                                &live.chunks[hash.as_str().expect("expected a JSON string")];
                            assert_eq!(
                                content_hash(content),
                                hash.as_str().expect("expected a JSON string")
                            );
                            serde_json::from_str(content).expect("parse generated JSON")
                        })
                        .collect(),
                )
            } else {
                serde_json::from_str(
                    &live.chunks[section["hash"].as_str().expect("expected a JSON string")],
                )
                .expect("parse generated JSON")
            };
            recovered.insert(name.clone(), value);
        }
        assert_eq!(Value::Object(recovered), store);
    }

    #[test]
    fn revisions_and_chunks_are_stable_but_changes_and_deletions_are_visible() {
        let first = snapshot(
            json!({"elements": [{"id": "first"}, {"id": "second"}]}),
            "ontology",
        );
        let same = snapshot(
            json!({"elements": [{"id": "first"}, {"id": "second"}]}),
            "ontology",
        );
        let reordered = snapshot(
            json!({"elements": [{"id": "second"}, {"id": "first"}]}),
            "ontology",
        );
        let deleted = snapshot(json!({"elements": [{"id": "second"}]}), "ontology");
        let ontology = snapshot(
            json!({"elements": [{"id": "first"}, {"id": "second"}]}),
            "changed",
        );
        assert_eq!(first.revision, same.revision);
        assert_eq!(first.chunks, same.chunks);
        assert_eq!(first.chunks, reordered.chunks);
        assert_ne!(first.revision, reordered.revision);
        assert_ne!(first.revision, deleted.revision);
        assert_ne!(first.revision, ontology.revision);
        assert_eq!(deleted.chunks.len(), 1);
    }

    #[test]
    fn batch_contains_only_requested_immutable_chunks_and_deduplicates() {
        let live = snapshot(
            json!({"elements": [{"id": "first"}, {"id": "second"}]}),
            "ontology",
        );
        let hash = content_hash("{\"id\":\"first\"}");
        let request = ChunkRequest {
            revision: live.revision.clone(),
            hashes: vec![hash.clone(), hash.clone()],
        };
        let response: Value = serde_json::from_str(
            &live
                .chunks_json(&request)
                .expect("retrieve requested test chunks"),
        )
        .expect("parse generated JSON");
        assert_eq!(response["revision"], live.revision);
        assert_eq!(
            response["chunks"]
                .as_object()
                .expect("expected a JSON object")
                .len(),
            1
        );
        assert_eq!(response["chunks"][&hash], "{\"id\":\"first\"}");
    }

    #[test]
    fn rejects_unknown_invalid_oversized_and_superseded_requests() {
        let live = snapshot(json!({"elements": []}), "ontology");
        let request = |revision: String, hashes: Vec<String>| ChunkRequest { revision, hashes };
        assert_eq!(
            live.chunks_json(&request("0".repeat(64), vec![])),
            Err(StatusCode::CONFLICT)
        );
        assert_eq!(
            live.chunks_json(&request("bad".into(), vec![])),
            Err(StatusCode::BAD_REQUEST)
        );
        assert_eq!(
            live.chunks_json(&request(live.revision.clone(), vec!["../file".into()])),
            Err(StatusCode::BAD_REQUEST)
        );
        assert_eq!(
            live.chunks_json(&request(live.revision.clone(), vec!["0".repeat(64)])),
            Err(StatusCode::NOT_FOUND)
        );
        assert_eq!(
            live.chunks_json(&request(
                live.revision.clone(),
                vec!["0".repeat(64); MAX_CHUNK_BATCH + 1]
            )),
            Err(StatusCode::BAD_REQUEST)
        );
    }
}
