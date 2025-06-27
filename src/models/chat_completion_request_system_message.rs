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
/// ChatCompletionRequestSystemMessage:
///   type: object
///   title: System message
///   description: >
///     Developer-provided instructions that the model should follow, regardless
///     of
///
///     messages sent by the user. With o1 models and newer, use `developer`
///     messages
///
///     for this purpose instead.
///   properties:
///     content:
///       description: The contents of the system message.
///       oneOf:
///         - type: string
///           description: The contents of the system message.
///           title: Text content
///         - type: array
///           description:
///             An array of content parts with a defined type. For system messages,
///             only type `text` is supported.
///           title: Array of content parts
///           items:
///             $ref: "#/components/schemas/ChatCompletionRequestSystemMessageContentPart"
///           minItems: 1
///     role:
///       type: string
///       enum:
///         - system
///       description: The role of the messages author, in this case `system`.
///       x-stainless-const: true
///     name:
///       type: string
///       description: An optional name for the participant. Provides the model
///         information to differentiate between participants of the same role.
///   required:
///     - content
///     - role
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestSystemMessage {
    /// The contents of the system message.
    #[serde(rename = "content")]
    pub content: ChatCompletionRequestSystemMessageContent,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContent {
    Text(String),
    Array(Vec<crate::ChatCompletionRequestSystemMessageContentPart>),
}
