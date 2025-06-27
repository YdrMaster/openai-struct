/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunStepDeltaStepDetailsToolCallsObject : Details of the tool call.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsObject {
    /// An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`.
    #[serde(rename = "tool_calls")]
    pub tool_calls: Option<Vec<Value>>,
    /// Always `tool_calls`.
    #[serde(rename = "type")]
    pub _type: String,
}
