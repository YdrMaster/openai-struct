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
pub struct FineTuningJobError {
    /// A machine-readable error code.
    #[serde(rename = "code")]
    pub code: String,
    /// A human-readable error message.
    #[serde(rename = "message")]
    pub message: String,
    /// The parameter that was invalid, usually `training_file` or `validation_file`. This field will be null if the failure was not parameter-specific.
    #[serde(rename = "param")]
    pub param: String,
}
