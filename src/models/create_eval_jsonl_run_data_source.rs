/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

/// pub CreateEvalJsonlRunDataSource : A JsonlRunDataSource object with that specifies a JSONL file that matches the eval
use serde_json::Value;

/// # on openapi.yaml
///
/// ```yaml
/// CreateEvalJsonlRunDataSource:
///   type: object
///   title: JsonlRunDataSource
///   description: >
///     A JsonlRunDataSource object with that specifies a JSONL file that
///     matches the eval
///   properties:
///     type:
///       type: string
///       enum:
///         - jsonl
///       default: jsonl
///       description: The type of data source. Always `jsonl`.
///       x-stainless-const: true
///     source:
///       oneOf:
///         - $ref: "#/components/schemas/EvalJsonlFileContentSource"
///         - $ref: "#/components/schemas/EvalJsonlFileIdSource"
///   required:
///     - type
///     - source
///   x-oaiMeta:
///     name: The file data source object for the eval run configuration
///     group: evals
///     example: |
///       {
///         "type": "jsonl",
///         "source": {
///           "type": "file_id",
///           "id": "file-9GYS6xbkWgWhmE7VoLUWFg"
///         }
///       }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEvalJsonlRunDataSource {
    #[serde(rename = "source")]
    pub source: Value,
    /// The type of data source. Always `jsonl`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "jsonl".into()
}
