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
pub struct ResponseError {
    #[serde(rename = "code")]
    pub code: crate::models::ResponseErrorCode,
    /// A human-readable description of the error.
    #[serde(rename = "message")]
    pub message: String,
}
