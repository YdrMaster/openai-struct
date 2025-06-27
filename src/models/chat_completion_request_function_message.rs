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
/// ChatCompletionRequestFunctionMessage:
///   type: object
///   title: Function message
///   deprecated: true
///   properties:
///     role:
///       type: string
///       enum:
///         - function
///       description: The role of the messages author, in this case `function`.
///       x-stainless-const: true
///     content:
///       nullable: true
///       type: string
///       description: The contents of the function message.
///     name:
///       type: string
///       description: The name of the function to call.
///   required:
///     - role
///     - content
///     - name
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestFunctionMessage {
    /// The contents of the function message.
    #[serde(rename = "content")]
    pub content: String,
    /// The name of the function to call.
    #[serde(rename = "name")]
    pub name: String,
}
