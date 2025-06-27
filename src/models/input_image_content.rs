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
pub struct InputImageContent {
    /// The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`.
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    /// The type of the input item. Always `input_image`.
    #[serde(rename = "type")]
    pub _type: String,
}
