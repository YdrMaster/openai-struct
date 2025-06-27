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
/// CreateEmbeddingRequest:
///   type: object
///   additionalProperties: false
///   properties:
///     input:
///       description: |
///         Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https:///cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. Some models may also impose a limit on total number of tokens summed across inputs.
///       example: The quick brown fox jumped over the lazy dog
///       oneOf:
///         - type: string
///           title: string
///           description: The string that will be turned into an embedding.
///           default: ""
///           example: This is a test.
///         - type: array
///           title: array
///           description: The array of strings that will be turned into an embedding.
///           minItems: 1
///           maxItems: 2048
///           items:
///             type: string
///             default: ""
///             example: "['This is a test.']"
///         - type: array
///           title: array
///           description: The array of integers that will be turned into an embedding.
///           minItems: 1
///           maxItems: 2048
///           items:
///             type: integer
///           example: "[1212, 318, 257, 1332, 13]"
///         - type: array
///           title: array
///           description:
///             The array of arrays containing integers that will be turned into an
///             embedding.
///           minItems: 1
///           maxItems: 2048
///           items:
///             type: array
///             minItems: 1
///             items:
///               type: integer
///           example: "[[1212, 318, 257, 1332, 13]]"
///     model:
///       description: >
///         ID of the model to use. You can use the [List
///         models](/docs/api-reference/models/list) API to see all of your
///         available models, or see our [Model overview](/docs/models) for
///         descriptions of them.
///       example: text-embedding-3-small
///       anyOf:
///         - type: string
///         - type: string
///           enum:
///             - text-embedding-ada-002
///             - text-embedding-3-small
///             - text-embedding-3-large
///       x-oaiTypeLabel: string
///     encoding_format:
///       description:
///         The format to return the embeddings in. Can be either `float` or
///         [`base64`](https:///pypi.org/project/pybase64/).
///       example: float
///       default: float
///       type: string
///       enum:
///         - float
///         - base64
///     dimensions:
///       description: >
///         The number of dimensions the resulting output embeddings should
///         have. Only supported in `text-embedding-3` and later models.
///       type: integer
///       minimum: 1
///     user:
///       type: string
///       example: user-1234
///       description: >
///         A unique identifier representing your end-user, which can help
///         OpenAI to monitor and detect abuse. [Learn
///         more](/docs/guides/safety-best-practices#end-user-ids).
///   required:
///     - model
///     - input
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmbeddingRequest {
    /// The number of dimensions the resulting output embeddings should have. Only supported in `text-embedding-3` and later models.
    #[serde(rename = "dimensions")]
    pub dimensions: Option<i32>,
    /// The format to return the embeddings in. Can be either `float` or [`base64`](https:///pypi.org/project/pybase64/).
    #[serde(rename = "encoding_format")]
    pub encoding_format: Option<String>,
    /// Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or fewer. [Example Python code](https:///cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. Some models may also impose a limit on total number of tokens summed across inputs.
    #[serde(rename = "input")]
    pub input: CreateEmbeddingRequestInput,
    /// ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.
    ///
    /// - text-embedding-ada-002
    /// - text-embedding-3-small
    /// - text-embedding-3-large
    #[serde(rename = "model")]
    pub model: String,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).
    #[serde(rename = "user")]
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmbeddingRequestInput {
    /// The string that will be turned into an embedding. example: This is a test.
    Text(String),
    /// The array of strings that will be turned into an embedding. example: "['This is a test.']"
    Texts(Vec<String>),
    /// The array of integers that will be turned into an embedding. example: "[1212, 318, 257, 1332, 13]"
    Embedding(Vec<isize>),
    /// The array of arrays containing integers that will be turned into an embedding. example: "[[1212, 318, 257, 1332, 13]]"
    Embeddings(Vec<Vec<isize>>),
}
