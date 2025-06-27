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
pub struct MessageDeltaContentTextAnnotationsFileCitationObject {
    #[serde(rename = "end_index")]
    pub end_index: Option<i32>,
    #[serde(rename = "file_citation")]
    pub file_citation:
        Option<crate::models::MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation>,
    /// The index of the annotation in the text content part.
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "start_index")]
    pub start_index: Option<i32>,
    /// The text in the message content that needs to be replaced.
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// Always `file_citation`.
    #[serde(rename = "type")]
    pub _type: String,
}
