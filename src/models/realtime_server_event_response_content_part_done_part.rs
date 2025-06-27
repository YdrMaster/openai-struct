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
pub struct RealtimeServerEventResponseContentPartDonePart {
    /// Base64-encoded audio data (if type is \"audio\").
    #[serde(rename = "audio")]
    pub audio: Option<String>,
    /// The text content (if type is \"text\").
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// The transcript of the audio (if type is \"audio\").
    #[serde(rename = "transcript")]
    pub transcript: Option<String>,
    /// The content type (\"text\", \"audio\").
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
