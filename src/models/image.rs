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
pub struct Image {
    /// The base64-encoded JSON of the generated image. Default value for `gpt-image-1`, and only present if `response_format` is set to `b64_json` for `dall-e-2` and `dall-e-3`.
    #[serde(rename = "b64_json")]
    pub b64_json: Option<String>,
    /// For `dall-e-3` only, the revised prompt that was used to generate the image.
    #[serde(rename = "revised_prompt")]
    pub revised_prompt: Option<String>,
    /// When using `dall-e-2` or `dall-e-3`, the URL of the generated image if `response_format` is set to `url` (default value). Unsupported for `gpt-image-1`.
    #[serde(rename = "url")]
    pub url: Option<String>,
}
