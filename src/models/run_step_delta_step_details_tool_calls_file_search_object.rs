/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject {
    /// For now, this is always going to be an empty object.
    #[serde(rename = "file_search")]
    pub file_search: Value,
    /// The ID of the tool call object.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The index of the tool call in the tool calls array.
    #[serde(rename = "index")]
    pub index: i32,
    /// The type of tool call. This is always going to be `file_search` for this type of tool call.
    #[serde(rename = "type")]
    pub _type: String,
}
