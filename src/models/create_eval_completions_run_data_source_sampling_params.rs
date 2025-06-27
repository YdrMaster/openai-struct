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
pub struct CreateEvalCompletionsRunDataSourceSamplingParams {
    /// The maximum number of tokens in the generated output.
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: Option<i32>,
    /// A seed value to initialize the randomness, during sampling.
    #[serde(rename = "seed")]
    pub seed: Option<i32>,
    /// A higher temperature increases randomness in the outputs.
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,
    /// An alternative to temperature for nucleus sampling; 1.0 includes all tokens.
    #[serde(rename = "top_p")]
    pub top_p: Option<f32>,
}
