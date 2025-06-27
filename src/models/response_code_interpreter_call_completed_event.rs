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
pub struct ResponseCodeInterpreterCallCompletedEvent {
    #[serde(rename = "code_interpreter_call")]
    pub code_interpreter_call: crate::models::CodeInterpreterToolCall,
    /// The index of the output item that the code interpreter call is in progress.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The type of the event. Always `response.code_interpreter_call.completed`.
    #[serde(rename = "type")]
    pub _type: String,
}
