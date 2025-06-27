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
pub struct LogProbProperties {
    /// The bytes that were used to generate the log probability.
    #[serde(rename = "bytes")]
    pub bytes: Vec<i32>,
    /// The log probability of the token.
    #[serde(rename = "logprob")]
    pub logprob: f32,
    /// The token that was used to generate the log probability.
    #[serde(rename = "token")]
    pub token: String,
}
