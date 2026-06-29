pub(crate) fn concept_local_name(name: &str) -> String {
    let mut local = String::new();
    for part in name
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
    {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            local.push(first.to_ascii_uppercase());
            for ch in chars {
                local.push(ch);
            }
        }
    }
    if local.is_empty() {
        "Concept".to_string()
    } else {
        local
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concept_local_name_camelcases_parts() {
        assert_eq!(concept_local_name("foo-bar"), "FooBar");
        assert_eq!(concept_local_name(""), "Concept");
        assert_eq!(concept_local_name("FOO_BAR baz"), "FOOBARBaz");
    }
}
