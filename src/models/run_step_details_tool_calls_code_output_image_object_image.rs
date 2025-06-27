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
pub struct RunStepDetailsToolCallsCodeOutputImageObjectImage {
    /// The [file](/docs/api-reference/files) ID of the image.
    #[serde(rename = "file_id")]
    pub file_id: String,
}
