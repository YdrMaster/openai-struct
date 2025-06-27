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
pub struct RealtimeSessionCreateResponseInputAudioTranscription {
    /// The model to use for transcription, `whisper-1` is the only currently  supported model.
    #[serde(rename = "model")]
    pub model: Option<String>,
}
