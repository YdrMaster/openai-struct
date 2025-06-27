/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpeechRequest {
    /// The text to generate audio for. The maximum length is 4096 characters.
    #[serde(rename = "input")]
    pub input: String,
    /// Control the voice of your generated audio with additional instructions. Does not work with `tts-1` or `tts-1-hd`.
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    /// One of the available [TTS models](/docs/models#tts): `tts-1`, `tts-1-hd` or `gpt-4o-mini-tts`.
    #[serde(rename = "model")]
    pub model: Value,
    /// The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`.
    #[serde(rename = "response_format")]
    pub response_format: Option<String>,
    /// The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default.
    #[serde(rename = "speed")]
    pub speed: Option<f32>,
    /// The voice to use when generating the audio. Supported voices are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer`, and `verse`. Previews of the voices are available in the [Text to speech guide](/docs/guides/text-to-speech#voice-options).
    #[serde(rename = "voice")]
    pub voice: crate::models::VoiceIdsShared,
}
