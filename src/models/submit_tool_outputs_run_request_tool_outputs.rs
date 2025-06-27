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
pub struct SubmitToolOutputsRunRequestToolOutputs {
    /// The output of the tool call to be submitted to continue the run.
    #[serde(rename = "output")]
    pub output: Option<String>,
    /// The ID of the tool call in the `required_action` object within the run object the output is being submitted for.
    #[serde(rename = "tool_call_id")]
    pub tool_call_id: Option<String>,
}
