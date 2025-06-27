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
pub struct RealtimeTranscriptionSessionCreateRequest {
    /// The set of items to include in the transcription. Current available items are: - `item.input_audio_transcription.logprobs`
    #[serde(rename = "include")]
    pub include: Option<Vec<String>>,
    /// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`. For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate,  single channel (mono), and little-endian byte order.
    #[serde(rename = "input_audio_format")]
    pub input_audio_format: Option<String>,
    #[serde(rename = "input_audio_noise_reduction")]
    pub input_audio_noise_reduction: Option<crate::models::RealtimeSessionInputAudioNoiseReduction>,
    #[serde(rename = "input_audio_transcription")]
    pub input_audio_transcription:
        Option<crate::models::RealtimeTranscriptionSessionCreateRequestInputAudioTranscription>,
    /// The set of modalities the model can respond with. To disable audio, set this to [\"text\"].
    /// Array can contain enum: - text  - audio
    #[serde(rename = "modalities")]
    pub modalities: Option<Vec<String>>,
    #[serde(rename = "turn_detection")]
    pub turn_detection:
        Option<crate::models::RealtimeTranscriptionSessionCreateRequestTurnDetection>,
}
