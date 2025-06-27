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
pub struct MessageDeltaContentTextObject {
    /// The index of the content part in the message.
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "text")]
    pub text: Option<crate::models::MessageDeltaContentTextObjectText>,
    /// Always `text`.
    #[serde(rename = "type")]
    pub _type: String,
}
