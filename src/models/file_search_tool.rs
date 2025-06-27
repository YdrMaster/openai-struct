/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub FileSearchTool : A tool that searches for relevant content from uploaded files. Learn more about the [file search tool](https://platform.openai.com/docs/guides/tools-file-search).
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSearchTool {
    #[serde(rename = "filters")]
    pub filters: Option<Value>,
    /// The maximum number of results to return. This number should be between 1 and 50 inclusive.
    #[serde(rename = "max_num_results")]
    pub max_num_results: Option<i32>,
    /// Ranking options for search.
    #[serde(rename = "ranking_options")]
    pub ranking_options: Option<crate::models::RankingOptions>,
    /// The type of the file search tool. Always `file_search`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
    /// The IDs of the vector stores to search.
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Vec<String>,
}

fn default_type() -> String {
    "file_search".into()
}
