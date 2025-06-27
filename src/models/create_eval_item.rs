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
/// CreateEvalItem:
///   title: CreateEvalItem
///   description:
///     A chat message that makes up the prompt or context. May include
///     variable references to the "item" namespace, ie {{item.name}}.
///   type: object
///   oneOf:
///     - type: object
///       title: SimpleInputMessage
///       properties:
///         role:
///           type: string
///           description: The role of the message (e.g. "system", "assistant", "user").
///         content:
///           type: string
///           description: The content of the message.
///       required:
///         - role
///         - content
///     - $ref: "#/components/schemas/EvalItem"
///   x-oaiMeta:
///     name: The chat message object used to configure an individual run
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum CreateEvalItem {
    ItemObject(ItemObject),
    EvalItem(crate::EvalItem),
}

/// # on openapi.yaml
///
/// ```yaml
/// - type: object
///   title: SimpleInputMessage
///   properties:
///     role:
///       type: string
///       description: The role of the message (e.g. "system", "assistant", "user").
///     content:
///       type: string
///       description: The content of the message.
///   required:
///     - role
///     - content
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemObject {
    /// The role of the message (e.g. "system", "assistant", "user").
    role: String,
    /// The content of the message.
    content: String,
}
