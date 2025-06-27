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
pub struct RunStepDetailsToolCallsFileSearchRankingOptionsObject {
    #[serde(rename = "ranker")]
    pub ranker: crate::models::FileSearchRanker,
    /// The score threshold for the file search. All values must be a floating point number between 0 and 1.
    #[serde(rename = "score_threshold")]
    pub score_threshold: f32,
}
