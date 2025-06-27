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
pub struct InputAudio {
    /// Base64-encoded audio data.
    #[serde(rename = "data")]
    pub data: String,
    /// The format of the audio data. Currently supported formats are `mp3` and `wav`.
    #[serde(rename = "format")]
    pub format: String,
    /// The type of the input item. Always `input_audio`.
    #[serde(rename = "type")]
    pub _type: String,
}
