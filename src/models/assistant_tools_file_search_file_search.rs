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
pub struct AssistantToolsFileSearchFileSearch {
    /// The maximum number of results the file search tool should output.
    /// The default is 20 for `gpt-4*` models and 5 for `gpt-3.5-turbo`.
    /// This number should be between 1 and 50 inclusive.
    /// Note that the file search tool may output fewer than `max_num_results` results.
    /// See the [file search tool documentation](/docs/assistants/tools/file-search#customizing-file-search-settings) for more information.
    #[serde(rename = "max_num_results")]
    pub max_num_results: Option<i32>,
    #[serde(rename = "ranking_options")]
    pub ranking_options: Option<crate::models::FileSearchRankingOptions>,
}
