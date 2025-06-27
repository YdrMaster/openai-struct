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
/// CodeInterpreterFileOutput:
///   type: object
///   title: Code interpreter file output
///   description: |
///     The output of a code interpreter tool call that is a file.
///   properties:
///     type:
///       type: string
///       enum:
///         - files
///       description: |
///         The type of the code interpreter file output. Always `files`.
///       x-stainless-const: true
///     files:
///       type: array
///       items:
///         type: object
///         properties:
///           mime_type:
///             type: string
///             description: |
///               The MIME type of the file.
///           file_id:
///             type: string
///             description: |
///               The ID of the file.
///         required:
///           - mime_type
///           - file_id
///   required:
///     - type
///     - files
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeInterpreterFileOutput {
    #[serde(rename = "files")]
    pub files: Vec<crate::models::CodeInterpreterFileOutputFiles>,
    /// The type of the code interpreter file output. Always `files`.
    #[serde(rename = "type")]
    pub _type: String,
}
