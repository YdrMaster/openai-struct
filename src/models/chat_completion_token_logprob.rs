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
/// ChatCompletionTokenLogprob:
///   type: object
///   properties:
///     token:
///       description: The token.
///       type: string
///     logprob:
///       description:
///         The log probability of this token, if it is within the top 20 most
///         likely tokens. Otherwise, the value `-9999.0` is used to signify
///         that the token is very unlikely.
///       type: number
///     bytes:
///       description:
///         A list of integers representing the UTF-8 bytes representation of
///         the token. Useful in instances where characters are represented by
///         multiple tokens and their byte representations must be combined to
///         generate the correct text representation. Can be `null` if there is
///         no bytes representation for the token.
///       type: array
///       items:
///         type: integer
///       nullable: true
///     top_logprobs:
///       description:
///         List of the most likely tokens and their log probability, at this
///         token position. In rare cases, there may be fewer than the number of
///         requested `top_logprobs` returned.
///       type: array
///       items:
///         type: object
///         properties:
///           token:
///             description: The token.
///             type: string
///           logprob:
///             description:
///               The log probability of this token, if it is within the top 20 most
///               likely tokens. Otherwise, the value `-9999.0` is used to
///               signify that the token is very unlikely.
///             type: number
///           bytes:
///             description:
///               A list of integers representing the UTF-8 bytes representation of
///               the token. Useful in instances where characters are
///               represented by multiple tokens and their byte representations
///               must be combined to generate the correct text representation.
///               Can be `null` if there is no bytes representation for the
///               token.
///             type: array
///             items:
///               type: integer
///             nullable: true
///         required:
///           - token
///           - logprob
///           - bytes
///   required:
///     - token
///     - logprob
///     - bytes
///     - top_logprobs
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionTokenLogprob {
    /// A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.
    #[serde(rename = "bytes")]
    pub bytes: Vec<i32>,
    /// The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely.
    #[serde(rename = "logprob")]
    pub logprob: f32,
    /// The token.
    #[serde(rename = "token")]
    pub token: String,
    /// List of the most likely tokens and their log probability, at this token position. In rare cases, there may be fewer than the number of requested `top_logprobs` returned.
    #[serde(rename = "top_logprobs")]
    pub top_logprobs: Vec<crate::models::ChatCompletionTokenLogprobTopLogprobs>,
}
