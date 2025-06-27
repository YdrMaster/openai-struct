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
pub struct RunStepDetailsToolCallsFileSearchResultObject {
    /// The content of the result that was found. The content is only included if requested via the include query parameter.
    #[serde(rename = "content")]
    pub content: Option<Vec<crate::models::RunStepDetailsToolCallsFileSearchResultObjectContent>>,
    /// The ID of the file that result was found in.
    #[serde(rename = "file_id")]
    pub file_id: String,
    /// The name of the file that result was found in.
    #[serde(rename = "file_name")]
    pub file_name: String,
    /// The score of the result. All values must be a floating point number between 0 and 1.
    #[serde(rename = "score")]
    pub score: f32,
}
