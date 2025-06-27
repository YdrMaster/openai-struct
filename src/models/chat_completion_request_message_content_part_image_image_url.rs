/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartImageImageUrl {
    /// Specifies the detail level of the image. Learn more in the [Vision guide](/docs/guides/vision#low-or-high-fidelity-image-understanding).
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// Either a URL of the image or the base64 encoded image data.
    #[serde(rename = "url")]
    pub url: String,
}
