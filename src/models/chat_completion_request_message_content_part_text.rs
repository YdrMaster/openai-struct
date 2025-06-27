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
/// ChatCompletionRequestMessageContentPartText:
///   type: object
///   title: Text content part
///   description: |
///     Learn about [text inputs](/docs/guides/text-generation).
///   properties:
///     type:
///       type: string
///       enum:
///         - text
///       description: The type of the content part.
///       x-stainless-const: true
///     text:
///       type: string
///       description: The text content.
///   required:
///     - type
///     - text
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartText {
    /// The text content.
    #[serde(rename = "text")]
    pub text: String,
    /// The type of the content part.
    #[serde(rename = "type")]
    pub _type: String,
}
