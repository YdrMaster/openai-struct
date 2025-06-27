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
pub struct ImagesResponseUsage {
    /// The number of tokens (images and text) in the input prompt.
    #[serde(rename = "input_tokens")]
    pub input_tokens: i32,
    #[serde(rename = "input_tokens_details")]
    pub input_tokens_details: crate::models::ImagesResponseUsageInputTokensDetails,
    /// The number of image tokens in the output image.
    #[serde(rename = "output_tokens")]
    pub output_tokens: i32,
    /// The total number of tokens (images and text) used for the image generation.
    #[serde(rename = "total_tokens")]
    pub total_tokens: i32,
}
