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
pub struct MessageContentImageFileObject {
    #[serde(rename = "image_file")]
    pub image_file: crate::models::MessageContentImageFileObjectImageFile,
    /// Always `image_file`.
    #[serde(rename = "type")]
    pub _type: String,
}
