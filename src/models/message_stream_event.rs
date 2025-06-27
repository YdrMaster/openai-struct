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
/// MessageStreamEvent:
///   oneOf:
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.message.created
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/MessageObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [message](/docs/api-reference/messages/object) is
///         created.
///       x-oaiMeta:
///         dataDescription: "`data` is a [message](/docs/api-reference/messages/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.message.in_progress
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/MessageObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [message](/docs/api-reference/messages/object) moves
///         to an `in_progress` state.
///       x-oaiMeta:
///         dataDescription: "`data` is a [message](/docs/api-reference/messages/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.message.delta
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/MessageDeltaObject"
///       required:
///         - event
///         - data
///       description: Occurs when parts of a
///         [Message](/docs/api-reference/messages/object) are being streamed.
///       x-oaiMeta:
///         dataDescription: "`data` is a [message
///           delta](/docs/api-reference/assistants-streaming/message-delta-obj\
///           ect)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.message.completed
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/MessageObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [message](/docs/api-reference/messages/object) is
///         completed.
///       x-oaiMeta:
///         dataDescription: "`data` is a [message](/docs/api-reference/messages/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.message.incomplete
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/MessageObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [message](/docs/api-reference/messages/object) ends
///         before it is completed.
///       x-oaiMeta:
///         dataDescription: "`data` is a [message](/docs/api-reference/messages/object)"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStreamEvent {}

// todo: 之后挨个实现
