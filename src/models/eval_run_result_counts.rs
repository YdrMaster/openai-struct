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
pub struct EvalRunResultCounts {
    /// Number of output items that resulted in an error.
    #[serde(rename = "errored")]
    pub errored: i32,
    /// Number of output items that failed to pass the evaluation.
    #[serde(rename = "failed")]
    pub failed: i32,
    /// Number of output items that passed the evaluation.
    #[serde(rename = "passed")]
    pub passed: i32,
    /// Total number of executed output items.
    #[serde(rename = "total")]
    pub total: i32,
}
