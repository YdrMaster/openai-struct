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
pub struct ResponseOutputItemDoneEvent {
    /// The output item that was marked done.
    #[serde(rename = "item")]
    pub item: crate::models::OutputItem,
    /// The index of the output item that was marked done.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The type of the event. Always `response.output_item.done`.
    #[serde(rename = "type")]
    pub _type: String,
}
