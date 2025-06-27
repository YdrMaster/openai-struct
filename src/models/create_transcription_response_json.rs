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
pub struct CreateTranscriptionResponseJson {
    /// The log probabilities of the tokens in the transcription. Only returned with the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe` if `logprobs` is added to the `include` array.
    #[serde(rename = "logprobs")]
    pub logprobs: Option<Vec<crate::models::CreateTranscriptionResponseJsonLogprobs>>,
    /// The transcribed text.
    #[serde(rename = "text")]
    pub text: String,
}
