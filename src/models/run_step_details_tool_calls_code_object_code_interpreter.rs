/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunStepDetailsToolCallsCodeObjectCodeInterpreter : The Code Interpreter tool call definition.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsCodeObjectCodeInterpreter {
    /// The input to the Code Interpreter tool call.
    #[serde(rename = "input")]
    pub input: String,
    /// The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type.
    #[serde(rename = "outputs")]
    pub outputs: Vec<Value>,
}
