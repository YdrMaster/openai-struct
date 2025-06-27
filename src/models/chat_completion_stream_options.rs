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
/// ChatCompletionStreamOptions:
///   description: >
///     Options for streaming response. Only set this when you set `stream:
///     true`.
///   type: object
///   nullable: true
///   default: null
///   properties:
///     include_usage:
///       type: boolean
///       description: >
///         If set, an additional chunk will be streamed before the `data:
///         [DONE]`
///
///         message. The `usage` field on this chunk shows the token usage
///         statistics
///
///         for the entire request, and the `choices` field will always be an
///         empty
///
///         array.
///
///
///         All other chunks will also include a `usage` field, but with a null
///
///         value. **NOTE:** If the stream is interrupted, you may not receive
///         the
///
///         final usage chunk which contains the total token usage for the
///         request.
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionStreamOptions {
    /// If set, an additional chunk will be streamed before the `data: [DONE]` message. The `usage` field on this chunk shows the token usage statistics for the entire request, and the `choices` field will always be an empty array.   All other chunks will also include a `usage` field, but with a null value. **NOTE:** If the stream is interrupted, you may not receive the final usage chunk which contains the total token usage for the request.
    #[serde(rename = "include_usage")]
    pub include_usage: Option<bool>,
}
