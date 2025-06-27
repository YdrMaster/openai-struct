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
pub struct RealtimeClientEventInputAudioBufferAppend {
    /// Base64-encoded audio bytes. This must be in the format specified by the  `input_audio_format` field in the session configuration.
    #[serde(rename = "audio")]
    pub audio: String,
    /// Optional client-generated ID used to identify this event.
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    /// The event type, must be `input_audio_buffer.append`.
    #[serde(rename = "type")]
    pub _type: String,
}
