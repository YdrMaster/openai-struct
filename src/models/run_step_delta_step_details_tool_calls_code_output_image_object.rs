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
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
    #[serde(rename = "image")]
    pub image: Option<crate::models::RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage>,
    /// The index of the output in the outputs array.
    #[serde(rename = "index")]
    pub index: i32,
    /// Always `image`.
    #[serde(rename = "type")]
    pub _type: String,
}
