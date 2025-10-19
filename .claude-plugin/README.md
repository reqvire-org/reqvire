# Reqvire MBSE Plugin for Claude Code

Official Claude Code plugin for [Reqvire](https://github.com/reqvire-org/reqvire) - The AI-Native Requirements As A Code framework for Modern Engineering Teams.

## Overview

This plugin provides specialized skills and commands for managing any project using systems engineering with Reqvire-adjusted agile methodology. The skills understand MBSE principles, requirements-as-code workflows, and how to leverage Reqvire for modern engineering teams.

## Features

### 🎯 Specialized Skills

#### System and Requirements Engineer Skill (`/syseng`)
Expert skill for managing any project's specifications and requirements using Reqvire:

- **Model Management**: Analyze and manage specifications and model structure
- **Create Requirements**: Add requirements with proper MBSE traceability
- **Update Specifications**: Modify requirements while maintaining consistency
- **Add Features**: Introduce new features following systems engineering principles
- **Analyze Relations**: Understand hierarchical and verification relationships
- **Find Issues**: Identify model quality issues and improvement opportunities
- **Change Impact**: Analyze how changes propagate through requirement hierarchies
- **Report Analysis**: Generate and interpret Reqvire reports (summary, traces, coverage, matrix)
- **MBSE Workflows**: Support requirements-as-code and agile MBSE methodologies

**When to use**: Managing specifications, requirements, verifications, or performing model analysis

**Invoke with**: `/syseng`

#### Task Master Skill (`/task-master`)
Expert skill for analyzing requirement changes and creating implementation plans:

- **Analyze Changes**: Runs Reqvire change-impact analysis from git commits
- **Trace Requirements**: Identifies new and modified requirements with full traceability
- **Generate Tasks**: Creates comprehensive TodoWrite task plans with checkboxes
- **Link to Source**: Provides git blob links to exact requirement versions
- **Test Mapping**: Maps requirements → verifications → tests for complete workflow
- **Track Progress**: Uses TodoWrite for real-time task tracking
- **Repository-Agnostic**: Creates abstract tasks without technology assumptions

**When to use**: Creating implementation plans from requirement changes on feature branches

**Invoke with**: `/task-master`

**Workflow**: Compares feature branch against base, analyzes changed requirements, generates phased task list with links, tests, and traceability updates

### ⚡ Commands

Commands provide focused functionality for specific tasks. Skills orchestrate these commands.

#### Analysis and Reporting

**`/analyze-model`** - Analyze model structure, identify issues, and provide recommendations
- Runs validation, summary, coverage, and lint checks
- Identifies errors, warnings, and improvement opportunities
- Provides prioritized action items

**`/analyze-coverage`** - Check verification coverage and identify gaps
- Analyzes verification coverage percentages
- Identifies unverified leaf requirements
- Distinguishes between leaf requirements (need verification) and parents (inherit coverage)

**`/analyze-impact`** - Analyze change impact from git commits
- Shows added, modified, and affected elements
- Identifies propagation through relations
- Recommends verification and implementation updates

**`/lint-model`** - Fix issues and identify items needing review
- Automatically fixes syntax and redundant verify relations
- Identifies hierarchical relations that may need manual review
- Always safe to run `reqvire lint --fix`

#### Requirements and Verifications

**`/add-requirement`** - Add new requirement with proper structure
- Follows ears patterns and MBSE best practices
- Links to parent requirements via derivedFrom
- Checks if verification needed (leaf vs parent)

**`/add-verification`** - Add verification for existing requirement
- Checks if verification needed based on hierarchy
- Reads all requirements in trace chain for comprehensive test criteria
- Links to tests for test-verifications only

**`/add-feature`** - Add complete feature (orchestrates other commands)
- Creates parent and leaf requirements
- Adds verifications for leaf requirements
- Validates and cleans up model

#### Task Planning

**`/generate-tasks`** - Generate implementation task plan from requirement changes
- Detects base branch and runs change-impact analysis
- Creates TodoWrite task plan with git blob links
- Maps requirements → verifications → tests

#### Utility

**`/find-redundant-verifications`** - Find redundant verify relations
- Analyzes verification traces
- Identifies verifications verifying both leaf and parent
- Shows which relations can be safely removed
- Requires `jq` to be installed

## Installation

### Method 1: From Local Repository (Recommended for Contributors)

If you're working in the Reqvire repository:

```bash
# Navigate to your Reqvire directory
cd /path/to/reqvire

# The plugin is already configured!
# Claude Code will automatically detect the plugin when you trust the repository
```

### Method 2: Via Marketplace URL

Add the Reqvire marketplace to Claude Code:

```bash
# In Claude Code, add the marketplace
/plugin marketplace add reqvire-org/reqvire

# Browse available plugins
/plugin marketplace browse

# Install the Reqvire MBSE Plugin
/plugin install reqvire
```

### Method 3: From Git Repository

```bash
# Install directly from the repository
/plugin install https://github.com/reqvire-org/reqvire
```

### Method 4: Team/Organization Setup

For teams, add to your repository's `.claude/settings.json`:

```json
{
  "plugins": [
    {
      "source": "https://github.com/reqvire-org/reqvire",
      "name": "reqvire"
    }
  ]
}
```

## Usage

### Using Skills

Skills are invoked using the `/` prefix followed by the skill name:

```bash
# Invoke System and Requirements Engineer skill
/syseng

# Invoke Task Master skill
/task-master
```

**Example workflows:**
```
/syseng analyze the current model structure and identify coverage gaps
/syseng add a new authentication feature with proper requirements and verifications
/task-master generate implementation plan for this feature branch
```

### Using Commands

Commands are available via the `/` prefix:

```bash
# Analysis commands
/analyze-model
/analyze-coverage
/analyze-impact
/lint-model

# Requirements and verifications
/add-requirement
/add-verification
/add-feature

# Task planning
/generate-tasks

# Utility
/find-redundant-verifications
```

## Updating the Plugin

Once installed, update the plugin to get the latest features and fixes:

```bash
# Update the Reqvire plugin
/plugin update reqvire

# Or update all plugins
/plugin update
```

**For local repository users**: Simply pull the latest changes from git - Claude Code will automatically use the updated plugin files.

## Documentation

The plugin works in conjunction with comprehensive CLAUDE.md guides:

- **[CLAUDE.md](CLAUDE.md)** - Main development guide covering system overview, MBSE workflow, and core architecture
- **[specifications/CLAUDE.md](specifications/CLAUDE.md)** - Guide for writing and editing specifications, requirements, and verifications
- **[tests/CLAUDE.md](tests/CLAUDE.md)** - Guide for writing and executing end-to-end tests

These files provide context to Claude Code automatically when working in the repository.

## MBSE Workflow

This plugin enforces Reqvire's MBSE development methodology:

1. **Requirements First**: Define requirements before implementation
2. **Verifications**: Specify how features will be verified
3. **Tests**: Create tests that satisfy verifications
4. **Implementation**: Implement code with proper traceability
5. **Satisfaction Links**: Link implementations to requirements

**Never skip the requirements step** - this plugin helps ensure proper MBSE workflow.

## Key Reqvire Commands

The skills know how to use these commands effectively:

```bash
# Validate specifications
./target/debug/reqvire validate [--json]

# Generate model summary
./target/debug/reqvire summary [--json]

# Analyze change impact
./target/debug/reqvire change-impact --git-commit=<hash> [--json]

# Generate verification traces
./target/debug/reqvire traces [--json]

# Generate coverage report
./target/debug/reqvire coverage [--json]

# Lint and fix model issues
./target/debug/reqvire lint [--fix]

# Generate traceability matrix
./target/debug/reqvire matrix

# Export HTML documentation
./target/debug/reqvire export --output <dir>

# Serve documentation
./target/debug/reqvire serve --port 8080
```

## Requirements

- **Claude Code**: Latest version with plugin support
- **Reqvire**: Built from source (`cargo build`)
- **jq**: Required for `/find-redundant-verifications` command
  - Mac: `brew install jq`
  - Linux: `sudo apt-get install jq` or `sudo yum install jq`

## Plugin Structure

```
.claude-plugin/
├── plugin.json                           # Plugin manifest
├── marketplace.json                      # Marketplace configuration
├── README.md                             # Plugin documentation
├── skills/
│   ├── syseng.md                         # System and Requirements Engineer skill
│   └── task-master.md                    # Task planning skill
├── commands/
│   └── find-redundant-verifications.md   # Redundancy analysis command
└── templates/
    ├── CLAUDE.md                         # Root development guide template
    └── specifications-CLAUDE.md          # Specifications guide template
```

## Contributing

Contributions to the plugin are welcome! Please:

1. Follow the MBSE methodology (requirements first!)
2. Add appropriate verifications for new features
3. Maintain consistency with existing skill patterns
4. Update documentation as needed

## Support

- **Issues**: https://github.com/reqvire-org/reqvire/issues
- **Documentation**: https://github.com/reqvire-org/reqvire
- **Discussions**: https://github.com/reqvire-org/reqvire/discussions

## License

Same license as the Reqvire project. See the main repository for details.

## Version History

### 1.0.0 (Initial Release)
- System and Requirements Engineer Skill (`/syseng`) - Manage specifications and requirements for any project
- Task Master Skill (`/task-master`) - Analyze requirement changes and generate implementation plans
- Find Redundant Verifications Command - Analyze and clean up model
- Template system with CLAUDE.md guides for bootstrapping
- Marketplace support for easy distribution
