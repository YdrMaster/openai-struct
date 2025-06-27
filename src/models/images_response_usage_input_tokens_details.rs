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
pub struct ImagesResponseUsageInputTokensDetails {
    /// The number of image tokens in the input prompt.
    #[serde(rename = "image_tokens")]
    pub image_tokens: i32,
    /// The number of text tokens in the input prompt.
    #[serde(rename = "text_tokens")]
    pub text_tokens: i32,
}
