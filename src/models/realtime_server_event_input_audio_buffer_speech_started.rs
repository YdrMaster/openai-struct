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
pub struct RealtimeServerEventInputAudioBufferSpeechStarted {
    /// Milliseconds from the start of all audio written to the buffer during the  session when speech was first detected. This will correspond to the  beginning of audio sent to the model, and thus includes the  `prefix_padding_ms` configured in the Session.
    #[serde(rename = "audio_start_ms")]
    pub audio_start_ms: i32,
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The ID of the user message item that will be created when speech stops.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The event type, must be `input_audio_buffer.speech_started`.
    #[serde(rename = "type")]
    pub _type: String,
}
