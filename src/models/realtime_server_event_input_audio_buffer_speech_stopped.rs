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
pub struct RealtimeServerEventInputAudioBufferSpeechStopped {
    /// Milliseconds since the session started when speech stopped. This will  correspond to the end of audio sent to the model, and thus includes the  `min_silence_duration_ms` configured in the Session.
    #[serde(rename = "audio_end_ms")]
    pub audio_end_ms: i32,
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The ID of the user message item that will be created.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The event type, must be `input_audio_buffer.speech_stopped`.
    #[serde(rename = "type")]
    pub _type: String,
}
