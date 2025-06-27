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
/// BatchRequestOutput:
///   type: object
///   description: The per-line object of the batch output and error files
///   properties:
///     id:
///       type: string
///     custom_id:
///       type: string
///       description:
///         A developer-provided per-request id that will be used to match
///         outputs to inputs.
///     response:
///       type: object
///       nullable: true
///       properties:
///         status_code:
///           type: integer
///           description: The HTTP status code of the response
///         request_id:
///           type: string
///           description:
///             An unique identifier for the OpenAI API request. Please include
///             this request ID when contacting support.
///         body:
///           type: object
///           x-oaiTypeLabel: map
///           description: The JSON body of the response
///     error:
///       type: object
///       nullable: true
///       description:
///         For requests that failed with a non-HTTP error, this will contain
///         more information on the cause of the failure.
///       properties:
///         code:
///           type: string
///           description: A machine-readable error code.
///         message:
///           type: string
///           description: A human-readable error message.
///   x-oaiMeta:
///     name: The request output object
///     example: >
///       {"id": "batch_req_wnaDys", "custom_id": "request-2", "response":
///       {"status_code": 200, "request_id": "req_c187b3", "body": {"id":
///       "chatcmpl-9758Iw", "object": "chat.completion", "created": 1711475054,
///       "model": "gpt-4o-mini", "choices": [{"index": 0, "message": {"role":
///       "assistant", "content": "2 + 2 equals 4."}, "finish_reason": "stop"}],
///       "usage": {"prompt_tokens": 24, "completion_tokens": 15,
///       "total_tokens": 39}, "system_fingerprint": null}}, "error": null}
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequestOutput {
    /// A developer-provided per-request id that will be used to match outputs to inputs.
    #[serde(rename = "custom_id")]
    pub custom_id: Option<String>,
    #[serde(rename = "error")]
    pub error: Option<crate::models::BatchRequestOutputError>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "response")]
    pub response: Option<crate::models::BatchRequestOutputResponse>,
}
