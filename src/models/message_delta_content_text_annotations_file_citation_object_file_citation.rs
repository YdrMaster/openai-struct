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
pub struct MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
    /// The ID of the specific File the citation is from.
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    /// The specific quote in the file.
    #[serde(rename = "quote")]
    pub quote: Option<String>,
}
