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
pub struct CreateTranslationResponseVerboseJson {
    /// The duration of the input audio.
    #[serde(rename = "duration")]
    pub duration: f32,
    /// The language of the output translation (always `english`).
    #[serde(rename = "language")]
    pub language: String,
    /// Segments of the translated text and their corresponding details.
    #[serde(rename = "segments")]
    pub segments: Option<Vec<crate::models::TranscriptionSegment>>,
    /// The translated text.
    #[serde(rename = "text")]
    pub text: String,
}
