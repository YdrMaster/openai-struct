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
pub struct CreateTranscriptionResponseJsonLogprobs {
    /// The bytes of the token.
    #[serde(rename = "bytes")]
    pub bytes: Option<Vec<f32>>,
    /// The log probability of the token.
    #[serde(rename = "logprob")]
    pub logprob: Option<f32>,
    /// The token in the transcription.
    #[serde(rename = "token")]
    pub token: Option<String>,
}
