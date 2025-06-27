/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

use serde_json::Value;

/// # on openapi.yaml
///
/// ```yaml
/// CreateEvalRunRequest:
///   type: object
///   title: CreateEvalRunRequest
///   properties:
///     name:
///       type: string
///       description: The name of the run.
///     metadata:
///       $ref: "#/components/schemas/Metadata"
///     data_source:
///       type: object
///       description: Details about the run's data source.
///       oneOf:
///         - $ref: "#/components/schemas/CreateEvalJsonlRunDataSource"
///         - $ref: "#/components/schemas/CreateEvalCompletionsRunDataSource"
///         - $ref: "#/components/schemas/CreateEvalResponsesRunDataSource"
///   required:
///     - data_source
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEvalRunRequest {
    /// Details about the run's data source.
    #[serde(rename = "data_source")]
    pub data_source: Value,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
    /// The name of the run.
    #[serde(rename = "name")]
    pub name: Option<String>,
}
