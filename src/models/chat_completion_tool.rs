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
/// ChatCompletionTool:
///   type: object
///   properties:
///     type:
///       type: string
///       enum:
///         - function
///       description: The type of the tool. Currently, only `function` is supported.
///       x-stainless-const: true
///     function:
///       $ref: "#/components/schemas/FunctionObject"
///   required:
///     - type
///     - function
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionTool {
    #[serde(rename = "function")]
    pub function: crate::models::FunctionObject,
    /// The type of the tool. Currently, only `function` is supported.
    #[serde(rename = "type")]
    pub _type: String,
}
