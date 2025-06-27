/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartAudioInputAudio {
    /// Base64 encoded audio data.
    #[serde(rename = "data")]
    pub data: String,
    /// The format of the encoded audio data. Currently, supports \"wav\" and \"mp3\".
    #[serde(rename = "format")]
    pub format: String,
}
