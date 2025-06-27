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
/// WebSearchContextSize:
///   type: string
///   description: >
///     High level guidance for the amount of context window space to use for
///     the
///
///     search. One of `low`, `medium`, or `high`. `medium` is the default.
///   enum:
///     - low
///     - medium
///     - high
///   default: medium
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchContextSize {
    Low,
    Medium,
    High,
}

impl Default for WebSearchContextSize {
    fn default() -> WebSearchContextSize {
        Self::Medium
    }
}
