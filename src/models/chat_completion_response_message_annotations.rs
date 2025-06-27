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
pub struct ChatCompletionResponseMessageAnnotations {
    /// The type of the URL citation. Always `url_citation`.
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "url_citation")]
    pub url_citation: crate::models::ChatCompletionResponseMessageUrlCitation,
}
