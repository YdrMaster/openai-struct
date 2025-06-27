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
pub struct MessageContentImageUrlObjectImageUrl {
    /// Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. Default value is `auto`
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// The external URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp.
    #[serde(rename = "url")]
    pub url: String,
}
