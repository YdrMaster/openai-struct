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
pub struct ResponseReasoningSummaryPartAddedEvent {
    /// The ID of the item this summary part is associated with.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The index of the output item this summary part is associated with.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    #[serde(rename = "part")]
    pub part: crate::models::ResponseReasoningSummaryPartAddedEventPart,
    /// The index of the summary part within the reasoning summary.
    #[serde(rename = "summary_index")]
    pub summary_index: i32,
    /// The type of the event. Always `response.reasoning_summary_part.added`.
    #[serde(rename = "type")]
    pub _type: String,
}
