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
pub struct ResponseAudioTranscriptDeltaEvent {
    /// The partial transcript of the audio response.
    #[serde(rename = "delta")]
    pub delta: String,
    /// The type of the event. Always `response.audio.transcript.delta`.
    #[serde(rename = "type")]
    pub _type: String,
}
