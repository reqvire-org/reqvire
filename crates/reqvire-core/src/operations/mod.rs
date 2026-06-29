//! Shared read/report operations used by the CLI and MCP tool layer.

use crate::change_impact;
use crate::containment::ContainmentHierarchy;
use crate::element::Element;
use crate::error::ReqvireError;
use crate::format::{format_files, render_diff_json, FormatResult};
use crate::git_commands;
use crate::graph_registry::GraphRegistry;
use crate::lint::{self, LintReport};
use crate::model_cache;
use crate::report;
use crate::search;
use crate::verification_trace::{self, VerificationTracesReport};
use crate::{ModelBuildOptions, ModelManager};
use globset::GlobSet;
use serde_json::Value;

pub fn load_model(excluded_filename_patterns: &GlobSet) -> Result<ModelManager, ReqvireError> {
    load_model_with_options(excluded_filename_patterns, false)
}

pub fn load_model_with_options(
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<ModelManager, ReqvireError> {
    model_cache::load_cached_model(
        excluded_filename_patterns,
        ModelBuildOptions {
            lenient: false,
            with_size_estimates,
        },
    )
}

pub fn load_model_lenient(
    excluded_filename_patterns: &GlobSet,
) -> Result<ModelManager, ReqvireError> {
    model_cache::load_cached_model(
        excluded_filename_patterns,
        ModelBuildOptions {
            lenient: true,
            with_size_estimates: false,
        },
    )
}

pub fn read_element<'a>(
    registry: &'a GraphRegistry,
    identifier: Option<&str>,
    name: Option<&str>,
) -> Result<&'a Element, ReqvireError> {
    if identifier.is_none() && name.is_none() {
        return Err(ReqvireError::ProcessError(
            "read_element requires 'identifier' or 'name'".to_string(),
        ));
    }

    if let Some(identifier) = identifier {
        registry
            .get_element(identifier)
            .ok_or_else(|| ReqvireError::ElementNotFound("Element not found".to_string()))
    } else {
        registry
            .get_element_by_name(name.expect("checked above"))
            .ok_or_else(|| ReqvireError::ElementNotFound("Element not found".to_string()))
    }
}

pub fn search_report(
    registry: &GraphRegistry,
    filters: &search::SearchFilters,
    json_output: bool,
    short: bool,
) -> Result<String, ReqvireError> {
    search::generate_search_report(registry, filters, json_output, short)
}

pub fn model_report(
    registry: &GraphRegistry,
    from: Option<&str>,
    reverse: bool,
    type_filter: Option<Vec<&str>>,
) -> Result<String, ReqvireError> {
    report::model::generate_model_report(registry, from, reverse, type_filter)
}

pub fn collect_report(
    registry: &GraphRegistry,
    element_name: &str,
    json_output: bool,
    direction: report::collect::CollectDirection,
) -> Result<String, ReqvireError> {
    let git_root = git_commands::get_git_root_dir()?;
    report::collect::generate_collect_report(
        registry,
        element_name,
        &git_root,
        json_output,
        direction,
    )
}

pub fn submodels_report(
    registry: &GraphRegistry,
    from: Option<&str>,
) -> Result<report::submodels::SubmodelsReport, ReqvireError> {
    report::submodels::generate_submodels_report(registry, from)
}

pub fn resources_report(registry: &GraphRegistry) -> report::resources::ResourcesReport {
    report::resources::generate_resources_report(registry)
}

pub fn coverage_report(registry: &GraphRegistry) -> report::coverage::CoverageReport {
    report::coverage::generate_coverage_report(registry)
}

pub fn traces_report(
    registry: &GraphRegistry,
    filter_id: Option<&str>,
    filter_name: Option<&str>,
    filter_type: Option<&str>,
) -> Result<VerificationTracesReport, ReqvireError> {
    let generator = verification_trace::VerificationTraceGenerator::new(registry);
    let mut report = generator.generate();
    if filter_id.is_some() || filter_name.is_some() || filter_type.is_some() {
        report = verification_trace::apply_filters(report, filter_id, filter_name, filter_type)?;
    }
    Ok(report)
}

pub fn lint_report(registry: &GraphRegistry) -> LintReport {
    lint::analyze_model(registry)
}

pub fn format_report(
    registry: &GraphRegistry,
    fix: bool,
    with_full_relations: bool,
) -> Result<String, ReqvireError> {
    let result = format_diff(registry, fix, with_full_relations)?;
    Ok(render_diff_json(&result))
}

pub fn format_diff(
    registry: &GraphRegistry,
    fix: bool,
    with_full_relations: bool,
) -> Result<FormatResult, ReqvireError> {
    let result = format_files(registry, !fix, with_full_relations)?;
    if fix {
        model_cache::invalidate();
    }
    Ok(result)
}

pub fn containment_hierarchy(
    registry: &GraphRegistry,
    short: bool,
) -> Result<ContainmentHierarchy, ReqvireError> {
    ContainmentHierarchy::build(registry, short)
}

pub fn change_impact_report(
    registry: &GraphRegistry,
    git_commit: &str,
    excluded_filename_patterns: &GlobSet,
) -> Result<(change_impact::ChangeImpactReport, String, String), ReqvireError> {
    let mut reference_model = ModelManager::new();
    match reference_model.parse_and_validate_with_mode(
        Some(git_commit),
        excluded_filename_patterns,
        false,
    ) {
        Ok(_) => {}
        Err(ReqvireError::ValidationError(_)) | Err(ReqvireError::ValidationDiagnostics { .. }) => {
            reference_model.parse_and_validate_with_mode(
                Some(git_commit),
                excluded_filename_patterns,
                true,
            )?;
        }
        Err(err) => return Err(err),
    }

    let base_url = git_commands::get_repository_base_url()?;
    let current_commit = git_commands::get_commit_hash()?;
    let report = change_impact::compute_change_impact(registry, &reference_model.graph_registry)
        .map_err(|err| ReqvireError::ProcessError(format!("{:?}", err)))?;

    Ok((report, base_url, current_commit))
}

pub fn value_from_json_string(json_str: String) -> Result<Value, ReqvireError> {
    serde_json::from_str(&json_str).map_err(ReqvireError::from)
}
