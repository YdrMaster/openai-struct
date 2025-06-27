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
/// AssistantToolsCode:
///   type: object
///   title: Code interpreter tool
///   properties:
///     type:
///       type: string
///       description: "The type of tool being defined: `code_interpreter`"
///       enum:
///         - code_interpreter
///       x-stainless-const: true
///   required:
///     - type
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantToolsCode {
    /// The type of tool being defined: `code_interpreter`
    #[serde(rename = "type")]
    pub _type: String,
}
