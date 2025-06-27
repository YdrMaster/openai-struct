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
pub struct OutputAudio {
    /// Base64-encoded audio data from the model.
    #[serde(rename = "data")]
    pub data: String,
    /// The transcript of the audio data from the model.
    #[serde(rename = "transcript")]
    pub transcript: String,
    /// The type of the output audio. Always `output_audio`.
    #[serde(rename = "type")]
    pub _type: String,
}
