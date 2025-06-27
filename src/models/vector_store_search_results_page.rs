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
pub struct VectorStoreSearchResultsPage {
    /// The list of search result items.
    #[serde(rename = "data")]
    pub data: Vec<crate::models::VectorStoreSearchResultItem>,
    /// Indicates if there are more results to fetch.
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// The token for the next page, if any.
    #[serde(rename = "next_page")]
    pub next_page: String,
    /// The object type, which is always `vector_store.search_results.page`
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "search_query")]
    pub search_query: Vec<String>,
}
