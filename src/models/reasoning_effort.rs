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
///     ReasoningEffort:
///       type: string
///       enum:
///         - low
///         - medium
///         - high
///       default: medium
///       nullable: true
///       description: |
///         **o-series models only**
///
///         Constrains effort on reasoning for
///         [reasoning models](https://platform.openai.com/docs/guides/reasoning).
///         Currently supported values are `low`, `medium`, and `high`. Reducing
///         reasoning effort can result in faster responses and fewer tokens used
///         on reasoning in a response.
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Low,
    Medium,
    High,
}

impl Default for ReasoningEffort {
    fn default() -> Self {
        Self::Medium
    }
}
