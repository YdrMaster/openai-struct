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
/// ErrorEvent:
///   type: object
///   properties:
///     event:
///       type: string
///       enum:
///         - error
///       x-stainless-const: true
///     data:
///       $ref: "#/components/schemas/Error"
///   required:
///     - event
///     - data
///   description:
///     Occurs when an [error](/docs/guides/error-codes#api-errors) occurs.
///     This can happen due to an internal server error or a timeout.
///   x-oaiMeta:
///     dataDescription: "`data` is an [error](/docs/guides/error-codes#api-errors)"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorEvent {
    #[serde(rename = "data")]
    pub data: crate::models::Error,
    #[serde(rename = "event")]
    pub event: String,
}
