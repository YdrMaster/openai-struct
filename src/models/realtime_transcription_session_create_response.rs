/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeTranscriptionSessionCreateResponse : A new Realtime transcription session configuration.  When a session is created on the server via REST API, the session object also contains an ephemeral key. Default TTL for keys is one minute. This  property is not present when a session is updated via the WebSocket API.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateResponse {
    #[serde(rename = "client_secret")]
    pub client_secret: crate::models::RealtimeTranscriptionSessionCreateResponseClientSecret,
    /// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    #[serde(rename = "input_audio_format")]
    pub input_audio_format: Option<String>,
    #[serde(rename = "input_audio_transcription")]
    pub input_audio_transcription:
        Option<crate::models::RealtimeTranscriptionSessionCreateResponseInputAudioTranscription>,
    /// The set of modalities the model can respond with. To disable audio, set this to [\"text\"].
    #[serde(rename = "modalities")]
    pub modalities: Option<Value>,
    #[serde(rename = "turn_detection")]
    pub turn_detection: Option<crate::models::RealtimeSessionCreateResponseTurnDetection>,
}
