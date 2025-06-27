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
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted {
    /// The index of the content part containing the audio.
    #[serde(rename = "content_index")]
    pub content_index: i32,
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The ID of the user message item containing the audio.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The log probabilities of the transcription.
    #[serde(rename = "logprobs")]
    pub logprobs: Option<Vec<crate::models::LogProbProperties>>,
    /// The transcribed text.
    #[serde(rename = "transcript")]
    pub transcript: String,
    /// The event type, must be `conversation.item.input_audio_transcription.completed`.
    #[serde(rename = "type")]
    pub _type: String,
}
