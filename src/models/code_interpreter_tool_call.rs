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
/// CodeInterpreterToolCall:
///   type: object
///   title: Code interpreter tool call
///   description: |
///     A tool call to run code.
///   properties:
///     id:
///       type: string
///       description: |
///         The unique ID of the code interpreter tool call.
///     type:
///       type: string
///       enum:
///         - code_interpreter_call
///       description: >
///         The type of the code interpreter tool call. Always
///         `code_interpreter_call`.
///       x-stainless-const: true
///     code:
///       type: string
///       description: |
///         The code to run.
///     status:
///       type: string
///       enum:
///         - in_progress
///         - interpreting
///         - completed
///       description: |
///         The status of the code interpreter tool call.
///     results:
///       type: array
///       items:
///         $ref: "#/components/schemas/CodeInterpreterToolOutput"
///       description: |
///         The results of the code interpreter tool call.
///   required:
///     - id
///     - type
///     - code
///     - status
///     - results
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeInterpreterToolCall {
    /// The code to run.
    #[serde(rename = "code")]
    pub code: String,
    /// The unique ID of the code interpreter tool call.
    #[serde(rename = "id")]
    pub id: String,
    /// The results of the code interpreter tool call.
    #[serde(rename = "results")]
    pub results: Vec<crate::models::CodeInterpreterToolOutput>,
    /// The status of the code interpreter tool call.
    #[serde(rename = "status")]
    pub status: String,
    /// The type of the code interpreter tool call. Always `code_interpreter_call`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "code_interpreter_call".into()
}
