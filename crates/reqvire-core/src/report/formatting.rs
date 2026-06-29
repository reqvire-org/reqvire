pub(super) fn format_identifier_markdown_link(label: &str, identifier: &str) -> String {
    if let Some(hash_pos) = identifier.rfind('#') {
        let file_part = &identifier[..hash_pos];
        let fragment_part = &identifier[hash_pos..];
        format!("[{}]({}{})", label, file_part, fragment_part)
    } else {
        format!("[{}]({})", label, identifier)
    }
}
