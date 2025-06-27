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
/// DoneEvent:
///   type: object
///   properties:
///     event:
///       type: string
///       enum:
///         - done
///       x-stainless-const: true
///     data:
///       type: string
///       enum:
///         - "[DONE]"
///       x-stainless-const: true
///   required:
///     - event
///     - data
///   description: Occurs when a stream ends.
///   x-oaiMeta:
///     dataDescription: "`data` is `[DONE]`"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct DoneEvent {
    /// enum: done
    #[serde(rename = "data")]
    pub data: String,
    /// enum: "\[DONE]"
    #[serde(rename = "event")]
    pub event: String,
}
