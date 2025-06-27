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
pub struct ResponseReasoningSummaryTextDeltaEvent {
    /// The text delta that was added to the summary.
    #[serde(rename = "delta")]
    pub delta: String,
    /// The ID of the item this summary text delta is associated with.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The index of the output item this summary text delta is associated with.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The index of the summary part within the reasoning summary.
    #[serde(rename = "summary_index")]
    pub summary_index: i32,
    /// The type of the event. Always `response.reasoning_summary_text.delta`.
    #[serde(rename = "type")]
    pub _type: String,
}
