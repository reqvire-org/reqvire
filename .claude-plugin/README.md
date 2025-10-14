# Reqvire MBSE Plugin for Claude Code

Official Claude Code plugin for [Reqvire](https://github.com/reqvire-org/reqvire) - The AI-Native Requirements As A Code framework for Modern Engineering Teams.

## Overview

This plugin provides specialized agents and commands to enhance your Reqvire development workflow in Claude Code. It includes intelligent agents that understand MBSE methodology, Reqvire's architecture, and best practices for requirements engineering and test-driven development.

## Features

### 🤖 Specialized Agents

#### Requirements Engineer Agent
Expert agent for managing Reqvire specifications and requirements:

- **Model Management**: Analyze and manage project specifications and model structure
- **Create Requirements**: Add new requirements with proper traceability and relations
- **Update Specifications**: Modify existing requirements while maintaining consistency
- **Add Features**: Introduce new feature requirements with complete traceability chain
- **Analyze Relations**: Understand derivedFrom, verify, and satisfiedBy relationships
- **Find Issues**: Identify redundant relations, coverage gaps, and structural problems
- **Change Impact**: Analyze how changes propagate through the requirement hierarchy
- **Report Analysis**: Generate and interpret summaries, traces, coverage, and matrix reports

**Model**: Claude Opus
**When to use**: Any task involving specifications, requirements, verifications, or model analysis

#### Project Bootstrapper Agent
Expert agent for setting up Reqvire in new projects:

- **Detect Setup**: Automatically detects if Reqvire is configured in a project
- **Create Structure**: Sets up specifications/ directory and file structure
- **Copy Templates**: Installs CLAUDE.md guides from plugin templates
- **Guide Installation**: Walks users through Reqvire CLI installation
- **Verify Setup**: Validates that everything is configured correctly
- **Optional Configuration**: Guides users to create .reqvireignore for file exclusions

**Model**: Claude Sonnet
**When to use**: Setting up Reqvire in a new project, or when Reqvire isn't configured yet

**Install Command**: `curl -fsSL https://raw.githubusercontent.com/reqvire-org/reqvire/main/scripts/install.sh | bash`

#### Task Master Agent
Expert agent for analyzing requirement changes and creating implementation plans:

- **Analyze Changes**: Runs change-impact analysis from git commits
- **Trace Requirements**: Identifies new and modified requirements with full traceability
- **Generate Tasks**: Creates comprehensive TodoWrite task plans with checkboxes
- **Link to Source**: Provides git blob links to exact requirement versions
- **Test Mapping**: Maps requirements → verifications → tests for complete workflow
- **Track Progress**: Uses TodoWrite for real-time task tracking
- **Repository-Agnostic**: Creates abstract tasks without implementation assumptions

**Model**: Claude Opus
**When to use**: Creating implementation plans from requirement changes on feature branches

**Workflow**: Compares feature branch against base branch, analyzes changed requirements, generates phased task list with links, tests, and traceability updates

### ⚡ Custom Commands

#### `/find-redundant-verifications`
Analyzes verification traces to identify and report redundant verify relations in your model.

**What it does:**
- Runs `reqvire traces --json` to analyze verification relationships
- Identifies verifications that directly verify both child and parent requirements
- Shows which verify relations can be safely removed (child verification is sufficient)
- Explains why each relation is redundant

**When to use:**
- After adding new requirements that change the hierarchy
- During requirements reviews to simplify the model
- Before major releases to ensure clean traceability
- When verification traces become complex

**Requirements:** `jq` must be installed

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

### Using Agents

Agents are automatically available based on context. Claude Code will use the appropriate agent when you're working on related tasks.

**Explicitly invoke an agent:**
```
Use the requirements-engineer agent to analyze the current model structure
```

**Let Claude Code decide:**
```
Add a new authentication feature with proper requirements and verifications
```

### Using Commands

Commands are available via the `/` prefix:

```bash
# Find redundant verify relations
/find-redundant-verifications
```

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

The agents know how to use these commands effectively:

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
├── plugin.json                        # Plugin manifest
├── marketplace.json                   # Marketplace configuration
├── README.md                          # Plugin documentation
├── agents/
│   ├── requirements-engineer.md       # Requirements management agent
│   ├── project-bootstrapper.md        # Project setup agent
│   └── task-master.md                 # Implementation planning agent
├── commands/
│   └── find-redundant-verifications.md   # Redundancy analysis command
└── templates/
    ├── CLAUDE.md                      # Root development guide template
    └── specifications-CLAUDE.md       # Specifications guide template
```

## Contributing

Contributions to the plugin are welcome! Please:

1. Follow the MBSE methodology (requirements first!)
2. Add appropriate verifications for new features
3. Maintain consistency with existing agent patterns
4. Update documentation as needed

## Support

- **Issues**: https://github.com/reqvire-org/reqvire/issues
- **Documentation**: https://github.com/reqvire-org/reqvire
- **Discussions**: https://github.com/reqvire-org/reqvire/discussions

## License

Same license as the Reqvire project. See the main repository for details.

## Version History

### 1.0.0 (Initial Release)
- Requirements Engineer Agent (Claude Opus) - Manage specifications and requirements
- Project Bootstrapper Agent (Claude Sonnet) - Set up Reqvire in new projects
- Task Master Agent (Claude Opus) - Analyze requirement changes and generate implementation plans
- Find Redundant Verifications Command - Analyze and clean up model
- Template system with CLAUDE.md guides for bootstrapping
- Marketplace support for easy distribution
