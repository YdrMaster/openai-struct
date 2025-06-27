/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
/// ComputerCallSafetyCheckParam:
///   properties:
///     id:
///       type: string
///       description: The ID of the pending safety check.
///     code:
///       anyOf:
///         - type: string
///           description: The type of the pending safety check.
///         - type: "null"
///     message:
///       anyOf:
///         - type: string
///           description: Details about the pending safety check.
///         - type: "null"
///   type: object
///   required:
///     - id
///   description: A pending safety check for the computer call.
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerCallSafetyCheckParam {
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// The ID of the pending safety check.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "message")]
    pub message: Option<String>,
}
