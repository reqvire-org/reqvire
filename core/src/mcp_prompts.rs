use crate::error::ReqvireError;
use serde_json::{json, Map, Value};

pub struct McpPromptDefinition {
    pub name: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub arguments: &'static [McpPromptArgumentDefinition],
    pub content: &'static str,
}

pub struct McpPromptArgumentDefinition {
    pub name: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub required: bool,
}

const QUESTION_ARGUMENT: McpPromptArgumentDefinition = McpPromptArgumentDefinition {
    name: "question",
    title: "Question",
    description: "The engineering question or user intent the assistant should answer.",
    required: false,
};

const SCOPE_ARGUMENT: McpPromptArgumentDefinition = McpPromptArgumentDefinition {
    name: "scope",
    title: "Scope",
    description: "Optional element name, capability, requirement, file, or submodel scope.",
    required: false,
};

const CHANGE_ARGUMENT: McpPromptArgumentDefinition = McpPromptArgumentDefinition {
    name: "change",
    title: "Change",
    description: "Optional proposed model or implementation change to analyze.",
    required: false,
};

const QUESTION_SCOPE_ARGUMENTS: &[McpPromptArgumentDefinition] =
    &[QUESTION_ARGUMENT, SCOPE_ARGUMENT];
const QUESTION_CHANGE_ARGUMENTS: &[McpPromptArgumentDefinition] =
    &[QUESTION_ARGUMENT, CHANGE_ARGUMENT];

pub const MCP_PROMPTS: &[McpPromptDefinition] = &[
    McpPromptDefinition {
        name: "reqvire.semantic.query",
        title: "Reqvire Semantic Query",
        description: "Build ontology-aware SPARQL queries using Reqvire semantic vocabulary and prefixes.",
        arguments: QUESTION_SCOPE_ARGUMENTS,
        content: include_str!("mcp_prompts/semantic_query.md"),
    },
    McpPromptDefinition {
        name: "reqvire.semantic.verification_search",
        title: "Reqvire Semantic Verification Search",
        description: "Find verified requirements or capability-scoped verification evidence using semantic query tools.",
        arguments: QUESTION_SCOPE_ARGUMENTS,
        content: include_str!("mcp_prompts/semantic_verification_search.md"),
    },
    McpPromptDefinition {
        name: "reqvire.semantic.contract_context_search",
        title: "Reqvire Semantic Contract Context Search",
        description: "Find cross-subgraph reused contract context and semantic-contract constraints.",
        arguments: QUESTION_SCOPE_ARGUMENTS,
        content: include_str!("mcp_prompts/semantic_contract_context_search.md"),
    },
    McpPromptDefinition {
        name: "reqvire.workflow.explore_model",
        title: "Reqvire Model Exploration",
        description: "Explore a Reqvire model with regular read-only MCP tools before answering.",
        arguments: QUESTION_SCOPE_ARGUMENTS,
        content: include_str!("mcp_prompts/workflow_explore_model.md"),
    },
    McpPromptDefinition {
        name: "reqvire.workflow.plan_change",
        title: "Reqvire Change Planning",
        description: "Plan a requirement, ontology, verification, or implementation change using Reqvire evidence.",
        arguments: QUESTION_CHANGE_ARGUMENTS,
        content: include_str!("mcp_prompts/workflow_plan_change.md"),
    },
    McpPromptDefinition {
        name: "reqvire.workflow.verify_coverage",
        title: "Reqvire Verification Coverage Review",
        description: "Review validation, lint, coverage, and verification trace evidence for a Reqvire scope.",
        arguments: QUESTION_SCOPE_ARGUMENTS,
        content: include_str!("mcp_prompts/workflow_verify_coverage.md"),
    },
];

pub fn prompt_definitions_json() -> Vec<Value> {
    MCP_PROMPTS.iter().map(prompt_definition_json).collect()
}

pub fn prompt_get_result_json(
    name: &str,
    arguments: Option<&Map<String, Value>>,
) -> Result<Value, ReqvireError> {
    let definition = MCP_PROMPTS
        .iter()
        .find(|definition| definition.name == name)
        .ok_or_else(|| ReqvireError::InvalidOperation(format!("Unknown MCP prompt '{}'", name)))?;
    let text = render_prompt_content(definition, arguments);
    Ok(json!({
        "description": definition.description,
        "messages": [
            {
                "role": "user",
                "content": {
                    "type": "text",
                    "text": text
                }
            }
        ]
    }))
}

fn prompt_definition_json(definition: &McpPromptDefinition) -> Value {
    json!({
        "name": definition.name,
        "title": definition.title,
        "description": definition.description,
        "arguments": definition.arguments.iter().map(prompt_argument_json).collect::<Vec<_>>()
    })
}

fn prompt_argument_json(argument: &McpPromptArgumentDefinition) -> Value {
    json!({
        "name": argument.name,
        "title": argument.title,
        "description": argument.description,
        "required": argument.required
    })
}

fn render_prompt_content(
    definition: &McpPromptDefinition,
    arguments: Option<&Map<String, Value>>,
) -> String {
    let mut text = String::from(definition.content.trim());
    if let Some(arguments) = arguments {
        if !arguments.is_empty() {
            let rendered = serde_json::to_string_pretty(arguments)
                .unwrap_or_else(|_| Value::Object(arguments.clone()).to_string());
            text.push_str("\n\nClient arguments:\n```json\n");
            text.push_str(&rendered);
            text.push_str("\n```");
        }
    }
    text
}
