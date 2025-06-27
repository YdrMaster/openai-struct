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
pub struct TranscriptionWord {
    /// End time of the word in seconds.
    #[serde(rename = "end")]
    pub end: f32,
    /// Start time of the word in seconds.
    #[serde(rename = "start")]
    pub start: f32,
    /// The text content of the word.
    #[serde(rename = "word")]
    pub word: String,
}
