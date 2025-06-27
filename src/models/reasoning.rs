/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct Reasoning {
    #[serde(rename = "effort")]
    pub effort: Option<crate::models::ReasoningEffort>,
    /// **Deprecated:** use `summary` instead.  A summary of the reasoning performed by the model. This can be useful for debugging and understanding the model's reasoning process. One of `auto`, `concise`, or `detailed`.
    #[serde(rename = "generate_summary")]
    pub generate_summary: Option<String>,
    /// A summary of the reasoning performed by the model. This can be useful for debugging and understanding the model's reasoning process. One of `auto`, `concise`, or `detailed`.
    #[serde(rename = "summary")]
    pub summary: Option<String>,
}
