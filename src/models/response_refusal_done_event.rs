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
pub struct ResponseRefusalDoneEvent {
    /// The index of the content part that the refusal text is finalized.
    #[serde(rename = "content_index")]
    pub content_index: i32,
    /// The ID of the output item that the refusal text is finalized.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The index of the output item that the refusal text is finalized.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The refusal text that is finalized.
    #[serde(rename = "refusal")]
    pub refusal: String,
    /// The type of the event. Always `response.refusal.done`.
    #[serde(rename = "type")]
    pub _type: String,
}
