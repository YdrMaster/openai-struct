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
pub struct MessageContentTextObjectText {
    #[serde(rename = "annotations")]
    pub annotations: Vec<MessageContentTextObjectTextAnnotations>,
    /// The data that makes up the text.
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContentTextObjectTextAnnotations {
    FileCitationObject(crate::MessageContentTextAnnotationsFileCitationObject),
    FilePathObject(crate::MessageContentTextAnnotationsFilePathObject),
}
