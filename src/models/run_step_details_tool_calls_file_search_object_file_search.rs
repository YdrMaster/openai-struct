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
pub struct RunStepDetailsToolCallsFileSearchObjectFileSearch {
    #[serde(rename = "ranking_options")]
    pub ranking_options:
        Option<crate::models::RunStepDetailsToolCallsFileSearchRankingOptionsObject>,
    /// The results of the file search.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::models::RunStepDetailsToolCallsFileSearchResultObject>>,
}
