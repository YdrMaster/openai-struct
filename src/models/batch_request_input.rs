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
/// BatchRequestInput:
///   type: object
///   description: The per-line object of the batch input file
///   properties:
///     custom_id:
///       type: string
///       description:
///         A developer-provided per-request id that will be used to match
///         outputs to inputs. Must be unique for each request in a batch.
///     method:
///       type: string
///       enum:
///         - POST
///       description:
///         The HTTP method to be used for the request. Currently only `POST`
///         is supported.
///       x-stainless-const: true
///     url:
///       type: string
///       description:
///         The OpenAI API relative URL to be used for the request. Currently
///         `/v1/chat/completions`, `/v1/embeddings`, and `/v1/completions` are
///         supported.
///   x-oaiMeta:
///     name: The request input object
///     example: >
///       {"custom_id": "request-1", "method": "POST", "url":
///       "/v1/chat/completions", "body": {"model": "gpt-4o-mini", "messages":
///       [{"role": "system", "content": "You are a helpful assistant."},
///       {"role": "user", "content": "What is 2+2?"}]}}
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequestInput {
    /// A developer-provided per-request id that will be used to match outputs to inputs. Must be unique for each request in a batch.
    #[serde(rename = "custom_id")]
    pub custom_id: Option<String>,
    /// The HTTP method to be used for the request. Currently only `POST` is supported.
    #[serde(rename = "method")]
    pub method: Option<String>,
    /// The OpenAI API relative URL to be used for the request. Currently `/v1/chat/completions`, `/v1/embeddings`, and `/v1/completions` are supported.
    #[serde(rename = "url")]
    pub url: Option<String>,
}
