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
pub struct RunToolCallObject {
    #[serde(rename = "function")]
    pub function: crate::models::RunToolCallObjectFunction,
    /// The ID of the tool call. This ID must be referenced when you submit the tool outputs in using the [Submit tool outputs to run](/docs/api-reference/runs/submitToolOutputs) endpoint.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of tool call the output is required for. For now, this is always `function`.
    #[serde(rename = "type")]
    pub _type: String,
}
