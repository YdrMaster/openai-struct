/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// # on openai.yaml
///
/// ```yaml
/// StopConfiguration:
///   description: |
///     Not supported with latest reasoning models `o3` and `o4-mini`.
///
///     Up to 4 sequences where the API will stop generating further tokens. The
///     returned text will not contain the stop sequence.
///   default: null
///   nullable: true
///   oneOf:
///     - type: string
///       default: <|endoftext|>
///       example: "\n"
///       nullable: true
///     - type: array
///       minItems: 1
///       maxItems: 4
///       items:
///         type: string
///         example: '["\n"]'
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopConfiguration {
    Text(String),
    Array(Vec<String>),
}
