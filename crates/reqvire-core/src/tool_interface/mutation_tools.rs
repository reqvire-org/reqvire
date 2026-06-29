use super::*;

pub(crate) fn add_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::add_element(
        &mut model,
        &required_string_arg(args, "content")?,
        &required_string_arg(args, "file")?,
        excluded_filename_patterns,
        &current_dir_path(),
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
        bool_arg(args, "override_existing", false),
    )?;
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}

pub(crate) fn remove_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let element_id = model
        .graph_registry
        .find_element_by_name(&required_string_arg(args, "element_name")?)?;
    let result = crud::remove_element(
        &mut model,
        &element_id,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}

pub(crate) fn move_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let element_id = model
        .graph_registry
        .find_element_by_name(&required_string_arg(args, "element_name")?)?;
    let result = crud::move_element(
        &mut model,
        &element_id,
        &required_string_arg(args, "file")?,
        excluded_filename_patterns,
        &current_dir_path(),
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}

pub(crate) fn rename_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let element_id = model
        .graph_registry
        .find_element_by_name(&required_string_arg(args, "element_name")?)?;
    let result = crud::rename_element(
        &mut model,
        &element_id,
        &required_string_arg(args, "new_name")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}

pub(crate) fn merge_elements_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::merge_elements(
        &mut model,
        &required_string_arg(args, "target")?,
        &string_array_arg(args, "sources")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}

pub(crate) fn move_file_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::move_file(
        &mut model,
        &required_string_arg(args, "source_file")?,
        &required_string_arg(args, "target_file")?,
        &current_dir_path(),
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
        bool_arg(args, "squash", false),
    )?;
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}

pub(crate) fn link_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let source = required_string_arg(args, "source")?;
    let relation_type = required_string_arg(args, "relation_type")?;
    let target = required_string_arg(args, "target")?;
    let git_root = git_commands::get_git_root_dir()?;
    let result = if relation_type == "bindContract" {
        if crate::utils::is_external_url(&target) {
            return Err(ReqvireError::ProcessError(
                "External URLs cannot be reused as contract context. Use a semantically specific relation only when the URL is valid evidence for that relation."
                    .to_string(),
            ));
        }
        crud::reuse_contract_element_identifier(
            &mut model,
            &source,
            &target,
            &git_root,
            bool_arg(args, "dry_run", false),
        )?
    } else {
        crud::link(
            &mut model,
            &source,
            &relation_type,
            &target,
            &git_root,
            bool_arg(args, "dry_run", false),
        )?
    };
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}

pub(crate) fn unlink_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::unlink(
        &mut model,
        &required_string_arg(args, "source")?,
        &required_string_arg(args, "target")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}

pub(crate) fn relink_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::relink(
        &mut model,
        &required_string_arg(args, "source")?,
        &required_string_arg(args, "relation_type")?,
        &required_string_arg(args, "from_target")?,
        &required_string_arg(args, "to_target")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}

pub(crate) fn move_asset_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::mv_asset(
        &mut model,
        &required_string_arg(args, "old_path")?,
        &required_string_arg(args, "new_path")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}

pub(crate) fn remove_asset_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::rm_asset(
        &mut model,
        &required_string_arg(args, "file_path")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    if !bool_arg(args, "dry_run", false) {
        crate::model_cache::invalidate();
    }
    parse_json_string(render_crud_json(&result))
}
