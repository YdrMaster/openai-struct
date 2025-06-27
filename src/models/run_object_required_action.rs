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
pub struct RunObjectRequiredAction {
    #[serde(rename = "submit_tool_outputs")]
    pub submit_tool_outputs: crate::models::RunObjectRequiredActionSubmitToolOutputs,
    /// For now, this is always `submit_tool_outputs`.
    #[serde(rename = "type")]
    pub _type: String,
}
