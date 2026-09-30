pub(super) fn format_identifier_markdown_link(label: &str, identifier: &str) -> String {
    identifier.rfind('#').map_or_else(
        || format!("[{}]({})", label, identifier),
        |hash_pos| {
            let file_part = &identifier[..hash_pos];
            let fragment_part = &identifier[hash_pos..];
            format!("[{}]({}{})", label, file_part, fragment_part)
        },
    )
}
