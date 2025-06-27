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
pub struct FileSearchToolCall {
    /// The unique ID of the file search tool call.
    #[serde(rename = "id")]
    pub id: String,
    /// The queries used to search for files.
    #[serde(rename = "queries")]
    pub queries: Vec<String>,
    /// The results of the file search tool call.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::models::FileSearchToolCallResults>>,
    /// The status of the file search tool call. One of `in_progress`,  `searching`, `incomplete` or `failed`,
    #[serde(rename = "status")]
    #[serde(default = "default_type")]
    pub status: String,
    /// The type of the file search tool call. Always `file_search_call`.
    #[serde(rename = "type")]
    pub _type: String,
}

fn default_type() -> String {
    "file_search_call".into()
}
