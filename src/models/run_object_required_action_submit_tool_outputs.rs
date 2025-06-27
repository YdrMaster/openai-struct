/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct RunObjectRequiredActionSubmitToolOutputs {
    /// A list of the relevant tool calls.
    #[serde(rename = "tool_calls")]
    pub tool_calls: Vec<crate::models::RunToolCallObject>,
}
