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
pub struct ReasoningItem {
    /// The unique identifier of the reasoning content.
    #[serde(rename = "id")]
    pub id: String,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Reasoning text contents.
    #[serde(rename = "summary")]
    pub summary: Vec<crate::models::ReasoningItemSummary>,
    /// The type of the object. Always `reasoning`.
    #[serde(rename = "type")]
    pub _type: String,
}
