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
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
    /// The index of the output in the outputs array.
    #[serde(rename = "index")]
    pub index: i32,
    /// The text output from the Code Interpreter tool call.
    #[serde(rename = "logs")]
    pub logs: Option<String>,
    /// Always `logs`.
    #[serde(rename = "type")]
    pub _type: String,
}
