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
pub struct ReasoningItemSummary {
    /// A short summary of the reasoning used by the model when generating the response.
    #[serde(rename = "text")]
    pub text: String,
    /// The type of the object. Always `summary_text`.
    #[serde(rename = "type")]
    pub _type: String,
}
