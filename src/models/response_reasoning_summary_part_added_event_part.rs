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
pub struct ResponseReasoningSummaryPartAddedEventPart {
    /// The text of the summary part.
    #[serde(rename = "text")]
    pub text: String,
    /// The type of the summary part. Always `summary_text`.
    #[serde(rename = "type")]
    pub _type: String,
}
