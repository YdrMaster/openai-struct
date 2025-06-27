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
pub struct EvalRunOutputItemSample {
    #[serde(rename = "error")]
    pub error: crate::models::EvalApiError,
    /// The reason why the sample generation was finished.
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    /// An array of input messages.
    #[serde(rename = "input")]
    pub input: Vec<crate::models::EvalRunOutputItemSampleInput>,
    /// The maximum number of tokens allowed for completion.
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: i32,
    /// The model used for generating the sample.
    #[serde(rename = "model")]
    pub model: String,
    /// An array of output messages.
    #[serde(rename = "output")]
    pub output: Vec<crate::models::EvalRunOutputItemSampleOutput>,
    /// The seed used for generating the sample.
    #[serde(rename = "seed")]
    pub seed: i32,
    /// The sampling temperature used.
    #[serde(rename = "temperature")]
    pub temperature: f32,
    /// The top_p value used for sampling.
    #[serde(rename = "top_p")]
    pub top_p: f32,
    #[serde(rename = "usage")]
    pub usage: crate::models::EvalRunOutputItemSampleUsage,
}
