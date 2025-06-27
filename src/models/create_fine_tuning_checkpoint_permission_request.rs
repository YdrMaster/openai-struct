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
/// CreateFineTuningCheckpointPermissionRequest:
///   type: object
///   additionalProperties: false
///   properties:
///     project_ids:
///       type: array
///       description: The project identifiers to grant access to.
///       items:
///         type: string
///   required:
///     - project_ids
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFineTuningCheckpointPermissionRequest {
    /// The project identifiers to grant access to.
    #[serde(rename = "project_ids")]
    pub project_ids: Vec<String>,
}
