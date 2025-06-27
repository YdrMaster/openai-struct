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
pub struct MessageDeltaContentImageFileObject {
    #[serde(rename = "image_file")]
    pub image_file: Option<crate::models::MessageDeltaContentImageFileObjectImageFile>,
    /// The index of the content part in the message.
    #[serde(rename = "index")]
    pub index: i32,
    /// Always `image_file`.
    #[serde(rename = "type")]
    pub _type: String,
}
