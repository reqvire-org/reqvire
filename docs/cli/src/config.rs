pub use reqvire::config::get_excluded_filename_patterns_glob_set;

#[cfg(test)]
mod config_tests {
    use crate::config::get_excluded_filename_patterns_glob_set;

    #[test]
    fn test_build_excluded_patterns() {
        // Build globset to verify it works correctly
        // (The actual patterns will be tested in e2e tests)
        let _globset = get_excluded_filename_patterns_glob_set();
        // Note: Whether files are excluded depends on .gitignore and .reqvireignore
        // This test just verifies the globset builds successfully
    }
}
