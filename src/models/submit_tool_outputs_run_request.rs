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
pub struct SubmitToolOutputsRunRequest {
    /// If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
    /// A list of tools for which the outputs are being submitted.
    #[serde(rename = "tool_outputs")]
    pub tool_outputs: Vec<crate::models::SubmitToolOutputsRunRequestToolOutputs>,
}
