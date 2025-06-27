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
pub struct CreateTranslationRequest {
    /// The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    #[serde(rename = "file")]
    pub file: Vec<u8>,
    /// ID of the model to use. Only `whisper-1` (which is powered by our open source Whisper V2 model) is currently available.
    /// enum: - whisper-1
    #[serde(rename = "model")]
    pub model: String,
    /// An optional text to guide the model's style or continue a previous audio segment. The [prompt](/docs/guides/speech-to-text#prompting) should be in English.
    #[serde(rename = "prompt")]
    pub prompt: Option<String>,
    /// The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.
    #[serde(rename = "response_format")]
    pub response_format: Option<String>,
    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,
}
