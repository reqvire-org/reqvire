use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Subsection {
    Metadata(HashMap<String, String>),
    Relations(Vec<String>),
    Details(String),
    Other(String, String),
}

impl Subsection {
    pub fn parse_metadata(content: &str) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        for line in content.lines() {
            if let Some((key, value)) = line.trim().split_once(": ") {
                metadata.insert(key.to_string(), value.to_string());
            }
        }
        metadata
    }

    pub fn parse_relations(content: &str) -> Vec<String> {
        content.lines()
            .filter(|line| line.trim().starts_with('*'))
            .map(|line| line.trim_start_matches("* ").to_string())
            .collect()
    }
}

