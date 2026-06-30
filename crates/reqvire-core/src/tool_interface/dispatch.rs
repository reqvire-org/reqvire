use super::*;

pub(crate) fn dispatch_tool(
    name: &str,
    args: &Value,
    enable_mutations: bool,
    with_size_estimates: bool,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    match name {
        "reqvire.workspace_status" => {
            workspace_status(excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.tool_contract" => Ok(json!({
            "mcp_protocol_version": MCP_PROTOCOL_VERSION,
            "tool_contract_version": TOOL_CONTRACT_VERSION,
            "mutation_tools_enabled": enable_mutations,
            "size_estimates_enabled": with_size_estimates,
            "tools": tool_definitions(enable_mutations)
        })),
        "reqvire.model_revision" => model_revision(excluded_filename_patterns, with_size_estimates),
        "reqvire.read_element" => {
            read_element(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.search" => search_tool(args, excluded_filename_patterns),
        "reqvire.model" => model_tool(args, excluded_filename_patterns, with_size_estimates),
        "reqvire.containment" => containment_tool(args, excluded_filename_patterns),
        "reqvire.collect" => collect_tool(args, excluded_filename_patterns),
        "reqvire.submodels" => submodels_tool(args, excluded_filename_patterns),
        "reqvire.semantic.export" => {
            semantic_export_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.ontologies" => {
            ontologies_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.shapes" => {
            shapes_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.concepts" => {
            concepts_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.model" => {
            semantic_model_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.concepts.list" => {
            concepts_list_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.concepts.get" => {
            concept_get_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.concept_schemes.list" => {
            concept_schemes_list_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.concept_mappings.list" => {
            concept_mappings_list_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.graph" => {
            semantic_graph_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.prefixes" => {
            semantic_prefixes_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.vocabulary" => {
            semantic_vocabulary_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.semantic.sparql" => {
            sparql_tool(args, excluded_filename_patterns, with_size_estimates)
        }
        "reqvire.lint" => lint_tool(args, excluded_filename_patterns),
        "reqvire.coverage" => coverage_tool(excluded_filename_patterns),
        "reqvire.traces" => traces_tool(args, excluded_filename_patterns),
        "reqvire.resources" => resources_tool(excluded_filename_patterns),
        "reqvire.change_impact" => change_impact_tool(args, excluded_filename_patterns),
        "reqvire.format" => format_tool(args, enable_mutations, excluded_filename_patterns),
        "reqvire.add_element" => add_element_tool(args, excluded_filename_patterns),
        "reqvire.remove_element" => remove_element_tool(args, excluded_filename_patterns),
        "reqvire.move_element" => move_element_tool(args, excluded_filename_patterns),
        "reqvire.rename_element" => rename_element_tool(args, excluded_filename_patterns),
        "reqvire.merge_elements" => merge_elements_tool(args, excluded_filename_patterns),
        "reqvire.move_file" => move_file_tool(args, excluded_filename_patterns),
        "reqvire.move_folder" => move_folder_tool(args, excluded_filename_patterns),
        "reqvire.link" => link_tool(args, excluded_filename_patterns),
        "reqvire.unlink" => unlink_tool(args, excluded_filename_patterns),
        "reqvire.relink" => relink_tool(args, excluded_filename_patterns),
        "reqvire.move_asset" => move_asset_tool(args, excluded_filename_patterns),
        "reqvire.remove_asset" => remove_asset_tool(args, excluded_filename_patterns),
        _ => Err(ReqvireError::ProcessError(format!(
            "Unknown Reqvire tool '{}'",
            name
        ))),
    }
}
