/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
/// ComputerToolCallSafetyCheck:
///   type: object
///   description: |
///     A pending safety check for the computer call.
///   properties:
///     id:
///       type: string
///       description: The ID of the pending safety check.
///     code:
///       type: string
///       description: The type of the pending safety check.
///     message:
///       type: string
///       description: Details about the pending safety check.
///   required:
///     - id
///     - code
///     - message
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerToolCallSafetyCheck {
    /// The type of the pending safety check.
    #[serde(rename = "code")]
    pub code: String,
    /// The ID of the pending safety check.
    #[serde(rename = "id")]
    pub id: String,
    /// Details about the pending safety check.
    #[serde(rename = "message")]
    pub message: String,
}
