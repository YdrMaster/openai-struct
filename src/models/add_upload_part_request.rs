/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml:
///
/// ```yaml
/// AddUploadPartRequest:
///   type: object
///   additionalProperties: false
///   properties:
///     data:
///       description: |
///         The chunk of bytes for this Part.
///       type: string
///       format: binary
///   required:
///     - data
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AddUploadPartRequest {
    /// The chunk of bytes for this Part.
    #[serde(rename = "data")]
    pub data: Vec<u8>,
}
