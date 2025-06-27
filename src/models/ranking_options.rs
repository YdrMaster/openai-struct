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
pub struct RankingOptions {
    /// The ranker to use for the file search.
    #[serde(rename = "ranker")]
    pub ranker: Option<String>,
    /// The score threshold for the file search, a number between 0 and 1. Numbers closer to 1 will attempt to return only the most relevant results, but may return fewer results.
    #[serde(rename = "score_threshold")]
    pub score_threshold: Option<f32>,
}
