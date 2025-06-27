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
pub struct WebSearchPreviewTool {
    /// High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default.
    #[serde(rename = "search_context_size")]
    pub search_context_size: Option<String>,
    /// The type of the web search tool. One of `web_search_preview` or `web_search_preview_2025_03_11`.
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "user_location")]
    pub user_location: Option<crate::ApproximateLocation>,
}
