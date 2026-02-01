# Elements

### CLI Git Commit Hash Flag Test

This test verifies that the system properly handles the git commit hash flag for change impact analysis.

#### Details

##### Acceptance Criteria
- System shall support --git-commit flag for change impact analysis
- System shall use specified commit hash as base for comparison
- System shall default to HEAD when flag is not specified
- System shall handle relative commit references (HEAD~1, etc.)

##### Test Criteria
- Command with explicit --git-commit flag runs successfully
- Command without flag defaults to HEAD commit
- Relative commit references are correctly resolved
- Invalid commit references are reported appropriately
- Change impact analysis correctly uses specified commit as baseline

##### Test Procedure
1. Create test fixtures with git repository containing multiple commits
2. Run Reqvire with --change-impact --git-commit=HEAD~1
3. Verify that the specified commit is used as baseline
4. Run Reqvire with --change-impact (no git-commit flag)
5. Verify that HEAD is used as default baseline
6. Run with invalid commit reference and verify appropriate error

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-detection/test.sh)
  * verify: [CLI Change Impact Report Command](../Commands.md#cli-change-impact-report-command)
---

### CLI Help Structure Verification

This test verifies that the CLI help output displays all commands and their options flattened in the main help, and that all CLI command requirements are covered.

#### Details

##### Acceptance Criteria
- Running `reqvire` (without arguments) displays the main help output as the default command
- Main help output shows Usage line: `reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]`
- Main help lists all available commands under "Commands:" section
- Main help lists all global options under "Options:" section
- Command-specific options are displayed under dedicated headings (e.g., "SUMMARY OPTIONS:", "FORMAT OPTIONS:")
- All command options are flattened and visible in the main help without needing to drill down
- Help output includes nested subcommands expanded (e.g., "traces", "coverage")
- Each command's help text is descriptive and clear

##### Test Criteria
1. **Main help output structure**
   Command: `reqvire`
   - exits code **0**
   - output contains "Usage: reqvire [OPTIONS] <COMMAND> [COMMAND OPTIONS]"
   - output contains "Commands:" section
   - output contains "Options:" section with `-h, --help`, `-V, --version`

2. **Command listing completeness**
   - All CLI commands are listed: html, format, validate, summary, section-summary, change-impact, verifications
   - Nested commands are expanded: "traces", "coverage"

3. **Options flattening - all command-specific option sections present**
   - FORMAT OPTIONS section visible
   - SUMMARY OPTIONS section visible
   - SECTION-SUMMARY OPTIONS section visible
   - CHANGE IMPACT OPTIONS section visible
   - VERIFICATIONS TRACES OPTIONS section visible
   - VERIFICATIONS COVERAGE OPTIONS section visible

4. **Help text quality**
   - Each command has descriptive help text
   - Options have clear descriptions

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-cli-help-structure/test.sh)
  * verify: [CLI Change Impact Report Command](../Commands.md#cli-change-impact-report-command)
  * verify: [CLI Coverage Command](../Commands.md#cli-coverage-command)
  * verify: [CLI JSON File Output Option](../Commands.md#cli-json-file-output-option)
  * verify: [CLI Search Command](../Commands.md#cli-search-command)
  * verify: [CLI Traces Command](../Commands.md#cli-traces-command)
  * verify: [Format Command](../Commands.md#format-command)
  * verify: [Validate Command](../Commands.md#validate-command)
  * verify: [HTML Export](../../WebInterface/Features.md#html-export)
---

### Verification Traces Element Navigation Test

Test verifies that verification element names in the traces report are clickable links.

#### Test Steps
1. Run `reqvire traces` command to generate traces report
2. Verify output contains verification headers as markdown links
3. Verify links follow format `[Verification Name](file_path#fragment)`

#### Expected Results
- Verification names are rendered as markdown links
- Links point to source file with verification fragment

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-verification-traces/test.sh)
  * verify: [Verification Traces Element Navigation](../Commands.md#verification-traces-element-navigation)
---
