# JQ filter to extract and validate parsing
# Returns array of elements with normalized identifiers

[
  .files
  | to_entries[]
  | .value.sections
  | to_entries[]
  | .value.elements[]
  | {
      identifier: .identifier,
      name: .name,
      type: .type,
      has_content: (.content | length > 0),
      has_relations: (.relations | length > 0),
      relation_types: [.relations[]?.relation_type],
      fragment: (.identifier | split("#")[1])
    }
]
