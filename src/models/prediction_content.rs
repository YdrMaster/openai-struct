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
/// PredictionContent:
///   type: object
///   title: Static Content
///   description: >
///     Static predicted output content, such as the content of a text file that
///     is
///
///     being regenerated.
///   required:
///     - type
///     - content
///   properties:
///     type:
///       type: string
///       enum:
///         - content
///       description: |
///         The type of the predicted content you want to provide. This type is
///         currently always `content`.
///       x-stainless-const: true
///     content:
///       description: >
///         The content that should be matched when generating a model response.
///
///         If generated tokens would match this content, the entire model
///         response
///
///         can be returned much more quickly.
///       oneOf:
///         - type: string
///           title: Text content
///           description: |
///             The content used for a Predicted Output. This is often the
///             text of a file you are regenerating with minor changes.
///         - type: array
///           description:
///             An array of content parts with a defined type. Supported options
///             differ based on the [model](/docs/models) being used to generate
///             the response. Can contain text inputs.
///           title: Array of content parts
///           items:
///             $ref: "#/components/schemas/ChatCompletionRequestMessageContentPartText"
///           minItems: 1
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionContent {
    /// The content that should be matched when generating a model response. If generated tokens would match this content, the entire model response can be returned much more quickly.
    #[serde(rename = "content")]
    pub content: PredictionContentContent,
    /// The type of the predicted content you want to provide. This type is currently always `content`.
    #[serde(rename = "type")]
    pub _type: String,
}

/// # on openai.yaml
///
/// ```yaml
/// oneOf:
///   - type: string
///     title: Text content
///     description: |
///       The content used for a Predicted Output. This is often the
///       text of a file you are regenerating with minor changes.
///   - type: array
///     description:
///       An array of content parts with a defined type. Supported options
///       differ based on the [model](/docs/models) being used to generate
///       the response. Can contain text inputs.
///     title: Array of content parts
///     items:
///       $ref: "#/components/schemas/ChatCompletionRequestMessageContentPartText"
///     minItems: 1
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PredictionContentContent {
    Text(String),
    Array(Vec<crate::ChatCompletionRequestMessageContentPartText>),
}
