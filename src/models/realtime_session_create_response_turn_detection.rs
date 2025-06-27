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
pub struct RealtimeSessionCreateResponseTurnDetection {
    /// Amount of audio to include before the VAD detected speech (in  milliseconds). Defaults to 300ms.
    #[serde(rename = "prefix_padding_ms")]
    pub prefix_padding_ms: Option<i32>,
    /// Duration of silence to detect speech stop (in milliseconds). Defaults  to 500ms. With shorter values the model will respond more quickly,  but may jump in on short pauses from the user.
    #[serde(rename = "silence_duration_ms")]
    pub silence_duration_ms: Option<i32>,
    /// Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A  higher threshold will require louder audio to activate the model, and  thus might perform better in noisy environments.
    #[serde(rename = "threshold")]
    pub threshold: Option<f32>,
    /// Type of turn detection, only `server_vad` is currently supported.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
