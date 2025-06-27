/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use crate::ThreadObject;

/// # on openapi.yaml
///
/// ```yaml
/// ThreadStreamEvent:
///   oneOf:
///     - type: object
///       properties:
///         enabled:
///           type: boolean
///           description: Whether to enable input audio transcription.
///         event:
///           type: string
///           enum:
///             - thread.created
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/ThreadObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a new [thread](/docs/api-reference/threads/object) is
///         created.
///       x-oaiMeta:
///         dataDescription: "`data` is a [thread](/docs/api-reference/threads/object)"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadStreamEvent {
    /// Whether to enable input audio transcription.
    pub enabled: Option<bool>,
    /// enum - thread.created
    pub event: String,
    /// Represents a thread that contains [messages](/docs/api-reference/messages).
    pub data: ThreadObject,
}
