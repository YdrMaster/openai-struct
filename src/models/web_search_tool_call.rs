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
pub struct WebSearchToolCall {
    /// The unique ID of the web search tool call.
    #[serde(rename = "id")]
    pub id: String,
    /// The status of the web search tool call.
    #[serde(rename = "status")]
    pub status: String,
    /// The type of the web search tool call. Always `web_search_call`.
    #[serde(rename = "type")]
    pub _type: String,
}
